param(
    [string]$ProjectRoot = "",
    [string]$ArtifactDir = "",
    [string]$PagesDir = "",
    [string]$RouteComparison = "",
    [int]$FailUnder = 90,
    [switch]$SkipArtifactLane
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Resolve-RepoRoot {
    param([string]$RequestedRoot)

    if (-not [string]::IsNullOrWhiteSpace($RequestedRoot)) {
        return (Resolve-Path -LiteralPath $RequestedRoot).Path
    }

    return (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..\..")).Path
}

function Invoke-CiStep {
    param(
        [string]$Name,
        [string]$FilePath,
        [string[]]$Arguments,
        [string]$WorkingDirectory
    )

    Write-Host ""
    Write-Host "==> $Name"
    Push-Location -LiteralPath $WorkingDirectory
    try {
        & $FilePath @Arguments
        if ($LASTEXITCODE -ne 0) {
            throw "$Name failed with exit code $LASTEXITCODE"
        }
    } finally {
        Pop-Location
    }
}

function Resolve-RepoOutputPath {
    param(
        [string]$RepoRoot,
        [string]$RequestedPath,
        [string]$DefaultRelativePath
    )

    $path = if ([string]::IsNullOrWhiteSpace($RequestedPath)) {
        Join-Path $RepoRoot $DefaultRelativePath
    } else {
        if ([System.IO.Path]::IsPathRooted($RequestedPath)) {
            $RequestedPath
        } else {
            Join-Path $RepoRoot $RequestedPath
        }
    }

    return [System.IO.Path]::GetFullPath($path)
}

function Resolve-DxWwwExecutable {
    param([string]$RepoRoot)

    $exeName = "dx-www"
    if ($env:OS -eq "Windows_NT") {
        $exeName = "dx-www.exe"
    }

    $candidate = Join-Path $RepoRoot (Join-Path "www\target\debug" $exeName)
    if (-not (Test-Path -LiteralPath $candidate)) {
        throw "Cannot find built dx-www executable at $candidate"
    }

    return (Resolve-Path -LiteralPath $candidate).Path
}

function Assert-ForgePublicRouteComparison {
    param([string]$Path)

    if (-not (Test-Path -LiteralPath $Path)) {
        throw "Cannot find Forge public route comparison JSON at $Path"
    }

    $requiredRoutes = @(
        "/forge",
        "/forge/scorecard",
        "/forge/ci",
        "/forge/evidence",
        "/forge/releases",
        "/forge/changelog",
        "/forge/adoption"
    )
    $report = Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json
    $routes = @($report.routes)
    $routeCount = $routes.Count
    if ($report.PSObject.Properties.Name -contains "route_count" -and $null -ne $report.route_count) {
        $routeCount = [int]$report.route_count
    }

    $issues = New-Object System.Collections.Generic.List[string]
    if ($routeCount -lt $requiredRoutes.Count) {
        $issues.Add("route_count is $routeCount; expected at least $($requiredRoutes.Count)")
    }

    foreach ($required in $requiredRoutes) {
        $route = $routes | Where-Object { $_.route -eq $required } | Select-Object -First 1
        if ($null -eq $route) {
            $issues.Add("missing public route $required")
            continue
        }

        if ($route.status -ne "measured") {
            $issues.Add("public route $required has status $($route.status)")
        }
        if ($route.route_delivery -ne "static") {
            $issues.Add("public route $required is not static")
        }
        if ($route.budget_passed -ne $true) {
            $issues.Add("public route $required failed its route budget")
        }
    }

    if ($issues.Count -gt 0) {
        throw "Forge public route comparison guard failed: $($issues -join '; ')"
    }
}

function Copy-RequiredFile {
    param(
        [string]$Source,
        [string]$Destination
    )

    if (-not (Test-Path -LiteralPath $Source)) {
        throw "Cannot find required file at $Source"
    }

    $parent = Split-Path -Parent $Destination
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Force -Path $parent | Out-Null
    }
    Copy-Item -LiteralPath $Source -Destination $Destination -Force
}

$root = Resolve-RepoRoot -RequestedRoot $ProjectRoot
$manifest = Join-Path $root "Cargo.toml"
$artifactRoot = Resolve-RepoOutputPath -RepoRoot $root -RequestedPath $ArtifactDir -DefaultRelativePath ".dx\ci"
$routeComparisonPath = Resolve-RepoOutputPath -RepoRoot $root -RequestedPath $RouteComparison -DefaultRelativePath "benchmarks\reports\forge-public-route-comparison.json"
$routeComparisonMarkdownPath = [System.IO.Path]::ChangeExtension($routeComparisonPath, ".md")
$pagesRoot = if ([string]::IsNullOrWhiteSpace($PagesDir)) {
    ""
} else {
    Resolve-RepoOutputPath -RepoRoot $root -RequestedPath $PagesDir -DefaultRelativePath ".dx\forge-pages"
}

if (-not (Test-Path -LiteralPath $manifest)) {
    throw "Cannot find Cargo manifest at $manifest"
}

Set-Location $root

Invoke-CiStep `
    -Name "cargo check dx-www lib" `
    -FilePath "cargo" `
    -Arguments @("check", "--manifest-path", $manifest, "-p", "dx-www", "--lib") `
    -WorkingDirectory $root

Invoke-CiStep `
    -Name "forge launch smoke test" `
    -FilePath "cargo" `
    -Arguments @("test", "--manifest-path", $manifest, "-p", "dx-www", "forge_smoke_command_runs_launch_path", "--lib") `
    -WorkingDirectory $root

Invoke-CiStep `
    -Name "cargo fmt dx-www check" `
    -FilePath "cargo" `
    -Arguments @("fmt", "--manifest-path", $manifest, "-p", "dx-www", "--", "--check") `
    -WorkingDirectory $root

Invoke-CiStep `
    -Name "git diff check" `
    -FilePath "git" `
    -Arguments @("-C", $root, "diff", "--check") `
    -WorkingDirectory $root

if (-not $SkipArtifactLane) {
    Invoke-CiStep `
        -Name "cargo build dx-www bin" `
        -FilePath "cargo" `
        -Arguments @("build", "--manifest-path", $manifest, "-p", "dx-www", "--bin", "dx-www") `
        -WorkingDirectory $root

    $dxCli = Resolve-DxWwwExecutable -RepoRoot $root
    $tempProject = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-ci-" + [System.Guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force -Path $tempProject | Out-Null
    New-Item -ItemType Directory -Force -Path $artifactRoot | Out-Null

    Invoke-CiStep `
        -Name "forge ci artifacts" `
        -FilePath $dxCli `
        -Arguments @(
            "forge", "ci",
            "--project", $tempProject,
            "--out", $artifactRoot,
            "--fail-under", $FailUnder.ToString(),
            "--quiet"
        ) `
        -WorkingDirectory $root

    if (Test-Path -LiteralPath (Join-Path $tempProject "node_modules")) {
        throw "Forge CI temp project created node_modules, which is not allowed."
    }

    if (-not [string]::IsNullOrWhiteSpace($pagesRoot)) {
        $pagesProject = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-ci-pages-" + [System.Guid]::NewGuid().ToString("N"))
        New-Item -ItemType Directory -Force -Path $pagesProject | Out-Null
        New-Item -ItemType Directory -Force -Path $pagesRoot | Out-Null

        Copy-Item `
            -LiteralPath (Join-Path $artifactRoot "forge-readiness-badge.json") `
            -Destination (Join-Path $pagesRoot "forge-readiness-badge.json") `
            -Force

        Invoke-CiStep `
            -Name "forge ci public route" `
            -FilePath $dxCli `
            -Arguments @(
                "prove", "vertical",
                "--fixture", "forge-ci",
                "--out", $pagesRoot,
                "--write",
                "--format", "json",
                "--quiet"
            ) `
            -WorkingDirectory $pagesProject

        Copy-Item `
            -LiteralPath (Join-Path $pagesRoot "proof.json") `
            -Destination (Join-Path $pagesRoot "forge\ci.proof.json") `
            -Force

        $forgeCiHtml = Join-Path $pagesRoot "forge\ci.html"
        $forgeCiIndexDir = Join-Path $pagesRoot "forge\ci"
        New-Item -ItemType Directory -Force -Path $forgeCiIndexDir | Out-Null
        Copy-Item -LiteralPath $forgeCiHtml -Destination (Join-Path $forgeCiIndexDir "index.html") -Force

        $publicReleaseHistoryPath = Join-Path $root "benchmarks\reports\forge-public-release-history.json"
        if (-not (Test-Path -LiteralPath $publicReleaseHistoryPath)) {
            throw "Cannot find Forge public release history JSON at $publicReleaseHistoryPath"
        }
        $pagesReleaseHistoryPath = Join-Path $pagesProject "benchmarks\reports\forge-public-release-history.json"
        New-Item -ItemType Directory -Force -Path (Split-Path -Parent $pagesReleaseHistoryPath) | Out-Null
        Copy-Item -LiteralPath $publicReleaseHistoryPath -Destination $pagesReleaseHistoryPath -Force

        Invoke-CiStep `
            -Name "forge releases public route" `
            -FilePath $dxCli `
            -Arguments @(
                "prove", "vertical",
                "--fixture", "forge-releases",
                "--out", $pagesRoot,
                "--write",
                "--format", "json",
                "--quiet"
            ) `
            -WorkingDirectory $pagesProject

        Copy-Item `
            -LiteralPath (Join-Path $pagesRoot "proof.json") `
            -Destination (Join-Path $pagesRoot "forge\releases.proof.json") `
            -Force

        $forgeReleasesHtml = Join-Path $pagesRoot "forge\releases.html"
        $forgeReleasesIndexDir = Join-Path $pagesRoot "forge\releases"
        New-Item -ItemType Directory -Force -Path $forgeReleasesIndexDir | Out-Null
        Copy-Item -LiteralPath $forgeReleasesHtml -Destination (Join-Path $forgeReleasesIndexDir "index.html") -Force

        Invoke-CiStep `
            -Name "forge changelog public route" `
            -FilePath $dxCli `
            -Arguments @(
                "prove", "vertical",
                "--fixture", "forge-changelog",
                "--out", $pagesRoot,
                "--write",
                "--format", "json",
                "--quiet"
            ) `
            -WorkingDirectory $pagesProject

        Copy-Item `
            -LiteralPath (Join-Path $pagesRoot "proof.json") `
            -Destination (Join-Path $pagesRoot "forge\changelog.proof.json") `
            -Force

        $forgeChangelogHtml = Join-Path $pagesRoot "forge\changelog.html"
        $forgeChangelogIndexDir = Join-Path $pagesRoot "forge\changelog"
        New-Item -ItemType Directory -Force -Path $forgeChangelogIndexDir | Out-Null
        Copy-Item -LiteralPath $forgeChangelogHtml -Destination (Join-Path $forgeChangelogIndexDir "index.html") -Force

        $adoptionProject = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-ci-adoption-pages-" + [System.Guid]::NewGuid().ToString("N"))
        New-Item -ItemType Directory -Force -Path $adoptionProject | Out-Null

        Invoke-CiStep `
            -Name "forge adoption app evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "adoption-smoke",
                "--project", $adoptionProject,
                "--format", "json",
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge adoption public route" `
            -FilePath $dxCli `
            -Arguments @(
                "prove", "vertical",
                "--fixture", "forge-adoption",
                "--out", $pagesRoot,
                "--write",
                "--format", "json",
                "--quiet"
            ) `
            -WorkingDirectory $adoptionProject

        Copy-Item `
            -LiteralPath (Join-Path $pagesRoot "proof.json") `
            -Destination (Join-Path $pagesRoot "forge\adoption.proof.json") `
            -Force

        $forgeAdoptionHtml = Join-Path $pagesRoot "forge\adoption.html"
        $forgeAdoptionIndexDir = Join-Path $pagesRoot "forge\adoption"
        New-Item -ItemType Directory -Force -Path $forgeAdoptionIndexDir | Out-Null
        Copy-Item -LiteralPath $forgeAdoptionHtml -Destination (Join-Path $forgeAdoptionIndexDir "index.html") -Force

        Copy-Item `
            -LiteralPath (Join-Path $pagesRoot "forge\ci.proof.json") `
            -Destination (Join-Path $pagesRoot "proof.json") `
            -Force

        if (Test-Path -LiteralPath (Join-Path $pagesProject "node_modules")) {
            throw "Forge CI pages project created node_modules, which is not allowed."
        }

        Invoke-CiStep `
            -Name "forge pages bundle verification" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "ci",
                "--verify-pages", $pagesRoot,
                "--format", "json",
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        if (-not (Test-Path -LiteralPath $routeComparisonPath)) {
            throw "Cannot find Forge public route comparison JSON at $routeComparisonPath"
        }

        Assert-ForgePublicRouteComparison -Path $routeComparisonPath

        $artifactRouteComparison = Join-Path $artifactRoot "forge-public-route-comparison.json"
        Copy-Item -LiteralPath $routeComparisonPath -Destination $artifactRouteComparison -Force

        $dashboardProject = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-release-dashboard-" + [System.Guid]::NewGuid().ToString("N"))
        New-Item -ItemType Directory -Force -Path $dashboardProject | Out-Null

        Invoke-CiStep `
            -Name "forge release-dashboard gate" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-dashboard",
                "--project", $dashboardProject,
                "--ci-artifacts", $artifactRoot,
                "--pages", $pagesRoot,
                "--history", (Join-Path $artifactRoot "forge-benchmark-history.json"),
                "--route-comparison", $artifactRouteComparison,
                "--format", "markdown",
                "--output", (Join-Path $artifactRoot "forge-release-dashboard.md"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        $dashboardJsonProject = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-release-dashboard-json-" + [System.Guid]::NewGuid().ToString("N"))
        New-Item -ItemType Directory -Force -Path $dashboardJsonProject | Out-Null

        Invoke-CiStep `
            -Name "forge release-dashboard json evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-dashboard",
                "--project", $dashboardJsonProject,
                "--ci-artifacts", $artifactRoot,
                "--pages", $pagesRoot,
                "--history", (Join-Path $artifactRoot "forge-benchmark-history.json"),
                "--route-comparison", $artifactRouteComparison,
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-release-dashboard.json"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        if (Test-Path -LiteralPath (Join-Path $dashboardProject "node_modules")) {
            throw "Forge release-dashboard project created node_modules, which is not allowed."
        }

        if (Test-Path -LiteralPath (Join-Path $dashboardJsonProject "node_modules")) {
            throw "Forge release-dashboard JSON project created node_modules, which is not allowed."
        }

        Invoke-CiStep `
            -Name "forge release-history record" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-history",
                "--dashboard", (Join-Path $artifactRoot "forge-release-dashboard.json"),
                "--route-comparison", $artifactRouteComparison,
                "--output", (Join-Path $artifactRoot "forge-public-release-history.json"),
                "--format", "markdown",
                "--quiet"
            ) `
            -WorkingDirectory $root

        if (Test-Path -LiteralPath (Join-Path $artifactRoot "node_modules")) {
            throw "Forge release-history artifact lane created node_modules, which is not allowed."
        }

        Invoke-CiStep `
            -Name "forge launch-changelog json evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "launch-changelog",
                "--history", (Join-Path $artifactRoot "forge-public-release-history.json"),
                "--output", (Join-Path $artifactRoot "forge-public-launch-changelog.json"),
                "--format", "json",
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge launch-changelog markdown evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "launch-changelog",
                "--history", (Join-Path $artifactRoot "forge-public-release-history.json"),
                "--output", (Join-Path $artifactRoot "forge-public-launch-changelog.md"),
                "--format", "markdown",
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        if (Test-Path -LiteralPath (Join-Path $artifactRoot "node_modules")) {
            throw "Forge launch-changelog artifact lane created node_modules, which is not allowed."
        }

        $publicEvidenceRoot = Join-Path $artifactRoot "forge-public-evidence"
        $publicEvidenceProject = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-public-evidence-" + [System.Guid]::NewGuid().ToString("N"))
        New-Item -ItemType Directory -Force -Path $publicEvidenceProject | Out-Null
        New-Item -ItemType Directory -Force -Path $publicEvidenceRoot | Out-Null

        foreach ($fixture in @("forge-scorecard", "forge-ci", "forge-evidence")) {
            Invoke-CiStep `
                -Name "forge public evidence route $fixture" `
                -FilePath $dxCli `
                -Arguments @(
                    "prove", "vertical",
                    "--fixture", $fixture,
                    "--out", $publicEvidenceRoot,
                    "--write",
                    "--format", "json",
                    "--quiet"
                ) `
                -WorkingDirectory $publicEvidenceProject
        }

        Copy-RequiredFile `
            -Source (Join-Path $artifactRoot "forge-readiness-badge.json") `
            -Destination (Join-Path $publicEvidenceRoot "forge-readiness-badge.json")
        Copy-RequiredFile `
            -Source $routeComparisonMarkdownPath `
            -Destination (Join-Path $publicEvidenceRoot "forge-public-route-comparison.md")
        Copy-RequiredFile `
            -Source (Join-Path $root "benchmarks\reports\forge-launch-delivery-comparison.md") `
            -Destination (Join-Path $publicEvidenceRoot "forge-launch-delivery-comparison.md")

        Invoke-CiStep `
            -Name "forge public evidence verification" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "public-evidence",
                "--verify", $publicEvidenceRoot,
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-public-evidence-verify.json"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge trust-regression json evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "trust-regression",
                "--project", $tempProject,
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-trust-regression.json"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge trust-regression markdown evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "trust-regression",
                "--project", $tempProject,
                "--format", "markdown",
                "--output", (Join-Path $artifactRoot "forge-trust-regression.md"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        $sourceReviewPath = Join-Path $root "benchmarks\reports\forge-source-owned-package-review.json"
        $staticEvidencePath = Join-Path $root "benchmarks\reports\forge-static-competitor-evidence.json"

        Invoke-CiStep `
            -Name "forge release-candidate json evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-candidate",
                "--project", $tempProject,
                "--ci-artifacts", $artifactRoot,
                "--pages", $pagesRoot,
                "--route-comparison", $artifactRouteComparison,
                "--source-review", $sourceReviewPath,
                "--static-evidence", $staticEvidencePath,
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-release-candidate.json"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge release-candidate markdown evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-candidate",
                "--project", $tempProject,
                "--ci-artifacts", $artifactRoot,
                "--pages", $pagesRoot,
                "--route-comparison", $artifactRouteComparison,
                "--source-review", $sourceReviewPath,
                "--static-evidence", $staticEvidencePath,
                "--format", "markdown",
                "--output", (Join-Path $artifactRoot "forge-release-candidate.md"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        $releaseBundleRoot = Join-Path $artifactRoot "forge-release-bundle-adoption"
        $releaseBundleProject = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-release-bundle-" + [System.Guid]::NewGuid().ToString("N"))
        $releaseBundleReports = Join-Path $releaseBundleProject "benchmarks\reports"
        New-Item -ItemType Directory -Force -Path $releaseBundleReports | Out-Null
        Copy-RequiredFile `
            -Source $artifactRouteComparison `
            -Destination (Join-Path $releaseBundleReports "forge-public-route-comparison.json")
        Copy-RequiredFile `
            -Source $routeComparisonMarkdownPath `
            -Destination (Join-Path $releaseBundleReports "forge-public-route-comparison.md")
        Copy-RequiredFile `
            -Source (Join-Path $artifactRoot "forge-public-release-history.json") `
            -Destination (Join-Path $releaseBundleReports "forge-public-release-history.json")
        Copy-RequiredFile `
            -Source (Join-Path $artifactRoot "forge-public-release-history.md") `
            -Destination (Join-Path $releaseBundleReports "forge-public-release-history.md")

        Invoke-CiStep `
            -Name "forge release-bundle adoption artifacts" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-bundle",
                "--project", $releaseBundleProject,
                "--out", $releaseBundleRoot,
                "--include-adoption",
                "--format", "json",
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        $publisherKeyRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("dx-forge-ci-publisher-" + [System.Guid]::NewGuid().ToString("N"))
        Invoke-CiStep `
            -Name "forge publisher-key generate" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "publisher-key", "generate",
                "--out", $publisherKeyRoot,
                "--signer", "dx-forge-ci",
                "--force",
                "--format", "json",
                "--quiet"
            ) `
            -WorkingDirectory $root

        $releaseManifestPath = Join-Path $releaseBundleRoot "forge-release-manifest.json"
        Invoke-CiStep `
            -Name "forge publisher-key sign release manifest" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "publisher-key", "sign",
                "--key", (Join-Path $publisherKeyRoot "publisher-key.private.json"),
                "--manifest", $releaseManifestPath,
                "--manifest-output", $releaseManifestPath,
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-manifest-signing.json"),
                "--quiet"
            ) `
            -WorkingDirectory $root

        $registrySmokeRoot = Join-Path $artifactRoot "forge-registry-smoke"
        Invoke-CiStep `
            -Name "forge registry smoke json evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "registry", "smoke",
                "--remote", "r2",
                "--local", $registrySmokeRoot,
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-registry-smoke.json"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge registry smoke markdown evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "registry", "smoke",
                "--remote", "r2",
                "--local", $registrySmokeRoot,
                "--format", "markdown",
                "--output", (Join-Path $artifactRoot "forge-registry-smoke.md"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge release-operations json evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-operations",
                "--project", $tempProject,
                "--release-manifest", $releaseManifestPath,
                "--trust-regression", (Join-Path $artifactRoot "forge-trust-regression.json"),
                "--release-candidate", (Join-Path $artifactRoot "forge-release-candidate.json"),
                "--ci-artifacts", $artifactRoot,
                "--public-evidence", $publicEvidenceRoot,
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-release-operations.json"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge release-operations markdown evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "release-operations",
                "--project", $tempProject,
                "--release-manifest", $releaseManifestPath,
                "--trust-regression", (Join-Path $artifactRoot "forge-trust-regression.json"),
                "--release-candidate", (Join-Path $artifactRoot "forge-release-candidate.json"),
                "--ci-artifacts", $artifactRoot,
                "--public-evidence", $publicEvidenceRoot,
                "--format", "markdown",
                "--output", (Join-Path $artifactRoot "forge-release-operations.md"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge publish-plan json evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "publish-plan",
                "--project", $tempProject,
                "--release-bundle", $releaseBundleRoot,
                "--pages", $pagesRoot,
                "--registry-smoke", (Join-Path $artifactRoot "forge-registry-smoke.json"),
                "--release-operations", (Join-Path $artifactRoot "forge-release-operations.json"),
                "--format", "json",
                "--output", (Join-Path $artifactRoot "forge-publish-plan.json"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        Invoke-CiStep `
            -Name "forge publish-plan markdown evidence" `
            -FilePath $dxCli `
            -Arguments @(
                "forge", "publish-plan",
                "--project", $tempProject,
                "--release-bundle", $releaseBundleRoot,
                "--pages", $pagesRoot,
                "--registry-smoke", (Join-Path $artifactRoot "forge-registry-smoke.json"),
                "--release-operations", (Join-Path $artifactRoot "forge-release-operations.json"),
                "--format", "markdown",
                "--output", (Join-Path $artifactRoot "forge-publish-plan.md"),
                "--fail-under", $FailUnder.ToString(),
                "--quiet"
            ) `
            -WorkingDirectory $root

        if (Test-Path -LiteralPath (Join-Path $artifactRoot "node_modules")) {
            throw "Forge public beta shipping artifact lane created node_modules, which is not allowed."
        }
    }
}

Write-Host ""
Write-Host "DX Forge CI passed."
Write-Host "Repository: $root"
if (-not $SkipArtifactLane) {
    Write-Host "Artifacts: $artifactRoot"
    if (-not [string]::IsNullOrWhiteSpace($pagesRoot)) {
        Write-Host "Pages bundle: $pagesRoot"
    }
}
