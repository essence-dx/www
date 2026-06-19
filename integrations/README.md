# DX-WWW Integration Source Map

These folders are staged references for building the real DX-WWW platform.

## Current Sources

- `flow-forge`: updated Forge source from `G:\Workspaces\flow\forge`.
- `flow-serializer`: updated serializer source from `G:\Workspaces\flow\serializer`.
- `flow-forge-bridge`: Flow-side Forge bridge from `G:\Workspaces\flow\src\forge_bridge`.
- `n8n-nodes-base`: official n8n built-in node source reference copied from `G:\WWW\inspirations\n8n\packages\nodes-base` for DX Automations connector mapping.

The primary current snapshot now lives in `..\flow`:

- `..\flow\src`: Flow runtime, traits, voice meter, Forge bridge, serializer bridge, and DX-facing facade.
- `..\flow\crates\flow-browser-core`: Flow browser crate.
- `..\flow\crates\forge`: current Forge source from `G:\Workspaces\flow\forge`.
- `..\flow\crates\serializer`: current serializer source from `G:\Workspaces\flow\serializer`.
- `..\flow\docs`: Flow documentation references.

The archived tool-tui tree remains the historical source for the original DX-WWW demo WASM artifacts, but it is no longer staged here as the main integration source.

## Intended Roles

- Flow Forge becomes the storage, sync, job, recovery, and versioning engine.
- Archived Forge ideas remain historical references for package orchestration and green/yellow/red update policy.
- Flow serializer informs compact metadata/config/tool-schema serialization.
- n8n node references inform DX-owned Automations connectors, credential schemas, and workflow import compatibility.
- DX-WWW consumes these as libraries/contracts after the current workspace is made self-contained.

## Copy Rules

These are reference copies. Build artifacts such as `target`, `node_modules`, `.git`, `.cache`, DLLs, EXEs, and PDBs were intentionally excluded.
