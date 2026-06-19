# DX-WWW n8n Studio Changelog

[2026-06-09] - Host-Executed Editor Session Requests

ADDED:
- Added a server-side editor-session execution helper for trusted DX/Zed host
  adapters. The helper collects pending dynamic option, resource locator,
  mapper schema, and credential metadata request plans, executes a bounded
  request batch through an injected host executor, and applies returned records
  through the governed editor-session response pipeline.

CHANGED:
- Updated automation scope so dynamic options/resource locators report
  host-executed request batch support while still marking the capability as
  partial until provider-specific executors are registered.

NOTES:
- The helper does not load secrets, own provider transports, or expose raw
  provider payloads. Response application continues to enforce the existing
  no-secret/no-live-provider editor-session decoder.

[2026-06-09] - Credential Validation Vault Receipts

ADDED:
- Added generic provider credential validation execution to the credential
  vault bridge. The vault can now resolve a credential by id, load its secret
  only inside the adapter boundary, call an injected validation transport, and
  return a redacted `dx.n8n-studio.credential-validation.receipt`.

CHANGED:
- Updated automation scope so the credential bridge is `partial` and names
  provider credential validation receipts instead of validation metadata only.

NOTES:
- Validation receipts may represent live provider validation, so they are
  separate from client-visible editor-session metadata. Returned receipts omit
  secret values, secret refs, provider response bodies, and request payloads.

[2026-06-09] - Generated Node Semantic Notices

ADDED:
- Added generated-only node summary notices for the local generated n8n
  connector catalog. Generic generated node parameter schemas now carry a
  non-expression `generatedNodeSummary` field with connector description,
  source file, runtime mode, auth kind, provenance, and governed adapter
  boundaries.
- Added first-class notice and description rendering in the parameter
  inspector so generated metadata appears as read-only context instead of a
  generic text input.

NOTES:
- This improves generated-only catalog semantics without pretending to have
  every node-specific custom field. Richer source-backed parameter extraction
  remains the path for nodes whose local n8n source exposes detailed
  properties, dynamic options, resource locators, and collection controls.

[2026-06-09] - Host-Injected Scheduler Route

ADDED:
- Added a reusable runtime execution-proof scheduler route factory. The
  factory keeps GET and the default public POST route redacted and
  credential-handoff-required, while trusted DX/Zed host code can inject a
  runtime-context resolver that supplies the credential vault, n8n API
  transport, workflow document, trigger receipt, and receipt root for bounded
  read-only proof retry execution.

CHANGED:
- Refactored `/api/n8n-studio/runtime-execution-proof-retry` to use the route
  factory without registering host context in the public module, preserving
  the current no-secret default behavior.

NOTES:
- The route factory returns real retry results only when host code supplies
  runtime context. It still does not accept credential values, webhook URLs,
  provider response bodies, request bodies, or secret refs through the public
  scheduler payload.

[2026-06-09] - Runtime Execution Proof Receipts

ADDED:
- Added injectable scheduler retry handoff for runtime execution-proof
  follow-through. Host-owned scheduler code can now combine an accepted
  id-only retry request with a credential vault, injected n8n API transport,
  workflow document, prior trigger receipt, and receipt root to run the
  bounded read-only execution-history retry executor without moving
  credential values into route state.
- Added id-only POST validation for the runtime execution-proof retry
  scheduler route. The route validates scheduler-owned credential ids,
  runtime proof receipt paths, and bounded retry attempts, rejects
  secret-bearing payloads without echoing field names or values, and returns a
  credential-handoff-required response until injected DX/Zed credential and
  n8n API transports are available.
- Added a governed DX/Zed scheduler action descriptor for runtime
  execution-proof retry follow-through. The Zed automation packet now exposes
  the disabled credential-handoff-required action, and
  `/api/n8n-studio/runtime-execution-proof-retry` serves the same redacted
  descriptor for scheduler discovery.
- Added a server-side runtime execution-proof receipt writer for the
  governed webhook trigger plus n8n execution-history proof flow. The writer
  materializes the known redacted proof fields as
  `n8n-studio/runtime/execution-proof-latest.sr` under an injected receipt
  root.
- Added a deterministic delayed-history retry plan for execution proof
  follow-through. The plan is read-only, requires `n8nApi` history import, and
  never calls n8n or sleeps by itself.
- Added a bounded retry executor for scheduler-owned follow-through. The
  executor accepts an injected read-only history importer, records retry plans
  while proof is unavailable, stops when imported proof is available, and
  writes the final redacted proof receipt.
- Exposed the pending runtime execution-proof receipt path in the Zed
  automation packet and updated automation scope wording around proof receipt
  persistence and delayed history retries.

NOTES:
- The receipt writer stores no webhook URL, request body, API key, secret ref,
  provider body, or node output payload. Manual/internal trigger execution and
  live scheduler execution with credential handoff remain governed follow-up
  work.

[2026-06-09] - Webhook Execution Proof Follow-Through

ADDED:
- Added a redacted runtime execution-proof receipt that summarizes a governed
  webhook trigger request plus imported n8n execution history.
- Added credential-vault submission that combines a webhook trigger
  credential and n8n API credential: the vault loads both secrets inside the
  adapter boundary, submits the trigger request, imports execution history
  through the governed API client, and returns proof only from imported
  execution receipts.

NOTES:
- The proof receipt stores counts, ids, origin, status, and receipt paths only.
  It does not store webhook URLs, request secrets, API keys, provider response
  bodies, or full node logs.

[2026-06-09] - Governed Webhook Runtime Trigger

ADDED:
- Added a source-backed runtime trigger bridge for n8n Webhook workflows.
  The bridge creates provider-gated trigger plans from local Webhook node
  metadata and blocks manual/internal trigger execution behind n8n provider
  controls.
- Added credential-vault submission for webhook trigger URLs. The vault loads
  the secret webhook URL inside the adapter boundary, submits a sanitized
  request payload through an injected transport, and returns a redacted
  trigger receipt with origin/status metadata only.
- Exposed runtime trigger planning in readiness for Zed Plugins and DX
  automation consumers.

CHANGED:
- Updated automation scope and Zed automation packet copy so runtime handoff,
  webhook trigger requests, and imported execution receipts are described as
  separate governed steps.

NOTES:
- Webhook trigger submission requests live provider execution, but it is not
  execution proof. Node outputs, timings, and provider errors still require an
  imported n8n execution receipt.

[2026-06-09] - Credential-Owned Runtime Handoff

ADDED:
- Added credential-vault runtime handoff submission. The vault can resolve an
  n8n API credential by id, load the API key inside the adapter boundary, and
  submit the governed runtime handoff through the injected transport.

CHANGED:
- Updated vault readiness text so Zed/DX consumers can see that credential
  loading now covers API clients and runtime handoff while still excluding
  secret values and secret refs from returned Studio state.

NOTES:
- This is still n8n API credential loading only. Provider-specific node
  credentials and live node execution remain governed follow-up work.

[2026-06-09] - Governed Runtime Handoff Receipt

ADDED:
- Added `submitRuntimeHandoff` to the injected n8n API client. The method
  composes sanitized workflow publish/update, optional activation, and optional
  execution-history import into a single redacted runtime handoff receipt.
- Added runtime handoff receipt metadata that records provider write,
  provider workflow id, request count, execution-history import status,
  no-secret boundaries, and `workflowExecutionRequested: false`.

CHANGED:
- Updated automation scope and Zed automation packet language so downstream
  DX/Zed consumers can discover that publish/history handoff exists while
  live provider execution remains disabled until trigger controls and
  execution receipts are available.

NOTES:
- This still does not run workflows. n8n live execution must happen through
  provider triggers or manual provider controls, with execution proof imported
  separately through governed receipts.

[2026-06-09] - Governed Option Overlay

ADDED:
- Added a parameter overlay helper that maps configured editor-session
  dynamic option and resource-locator response plans into selected-node
  parameter fields.
- Added option and multi-option select rendering for configured parameter
  options, plus resolved resource-locator result display metadata.

NOTES:
- The overlay consumes already-governed response batches only. It does not add
  live provider calls or credential secret loading.

[2026-06-09] - Current Workflow Export UI

ADDED:
- Added current-workflow export state for idle, exporting, exported, and
  failed UI states with no-live-provider/no-secret boundaries.
- Wired the Import / export panel export button to POST the active
  `WorkflowDocument` to `/api/n8n-studio/export` through the Studio app state.

CHANGED:
- The panel now displays the latest current-export status and receipt metadata
  while keeping exported workflow payloads out of client state.

NOTES:
- This is workflow JSON export reachability only. Runtime publish and
  execution remain governed by separate credential/runtime bridges.

[2026-06-09] - Zed Receipt Export

ADDED:
- Added a receipt writer for the Zed automation bridge that materializes the
  automation-list packet and readiness snapshot under a configurable receipt
  root.
- Added focused coverage proving the written receipts keep the
  `dx.agents.zed.automation_list.v1` and `dx.n8n-studio.readiness` schemas,
  point to Zed-readable paths, and do not serialize secret field names or
  values.

CHANGED:
- Marked n8n runtime support as partial in automation scope because workflow
  publish handoff exists; execution receipt import remains the blocker for
  live run enablement.

NOTES:
- The receipt writer does not run workflows. It only publishes discovery and
  readiness evidence for Zed and DX automation consumers.

[2026-06-09] - Governed Workflow Publish Handoff

ADDED:
- Added sanitized n8n workflow publish body generation for DX
  `WorkflowDocument` state, matching n8n's own create/update body shape:
  `name`, `nodes`, `connections`, `settings`, and `staticData`.
- Extended the injected n8n API client with governed create/update publish
  requests and explicit activation requests through source-backed n8n public
  API endpoints.
- Added redacted workflow publish receipts that distinguish provider write
  submission from workflow execution proof.

NOTES:
- Publish handoff still requires an injected transport and n8n API credential
  boundary. It does not run workflows or import execution receipts.

[2026-06-09] - Current Workflow Export

ADDED:
- Added POST support to `/api/n8n-studio/export` for exporting the current
  edited DX `WorkflowDocument`.
- Added payload sanitation for current workflow exports so secret-like
  parameter fields and credential payload fields are removed before n8n-shaped
  workflow JSON is returned.

NOTES:
- This completes the API side of current-state workflow JSON export. Submitting
  that workflow to an n8n backend remains a separate governed runtime bridge.

[2026-06-09] - Zed Automation Bridge Packet

ADDED:
- Added a source-only Zed automation-list bridge helper and
  `/api/n8n-studio/zed-automation` route.
- The bridge projects n8n Studio editor-session readiness into a
  `dx.agents.zed.automation_list.v1` row with runtime unavailable, redaction
  metadata, redacted receipt references, and no executable secret-bearing
  actions.

NOTES:
- This gives Zed plugin and DX automation surfaces a native status packet
  without copying n8n's Vue editor or claiming live n8n execution.

[2026-06-09] - DX Automation Scope Contract

ADDED:
- Added an explicit automation scope contract to readiness. The contract
  names the target as n8n runtime-backed DX automation, keeps
  `editorPortTarget` false, and marks the n8n Vue editor port as a non-goal.
- Added capability states for node metadata catalog, workflow JSON authoring,
  node connection authoring, n8n runtime execution, credential bridge, and
  dynamic option/resource-locator support.

NOTES:
- This corrects the product target for Zed plugin and DX automation surfaces:
  use n8n metadata and runtime through governed DX bridges, not a Vue website
  port.

[2026-06-09] - Governed Editor-Session Request Batches

ADDED:
- Added a server request-batch helper that projects pending blocked
  editor-session request plans with selected-node metadata, request-kind
  counts, provider-boundary flags, and redaction metadata.
- Added `GET /api/n8n-studio/editor-session` for DX Agents and Zed plugin
  panel consumers to discover current editor-session work without scraping
  readiness internals.

NOTES:
- Request batches expose planning metadata only. Live provider calls, secret
  loading, and request execution remain behind separate governed adapters.

[2026-06-09] - Governed Credential Validation Responses

ADDED:
- Added selected-credential `credential-test` request plans to the
  editor-session readiness model.
- Added governed credential validation response application with
  valid/invalid/unknown status metadata, optional validation timestamps, and
  redacted operator messages.
- Added runtime decoding for credential validation responses through the
  editor-session response batch helper, including unsafe/live/secret-bearing
  payload rejection before data reaches Studio state.

CHANGED:
- The readiness bridge now reports credential validation request counts for
  DX Agents and Zed plugin panel consumers.

NOTES:
- Credential validation response application records bridge-owned validation
  metadata only. It does not load credential secrets or call live providers.

[2026-06-09] - Governed Editor-Session Response Batches

ADDED:
- Added a store-level editor-session action for applying governed transport
  response batches to the active Studio state.
- Added a server response helper and `/api/n8n-studio/editor-session` POST
  route that decode response batches, reject unsafe/live/secret-bearing
  records, apply accepted responses, and return updated readiness metadata.

NOTES:
- This is DX-owned response application plumbing only. Live provider calls,
  credential validation, and execution-backed requests remain separate guarded
  follow-up work.

[2026-06-09] - Governed Credential List Responses

ADDED:
- Added governed credential-list response application to the editor-session
  adapter. Matching credential request plans can now become configured with
  sanitized picker options and selected credential id/name metadata from
  DX-redacted transport responses.

NOTES:
- Credential list application stores credential metadata only. Secret values,
  API keys, and unexpected provider fields are dropped before data enters
  Studio state.

[2026-06-09] - Governed Resource Mapper Schema Responses

ADDED:
- Added governed resource mapper schema response application to the
  editor-session adapter. Matching mapper request plans can now become
  configured with sanitized field schemas, field words, mode metadata, and
  resolved field counts from DX-redacted transport responses.

NOTES:
- Mapper schema application remains source-state plumbing only. It does not
  call Google Sheets or any live provider, and unexpected response fields are
  dropped before data reaches Studio state.

[2026-06-09] - Governed Resource Locator Search Responses

ADDED:
- Added governed resource-locator search response application to the
  editor-session adapter. Matching request plans can now become configured
  with sanitized locator results, resolved query metadata, and pagination
  cursors from DX-redacted transport responses.

NOTES:
- This extends the response application path only. It does not add live Slack
  or provider list calls, and it continues to reject secret-bearing response
  fields from stored locator results.

[2026-06-09] - Governed Editor-Session Option Responses

ADDED:
- Added governed editor-session transport response application for dynamic
  option request plans. Responses must remain inside the provider boundary,
  declare no live provider execution, declare no included secrets, and use the
  Studio redaction policy before they can configure a request plan.
- Added sanitized resolved option storage on configured editor-session request
  plans and a fulfilled request count on editor-session readiness.

CHANGED:
- The readiness bridge now reports `fulfilledRequestCount` so DX Agents and
  Zed plugin surfaces can distinguish configured editor-session requests from
  requests still blocked behind future governed transport.

NOTES:
- This pass applies already-governed transport responses only. It does not add
  live provider calls, credential secret loading, or resource mapper execution.

[2026-06-09] - Generated Trigger Configuration Boundaries

ADDED:
- Added a source-only `triggerConfiguration` notice for generated trigger node
  descriptions, so sparse generated trigger records such as Acuity Scheduling
  Trigger no longer render as credential-only inspector surfaces.

NOTES:
- The boundary is intentionally non-executable. Detailed event fields and live
  webhook execution still require source-backed trigger adapters and governed
  editor-session/runtime transport.

[2026-06-09] - Generated Executable Operation Metadata

CHANGED:
- Generic generated node descriptions now prefer generated executable
  `actions` for operation selectors and catalog operation summaries when a
  local generated record exposes them.
- Nodes such as Chargebee now keep real executable actions like `cancel` and
  `delete` while avoiding operator-like filter values such as `is` and `gte`
  in top-level operation surfaces.

NOTES:
- This is still generated metadata rendering, not live provider execution.
  Source-backed adapters continue to override generated shells for first-party
  nodes with richer local source coverage.

[2026-06-09] - Source-Backed Discord Custom Form Fields

ADDED:
- Added source-backed Discord v2 send-and-wait custom-form field extraction
  from local n8n Form node helper sources. Custom-form responses now preserve
  `defineForm`, JSON definitions, and field-based `formFields`
  fixed-collection children such as field type, field options, required flags,
  and file/date/text-specific controls.

CHANGED:
- Extended the shared n8n source-description reader so object-spread content
  can expand shared array properties such as `options: formOptions` and
  `options: formElementTypes`, allowing Form helper fields to remain
  source-backed instead of hand-authored.

NOTES:
- Live Discord execution remains blocked by the governed execution adapter
  boundary. This pass preserves editor parameter schema and child display
  behavior only.

[2026-06-09] - Source-Backed Discord Send And Wait Parameters

ADDED:
- Added source-backed Discord v2 send-and-wait extraction from local n8n
  helper sources, preserving `sendAndWait` as a real message operation plus
  send-to resource locators, message text, response type choices, approval
  options, wait-time controls, attribution toggles, and free-text/custom-form
  response options.

CHANGED:
- Extended generated Studio boot coverage so `n8n-nodes-base.discord` exposes
  source-backed send-and-wait parameter schemas instead of falling back to the
  generic generated connector shell.

NOTES:
- The first send-and-wait pass preserves the helper-owned response controls
  and wait options. Full Form node dynamic field helper expansion remains a
  separate source-reader target. Live Discord execution remains blocked by the
  governed execution adapter boundary.

[2026-06-09] - Source-Backed Discord Message Lookup And Reaction Operations

ADDED:
- Added source-backed Discord v2 message delete, get, get many, and react
  operation extraction from local `nodes/Discord/v2/actions/message` source.
- Added generated Studio boot coverage for message IDs, channel resource
  locators, simplify options, return-all/limit controls, emoji input, and
  channel-search dynamic boundaries.

NOTES:
- Discord send-and-wait still uses n8n's generated `getSendAndWaitProperties`
  helper and remains a separate parser expansion target. Live Discord
  execution remains blocked by the governed execution adapter boundary.

[2026-06-09] - Source-Backed Discord Channel And Member Operations

ADDED:
- Added source-backed Discord v2 channel and member operation extraction from
  local `nodes/Discord/v2/actions` source. Channel schemas now include
  create, delete, get, get many, and update parameters; member schemas now
  include get many, role add, and role remove parameters.
- Added source-backed parsing for the shared n8n `returnAllOrLimit` utility
  array so Discord get-many schemas keep their real return-all and limit
  controls.

CHANGED:
- Extended generated Studio boot overlay coverage for `n8n-nodes-base.discord`
  so channel/member resource locators, parent-category search, user search,
  role dynamic options, and member simplify options are available from local
  source-backed metadata.

NOTES:
- Live Discord execution remains blocked by the existing governed execution
  adapter boundary; this change preserves editor parameter schema and dynamic
  request boundaries only.

[2026-06-09] - Source-Backed Discord V2 Message Send

ADDED:
- Added source-backed Discord v2 message-send extraction from local
  `nodes/Discord/v2/actions` source, including connection type credentials,
  message operation selection, server/channel/user resource locators, content,
  options, embed/file fixed collections, and list-search dynamic boundaries.

CHANGED:
- Extended generated Studio boot overlay coverage so
  `n8n-nodes-base.discord` uses the local Discord v2 action source for message
  send schemas instead of the generic generated connector shell.

NOTES:
- Live Discord execution remains blocked by the existing governed execution
  adapter boundary; this change preserves editor parameter schema and dynamic
  request boundaries only.

[2026-06-09] - Source-Backed Notion Search Filters

ADDED:
- Added source-backed expansion for Notion v2
  `getSearchFilters('databasePage')` and `filters(getConditions())`
  fragments from local `GenericFunctions.ts` and `Filters.ts`, preserving
  `filterType`, `matchType`, `filters.conditions`, typed condition options,
  manual filter value fields, and the `getFilterProperties` dynamic option
  boundary.

CHANGED:
- Extended Notion v2 generated boot overlay coverage so database-page `getAll`
  schemas include source-backed manual filter controls instead of stopping at
  static description arrays.

NOTES:
- Live Notion execution remains blocked by the existing governed execution
  adapter boundary; this change preserves the editor parameter schema only.

[2026-06-09] - Source-Backed Notion Block Builder

ADDED:
- Added source-backed expansion for Notion v2 `blocks(resource, operation)`
  fragments from local `Blocks.ts`, preserving the `blockUi` fixed collection,
  block type dynamic option boundary, and nested text/rich-text block controls.

CHANGED:
- Extended Notion v2 generated boot overlay coverage so database-page creation
  schemas include source-backed block-builder fields instead of stopping at
  static description arrays.

[2026-06-09] - Source-Backed Notion V2 Parameters

ADDED:
- Added source-backed Notion v2 parameter extraction from local
  `nodes/Notion/v2/VersionDescription.ts` and shared Notion description arrays.
- Added generated Studio boot overlay coverage so `n8n-nodes-base.notionV2`
  uses source-backed credentials, resource locators, database-page property
  collections, and dynamic option boundaries instead of the generic generated
  connector shell.

NOTES:
- Live Notion execution remains blocked by the existing governed execution
  adapter boundary; later entries cover function-generated block-builder and
  search-filter source-reader expansion.

[2026-06-09] - Shared Reader Google Sheets Object Spreads

ADDED:
- Added recursive object-spread expansion to the shared source-description
  reader for same-source objects and nested shared option fragments.

CHANGED:
- Refactored the Google Sheets source-backed adapter to use the shared reader
  for `readFilter`, `combineFiltersOptions`, and nested shared option children,
  removing its custom recursive object-spread parser.

[2026-06-09] - Shared Reader Gmail Array Parsing

CHANGED:
- Refactored Gmail operation-field array extraction to use the shared
  source-description reader, preserving existing dynamic option and nested
  collection schema coverage.

[2026-06-09] - Shared Reader Same-Source Array Spreads

ADDED:
- Added same-source array spread expansion to the shared source-description
  reader, including block-comment-prefixed spreads matching local OpenAI
  description files.

CHANGED:
- Refactored the OpenAI source-backed adapter to use the shared reader for
  spread-backed chat, image, and text parameter arrays.

[2026-06-09] - Shared Reader Gmail Base Metadata

CHANGED:
- Refactored the Gmail source-backed adapter to use the shared
  source-description reader for `versionDescription.properties` and credential
  extraction while preserving Gmail-specific operation-field array expansion.

[2026-06-09] - Shared Reader Google Sheets Base Metadata

CHANGED:
- Extended the shared source-description reader with leading const object
  parameter support for nodes such as Google Sheets that declare
  `authentication` outside `versionDescription.properties`.
- Refactored the Google Sheets source-backed adapter to use the shared reader
  for base parameter and credential extraction while retaining its local nested
  shared-spread expansion for operation arrays.

[2026-06-09] - Shared Source Description Reader

ADDED:
- Added a shared n8n source-description reader for source-backed node parameter adapters, covering object entries, shared object references, shared option arrays, n8n `updateDisplayOptions` merge semantics, version properties, and credential extraction.

CHANGED:
- Refactored Airtable and Postgres source-backed parameter adapters to use the shared reader while preserving their generated Studio boot overlays and existing source-only provider boundaries.

[2026-06-09] - Source-Backed Postgres Parameters

ADDED:
- Added source-backed Postgres v2 database operation parameter extraction from local `nodes/Postgres/v2/actions` TypeScript descriptions, including schema/table resource locator modes, insert/update/upsert `resourceMapper` metadata, SQL query options, select/delete fixed collections, and shared option children.
- Added generated Studio boot overlay coverage so `n8n-nodes-base.postgres` uses the source-backed v2 action description instead of the generic generated connector shell.

NOTES:
- Postgres schema/table searches, column option loads, and resource mapper calls remain source-only editor-session boundaries; no live database execution or credential secret handling was enabled.

[2026-06-09] - Source-Backed Airtable Parameters

ADDED:
- Added source-backed Airtable v2 base/record operation parameter extraction from local `nodes/Airtable/v2/actions` TypeScript descriptions, including base/table/view resource locator modes, create/update/upsert column `resourceMapper` metadata, search/sort fields, insert/update option children, and root-path display rules such as `/columns.mappingMode`.
- Added generated Studio boot overlay coverage so `n8n-nodes-base.airtable` uses the source-backed v2 action description instead of the generic generated connector shell.

NOTES:
- Airtable provider list/search and mapper calls remain source-only editor-session boundaries; no live provider execution or credential secret handling was enabled.

[2026-06-08] - Studio Foundation

ADDED:
- Added a modern DX-WWW n8n Studio example app under `examples/n8n-studio`.
- Added source-backed n8n catalog, workflow document, parameter, credential, resource locator, pinned-data, execution, AI-tool, import/export, and receipt contracts.
- Added readiness, catalog, and export route handlers.
- Added the initial Studio shell with editor surfaces mapped to the DX-native domain state.
- Added semantic node metadata modules for HTTP Request, Slack, and OpenAI.
- Added metadata-driven parameter schema construction, dynamic load boundaries, and n8n workflow import sanitation.
- Added generated local connector ingestion from `integrations/n8n-nodes-base/generated/dx-automations-connectors.json`, including deterministic node registry projection, generated resource/operation and credential parameter shells, and generated catalog response metadata.
- Added source-backed Slack message parameter extraction from `nodes/Slack/V2/MessageDescription.ts`, including spread resource locator resolution, message post fields, collection boundary metadata, and parameter inspector visibility for complex source fields.
- Added nested child-parameter extraction for Slack collection and fixed-collection fields, including attachment child fields and nested field groups.
- Added recursive source child parameter controls backed by nested collection item values for Slack attachment/fixedCollection fields.
- Added source-owned add/remove/update mutation helpers for nested collection values, selected-node schema refresh, store action wiring, and callback-gated collection control hooks.
- Added live Studio shell wiring for selected-node nested parameter mutations, passing real add/remove/update handlers through the inspector into recursive parameter controls.
- Added type-aware parameter input coercion so booleans, numbers, JSON text, and resource-locator payloads enter workflow state as typed values.
- Added resource-locator draft modeling for active modes, selected values, cached labels, source URL modes, and source-only list/search boundaries.
- Added selected-node credential picker readiness with redacted credential options and provider-boundary markers.
- Added import/export session readiness with source-only import source options, sanitized preview summaries, issue actions, non-executable import boundaries, and export receipt metadata shared by the UI panel and export route.
- Added execution debug readiness with a blocked manual-run attempt, per-node debug log rows, execution receipt boundaries, no-secret markers, and readiness API counters for bridge consumers.
- Added execution receipt import for replayable run evidence, including known-node success/error logs, timings, item counts, provider error summaries, unknown-node issue reporting, and secret-like field stripping.
- Added source-backed HTTP Request V3 parameter extraction from `nodes/HttpRequest/V3/Description.ts` and `nodes/HttpRequest/shared/optimizeResponse.ts`, including curl import, credential-select arrays, boolean and hide display rules, fixedCollection children, and tool-only optimize-response fields.
- Added n8n public API execution-history import planning from local `nodes/N8n/ExecutionDescription.ts` and `n8n-api-coverage.json`, plus sanitized conversion of `GET /executions` history payloads into governed execution receipts.
- Added an injected-transport authenticated n8n public API client boundary for `GET /workflows/{id}` and `GET /executions`, including redacted readiness metadata, sanitized workflow import previews, execution-history receipt import, and no secret persistence in returned Studio state.
- Added a credential-vault bridge for n8n API credentials that reports redacted vault readiness, resolves credentials by id through an injected secret loader, and keeps secret values and secret refs out of returned Studio state.
- Added editor-session adapter readiness for dynamic node parameter loads, resource-locator list/search requests, and selected-node credential picker data, with source-owned request plans surfaced through Studio state and the readiness API without live provider execution.
- Added source-owned canvas interaction runtime for selection, pointer drag, canvas pan, zoom, fit, tidy, guarded delete, select-all, and keyboard nudging, with selected-node inspector data refreshed from canvas selection.
- Added source-owned canvas edge handling with validated source/target handle connection creation, duplicate and self-loop rejection, dynamic edge rendering, draggable reconnect handles, and endpoint-specific source/target reconnect validation.
- Added source-owned import apply/save flows with a typed action reducer, blocker-aware preview apply, real document/canvas/inspector refresh after clean imports, source-only draft save receipt metadata, and first-party Manual Trigger metadata for common n8n imports.
- Added source-backed OpenAI parameter extraction from local `nodes/OpenAi/ChatDescription.ts`, `TextDescription.ts`, and `ImageDescription.ts`, plus default OpenAI metadata for chat fixed-collection prompts, text edit inputs, image generation fields, dynamic model picker boundaries, and source-only editor-session request planning.
- Added source-owned expression editor state with selected-parameter expression parsing, reference discovery, credential/env diagnostics, recursive field annotations, and a real inspector panel backed by Studio state.

NOTES:
- Live provider execution is intentionally blocked until the n8n adapter and credential receipts are real; imported execution history is treated as replayable evidence only.
