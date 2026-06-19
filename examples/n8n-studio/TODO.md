# DX-WWW n8n Studio TODO

IN PROGRESS:
- Build the DX-native n8n-compatible automation path: metadata-backed node
  search, n8n-shaped workflow JSON, connection authoring, governed n8n runtime
  execution, safe credential bridge, and dynamic option/resource-locator
  adapters.

PENDING:
- Continue semantic parameter rendering from generated n8n node type
  descriptions beyond catalog summary notices into richer node-specific
  parameter descriptions where local source metadata exposes them.
- Register a DX/Zed host runtime-context provider for the scheduler route
  factory so production hosts can supply vault, transport, workflow, trigger
  receipt, and receipt-root inputs outside the public route module.
- Expand credential bridge from generic provider credential validation receipts
  into provider-specific node credential loading policies and validation
  adapters.
- Register provider-specific host executors for governed dynamic options,
  resource locators, mapper schemas, and credential metadata request plans.
- Expand workflow import sanitation against generated node descriptions and credential requirements.

COMPLETED:
- Added governed host-executed editor-session request batches. The server
  helper can now collect pending editor-session request plans, let trusted
  DX/Zed host adapters execute a bounded request batch, apply returned records
  through the governed no-secret response pipeline, and expose the capability
  through automation readiness. Guarded by
  `dx run --test .\benchmarks\n8n-studio-editor-session-transport.test.ts .\benchmarks\n8n-studio-automation-scope.test.ts --json`.
- Added provider credential validation execution to the credential vault
  bridge. The vault can now resolve any credential record by id, load the
  secret only inside the adapter boundary, call an injected provider
  validator, and return a redacted
  `dx.n8n-studio.credential-validation.receipt` with provider response bodies,
  secret refs, and secret values omitted. Automation scope now marks the
  credential bridge as partial instead of metadata-only. Guarded by
  `dx run --test .\benchmarks\n8n-studio-credential-vault.test.ts .\benchmarks\n8n-studio-automation-scope.test.ts --json`.
- Added semantic generated-only node summary rendering across the local
  generated catalog. Generic generated nodes now expose a non-expression
  `generatedNodeSummary` notice with description, source file, runtime mode,
  auth kind, provenance, and governed adapter boundaries, and the inspector
  renders notice fields plus parameter descriptions as first-class read-only
  UI. Guarded by
  `dx run --test .\benchmarks\n8n-studio-generated-metadata.test.ts --json`.
- Added a host-injected scheduler route factory for runtime execution-proof
  retries. Trusted DX/Zed host code can now compose the public GET/POST route
  with a runtime-context resolver that supplies the credential vault, n8n API
  transport, workflow document, prior trigger receipt, and receipt root, while
  the default public route keeps returning the redacted
  credential-handoff-required response when no host context is registered.
  Guarded by
  `dx run --test .\benchmarks\n8n-studio-runtime-scheduler.test.ts --json`.
- Added injectable scheduler retry handoff for runtime execution-proof
  follow-through. Accepted id-only scheduler requests can now run the bounded
  read-only history retry executor through caller-supplied credential vault,
  n8n API transport, workflow document, prior trigger receipt, and receipt
  root inputs without giving the public route any credential values or
  transport ownership.
- Added id-only POST validation for the runtime execution-proof retry
  scheduler route. The route now accepts the advertised `apiCredentialId`,
  runtime proof receipt path, and bounded `maxAttempts` envelope, rejects
  secret-bearing scheduler payloads without echoing field names or values, and
  returns an explicit credential-handoff-required response while live
  scheduler execution remains disabled.
- Exposed the runtime execution-proof retry executor as a governed DX/Zed
  scheduler action contract. The Zed automation packet now carries a disabled
  credential-handoff-required action, and
  `/api/n8n-studio/runtime-execution-proof-retry` returns the same redacted
  action descriptor without serializing credential values, webhook URLs,
  provider payloads, or secret-like field names.
- Added a governed runtime execution-proof retry executor. The executor accepts
  an injected read-only n8n execution-history importer, runs bounded attempts
  without owning timers or secrets, records retry plans for delayed history,
  stops when imported proof becomes available, and writes the final redacted
  proof receipt.
- Added durable runtime execution-proof receipt persistence and delayed
  history retry planning. The server bridge can now write the redacted
  `dx.n8n-studio.runtime-execution-proof.receipt` to the injected receipt root,
  produce a deterministic read-only n8n execution-history retry plan, and
  expose the pending proof receipt path in the Zed automation packet without
  storing webhook URLs, API keys, request bodies, provider bodies, or secret
  refs.
- Added webhook runtime execution proof follow-through. Credential-vault
  runtime trigger submission can now combine the webhook trigger credential
  and n8n API credential, submit the trigger request, import n8n execution
  history, and return a redacted execution-proof receipt when imported
  execution history supplies the proof.
- Added governed webhook runtime trigger requests. Webhook workflows can now
  produce a provider-gated trigger plan from local n8n Webhook source
  semantics, submit the secret webhook URL through credential-vault-owned
  loading, and receive a redacted trigger request receipt that records live
  execution was requested while requiring imported execution receipts for
  proof.
- Bridged runtime handoff through the credential vault. The vault now resolves
  n8n API credentials by id, loads the API key only inside the adapter
  boundary, submits runtime handoff through the injected transport, and
  returns redacted receipt metadata without exposing API keys or secret refs.
- Added governed runtime handoff receipts to the injected n8n API client.
  `submitRuntimeHandoff` now composes sanitized publish/update, optional
  activation, and optional execution-history import, tracks the provider
  workflow id before history lookup, and returns redacted proof metadata while
  refusing to pretend that Studio called a generic workflow run endpoint.
- Overlaid governed editor-session dynamic option responses into visible
  parameter state and rendered configured options/resource-locator results
  without claiming live provider execution.
- Wired the Import / export panel to export the current editor document
  through `/api/n8n-studio/export`, tracking redacted status and receipt
  metadata without storing exported workflow payloads in client state.
- Added Zed-readable automation and readiness receipt export for n8n Studio.
  The bridge can now write `agents/automate-list-latest.json` and
  `n8n-studio/readiness-latest.json` under an injected receipt root while
  preserving no-secret/no-live-execution boundaries.
- Updated automation scope so n8n runtime support is `partial`: workflow
  publish handoff exists, and execution receipt import remains the live-run
  blocker.
- Added governed n8n workflow publish handoff through the injected n8n API
  client. The publish path submits a stricter, sanitized n8n create/update
  body, supports explicit activation, and returns a redacted publish receipt
  without claiming workflow execution proof.
- Added source-owned current workflow JSON export. The export route now
  accepts POSTed DX `WorkflowDocument` payloads, strips secret-like parameter
  and credential fields, and returns n8n-shaped workflow JSON plus export
  receipt metadata for the edited workflow.
- Exposed a source-only Zed automation-list bridge packet for n8n Studio.
  `/api/n8n-studio/zed-automation` now maps editor-session readiness into a
  `dx.agents.zed.automation_list.v1` row for Zed Plugins and DX automation
  surfaces, keeping runtime unavailable and omitting secret-bearing actions.
- Added an explicit automation scope contract to readiness. DX Agents, Zed
  plugin surfaces, and the DX automation screen can now see that the target is
  n8n runtime-backed DX automation rather than an n8n Vue editor port, with
  capability states for metadata catalog, workflow JSON, connections, runtime
  execution, credentials, and dynamic options/resource locators.
- Exposed governed editor-session request batches through
  `/api/n8n-studio/editor-session` GET. DX Agents and Zed plugin panel
  consumers can now discover pending blocked request plans, request-kind
  counts, selected-node metadata, provider-boundary flags, and redaction
  metadata without reading internal Studio state or receiving secret values.
- Added governed editor-session credential validation response handling.
  Selected credentials now expose `credential-test` request plans, accepted
  governed validation responses store valid/invalid/unknown metadata, unsafe
  validation payloads are rejected by the runtime decoder, and readiness
  projections expose credential validation request counts without live
  provider execution.
- Wired governed editor-session response batches into the Studio store and
  `/api/n8n-studio/editor-session` POST route, including runtime decoding,
  unsafe response rejection, updated readiness projection, and no-live-provider
  response metadata.
- Applied governed editor-session credential-list responses to matching request
  plans, including sanitized id/name/type/source/redaction options, selected
  credential metadata updates, and no-live-provider/no-secret boundaries.
- Applied governed editor-session resource mapper schema responses to matching
  request plans, including sanitized fields, field words, mode metadata,
  resolved field counts, and no-live-provider/no-secret boundaries.
- Applied governed editor-session resource-locator search responses to
  matching request plans, including sanitized result options, resolved query
  metadata, pagination cursor preservation, and no-live-provider/no-secret
  boundaries.
- Applied governed editor-session dynamic option responses to matching
  request plans, including sanitized resolved options, fulfilled request
  counts, readiness bridge projection, and explicit no-live-provider/no-secret
  boundaries.
- Rendered generated trigger nodes with source-only trigger configuration
  boundaries so sparse generated trigger records expose an honest
  non-expression notice alongside credential pickers.
- Preferred generated executable action metadata for generic generated node
  operation selectors and catalog operation summaries, avoiding operator-like
  filter values in generated-only node operation surfaces.
- Preserved Discord v2 send-and-wait custom-form fields from local n8n Form
  node helper sources, including `defineForm`, JSON form definitions,
  field-based `formFields`, dropdown option children, required flags, and
  source-owned child display rules in generated Studio boot.
- Added source-backed Discord v2 send-and-wait extraction from local n8n
  helper sources, including operation normalization, send-to resource
  locators, message text, approval/free-text response options, wait-time
  controls, and attribution toggles in generated Studio boot.
- Expanded source-backed Discord v2 message operation extraction beyond send,
  including delete/get/get-many/react schemas, message IDs, simplify options,
  return-all/limit controls, emoji input, and channel-search dynamic
  boundaries in generated Studio boot.
- Expanded source-backed Discord v2 channel and member operation extraction,
  including channel create/delete/get/get-many/update schemas, member
  get-many/role add/role remove schemas, parent-category resource locator
  search, role dynamic options, and shared `returnAllOrLimit` fields in
  generated Studio boot.
- Added source-backed Discord v2 message-send extraction and generated boot
  overlay coverage for local action-source credentials, resource locators,
  embed/file fixed collections, and list-search dynamic option boundaries.
- Preserved Notion v2 function-generated database-page search filters from
  local `GenericFunctions.ts` and `Filters.ts`, including `filterType`,
  `matchType`, `filters.conditions`, typed condition options, and
  `getFilterProperties` dynamic option boundaries for generated Studio boot.
- Preserved Notion v2 function-generated block builder parameters from local
  `Blocks.ts`, including `blockUi`, nested rich-text/text controls, and
  `getBlockTypes` dynamic option boundaries for block/page/database-page
  creation surfaces.
- Added source-backed Notion v2 parameter extraction and generated boot
  overlay coverage for local source-backed resource operations, Notion
  credentials, resource locators, database-page property collections, and
  dynamic option boundaries.
- Moved Google Sheets same-source and nested shared object-spread expansion
  into the shared n8n source-description reader, preserving append/read schema
  coverage while removing the adapter-local recursive parser.
- Routed Gmail operation-field arrays through the shared n8n
  source-description reader while preserving its existing dynamic option and
  nested collection schemas.
- Added same-source array spread support to the shared n8n source-description
  reader and refactored OpenAI source-backed parameters to use it for local
  chat/image/text field arrays.
- Reused the shared n8n source-description reader for Gmail base metadata,
  keeping Gmail operation-field array expansion local while sharing
  `versionDescription` parameter and credential extraction.
- Reused the shared n8n source-description reader for Google Sheets base
  metadata by adding leading const parameter support and routing
  `authentication`, version properties, and credentials through the common
  helper.
- Added a shared source-description reader for n8n parameter adapters, covering shared object references, shared option arrays, n8n display-option merge semantics, version properties, and credential extraction for Airtable/Postgres follow-on adapters.
- Added source-backed Postgres v2 database operation extraction, including schema/table resource locators, SQL query options, select/delete fixed collections, and insert/update/upsert resource mapper modes.
- Added source-backed Airtable v2 base/record operation extraction, including base/table/view resource locators, record column resource mapping, child option display rules, and generated-boot registry overlay.
- Added the initial DX-WWW app contract, catalog-backed state model, Studio shell, route handlers, and source guards.
- Preserved nested editor-session dynamic parameter paths for source-backed child fields such as Google Sheets read filters.
- Applied child parameter display rules for source-backed schemas, including root-path rules such as `/columns.mappingMode` and stricter top-level source property reads.
- Preserved nested Google Sheets shared option spread variants for append option children, including the duplicate `handlingExtraData` display-rule shape.
- Added generated-boot coverage proving the app-level merged registry preserves Google Sheets read filters and shared read-option child controls.
- Imported Google Sheets shared read-option fragments from local `commonDescription.ts`, preserving data-location and output-formatting child controls in the read schema.
- Added same-file object-spread resolution for Google Sheets read-operation parameters so `filtersUI`, nested lookup-column dynamic options, and `combineFilters` render from local n8n source.
- Expanded source-backed Google Sheets v2 operation extraction beyond append, including sheet append-or-update, clear, create, delete, read, update, and spreadsheet create/delete description arrays in both local tests and generated Studio boot.
- Added editable `resourceMapper` mode/value controls for Google Sheets column mapping in the parameter inspector.
- Exposed `resourceMapper` schema requests through editor-session readiness and the readiness bridge without live provider execution.
- Rendered `resourceMapper` parameters as source-only mapper boundaries with method/dependency/live-provider markers.
- Added source-backed Google Sheets v2 append parameter extraction with `resourceMapper` schema boundaries and generated-boot registry overlay.
- Added a generated-catalog-aware DX store factory with injected semantic registry support for node creator, parameter, and import/export actions.
- Wired the export API to generated catalog boot context while keeping exported workflow JSON source-owned, redacted, and non-executable.
- Wired the readiness API to the local generated connector boot state so DX Agents/Zed bridge consumers see generated metadata and full catalog node counts without enabling live provider execution.
- Added source-owned semantic metadata for HTTP Request, Slack, and OpenAI with display-rule filtering, credential selects, resource locator modes, and import sanitation.
- Added generated local connector ingestion for the 536-record n8n generated catalog, including deduped node type descriptions, generated resource/operation/credential parameter shells, catalog projection, and catalog response wiring.
- Added source-backed Slack message parameter extraction from local n8n TypeScript source, including spread resource locator resolution and complex collection rendering-boundary metadata.
- Added nested source child-parameter extraction for Slack collection/fixed-collection fields and parameter inspector summaries for complex child fields.
- Added nested collection item state and recursive parameter controls for source-backed Slack collection/fixedCollection values.
- Added immutable nested parameter mutation helpers, selected-node schema refresh, store action wiring, and callback-gated collection controls.
- Wired nested parameter mutations into the live Studio shell so selected-node add/remove/update actions update source-owned client state.
- Added type-aware parameter value coercion for checkbox booleans, numeric input, JSON fields, and structured resource-locator payloads.
- Added field-level resource-locator draft state, source-preserved URL modes, mode/value controls, and explicit source-only list/search adapter boundaries.
- Added selected-node credential picker readiness with id/name-only credential options, required credential status, and no secret-value storage.
- Added import/export session state with source-only import source options, sanitized preview summaries, issue actions, non-executable import boundaries, and export receipt metadata.
- Added execution debug readiness with a blocked manual-run attempt, node log rows, execution receipt boundaries, and readiness API counters.
- Added execution receipt import that maps replayable receipt data onto known workflow node logs, strips secret-like fields, drops unknown-node logs, and keeps live provider execution false.
- Added source-backed HTTP Request V3 extraction from local n8n TypeScript source, including curl import, credential-select metadata, boolean/hide display rules, fixedCollection query/header/body fields, and tool-only optimize-response parameters.
- Added n8n public API execution history import planning and sanitized conversion from `GET /executions` history payloads into governed execution receipts.
- Added an injected-transport authenticated n8n public API client boundary for workflow import and execution history import, with redacted readiness metadata and no secret persistence in returned Studio state.
- Added a credential-vault bridge that exposes redacted credential readiness, resolves n8n API credentials by id through an injected secret loader, and keeps secret values and secret refs out of returned Studio state.
- Added editor-session adapter readiness with source-owned request plans for dynamic parameters, resource locator search, and selected-node credential picker data, surfaced through Studio state and the readiness API without live provider execution.
- Added source-owned canvas pointer and keyboard interaction runtime for node selection, drag, pan, zoom, fit, tidy, guarded delete, select-all, and keyboard nudging, with selected-node inspector surfaces refreshed from canvas state.
- Added source-owned canvas edge handling for validated connection creation, duplicate/self-loop rejection, edge reconnect handles, source/target reconnect validation, and dynamic edge rendering.
- Added source-owned import apply/save flows that load sanitized previews, block unsafe previews, apply clean imports into the real editor state, refresh dependent surfaces, and record source-only draft save receipt metadata.
- Added source-backed OpenAI parameter extraction from local n8n TypeScript source, expanded the default OpenAI metadata to chat/text/image parameter shapes, and projected dynamic model picker requests into editor-session readiness without live provider calls.
- Added source-owned expression editor state, secret-reference diagnostics, recursive parameter field annotations, and a real expression editor panel driven by selected-node Studio state.
- Added source-backed Gmail v2 parameter extraction from local n8n TypeScript source, including message/label/draft/thread parameter arrays, `multiOptions` label fields, source credential metadata, n8n `loadOptionsMethod` dynamic option boundaries, and nested collection boundary paths for editor-session readiness.
- Added a real source-owned node creator path with query-backed catalog filtering, semantic-registry addability, mutation-backed insertion for supported first-party nodes, selected-node surface refresh, and blocked unsupported entries without fake workflow nodes.
- Added registry-aware node creator insertion for generated connector metadata so generated catalog entries such as Gmail can be added with real resource/operation/credential parameter surfaces and editor-session readiness when the generated registry is injected.
- Added registry-aware workflow import sanitation so generated-registry workflows can keep supported nodes such as Gmail, preserve valid connections and pin data, and still strip secret-like parameters without marking generated nodes as unknown.
- Added registry-aware import preview application so generated Gmail previews refresh selected parameters, credentials, expression state, and editor-session readiness when applied.
- Booted the default n8n Studio page from local generated connector metadata, with generated catalog state and a merged generated/source-backed node type registry passed into the client app.

BLOCKED:
- Manual/internal triggers still require n8n provider controls before DX can
  import execution receipts. Live scheduler-owned retry execution remains
  gated on DX/Zed host runtime context for the public route; the core handoff
  helper already accepts injected credential vault, n8n API transport,
  workflow, and trigger receipt inputs.
