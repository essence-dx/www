# DX-WWW n8n Studio

DX-WWW n8n Studio is a source-owned automation editor shell for working with local n8n connector metadata through DX-native app, style, icon, import, Forge, and receipt contracts.

This app reads against the local `integrations/n8n-nodes-base/dx-node-source-manifest.json` contract and the generated `integrations/n8n-nodes-base/generated/dx-automations-connectors.json` connector catalog. Live provider execution remains explicitly blocked until a real n8n adapter, credential boundary, and execution receipt are present.

The product target is n8n runtime-backed DX automation, not an n8n Vue editor port.

## Scope

- Automation scope readiness for node metadata, workflow JSON authoring, connection authoring, n8n runtime execution, credential bridge, and dynamic option/resource-locator capability states.
- Source-only Zed automation-list bridge packet for projecting n8n Studio editor-session readiness into Zed Plugins and DX automation surfaces.
- Connector catalog summary and source provenance.
- Generated connector catalog ingestion for the local 536-record n8n metadata artifact.
- Workflow document model compatible with n8n workflow JSON.
- Canvas projection model for workflow nodes and connections.
- Source-owned canvas pointer and keyboard interaction runtime for selection, drag, pan, zoom, fit, tidy, guarded delete, select-all, and nudging.
- Source-owned canvas edge handling for validated connection creation, reconnect handles, dynamic edge paths, duplicate rejection, and self-loop rejection.
- Seeded semantic node metadata for HTTP Request, Slack, and OpenAI, including display rules, credential selects, and resource locator modes.
- Source-backed Slack message parameter extraction from local n8n TypeScript parameter descriptions.
- Source-backed HTTP Request V3 parameter extraction from local n8n TypeScript descriptions and shared optimizer parameter source.
- Source-backed OpenAI parameter extraction from local n8n chat, text, and image TypeScript descriptions, including dynamic model picker boundaries.
- Nested child-parameter metadata and value-bound controls for source-backed Slack collection and fixed-collection fields.
- Source-owned nested collection add/remove/update mutation helpers with selected-node parameter schema refresh.
- Live client shell wiring for selected-node nested parameter mutations.
- Type-aware parameter value coercion for booleans, numbers, JSON fields, and resource locator payloads.
- Current edited workflow export through POSTed DX `WorkflowDocument` payloads with secret-like field sanitation and n8n-shaped JSON output.
- Field-level resource-locator draft state with mode/value controls and source-only list/search adapter boundaries.
- Selected-node credential picker readiness with id/name-only references and no secret-value storage.
- Editor-session adapter readiness and governed response-batch application for dynamic parameters, resource locator search, resource mapper schema, credential picker data, and credential validation metadata without live provider calls.
- Source-owned expression editor state with selected-node expression parsing, reference discovery, secret-reference diagnostics, and recursive field annotations.
- Import/export session state with sanitized preview summaries, issue actions, non-executable import boundaries, and export receipt metadata.
- Import apply/save actions that move clean sanitized previews into the real editor state and record source-only draft save receipt metadata.
- Execution debug readiness with blocked manual-run attempts, node log rows, receipt boundaries, and bridge-ready counters.
- Execution receipt import for replayable success/error logs, timings, item counts, provider error summaries, and redacted issue reporting.
- n8n public API execution-history import planning and sanitized `GET /executions` history-to-receipt conversion.
- Injected-transport n8n public API client boundary for governed workflow import and execution history import.
- Credential-vault bridge for redacted readiness and id-based n8n API credential resolution.
- Metadata-driven parameter, expression editor, credential, resource locator, pinned-data, execution, AI-tool, import/export, and receipt surfaces.
- Workflow import sanitation that strips secrets, drops unknown nodes, and keeps local drafts non-executable until receipts prove readiness.
- API routes for readiness, catalog, export, governed editor-session request batches, governed editor-session response batch contracts, and Zed automation bridge packets.
- Focused source-guard tests.

## Commands

- `dx dev`
- `dx build`
- `dx check`

Use focused benchmarks for source-only verification in low-memory lanes.
