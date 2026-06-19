# DX Automations Bridge Brief

## Source Copy Status

The local n8n nodes-base source copy is complete for the launch connector surface:

- `nodes/`: 6,047 files copied from `G:\WWW\inspirations\n8n\packages\nodes-base\nodes`.
- `credentials/`: 475 files copied from `G:\WWW\inspirations\n8n\packages\nodes-base\credentials`.
- `types/`: 2 files copied from `G:\WWW\inspirations\n8n\packages\nodes-base\types`.
- `utils/`: 28 files copied from `G:\WWW\inspirations\n8n\packages\nodes-base\utils`.
- Node implementation files: 536 copied.
- Credential implementation files: 396 copied.

## UI Study Rule

DX should learn from the n8n editor experience, but the DX Automations UI must be rebuilt in DX-WWW and GPUI style. Do not copy n8n app UI source into the product UI.

Useful local n8n UI study paths:

- `G:\WWW\inspirations\n8n\packages\frontend\editor-ui`
- `G:\WWW\inspirations\n8n\packages\frontend\editor-ui\src\features\workflows\canvas`
- `G:\WWW\inspirations\n8n\packages\frontend\editor-ui\src\features\shared\nodeCreator`
- `G:\WWW\inspirations\n8n\packages\frontend\editor-ui\src\features\credentials`
- `G:\WWW\inspirations\n8n\packages\frontend\editor-ui\src\features\settings\communityNodes`
- `G:\WWW\inspirations\n8n\packages\frontend\@n8n\design-system\src\components\N8nNodeCreatorNode`
- `G:\WWW\inspirations\n8n\packages\frontend\@n8n\design-system\src\components\N8nNodeIcon`

Use those paths to understand information architecture, workflow canvas behavior, node search, credential setup, and community-node management. Rebuild the visual language for DX.

## DX-WWW Automation Routes

The WWW worker should create DX-owned routes and components:

- `/automations`: connector registry, workflow canvas, node search, credential readiness, run receipts.
- `/automations/connectors`: node catalog from `dx-node-source-manifest.json`.
- `/automations/credentials`: credential schema readiness and keychain-backed setup states.
- `/automations/workflows`: workflow draft/import/export surface.

The page should prove that DX has hundreds of real connectors immediately. It should not show dummy cards.

## Zed/Rust Bridge Contract

Zed should not parse arbitrary TypeScript node source directly at runtime. DX-WWW or a small DX tooling command should generate compact connector metadata that Rust can read quickly.

Launch metadata shape:

- connector id
- display name
- source file
- credential type names
- operations/resources/actions discovered from node descriptions
- auth kind: api key, OAuth2, bearer token, basic auth, none, custom
- risk/status: ready, needs credential, unsupported, review needed
- source provenance: `n8n-nodes-base` or `n8n-community-node-package`

Zed/GPUI should consume this metadata through one of these surfaces:

- `dx automations connectors --json`
- `dx automations credentials --json`
- `dx automations run --json`
- generated `G:\Dx\.dx\receipts\automations\*.json`

For production speed, the metadata should later be serialized with `rkyv` and read with `memmap2`. JSON is acceptable for the first launch slice if it is generated once and read as a small manifest.

## Credential Safety

DX can use the credential schemas from the copied n8n nodes, but secrets must be stored through the DX/Zed credential path:

- never commit secrets,
- never display raw secrets,
- never copy another tool's credential storage silently,
- use explicit user approval for imports,
- store approved secrets in the system keychain or the existing Zed credential provider,
- keep receipts redacted.

The GPUI settings surface in Zed should show:

- connector readiness,
- credential readiness,
- account connection status,
- last run receipt,
- missing configuration action,
- open in DX Automations action.

## Worker Instruction

Build from the node source now. Do not spend launch time reimplementing hundreds of connectors. The product layer is DX-owned; the connector source material is copied locally for speed and compatibility.
