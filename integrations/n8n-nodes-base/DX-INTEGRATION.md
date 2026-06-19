# n8n Nodes Base Reference

This folder is a local copy of `packages/nodes-base` from the n8n source tree at `G:\WWW\inspirations\n8n`.

It is here so DX-WWW and Forge workers can build the DX Automations connector registry from real node source material instead of guessing connector shapes or reimplementing hundreds of integrations from scratch.

## Contents

- `nodes/`: official n8n built-in node source folders.
- `credentials/`: credential type definitions used by those nodes.
- `utils/` and `types/`: shared helper and type files required to inspect node behavior.
- `dx-node-source-manifest.json`: generated DX manifest with node and credential file paths.
- `AUTOMATIONS-BRIDGE.md`: launch worker brief for the DX-WWW automation routes and Zed/Rust credential bridge.
- `LICENSE.n8n.md` and `LICENSE_EE.n8n.md`: upstream n8n license files preserved with the copy.

## DX Usage Rules

- Build the DX Automations UI in DX-WWW code, not by copying n8n app UI.
- Use these files as the launch source for connector operations, credential shapes, and workflow-node behavior.
- Prefer direct Forge imports from independent `n8n-community-node-package` npm packages when those packages declare MIT, Apache-2.0, BSD, or ISC.
- Preserve upstream notices while exposing the connectors through DX-owned Forge metadata, receipts, and UI.

## Launch Target

The launch path is a DX-owned `/automations` page with:

- connector count and license readiness,
- workflow/node catalog search,
- credential-safe setup states,
- Forge receipts for imported connectors,
- no dependency on the n8n runtime.
