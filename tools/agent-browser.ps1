$ErrorActionPreference = "Stop"

$node = Join-Path $env:USERPROFILE ".cache\codex-runtimes\codex-primary-runtime\dependencies\node\bin\node.exe"
$nodeModules = Join-Path $env:USERPROFILE ".cache\codex-runtimes\codex-primary-runtime\dependencies\node\node_modules"

if (Test-Path -LiteralPath $node) {
    $env:NODE_PATH = $nodeModules
    & $node (Join-Path $PSScriptRoot "agent-browser.mjs") @args
    exit $LASTEXITCODE
}

& node (Join-Path $PSScriptRoot "agent-browser.mjs") @args
exit $LASTEXITCODE
