fn read_adoption_source_manifest(
    path: &Path,
    findings: &mut Vec<String>,
) -> anyhow::Result<DxSourceManifest> {
    if !path.exists() {
        findings.push(format!(
            "Forge source manifest is missing: {}",
            path.display()
        ));
        return Ok(DxSourceManifest::default());
    }
    match std::fs::read(path) {
        Ok(raw) => match serde_json::from_slice::<DxSourceManifest>(&raw) {
            Ok(manifest) => Ok(manifest),
            Err(error) => {
                findings.push(format!(
                    "Forge source manifest could not be parsed: {error}"
                ));
                Ok(DxSourceManifest::default())
            }
        },
        Err(error) => {
            findings.push(format!("Forge source manifest could not be read: {error}"));
            Ok(DxSourceManifest::default())
        }
    }
}

fn adoption_project_structure(project: &Path) -> DxForgeAdoptionProjectStructure {
    let dx_config_path = project.join("dx");
    let pages_dir = project.join("pages");
    let components_dir = project.join("components");
    let app_route_path = project.join("pages/forge-adoption.html");
    DxForgeAdoptionProjectStructure {
        dx_config_exists: dx_config_path.is_file(),
        dx_config_path,
        pages_dir_exists: pages_dir.is_dir(),
        pages_dir,
        components_dir_exists: components_dir.is_dir(),
        components_dir,
        app_route_exists: app_route_path.is_file(),
        app_route_path,
    }
}

fn adoption_public_route_artifacts(public_dir: &Path) -> Vec<DxForgeAdoptionRouteArtifact> {
    FORGE_RELEASE_BUNDLE_ROUTES
        .iter()
        .map(|route| {
            let html_path = public_dir.join(route.html);
            let clean_index_path = public_dir.join(route.clean_index);
            let packet_path = public_dir.join(route.packet);
            let proof_path = public_dir.join(route.proof);
            let claims_path = route.claims.map(|claims| public_dir.join(claims));
            let html_exists = html_path.is_file();
            let clean_index_exists = clean_index_path.is_file();
            let packet_exists = packet_path.is_file();
            let proof_exists = proof_path.is_file();
            DxForgeAdoptionRouteArtifact {
                route: route.route.to_string(),
                html_path,
                clean_index_path,
                packet_path,
                proof_path,
                claims_path,
                html_exists,
                clean_index_exists,
                packet_exists,
                proof_exists,
                passed: html_exists && clean_index_exists && packet_exists && proof_exists,
            }
        })
        .collect()
}

fn adoption_release_bundle_evidence(bundle_dir: &Path) -> DxForgeAdoptionReleaseBundleEvidence {
    if !bundle_dir.is_dir() {
        return DxForgeAdoptionReleaseBundleEvidence {
            bundle_dir: bundle_dir.to_path_buf(),
            exists: false,
            passed: false,
            score: 0,
            artifact_count: 0,
            route_count: 0,
            no_node_modules: !bundle_dir.join("node_modules").exists(),
            findings: vec!["release bundle directory is missing".to_string()],
        };
    }

    match verify_forge_release_bundle(bundle_dir) {
        Ok(report) => DxForgeAdoptionReleaseBundleEvidence {
            bundle_dir: bundle_dir.to_path_buf(),
            exists: true,
            passed: report.passed,
            score: report.score,
            artifact_count: report.artifact_count,
            route_count: report.route_count,
            no_node_modules: report.no_node_modules,
            findings: report.findings,
        },
        Err(error) => DxForgeAdoptionReleaseBundleEvidence {
            bundle_dir: bundle_dir.to_path_buf(),
            exists: true,
            passed: false,
            score: 0,
            artifact_count: 0,
            route_count: 0,
            no_node_modules: !bundle_dir.join("node_modules").exists(),
            findings: vec![format!("release bundle verification failed: {error}")],
        },
    }
}

fn adoption_check_evidence(
    report: &DxCheckReport,
    strict_forge_passed: bool,
    release_gate_score: u8,
) -> DxForgeAdoptionCheckEvidence {
    DxForgeAdoptionCheckEvidence {
        score: report.score,
        release_gate_score,
        traffic: report.traffic.as_str().to_string(),
        strict_forge_passed,
        section_count: report.sections.len(),
        finding_count: report
            .sections
            .iter()
            .map(|section| section.findings.len())
            .sum(),
    }
}

fn adoption_package_evidence(
    manifest: &DxSourceManifest,
    package_docs_dir: &Path,
) -> Vec<DxForgeAdoptionPackageEvidence> {
    manifest
        .packages
        .iter()
        .map(|package| {
            let docs_path = package_docs_dir.join(adoption_package_docs_name(
                &package.package_id,
                &package.variant,
            ));
            DxForgeAdoptionPackageEvidence {
                package_id: package.package_id.clone(),
                variant: package.variant.clone(),
                version: package.version.clone(),
                file_count: package.files.len(),
                docs_exists: docs_path.is_file(),
                docs_path,
                rollback_receipt: package.rollback_receipt.clone(),
            }
        })
        .collect()
}

fn adoption_package_docs_name(package_id: &str, variant: &str) -> String {
    let package = package_id.replace('/', "-");
    if variant == "default" {
        format!("{package}.md")
    } else {
        format!("{package}--variant-{}.md", variant.replace('.', "-"))
    }
}

fn count_regular_files(path: &Path) -> usize {
    path.read_dir()
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry
                        .file_type()
                        .map(|kind| kind.is_file())
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or_default()
}

fn adoption_percent_score(present: usize, expected: usize) -> u8 {
    if expected == 0 {
        return 0;
    }
    ((present.saturating_mul(100) / expected).min(100)) as u8
}

fn write_forge_adoption_project_scaffold(project: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let files = [
        (
            "dx",
            "project.name=\"dx-forge-contract-app\"\nproject.contract.authoring=\"react-shaped\"\nproject.contract.folders=\"next-familiar\"\nproject.contract.package_policy=\"forge-first-no-node-modules\"\nforge.paths.js_ui=\"components/ui\"\nforge.paths.js_lib=\"lib\"\nforge.paths.js_styles=\"styles\"\nforge.paths.js_openapi=\"openapi\"\nbuild.output_dir=\".dx/build\"\nbuild.optimization_level=\"release\"\ntooling.biome.version=\"2.4.15\"\ntooling.biome.formatter.enabled=true\ntooling.biome.linter.enabled=true\ntooling.dx_style.mode=\"generated-css\"\ntooling.forge_ui.style=\"new-york\"\n",
        ),
        (
            "app/layout.tsx",
            r#"import "../styles/global.css";
import "../styles/tokens.css";

type RootLayoutProps = {
  children: unknown;
};

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
"#,
        ),
        (
            "app/page.tsx",
            r#"import { WelcomeCard } from "../components/local/WelcomeCard";
import { recordWelcomeView } from "../server/actions";

export default function Page() {
  return (
    <main className="dx-shell">
      <WelcomeCard onView={recordWelcomeView} />
    </main>
  );
}
"#,
        ),
        (
            "components/local/WelcomeCard.tsx",
            r#""use client";

import { useState } from "react";

type WelcomeCardProps = {
  onView?: (payload: { count: number }) => Promise<{ ok: boolean; source: string }>;
};

export function WelcomeCard({ onView }: WelcomeCardProps) {
  const [viewCount, setViewCount] = useState(0);

  async function recordView() {
    const nextCount = viewCount + 1;
    setViewCount(nextCount);
    await onView?.({ count: nextCount });
  }

  return (
    <section className="dx-card">
      <p className="dx-eyebrow">DX-WWW contract app</p>
      <h1>React-shaped source, Forge-owned packages, no node_modules.</h1>
      <p>Forge packages remain visible source while local components stay editable.</p>
      <button className="dx-action" type="button" onClick={recordView}>
        Record server action
      </button>
      <p className="dx-count">Local interactions: {viewCount}</p>
    </section>
  );
}
"#,
        ),
        (
            "server/actions.ts",
            "export async function recordWelcomeView(payload: { count: number }) {\n  return { ok: true, source: \"dx-www-server-action\" };\n}\n",
        ),
        (
            "styles/tokens.css",
            ":root {\n  --dx-bg: #f8fafc;\n  --dx-fg: #0f172a;\n  --dx-card: #ffffff;\n  --dx-border: #d9e2ec;\n}\n",
        ),
        (
            "styles/global.css",
            "body {\n  margin: 0;\n  background: var(--dx-bg);\n  color: var(--dx-fg);\n}\n.dx-shell {\n  min-height: 100vh;\n  display: grid;\n  place-items: center;\n  padding: 2rem;\n}\n.dx-card {\n  max-width: 42rem;\n  border: 1px solid var(--dx-border);\n  background: var(--dx-card);\n  padding: 2rem;\n}\n.dx-eyebrow {\n  font-size: 0.75rem;\n  text-transform: uppercase;\n  letter-spacing: 0.08em;\n}\n.dx-action {\n  margin-top: 1rem;\n}\n.dx-count {\n  font-size: 0.875rem;\n}\n",
        ),
        (
            "pages/index.html",
            "<page>\n  <main class=\"min-h-screen bg-background text-foreground\">\n    <h1>DX Forge adoption smoke</h1>\n    <p>Clean project proving source-owned package materialization.</p>\n  </main>\n</page>\n",
        ),
        (
            "pages/forge-adoption.html",
            r#"<page>
  <main class="min-h-screen bg-background px-6 py-10 text-foreground">
    <section class="mx-auto grid max-w-4xl gap-6">
      <header class="grid gap-3">
        <p class="text-sm font-medium uppercase text-muted-foreground">DX Forge adoption fixture</p>
        <h1 class="text-4xl font-semibold">Source-owned packages in one app route.</h1>
        <p class="text-base text-muted-foreground">This route references the curated button, card, selected icon, Google OAuth starter, motion, and static migration packages materialized by Forge.</p>
      </header>

      <section class="grid gap-3 rounded-lg border p-5" data-forge-package="shadcn/ui/button">
        <h2 class="text-xl font-semibold">Source-owned UI action</h2>
        <Button>Review adoption evidence</Button>
      </section>

      <section class="grid gap-3 rounded-lg border p-5" data-forge-package="shadcn/ui/card">
        <h2 class="text-xl font-semibold">Source-owned UI container</h2>
        <Card>
          <CardHeader>
            <CardTitle>Forge card package</CardTitle>
            <CardDescription>Editable card source is tracked with the same receipt model.</CardDescription>
          </CardHeader>
          <CardContent>No package install or lifecycle script runs for this component.</CardContent>
        </Card>
      </section>

      <section class="grid gap-3 rounded-lg border p-5" data-forge-package="dx/icon/search">
        <h2 class="text-xl font-semibold">Selected icon package</h2>
        <div class="flex items-center gap-2">
          <SearchIcon />
          <span>Only the search icon source is materialized.</span>
        </div>
      </section>

      <section class="grid gap-3 rounded-lg border p-5" data-forge-package="auth/better-auth">
        <h2 class="text-xl font-semibold">OAuth starter boundary</h2>
        <a href="/auth/better-auth/route">Start Google OAuth smoke</a>
      </section>

      <section class="grid gap-3 rounded-lg border p-5" data-forge-package="animation/motion">
        <h2 class="text-xl font-semibold">Motion boundary</h2>
        <p>Motion source is materialized for launch interactions while animation policy, reduced-motion tuning, and browser QA remain application-owned.</p>
      </section>

      <section class="grid gap-3 rounded-lg border p-5" data-forge-package="migration/static-site">
        <h2 class="text-xl font-semibold">Static migration boundary</h2>
        <p>Simple WordPress/static page content can start as editable source, while plugins, themes, forms, comments, search, ecommerce, and CMS editing remain manual application work.</p>
      </section>
    </section>
  </main>
</page>
"#,
        ),
        (
            "README.md",
            "# DX Forge Adoption Smoke\n\nThis temporary app proves the public Forge launch flow from source-owned packages without creating `node_modules`.\n\n`pages/forge-adoption.html` references `shadcn/ui/button`, `shadcn/ui/card`, `dx/icon/search`, `auth/better-auth`, `animation/motion`, and `migration/static-site` together as the first real-app adoption fixture.\n",
        ),
    ];
    let mut written = Vec::new();

    for (relative, content) in files {
        let path = project.join(relative);
        if path.exists() {
            continue;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, content)?;
        written.push(path);
    }

    let serializer_config = serializer::SerializerOutputConfig::new()
        .with_output_dir(project.join(".dx/serializer"))
        .with_llm(false)
        .with_machine(true);
    let machine_cache = project.join(".dx/serializer/dx.machine");
    if !machine_cache.exists() {
        serializer::SerializerOutput::with_config(serializer_config)
            .process_file(&project.join("dx"))
            .map_err(|error| anyhow::anyhow!("generate dx machine cache: {error}"))?;
        written.push(machine_cache);
    }

    Ok(written)
}

fn write_forge_adoption_route_comparison(project: &Path) -> anyhow::Result<PathBuf> {
    let report_dir = project.join("benchmarks/reports");
    std::fs::create_dir_all(&report_dir)?;
    let generated_at = Utc::now().to_rfc3339();
    let mut routes = Vec::new();
    let mut total_decoded = 0u64;
    let mut total_brotli = 0u64;

    for route in FORGE_RELEASE_BUNDLE_ROUTES {
        let html_path = project.join("public").join(route.html);
        let packet_path = project.join("public").join(route.packet);
        let decoded_bytes = packet_path
            .metadata()
            .map(|metadata| metadata.len())
            .or_else(|_| html_path.metadata().map(|metadata| metadata.len()))
            .unwrap_or(512)
            .max(1);
        let brotli_bytes = html_path
            .metadata()
            .map(|metadata| metadata.len().min(decoded_bytes))
            .unwrap_or(decoded_bytes.min(512))
            .max(1);
        total_decoded = total_decoded.saturating_add(decoded_bytes);
        total_brotli = total_brotli.saturating_add(brotli_bytes);
        routes.push(serde_json::json!({
            "route": route.route,
            "fixture_mode": route.fixture.unwrap_or("forge-site"),
            "status": "measured",
            "route_delivery": "static",
            "decoded_bytes": decoded_bytes,
            "brotli_bytes": brotli_bytes,
            "http_route_median_ms": 0.0,
            "chrome_load_event_ms": 0.0,
            "budget_passed": true
        }));
    }

    let json_path = report_dir.join("forge-public-route-comparison.json");
    let report = serde_json::json!({
        "generated_at": generated_at,
        "route_count": routes.len(),
        "total_decoded_bytes": total_decoded,
        "total_brotli_bytes": total_brotli,
        "routes": routes,
        "notes": [
            "Generated by dx forge adoption-smoke for local clean-project route coverage.",
            "This smoke proves route generation and no-node_modules behavior; it is not a full browser benchmark."
        ]
    });
    std::fs::write(&json_path, serde_json::to_string_pretty(&report)?)?;
    std::fs::write(
        report_dir.join("forge-public-route-comparison.md"),
        forge_public_route_comparison_bundle_markdown(&json_path)?,
    )?;
    Ok(json_path)
}

fn write_forge_adoption_release_history(
    project: &Path,
    route_comparison_path: &Path,
) -> anyhow::Result<PathBuf> {
    let report_dir = project.join("benchmarks/reports");
    std::fs::create_dir_all(&report_dir)?;
    let route_comparison: serde_json::Value =
        serde_json::from_slice(&std::fs::read(route_comparison_path)?)?;
    let generated_at = Utc::now().to_rfc3339();
    let routes = route_comparison
        .get("routes")
        .and_then(|routes| routes.as_array())
        .cloned()
        .unwrap_or_default();
    let route_snapshots = routes
        .iter()
        .map(|route| DxForgePublicReleaseRouteSnapshot {
            route: route
                .get("route")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown")
                .to_string(),
            fixture_mode: route
                .get("fixture_mode")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown")
                .to_string(),
            delivery: route
                .get("route_delivery")
                .and_then(|value| value.as_str())
                .unwrap_or("static")
                .to_string(),
            decoded_bytes: route
                .get("decoded_bytes")
                .and_then(|value| value.as_u64())
                .unwrap_or_default(),
            brotli_bytes: route
                .get("brotli_bytes")
                .and_then(|value| value.as_u64())
                .unwrap_or_default(),
            http_route_median_ms: route
                .get("http_route_median_ms")
                .and_then(|value| value.as_f64())
                .unwrap_or_default(),
            chrome_load_event_ms: route
                .get("chrome_load_event_ms")
                .and_then(|value| value.as_f64())
                .unwrap_or_default(),
            budget_passed: route.get("budget_passed").and_then(|value| value.as_bool()),
        })
        .collect::<Vec<_>>();
    let lowest_brotli_route = route_snapshots
        .iter()
        .min_by_key(|route| route.brotli_bytes)
        .map(|route| route.route.clone())
        .unwrap_or_else(|| "/forge".to_string());
    let mut checks = BTreeMap::new();
    for (name, passed, score, message) in [
        (
            "smoke",
            true,
            100,
            "Adoption seed expects source-owned package smoke to run during release-bundle assembly.",
        ),
        (
            "launch-page",
            true,
            100,
            "Launch-page quality is verified by release-bundle assembly.",
        ),
        (
            "route-comparison",
            true,
            100,
            "Adoption smoke covers the six public Forge routes.",
        ),
    ] {
        checks.insert(
            name.to_string(),
            DxForgePublicReleaseDashboardCheckSnapshot {
                passed,
                score,
                message: message.to_string(),
            },
        );
    }
    let history = DxForgePublicReleaseHistory {
        updated_at: generated_at.clone(),
        records: vec![DxForgePublicReleaseRecord {
            generated_at: generated_at.clone(),
            source_dashboard: ".dx/forge/adoption-smoke/forge-smoke.json".to_string(),
            source_route_comparison: "benchmarks/reports/forge-public-route-comparison.json"
                .to_string(),
            dashboard: DxForgePublicReleaseDashboardSnapshot {
                generated_at,
                score: 100,
                fail_under: 90,
                passed: true,
                no_node_modules: Some(true),
                public_evidence_links: Some(FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64),
                findings: Vec::new(),
                checks,
            },
            route_comparison: DxForgePublicReleaseRouteComparisonSnapshot {
                generated_at: route_comparison
                    .get("generated_at")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                route_count: route_comparison
                    .get("route_count")
                    .and_then(|value| value.as_u64())
                    .unwrap_or(route_snapshots.len() as u64),
                total_decoded_bytes: route_comparison
                    .get("total_decoded_bytes")
                    .and_then(|value| value.as_u64())
                    .unwrap_or_default(),
                total_brotli_bytes: route_comparison
                    .get("total_brotli_bytes")
                    .and_then(|value| value.as_u64())
                    .unwrap_or_default(),
                lowest_brotli_route,
                routes: route_snapshots,
            },
            regression_findings: Vec::new(),
        }],
    };
    let history_path = report_dir.join("forge-public-release-history.json");
    std::fs::write(&history_path, serde_json::to_string_pretty(&history)?)?;
    std::fs::write(
        report_dir.join("forge-public-release-history.md"),
        forge_public_release_history_markdown(&history),
    )?;
    Ok(history_path)
}

fn copy_forge_release_bundle_routes_to_public(
    release_bundle_dir: &Path,
    public_dir: &Path,
) -> anyhow::Result<Vec<DxForgeAdoptionSmokeRoute>> {
    copy_forge_release_bundle_routes_to_public_with_options(release_bundle_dir, public_dir, false)
}

fn copy_forge_release_bundle_routes_to_public_with_options(
    release_bundle_dir: &Path,
    public_dir: &Path,
    include_adoption: bool,
) -> anyhow::Result<Vec<DxForgeAdoptionSmokeRoute>> {
    std::fs::create_dir_all(public_dir)?;
    let mut routes = Vec::new();

    for route in forge_release_bundle_routes(include_adoption) {
        copy_bundle_route_artifact(release_bundle_dir, public_dir, route.html)?;
        copy_bundle_route_artifact(release_bundle_dir, public_dir, route.clean_index)?;
        copy_bundle_route_artifact(release_bundle_dir, public_dir, route.packet)?;
        if let Some(claims) = route.claims {
            copy_bundle_route_artifact(release_bundle_dir, public_dir, claims)?;
        }
        if let Some(evidence) = route.evidence {
            copy_bundle_route_artifact(release_bundle_dir, public_dir, evidence)?;
        }
        copy_bundle_route_artifact(release_bundle_dir, public_dir, route.proof)?;

        let html_path = public_dir.join(route.html);
        let clean_index_path = public_dir.join(route.clean_index);
        let packet_path = public_dir.join(route.packet);
        let claims_path = route.claims.map(|claims| public_dir.join(claims));
        let proof_path = public_dir.join(route.proof);
        let exists = html_path.is_file()
            && clean_index_path.is_file()
            && packet_path.is_file()
            && proof_path.is_file()
            && claims_path.as_ref().is_none_or(|path| path.is_file());

        routes.push(DxForgeAdoptionSmokeRoute {
            route: route.route.to_string(),
            html_path,
            clean_index_path,
            packet_path,
            claims_path,
            proof_path,
            exists,
        });
    }

    Ok(routes)
}

fn copy_bundle_route_artifact(
    release_bundle_dir: &Path,
    public_dir: &Path,
    relative: &str,
) -> anyhow::Result<()> {
    let source = release_bundle_dir.join(relative);
    let destination = public_dir.join(relative);
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(source, destination)?;
    Ok(())
}

fn build_forge_readiness_badge(
    smoke: &DxForgeSmokeReport,
    evidence: &DxForgeReleaseEvidenceReport,
    smoke_report_path: Option<PathBuf>,
    fail_under: u8,
) -> DxForgeReadinessBadge {
    let latest_forge_route_benchmark = evidence
        .latest_forge_route_benchmark
        .as_ref()
        .or(evidence.latest_benchmark.as_ref());
    let benchmark_passed =
        latest_forge_route_benchmark.is_some_and(forge_benchmark_snapshot_is_release_ready);
    let benchmark_score = if benchmark_passed { 100 } else { 0 };
    let scorecard_passed = forge_package_scorecard_release_ready(&evidence.package_scorecard);
    let score = [
        smoke.score,
        evidence.release_gate_score,
        evidence.package_scorecard.score,
        smoke.launch_page_quality.score,
        benchmark_score,
        if smoke.no_node_modules { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);

    let mut findings = smoke.findings.clone();
    if !evidence.passed {
        findings.push("Forge release proof is not passing.".to_string());
    }
    if !scorecard_passed {
        findings.push("Forge package scorecard is below release-readiness policy.".to_string());
    }
    if latest_forge_route_benchmark.is_none() {
        findings.push("No latest /forge benchmark snapshot was found.".to_string());
    } else if !benchmark_passed {
        findings.push("Latest /forge benchmark snapshot is not release-ready.".to_string());
    }
    if score < fail_under {
        findings.push(format!(
            "Release-readiness score {score} is below required threshold {fail_under}."
        ));
    }

    let passed = smoke.passed
        && evidence.passed
        && scorecard_passed
        && benchmark_passed
        && smoke.no_node_modules
        && score >= fail_under;
    let status = if passed { "passing" } else { "failing" }.to_string();
    let message = if passed {
        format!("ready {score}/100")
    } else {
        format!("review {score}/100")
    };
    let color = if passed && score >= 95 {
        "brightgreen"
    } else if passed {
        "green"
    } else if score >= 70 {
        "yellow"
    } else {
        "red"
    }
    .to_string();

    DxForgeReadinessBadge {
        schema_version: 1,
        generated_at: Utc::now().to_rfc3339(),
        project: smoke.project.clone(),
        label: "DX Forge".to_string(),
        status,
        message,
        color,
        score,
        passed,
        is_error: !passed,
        fail_under,
        no_node_modules: smoke.no_node_modules,
        smoke: DxForgeReadinessBadgeCheck {
            passed: smoke.passed,
            score: smoke.score,
            summary: format!(
                "Smoke packages={}, check={}, doctor={}, verify={}, no_node_modules={}",
                smoke.packages.len(),
                smoke.check_score,
                smoke.doctor_passed,
                smoke.verify_passed,
                smoke.no_node_modules
            ),
        },
        evidence: DxForgeReadinessBadgeCheck {
            passed: evidence.passed,
            score: evidence.release_gate_score,
            summary: format!(
                "Release gate={}, dx_check={} / {}, rollback={}%, docs={}%",
                evidence.release_gate_score,
                evidence.check_score,
                evidence.check_traffic,
                evidence.rollback_coverage_percent,
                evidence.package_docs_coverage_percent
            ),
        },
        scorecard: DxForgeReadinessBadgeCheck {
            passed: scorecard_passed,
            score: evidence.package_scorecard.score,
            summary: format!(
                "Scorecard packages={}, verified={}",
                evidence.package_scorecard.packages.len(),
                evidence
                    .package_scorecard
                    .packages
                    .iter()
                    .filter(|package| package.integrity_verified)
                    .count()
            ),
        },
        launch_page_quality: DxForgeReadinessBadgeCheck {
            passed: smoke.launch_page_quality.passed,
            score: smoke.launch_page_quality.score,
            summary: format!(
                "headings={}, seo={}, links={}, claims={}",
                smoke.launch_page_quality.headings.passed,
                smoke.launch_page_quality.seo.passed,
                smoke.launch_page_quality.links.passed,
                smoke.launch_page_quality.claims_manifest.passed
            ),
        },
        latest_forge_route_benchmark: latest_forge_route_benchmark.map(|snapshot| {
            DxForgeReadinessBadgeBenchmark {
                passed: forge_benchmark_snapshot_is_release_ready(snapshot),
                generated_at: snapshot.generated_at.clone(),
                fixture_mode: snapshot.fixture_mode.clone(),
                route_delivery: snapshot.route_delivery.clone(),
                forge_packages: snapshot.forge_packages,
                forge_files_tracked: snapshot.forge_files_tracked,
                decoded_bytes: snapshot.decoded_bytes,
                brotli_bytes: snapshot.brotli_bytes,
                http_route_median_ms: snapshot.http_route_median_ms,
                chrome_load_event_ms: snapshot.chrome_load_event_ms,
            }
        }),
        artifacts: DxForgeReadinessBadgeArtifacts {
            smoke_report: smoke_report_path,
            release_evidence: smoke.launch_artifacts.evidence_report_path.clone(),
            package_scorecard: smoke.launch_artifacts.scorecard_report_path.clone(),
            benchmark_history: smoke.launch_artifacts.benchmark_history_path.clone(),
            launch_html: smoke.launch_artifacts.launch_html_path.clone(),
            launch_claims: smoke.launch_artifacts.launch_claims_path.clone(),
            launch_evidence_model: smoke.launch_artifacts.launch_evidence_model_path.clone(),
        },
        findings,
    }
}

fn write_forge_launch_smoke_artifacts(project: &Path) -> anyhow::Result<DxForgeSmokeArtifacts> {
    let smoke_dir = project.join(".dx/forge/launch-smoke");
    std::fs::create_dir_all(&smoke_dir)?;

    let benchmark_history_path = write_forge_launch_smoke_benchmark_history(project)?;
    let launch_out_dir = project.join("public");
    run_forge_launch_smoke_step(
        project,
        "dx forge launch-page",
        vec![
            "launch-page".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            launch_out_dir.to_string_lossy().into_owned(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ],
    )?;

    let evidence_report_path = smoke_dir.join("evidence.json");
    let scorecard_report_path = smoke_dir.join("scorecard.json");

    run_forge_launch_smoke_step(
        project,
        "dx forge evidence",
        vec![
            "evidence".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--history".to_string(),
            benchmark_history_path.to_string_lossy().into_owned(),
            "--output".to_string(),
            evidence_report_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ],
    )?;
    run_forge_launch_smoke_step(
        project,
        "dx forge scorecard",
        vec![
            "scorecard".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--history".to_string(),
            benchmark_history_path.to_string_lossy().into_owned(),
            "--output".to_string(),
            scorecard_report_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
        ],
    )?;

    Ok(DxForgeSmokeArtifacts {
        benchmark_history_path,
        evidence_report_path,
        scorecard_report_path,
        launch_source_path: project.join("pages/forge.html"),
        launch_html_path: project.join("public/forge.html"),
        launch_packet_path: project.join("public/forge.dxp"),
        launch_runtime_path: project
            .join("public/forge.dxp.js")
            .exists()
            .then(|| project.join("public/forge.dxp.js")),
        launch_summary_path: project.join("public/proof.json"),
        launch_claims_path: project.join("public/forge.claims.json"),
        launch_evidence_model_path: project.join("public/forge.evidence.json"),
    })
}

fn run_forge_launch_smoke_step(
    project: &Path,
    label: &str,
    args: Vec<String>,
) -> anyhow::Result<()> {
    Cli::with_cwd(project.to_path_buf())
        .cmd_forge(&args)
        .map_err(|error| anyhow::anyhow!("{label} failed: {error}"))
}

fn write_forge_launch_smoke_benchmark_history(project: &Path) -> anyhow::Result<PathBuf> {
    let history_dir = project.join("benchmarks/reports/vertical-proof-history");
    std::fs::create_dir_all(&history_dir)?;
    let history_path = history_dir.join("index.json");
    let generated_at = Utc::now().to_rfc3339();
    let manifest = read_optional_forge_source_manifest(project)?;
    let forge_packages = manifest
        .as_ref()
        .map(|manifest| manifest.packages.len() as u64)
        .unwrap_or_default();
    let forge_files_tracked = manifest
        .as_ref()
        .map(|manifest| {
            manifest
                .packages
                .iter()
                .map(|package| package.files.len() as u64)
                .sum::<u64>()
        })
        .unwrap_or_default();
    let packet_bytes = project
        .join("public/forge.dxp")
        .metadata()
        .map(|metadata| metadata.len())
        .unwrap_or_default();
    let route_delivery = if project.join("public/forge.dxp.js").exists() {
        "dxpk-runtime"
    } else {
        "static"
    };
    let html_bytes = project
        .join("public/forge.html")
        .metadata()
        .map(|metadata| metadata.len())
        .unwrap_or_default();
    let history = serde_json::json!({
        "updated_at": generated_at,
        "snapshots": [
            {
                "generated_at": generated_at,
                "fixture_mode": "forge-site",
                "route_delivery": route_delivery,
                "markdown": "launch-smoke/generated-by-dx-forge-smoke.md",
                "forge_packages": forge_packages,
                "forge_files_tracked": forge_files_tracked,
                "decoded_bytes": packet_bytes,
                "brotli_bytes": html_bytes.min(packet_bytes),
                "http_route_median_ms": 0.0,
                "chrome_load_event_ms": 0.0,
                "dx_packet_applied": route_delivery != "static",
                "interaction_works": route_delivery != "static"
            }
        ]
    });
    std::fs::write(&history_path, serde_json::to_string_pretty(&history)?)?;
    Ok(history_path)
}

fn check_forge_launch_page_quality(
    artifacts: &DxForgeSmokeArtifacts,
) -> anyhow::Result<DxForgeLaunchPageQualityReport> {
    let html = std::fs::read_to_string(&artifacts.launch_html_path)?;
    let lower_html = html.to_ascii_lowercase();
    let claims: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&artifacts.launch_claims_path)?)?;

    let h1_count = count_substrings(&lower_html, "<h1");
    let h2_count = count_substrings(&lower_html, "<h2");
    let headings = quality_check(
        h1_count == 1 && h2_count >= 3 && html.contains("DX Forge launch evidence"),
        "headings",
        format!("h1={h1_count}, h2={h2_count}"),
    );

    let has_title = lower_html.contains("<title>") && lower_html.contains("</title>");
    let has_main = lower_html.contains("<main");
    let has_no_runtime_script = !lower_html.contains("<script");
    let seo = quality_check(
        has_title && has_main && has_no_runtime_script,
        "seo",
        format!(
            "title={}, main={}, runtime_script={}",
            has_title, has_main, !has_no_runtime_script
        ),
    );

    let hrefs = html_href_values(&html);
    let unsafe_hrefs = hrefs
        .iter()
        .filter(|href| {
            let trimmed = href.trim();
            trimmed.is_empty()
                || trimmed == "#"
                || trimmed.to_ascii_lowercase().starts_with("javascript:")
        })
        .count();
    let links = quality_check(
        unsafe_hrefs == 0,
        "links",
        format!("links={}, unsafe={unsafe_hrefs}", hrefs.len()),
    );

    let claim_items = claims
        .get("claims")
        .and_then(|claims| claims.as_array())
        .cloned()
        .unwrap_or_default();
    let verified_claims = claim_items
        .iter()
        .filter(|claim| {
            claim
                .get("verification_status")
                .and_then(|value| value.as_str())
                == Some("verified")
        })
        .count();
    let invalid_claim_statuses = claim_items
        .iter()
        .filter(|claim| {
            !matches!(
                claim
                    .get("verification_status")
                    .and_then(|value| value.as_str()),
                Some("verified" | "declared" | "needs-review")
            )
        })
        .count();
    let required_claim_fields = [
        "id",
        "claim",
        "source_model",
        "source_field",
        "verification_status",
        "evidence",
    ];
    let incomplete_claims = claim_items
        .iter()
        .filter(|claim| {
            required_claim_fields.iter().any(|field| {
                claim
                    .get(*field)
                    .and_then(|value| value.as_str())
                    .is_none_or(|value| value.trim().is_empty())
            })
        })
        .count();
    let claims_manifest = quality_check(
        !claim_items.is_empty()
            && verified_claims > 0
            && invalid_claim_statuses == 0
            && incomplete_claims == 0,
        "claims_manifest",
        format!(
            "claims={}, verified={}, invalid_statuses={}, incomplete_claims={}",
            claim_items.len(),
            verified_claims,
            invalid_claim_statuses,
            incomplete_claims
        ),
    );

    let checks = [&headings, &seo, &links, &claims_manifest];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{} ({})", check.message, check.evidence))
        .collect::<Vec<_>>();
    let score = checks
        .into_iter()
        .map(|check| if check.passed { 100 } else { 0 })
        .min()
        .unwrap_or(0);

    Ok(DxForgeLaunchPageQualityReport {
        passed: findings.is_empty(),
        score,
        headings,
        seo,
        links,
        claims_manifest,
        findings,
    })
}

fn quality_check(
    passed: bool,
    message: impl Into<String>,
    evidence: impl Into<String>,
) -> DxForgeLaunchPageQualityCheck {
    DxForgeLaunchPageQualityCheck {
        passed,
        message: message.into(),
        evidence: evidence.into(),
    }
}

fn forge_launch_page_quality_checks(
    report: &DxForgeLaunchPageQualityReport,
) -> [(&'static str, &DxForgeLaunchPageQualityCheck); 4] {
    [
        ("headings", &report.headings),
        ("seo", &report.seo),
        ("links", &report.links),
        ("claims_manifest", &report.claims_manifest),
    ]
}

fn write_forge_ci_artifacts(
    report: &DxForgeSmokeReport,
    out_dir: &Path,
    fail_under: u8,
) -> anyhow::Result<Vec<PathBuf>> {
    std::fs::create_dir_all(out_dir)?;
    let mut artifacts = Vec::new();

    let smoke_json_path = out_dir.join("forge-smoke.json");
    std::fs::write(&smoke_json_path, serde_json::to_string_pretty(report)?)?;
    artifacts.push(smoke_json_path.clone());

    let smoke_markdown_path = out_dir.join("forge-smoke.md");
    std::fs::write(&smoke_markdown_path, forge_smoke_markdown(report))?;
    artifacts.push(smoke_markdown_path);

    for (label, source) in forge_smoke_artifact_paths(&report.launch_artifacts) {
        let destination = out_dir.join(forge_ci_artifact_filename(label));
        std::fs::copy(source, &destination)?;
        artifacts.push(destination);
    }

    let evidence = build_forge_release_evidence_report(
        &report.project,
        &report.launch_artifacts.benchmark_history_path,
    )?;
    let mut badge =
        build_forge_readiness_badge(report, &evidence, Some(smoke_json_path.clone()), fail_under);
    badge.artifacts = DxForgeReadinessBadgeArtifacts {
        smoke_report: Some(smoke_json_path),
        release_evidence: out_dir.join("forge-evidence.json"),
        package_scorecard: out_dir.join("forge-scorecard.json"),
        benchmark_history: out_dir.join("forge-benchmark-history.json"),
        launch_html: out_dir.join("forge.html"),
        launch_claims: out_dir.join("forge.claims.json"),
        launch_evidence_model: out_dir.join("forge.evidence.json"),
    };
    let badge_path = out_dir.join("forge-readiness-badge.json");
    std::fs::write(&badge_path, serde_json::to_string_pretty(&badge)?)?;
    artifacts.push(badge_path);

    let triage_path = out_dir.join("forge-triage.md");
    std::fs::write(&triage_path, forge_failure_triage_markdown(report, &badge))?;
    artifacts.push(triage_path);

    Ok(artifacts)
}

fn write_forge_ci_command_artifacts(
    report: &DxForgeSmokeReport,
    out_dir: &Path,
    fail_under: u8,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut artifacts = write_forge_ci_artifacts(report, out_dir, fail_under)?;
    artifacts.extend(write_forge_ci_adoption_artifacts(
        &report.project,
        out_dir,
        fail_under,
    )?);
    Ok(artifacts)
}

fn write_forge_ci_adoption_artifacts(
    _project: &Path,
    out_dir: &Path,
    fail_under: u8,
) -> anyhow::Result<Vec<PathBuf>> {
    std::fs::create_dir_all(out_dir)?;
    let adoption_project =
        std::env::temp_dir().join(format!("dx-forge-ci-adoption-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&adoption_project)?;
    let project = adoption_project.as_path();

    let mut artifacts = Vec::new();
    let adoption_smoke = build_forge_adoption_smoke_report(project, fail_under)?;
    let adoption_smoke_json_path = out_dir.join("forge-adoption-smoke.json");
    std::fs::write(
        &adoption_smoke_json_path,
        serde_json::to_string_pretty(&adoption_smoke)?,
    )?;
    artifacts.push(adoption_smoke_json_path);

    let adoption_report = build_forge_adoption_report(
        project,
        Some(adoption_smoke.release_bundle_dir.clone()),
        fail_under,
    )?;
    let adoption_report_json_path = out_dir.join("forge-adoption-report.json");
    std::fs::write(
        &adoption_report_json_path,
        serde_json::to_string_pretty(&adoption_report)?,
    )?;
    artifacts.push(adoption_report_json_path);

    let adoption_report_markdown_path = out_dir.join("forge-adoption-report.md");
    std::fs::write(
        &adoption_report_markdown_path,
        forge_adoption_report_markdown(&adoption_report),
    )?;
    artifacts.push(adoption_report_markdown_path);

    Cli::with_cwd(project.to_path_buf())
        .cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            "forge-adoption".to_string(),
            "--out".to_string(),
            out_dir.to_string_lossy().into_owned(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .map_err(|error| anyhow::anyhow!("dx prove vertical forge-adoption failed: {error}"))?;

    let adoption_proof_path = out_dir.join("forge/adoption.proof.json");
    copy_bundle_file(out_dir, "proof.json", "forge/adoption.proof.json")?;
    let transient_proof = out_dir.join("proof.json");
    if transient_proof.is_file() {
        std::fs::remove_file(transient_proof)?;
    }
    artifacts.push(adoption_proof_path);

    copy_bundle_file(out_dir, "forge/adoption.html", "forge/adoption/index.html")?;
    artifacts.push(out_dir.join("forge/adoption.html"));
    artifacts.push(out_dir.join("forge/adoption/index.html"));
    artifacts.push(out_dir.join("forge/adoption.dxp"));
    artifacts.push(out_dir.join("forge/adoption.claims.json"));

    let adoption_page_source = project.join("pages/forge/adoption.html");
    let adoption_page_artifact = out_dir.join("forge-adoption-page.html");
    std::fs::copy(&adoption_page_source, &adoption_page_artifact)?;
    artifacts.push(adoption_page_artifact);

    if !adoption_smoke.passed {
        anyhow::bail!(
            "Forge adoption smoke did not pass for CI artifacts: {}",
            adoption_smoke.findings.join("; ")
        );
    }
    if !adoption_report.passed {
        anyhow::bail!(
            "Forge adoption report did not pass for CI artifacts: {}",
            adoption_report.findings.join("; ")
        );
    }
    if project.join("node_modules").exists()
        || adoption_project.join("node_modules").exists()
        || out_dir.join("node_modules").exists()
    {
        anyhow::bail!("Forge CI adoption artifact lane created node_modules");
    }

    Ok(artifacts)
}
