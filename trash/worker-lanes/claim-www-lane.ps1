param(
    [int]$MaxLanes = 30,
    [int]$MaxPasses = 3,
    [string]$Scope = "www-30-agent",
    [string]$StateDir = "",
    [string]$AgentName = "",
    [string]$WorkerId = "",
    [switch]$Peek,
    [switch]$Reset,
    [switch]$Json,
    [switch]$Cycle
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($StateDir)) {
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $StateDir = Join-Path $scriptDir "state"
}

New-Item -ItemType Directory -Force -Path $StateDir | Out-Null

$safeScope = $Scope -replace "[^A-Za-z0-9_.-]", "-"
$counterPath = Join-Path $StateDir "$safeScope.counter.txt"
$claimsPath = Join-Path $StateDir "$safeScope.claims.jsonl"
$workersDir = Join-Path $StateDir "$safeScope.workers"
$lockPath = Join-Path $StateDir "$safeScope.lock"
New-Item -ItemType Directory -Force -Path $workersDir | Out-Null

function Read-Counter {
    param([string]$Path)

    if (-not (Test-Path -LiteralPath $Path)) {
        return 0
    }

    $raw = (Get-Content -LiteralPath $Path -Raw).Trim()
    if ([string]::IsNullOrWhiteSpace($raw)) {
        return 0
    }

    $value = 0
    if (-not [int]::TryParse($raw, [ref]$value)) {
        throw "Counter file is not an integer: $Path contains '$raw'"
    }

    if ($value -lt 0) {
        throw "Counter file is negative: $Path contains '$raw'"
    }

    return $value
}

function New-WorkerId {
    $stamp = Get-Date -Format "yyyyMMdd-HHmmssfff"
    $suffix = [Guid]::NewGuid().ToString("N").Substring(0, 10)
    return "www-$stamp-$suffix"
}

function Get-WorkerPath {
    param([string]$Id)
    $safeId = $Id -replace "[^A-Za-z0-9_.-]", "-"
    return Join-Path $workersDir "$safeId.json"
}

function Write-Result {
    param(
        [hashtable]$Result,
        [bool]$AsJson
    )

    if ($AsJson) {
        $Result | ConvertTo-Json -Depth 8
        return
    }

    if ($Result.status -eq "reset") {
        Write-Output "DX-WWW worker lane counter reset."
        Write-Output "Scope: $($Result.scope)"
        Write-Output "Current lane counter: $($Result.current)"
        Write-Output "Next lane: $($Result.nextLane)"
        Write-Output "Counter: $($Result.counterPath)"
        return
    }

    if ($Result.status -eq "peek") {
        Write-Output "DX-WWW worker lane counter peek."
        Write-Output "Scope: $($Result.scope)"
        Write-Output "Current lane counter: $($Result.current)"
        Write-Output "Next lane: $($Result.nextLane)"
        Write-Output "Max lanes: $($Result.maxLanes)"
        return
    }

    Write-Output "DX-WWW worker lane/pass assigned."
    Write-Output "AGENT_NUMBER: $($Result.agentNumber)"
    Write-Output "PASS_NUMBER: $($Result.passNumber)"
    Write-Output "WORKER_ID: $($Result.workerId)"
    Write-Output "Lane: $($Result.agentNumber) / $($Result.maxLanes)"
    Write-Output "Pass: $($Result.passNumber) / $($Result.maxPasses)"
    Write-Output "Scope: $($Result.scope)"
    Write-Output "Next pass command:"
    Write-Output "powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -WorkerId $($Result.workerId)"
}

$lockStream = $null
for ($attempt = 1; $attempt -le 80; $attempt++) {
    try {
        $lockStream = [System.IO.File]::Open(
            $lockPath,
            [System.IO.FileMode]::OpenOrCreate,
            [System.IO.FileAccess]::ReadWrite,
            [System.IO.FileShare]::None
        )
        break
    } catch [System.IO.IOException] {
        if ($attempt -eq 80) {
            throw "Could not acquire lane counter lock after $attempt attempts: $lockPath"
        }
        Start-Sleep -Milliseconds (25 + (Get-Random -Minimum 0 -Maximum 75))
    }
}

try {
    $current = Read-Counter -Path $counterPath

    if ($Reset) {
        if (Test-Path -LiteralPath $claimsPath) {
            $stamp = Get-Date -Format "yyyyMMdd-HHmmss"
            $archivePath = Join-Path $StateDir "$safeScope.claims.$stamp.jsonl"
            Move-Item -LiteralPath $claimsPath -Destination $archivePath -Force
        }
        if (Test-Path -LiteralPath $workersDir) {
            Remove-Item -LiteralPath $workersDir -Recurse -Force
        }
        New-Item -ItemType Directory -Force -Path $workersDir | Out-Null
        Set-Content -LiteralPath $counterPath -Value "0" -Encoding Ascii
        Write-Result -AsJson:$Json -Result @{
            status = "reset"
            scope = $Scope
            current = 0
            nextLane = 1
            maxLanes = $MaxLanes
            maxPasses = $MaxPasses
            counterPath = $counterPath
            claimsPath = $claimsPath
        }
        exit 0
    }

    if ($Peek) {
        $nextLane = $current + 1
        if ($nextLane -gt $MaxLanes) {
            if ($Cycle) {
                $nextLane = 1
            } else {
                $nextLane = $null
            }
        }
        Write-Result -AsJson:$Json -Result @{
            status = "peek"
            scope = $Scope
            current = $current
            nextLane = $nextLane
            maxLanes = $MaxLanes
            maxPasses = $MaxPasses
            counterPath = $counterPath
            claimsPath = $claimsPath
        }
        exit 0
    }

    if (-not [string]::IsNullOrWhiteSpace($WorkerId)) {
        $workerPath = Get-WorkerPath -Id $WorkerId
        if (-not (Test-Path -LiteralPath $workerPath)) {
            throw "Unknown WORKER_ID '$WorkerId'. Run without -WorkerId to claim a new lane, or check the id from the previous pass."
        }

        $worker = Get-Content -LiteralPath $workerPath -Raw | ConvertFrom-Json
        $nextPass = [int]$worker.passNumber + 1
        if ($nextPass -gt $MaxPasses) {
            throw "WORKER_ID '$WorkerId' already completed pass $($worker.passNumber)/$MaxPasses."
        }

        $worker.passNumber = $nextPass
        $worker.updatedAt = (Get-Date).ToString("o")
        $worker | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $workerPath -Encoding UTF8

        $claim = [ordered]@{
            scope = $Scope
            workerId = $WorkerId
            agentNumber = [int]$worker.agentNumber
            passNumber = $nextPass
            maxLanes = $MaxLanes
            maxPasses = $MaxPasses
            claimedAt = (Get-Date).ToString("o")
            agentName = $AgentName
            computer = $env:COMPUTERNAME
            pid = $PID
            cwd = (Get-Location).Path
            mode = "next-pass"
        }
        Add-Content -LiteralPath $claimsPath -Value ($claim | ConvertTo-Json -Compress) -Encoding UTF8

        Write-Result -AsJson:$Json -Result @{
            status = "assigned"
            scope = $Scope
            workerId = $WorkerId
            agentNumber = [int]$worker.agentNumber
            passNumber = $nextPass
            maxLanes = $MaxLanes
            maxPasses = $MaxPasses
            counterPath = $counterPath
            claimsPath = $claimsPath
        }
        exit 0
    }

    $nextLane = $current + 1
    if ($nextLane -gt $MaxLanes) {
        if ($Cycle) {
            $nextLane = 1
        } else {
            throw "All $MaxLanes lanes are already claimed for scope '$Scope'. Use -Reset only when the manager wants a fresh assignment round."
        }
    }

    $newWorkerId = New-WorkerId
    $workerPath = Get-WorkerPath -Id $newWorkerId
    Set-Content -LiteralPath $counterPath -Value ([string]$nextLane) -Encoding Ascii

    $workerState = [ordered]@{
        scope = $Scope
        workerId = $newWorkerId
        agentNumber = $nextLane
        passNumber = 1
        maxLanes = $MaxLanes
        maxPasses = $MaxPasses
        agentName = $AgentName
        createdAt = (Get-Date).ToString("o")
        updatedAt = (Get-Date).ToString("o")
    }
    $workerState | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $workerPath -Encoding UTF8

    $claim = [ordered]@{
        scope = $Scope
        workerId = $newWorkerId
        agentNumber = $nextLane
        passNumber = 1
        maxLanes = $MaxLanes
        maxPasses = $MaxPasses
        claimedAt = (Get-Date).ToString("o")
        agentName = $AgentName
        computer = $env:COMPUTERNAME
        pid = $PID
        cwd = (Get-Location).Path
        mode = "new-worker"
    }
    Add-Content -LiteralPath $claimsPath -Value ($claim | ConvertTo-Json -Compress) -Encoding UTF8

    Write-Result -AsJson:$Json -Result @{
        status = "assigned"
        scope = $Scope
        workerId = $newWorkerId
        agentNumber = $nextLane
        passNumber = 1
        maxLanes = $MaxLanes
        maxPasses = $MaxPasses
        counterPath = $counterPath
        claimsPath = $claimsPath
    }
} finally {
    if ($null -ne $lockStream) {
        $lockStream.Dispose()
    }
}
