param(
    [int]$MaxLanes = 30,
    [int]$MaxPasses = 3,
    [string]$Scope = "www-30-agent",
    [string]$AgentName = "",
    [string]$WorkerId = "",
    [switch]$Peek,
    [switch]$Reset,
    [switch]$Json,
    [switch]$Cycle
)

$script = Join-Path $PSScriptRoot "worker-lanes\claim-www-lane.ps1"

& $script `
    -MaxLanes $MaxLanes `
    -MaxPasses $MaxPasses `
    -Scope $Scope `
    -AgentName $AgentName `
    -WorkerId $WorkerId `
    -Peek:$Peek `
    -Reset:$Reset `
    -Json:$Json `
    -Cycle:$Cycle
