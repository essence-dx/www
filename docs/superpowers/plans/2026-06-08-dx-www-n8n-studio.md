# DX-WWW n8n Studio Foundation Plan

Date: 2026-06-08

## Objective

Build the first production slice of a DX-native n8n Studio inside DX-WWW. The slice must establish the app contract, domain boundaries, source-backed connector catalog, workflow document model, parameter/schema rendering model, credential readiness, execution readiness, AI tooling boundary, import/export surface, and receipt policy without pretending that live n8n execution or credential storage is already wired.

## Findings

- DX-WWW apps must use the modern `app/`, `components/`, `lib/`, `lib/stores`, `server`, `styles`, `public`, root extensionless `dx`, and `.dx` contract.
- DX Style owns `styles/theme.css`, `styles/generated.css`, and `styles/globals.css`; generated CSS must not be hand-edited.
- DX Icon must remain first-party through the configured `Icon` component or `<dx-icon>` runtime tag.
- `dx run` is available and uses the approved G-drive Bun path, but this checkout has no public `dx js` command and no wired Rolldown build integration.
- Local n8n connector source material exists under `integrations/n8n-nodes-base` with `dx-node-source-manifest.json` reporting 536 node files and 396 credential files.
- n8n editor architecture should be mirrored as domains, not copied as Vue/Vite implementation: workflow document, node metadata registry, canvas projection, parameter editor, credentials, resource locator, pinned data, execution debug state, AI tools, and import/export sanitation.
- Stable live integration must go through explicit adapter boundaries: public API for workflow/credential/execution CRUD and editor/session adapter for node catalog, dynamic parameters, manual runs, and credential picker behavior.

## Implementation Slice

1. Add `examples/n8n-studio` as a modern DX-WWW example app.
2. Define typed domain contracts under `lib/n8n-studio`:
   - catalog metadata and source manifest summary
   - workflow document and canvas projection
   - parameter schema and expression/editor readiness
   - credential readiness with redaction guarantees
   - resource locator and pinned-data states
   - execution boundary and AI tool states
   - import/export and receipt summaries
3. Render a real Studio shell with panels for catalog, canvas, parameters, credentials, resource locator, pinned data, execution/debug, AI tools, import/export, and receipts.
4. Add route handlers for readiness, catalog, and export responses.
5. Add focused source-guard tests proving:
   - the Studio uses DX-WWW app/config/style/icon conventions
   - the manifest counts match the local n8n source manifest
   - required editor surfaces are present
   - live execution remains blocked at the adapter boundary
   - no forbidden local package dependency or upstream UI runtime is pulled in
6. Update `TODO.md` and `CHANGELOG.md`.
7. Run focused checks only: the new benchmark and `git diff --check`.
8. Commit only related changes.

## Semantic Metadata Slice

1. Add a focused failing benchmark for semantic n8n metadata behavior before implementation.
2. Introduce a source-owned node type registry with per-node metadata modules for HTTP Request, Slack, and OpenAI.
3. Generate parameter fields from node metadata, including display rules, `credentialsSelect` fields, expression enablement, resource locator modes, and dynamic load boundaries.
4. Add n8n workflow import sanitation that:
   - drops unknown node types
   - strips secret-like parameter and credential fields
   - keeps credential id/name references only
   - removes connections and pinned data targeting dropped nodes
   - regenerates imported webhook identifiers for local draft safety
5. Keep live provider execution blocked and receipt-visible.
6. Guard with `dx run --test .\benchmarks\n8n-studio-semantic-metadata.test.ts --json`, the foundation contract, targeted `dx run --check`, and `git diff --check`.

## Generated Connector Metadata Slice

1. Add a focused failing benchmark for generated local n8n connector ingestion.
2. Adapt `integrations/n8n-nodes-base/generated/dx-automations-connectors.json`
   into deterministic DX-owned node type descriptions without importing n8n's
   runtime or UI.
3. Deduplicate versioned/generated connector records by node id while
   preferring canonical source files and richer operation/resource metadata.
4. Generate baseline parameter fields for generated nodes:
   - resource options
   - operation options
   - credential picker type boundaries
5. Project generated connector records into the n8n Studio catalog response
   while preserving source manifest counts, provider boundaries, and
   `liveProviderExecution=false`.
6. Keep richer curated node descriptions as overrides for HTTP Request, Slack,
   and OpenAI until full generated parameter extraction exists.
7. Guard with `dx run --test .\benchmarks\n8n-studio-generated-metadata.test.ts --json`, the semantic/foundation contracts, targeted `dx run --check`, and `git diff --check`.

## Source Parameter Extraction Slice

1. Add a focused failing benchmark for source-backed Slack message parameters.
2. Read local n8n parameter descriptions from
   `integrations/n8n-nodes-base/nodes/Slack/V2/MessageDescription.ts` without
   importing n8n runtime packages.
3. Resolve the source's common exported parameter-object and array patterns:
   - `sendToSelector`
   - `channelRLC`
   - `userRLC`
   - spread overrides inside `messageFields`
4. Convert the supported source fields into DX Studio parameter definitions:
   resource, operation, select, channel/user resource locators, message type,
   text, block JSON text, notice fields, and collection/fixed-collection
   boundary fields.
5. Mark complex collection-style source fields as explicit rendering
   boundaries so UI can show them honestly until full nested collection editing
   exists.
6. Guard with `dx run --test .\benchmarks\n8n-studio-source-parameters.test.ts --json`, the generated/semantic/foundation contracts, targeted `dx run --check`, and `git diff --check`.

## HTTP Request Source Parameter Slice

1. Extend the source-parameter benchmark to require HTTP Request V3 parameters
   from local n8n source.
2. Read `integrations/n8n-nodes-base/nodes/HttpRequest/V3/Description.ts`
   and the shared `optimizeResponse.ts` parameter source without importing n8n
   runtime packages.
3. Preserve curl import fields, credential-select type arrays, boolean/numeric
   display rules, hide rules, and fixedCollection child fields for query,
   header, and body parameter groups.
4. Attach tool-only response optimizer fields behind an explicit `@tool`
   display rule so the AI-tool branch stays visible without pretending live
   provider execution is available.
5. Guard with `dx run --test .\benchmarks\n8n-studio-source-parameters.test.ts --json`, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Nested Source Parameter Slice

1. Extend the source-parameter benchmark to require nested child metadata for
   Slack collection and fixed-collection fields.
2. Parse `options` and `values` blocks from supported local n8n parameter
   source objects without executing upstream code.
3. Preserve nested child definitions for fields such as Slack attachments,
   attachment field groups, and other options.
4. Carry child definitions through `createParameterField` into rendered
   parameter state.
5. Summarize nested child fields in the parameter inspector while keeping
   complex fields marked as rendering boundaries until full nested editing is
   implemented.
6. Guard with `dx run --test .\benchmarks\n8n-studio-source-parameters.test.ts --json`, the generated/semantic/foundation contracts, targeted `dx run --check`, and `git diff --check`.

## Nested Collection Editor State Slice

1. Extend the source-parameter benchmark to require value-bound nested
   collection state for Slack attachment parameters.
2. Add a typed `collectionItems` model to parameter fields so repeated
   collection/fixedCollection values keep stable item keys, labels, and child
   field values.
3. Scope child parameter field construction to the parent collection item or
   fixedCollection group value instead of always using empty defaults.
4. Extract recursive parameter field rendering into a focused component that
   renders nested source children and repeated collection items without copying
   upstream n8n UI code.
5. Keep add/remove/update mutation controls as the next slice; this checkpoint
   proves editor-ready nested state and rendered controls, not full mutation.
6. Guard with `dx run --test .\benchmarks\n8n-studio-source-parameters.test.ts --json`, targeted `dx run --check`, full focused n8n Studio benchmarks, and `git diff --check`.

## Nested Collection Mutation Slice

1. Add a focused failing benchmark for nested collection/fixedCollection
   add/remove/update behavior.
2. Introduce immutable parameter path mutation helpers that can append
   collection items, remove collection items, and update deeply nested
   parameter values without mutating the previous workflow node.
3. Apply mutations at the workflow-node and Studio-state level so the document
   nodes, canvas nodes, and selected-node parameter schema stay synchronized.
4. Add store action wiring for the selected-node mutation path.
5. Keep UI controls callback-gated: the recursive parameter field component
   exposes add/remove/update hooks and data markers only when real callbacks are
   supplied, so the static shell does not render dummy interactive controls.
6. Guard with `dx run --test .\benchmarks\n8n-studio-collection-actions.test.ts --json`, the source-parameter/generated/semantic/foundation benchmarks, targeted `dx run --check`, and `git diff --check`.

## Client Mutation Wiring Slice

1. Add a focused failing benchmark proving the rendered Studio shell owns
   editable client state and wires selected-node parameter mutations into the
   inspector.
2. Keep the page entrypoint server-friendly by passing an initial source-owned
   Studio state into a client shell.
3. Apply collection add/remove and value update operations through
   `applyParameterMutationToStudioState` so document nodes, canvas nodes, and
   selected-node parameter fields remain synchronized.
4. Pass real mutation callbacks from the shell through `ParameterInspector` into
   recursive `ParameterFieldControl` instances.
5. Preserve provider/execution boundaries: live n8n execution remains false,
   and no credential values are introduced into client state.
6. Guard with `dx run --test .\benchmarks\n8n-studio-collection-actions.test.ts --json`, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Type-Aware Parameter Value Slice

1. Add a focused failing benchmark proving live parameter edits do not collapse
   every value to a string before mutation.
2. Introduce a small source-owned coercion utility keyed by `ParameterField`
   type.
3. Coerce boolean checkbox edits, finite numeric text input, JSON fields, and
   structured resource-locator payloads before applying selected-node mutation
   state.
4. Broaden mutation callback value types to `unknown` so typed values travel
   through the client shell, inspector, recursive controls, and immutable
   mutation reducer without re-stringifying.
5. Preserve invalid or incomplete structured input as text instead of hiding
   parsing failures behind silent data loss.
6. Guard with `dx run --test .\benchmarks\n8n-studio-collection-actions.test.ts --json`, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Resource Locator Draft Slice

1. Add a focused failing benchmark for resource-locator field state instead of
   only global resource-locator panel metadata.
2. Add source-owned draft state to resource-locator parameter fields:
   active mode, selected value, cached label/query, searchable mode metadata,
   and explicit source-only list/search boundaries.
3. Preserve URL modes from local n8n source even when upstream describes the raw
   input type as a string and the mode name is `url`.
4. Render resource-locator mode and value controls through the recursive
   parameter field component, while keeping dynamic list/search read-only until
   the editor-session adapter exists.
5. Keep live provider execution false and visible at the field boundary; no
   credential or provider call is introduced.
6. Guard with `dx run --test .\benchmarks\n8n-studio-resource-locator.test.ts --json`, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Credential Picker Readiness Slice

1. Add a focused failing benchmark proving credential readiness is scoped to the
   selected workflow node and node-type credential requirements.
2. Introduce a source-owned credential sanitizer that keeps only credential id,
   display name, credential type, source, and redaction metadata.
3. Derive credential picker state from workflow-node credential references and
   node-type requirements, including required/missing status and redacted option
   lists.
4. Render credential picker data in the credentials panel with explicit
   provider-boundary and no-secret markers.
5. Keep live credential loading, vault validation, and provider execution behind
   the future n8n editor-session adapter.
6. Guard with `dx run --test .\benchmarks\n8n-studio-credentials.test.ts --json`, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Import Export Session Slice

1. Add a focused failing benchmark proving import/export state is more than a
   static panel: source-only import sources, sanitized preview summaries, issue
   actions, export receipt detail, and route response metadata.
2. Reuse the source-owned workflow import sanitizer to create a preview model
   from imported workflow payloads without preserving credential or parameter
   secret values.
3. Keep the default Studio state honest as awaiting input; do not render a fake
   imported workflow when no source adapter has supplied a payload.
4. Add export receipt detail derived from the current workflow document:
   node/connection/pinned/credential-reference counts, download name, route
   path, redaction, and provider-boundary metadata.
5. Render stable import preview, sanitation issue, sanitized field, and export
   receipt markers in the panel while keeping clipboard, file, and URL source
   adapters disabled until the editor session can provide real payloads.
6. Guard with `dx run --test .\benchmarks\n8n-studio-import-export.test.ts --json`, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Execution Debug Readiness Slice

1. Add a focused failing benchmark proving execution debug is more than a
   static blocked message: blocked attempts, node log rows, receipt boundaries,
   UI markers, and readiness API counters.
2. Derive a source-owned blocked manual-run attempt from the current workflow
   document, including workflow id/name, trigger node, selected node, receipt
   path, and no-secret/provider-boundary metadata.
3. Project per-node debug rows for every workflow node without preserving
   credential or parameter secret values; source-inspectable nodes can be ready
   while provider-backed output remains blocked.
4. Render execution attempts, node logs, available actions, and execution
   receipt import boundaries in the execution panel with stable data markers.
5. Expose compact execution debug counts through the readiness response for the
   DX Agents bridge.
6. Keep live n8n execution, provider timings, provider errors, and output data
   behind future execution receipt import.
7. Guard with `dx run --test .\benchmarks\n8n-studio-execution-debug.test.ts --json`, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Execution Receipt Import Slice

1. Add a focused failing benchmark proving Studio can consume a replayable
   execution receipt without running n8n live.
2. Import receipt-level attempt metadata: execution id, workflow id, status,
   mode, started/finished timestamps, duration, receipt path, and imported-at
   timestamp.
3. Map receipt node logs only onto known workflow nodes; drop unknown-node logs
   into explicit import issues.
4. Preserve node status, timing, item-count summaries, data preview labels, and
   provider error messages without storing raw output payloads.
5. Strip secret-like fields and secret-like field names from receipt state and
   import issues.
6. Keep `liveProviderExecution=false` because the receipt is imported evidence,
   not a live Studio-triggered provider call.
7. Render imported receipt issues and provider-error markers in the execution
   panel, and expose receipt issue/import metadata through the readiness
   response for bridge consumers.
8. Guard with `dx run --test .\benchmarks\n8n-studio-execution-receipt-import.test.ts --json`, the execution-debug benchmark, the focused n8n Studio benchmarks, targeted `dx run --check`, and `git diff --check`.

## Execution History API Import Slice

1. Add a focused failing benchmark for the n8n public API execution-history
   adapter before implementation.
2. Prove the request plan from local n8n source only: `GET /executions` and
   `GET /executions/{id}` from `nodes/N8n/ExecutionDescription.ts`, backed by
   covered endpoint entries in `n8n-api-coverage.json`.
3. Keep the plan read-only and credential-bound to `n8nApi`; do not embed
   secret values, base URLs, or a live provider call.
4. Convert `GET /executions` history payloads into the existing replayable
   execution receipt shape, preserving execution id, workflow id, mode, status,
   timestamps, duration, node timings, item counts, and provider error messages.
5. Drop raw output item JSON, error stacks, token/key field names, and
   secret-like values before data reaches Studio execution state.
6. Reuse the receipt importer so unknown node logs, workflow mismatches, and
   `liveProviderExecution=false` keep the same governed behavior.
7. Expose history import availability through the readiness response for the DX
   Agents bridge.
8. Guard with `dx run --test .\benchmarks\n8n-studio-execution-history-api.test.ts --json`, the receipt/debug/contract benchmarks, targeted `dx run --check`, and `git diff --check`.

## n8n API Client Boundary Slice

1. Add a focused failing benchmark for an authenticated n8n public API client
   boundary that covers workflow import and execution-history import.
2. Prove the client remains tied to local n8n source: `GET /workflows`,
   `GET /workflows/{id}`, and `GET /executions` must be covered by
   `WorkflowDescription.ts`, `ExecutionDescription.ts`, and
   `n8n-api-coverage.json`.
3. Use an injected transport so tests and future server adapters can provide the
   request executor without hiding live network behavior in client state.
4. Send the API key only inside the transport request headers; returned
   readiness, workflow preview, execution state, and bridge metadata must not
   preserve secret values or secret-like field names.
5. Route workflow payloads through the existing workflow import sanitizer and
   route execution history payloads through the governed execution receipt
   importer.
6. Expose a blocked/default n8n API client readiness boundary through the
   readiness response so DX Agents and Zed can discover the missing credential
   step without claiming provider execution.
7. Guard with `dx run --test .\benchmarks\n8n-studio-n8n-api-client.test.ts --json`, adjacent import/export and execution benchmarks, targeted `dx run --check`, and `git diff --check`.

## Credential Vault Bridge Slice

1. Add a focused failing benchmark proving Studio can resolve an `n8nApi`
   credential by id without storing secret values in returned state.
2. Model a small vault record contract with credential id, display name,
   credential type, base URL, and secret reference; returned readiness must keep
   only redacted credential metadata.
3. Use an injected secret loader to materialize the API key only when creating
   an n8n API client. The secret may enter the transport request header, but it
   must not appear in vault readiness, client readiness, import preview, or
   execution state.
4. Keep unavailable secrets and missing credentials as explicit blocked errors
   without naming secret refs.
5. Expose default blocked credential-vault readiness through the readiness
   response for DX Agents and Zed.
6. Guard with `dx run --test .\benchmarks\n8n-studio-credential-vault.test.ts --json`, adjacent credential/API-client/import-export/execution benchmarks, targeted `dx run --check`, and `git diff --check`.

## Non-Goals For This Slice

- No package manager install.
- No dev server.
- No heavy Cargo command.
- No `dx build`.
- No live n8n run.
- No raw credential storage.
- No claim that Rolldown backs DX-WWW builds.
- No copy of n8n's Vue editor implementation.
