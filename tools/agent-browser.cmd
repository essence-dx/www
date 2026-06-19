@echo off
setlocal
set "CODEX_NODE=%USERPROFILE%\.cache\codex-runtimes\codex-primary-runtime\dependencies\node\bin\node.exe"
set "CODEX_NODE_MODULES=%USERPROFILE%\.cache\codex-runtimes\codex-primary-runtime\dependencies\node\node_modules"

if exist "%CODEX_NODE%" (
  set "NODE_PATH=%CODEX_NODE_MODULES%"
  "%CODEX_NODE%" "%~dp0agent-browser.mjs" %*
) else (
  node "%~dp0agent-browser.mjs" %*
)
