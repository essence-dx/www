# AI SDK Forge Package

Official DX package name: `AI SDK`
Package id: `ai/vercel-ai`
Aliases: `vercel-ai`, `ai-sdk`, `@vercel/ai`
Upstream package: `ai`
Upstream version: `7.0.0-canary.146`
Source mirror: `G:\WWW\inspirations\vercel-ai`
Provenance: Vercel AI SDK, Apache-2.0, https://github.com/vercel/ai and https://ai-sdk.dev/docs

## Real Upstream Surface

This Forge slice is based on the public AI SDK package surface, not a fake adapter.

- `streamText` for server-owned streaming text generation.
- `convertToModelMessages` and `DefaultChatTransport` for UI message transport.
- `tool` for tool definitions.
- `gateway`, `createGateway`, and `createProviderRegistry` for provider freedom.

## Exported Files

- `lib/ai/model.ts`
- `lib/ai/chat-route.ts`
- `lib/ai/client-chat.tsx`
- `lib/ai/provider-freedom.ts`
- `lib/ai/dashboard-readiness.ts`
- `lib/ai/text-stream.ts`
- `lib/ai/ui-message-stream.ts`
- `components/ai/ai-launch-assistant.tsx`
- `lib/ai/metadata.ts`

Generated metadata also exposes `documentation.packageDoc`,
`documentation.dashboardWorkflow`, and `documentation.launchProof` so DX CLI,
Zed, and the dashboard starter can discover the package doc and visible proof
without scanning prose.

DX CLI and Studio discovery metadata identify this as the AI SDK Launch
Assistant, including `AI_PROVIDER_API_KEY`, `AI_GATEWAY_API_KEY`, `/api/ai/chat`,
`components/launch/ai-chat-status.tsx`, and the prompt/action provider-readiness
markers.

The launch route contract records a `vercelAiLaunchAssistant` package workflow
receipt so Friday/Zed can review the assistant as a dashboard workflow with a
materialized component, source guard, env boundary, and stable selectors.
That receipt is included in `launchRouteMaterializedFiles` as
`.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json`.
The receipt artifact exists in the launch template and records upstream APIs,
local readiness interactions, stable selectors, env gates, app-owned boundaries, and
runtime-deferred features.

## App-Owned Boundary

Apps own model selection, provider credentials, prompt policy, token limits, moderation, persistence, rate limits, and observability. The Forge package exposes typed seams and honest readiness states; it does not import credentials from other tools and does not simulate hosted AI success.

Required env:

- `AI_PROVIDER_API_KEY`

Optional gateway env:

- `AI_GATEWAY_API_KEY`

## Dashboard Workflow

The dashboard starter consumes this package through `examples/dashboard/src/components/AiLaunchAssistant.tsx`. The visible workflow shows provider readiness, required env, upstream public API names, a prompt field, and a safe local missing-config receipt.

The `/launch` App Router dashboard consumes the package through
`examples/template/ai-chat-status.tsx`. It provides a real dashboard
assistant workflow with provider selection, a prompt textarea, a safe route
contract action against `/api/ai/chat`, and an honest missing-config receipt
when provider credentials are absent.

The materialized static `/launch` runtime proof uses the same assistant
workflow identity in `the static launch runtime template` and
`tools/launch/runtime-template/assets/launch-runtime.ts`, so generated apps
do not fall back to the older package-card marker.

Generated provider-backed `/api/ai/*` route stubs now return explicit `501`
JSON receipts with `runtimeExecution: false`, `modelStreaming: false`,
`providerRuntime: false`, and `secretValues: []`. The default launch proof only
covers `/api/ai/chat` as a dry-run route contract. Extended routes such as
`/api/ai/agent`, image, object, speech, text stream, transcription, UI stream,
file upload, and video stay outside the default launch AI proof surface until
an app owner sets
`DX_ENABLE_EXTENDED_AI_ROUTES=true` and the required `AI_PROVIDER_API_KEY` or
`AI_GATEWAY_API_KEY`. A configured application can then replace or explicitly
enable those mounts for `streamText`, UI message streaming, tool agents, image,
object, speech, transcription, file upload, or gateway video generation.
The source-owned text-stream and UI-message stream bridges still stop at
missing-config when `AI_PROVIDER_API_KEY` is absent, so enabling the extended
route flag alone is not treated as provider readiness.
The runtime dashboard treats that `501` as a visible missing-config state, not
as a fake successful model response or an opaque route failure.
The static runtime also mirrors `data-dx-ai-request-id` from the preview click
to the visible receipt so Zed Web Preview can map the request back to the
source-owned route contract.
Empty prompts stay local: the React component and materialized runtime mark
`data-dx-ai-prompt-state="empty"` and report `invalid-prompt` before any route
preview request is made.

Stable launch markers:

- `data-dx-package="ai/vercel-ai"`
- `data-dx-component="launch-ai-assistant-dashboard-workflow"`
- `data-dx-component="dashboard-ai-launch-assistant"`
- `data-dx-component="launch-ai-dashboard-workflow"`
- `data-dx-dashboard-workflow="launch-ai-assistant"`
- `data-dx-dashboard-workflow="prompt-action-provider-readiness"`
- `data-dx-ai-route-contract="/api/ai/chat"`
- `data-dx-ai-dashboard-workflow="launch-risk-review"`
- `data-dx-ai-action="safe-local-preview"`
- `data-dx-ai-action="safe-stream-contract-preview"`
- `data-dx-ai-action-state`
- `data-dx-node-modules="forbidden"`
- `data-dx-ai-prompt-state`
- `data-dx-ai-request-id`

DX Icons are used through `<dx-icon name="pack:ai" />`, `<dx-icon name="pack:settings" />`, and `<dx-icon name="pack:play" />`.

The generated Forge component uses token-oriented DX class names such as `dx-ai-assistant-panel`, `dx-provider-options`, `dx-prompt-field`, and `dx-primary-action` so the official template can inherit theme output instead of embedding package-specific colors.

## DX-Style Compatibility

The visible AI SDK launch assistant declares
`data-dx-style-surface="ai-sdk"` and `data-dx-token-scope="ai/vercel-ai"`.
The package receipt and package-status row expose
`dx.forge.package.dx_style_compatibility` with token source
`styles/theme.css`, generated CSS `styles/generated.css`, and
`runtime_proof: false`.

Rust `dx check` reports `ai_sdk_dx_style_compatibility_present` when that
source-marker evidence is present and
`ai_sdk_dx_style_compatibility_missing` plus
`ai-sdk-missing-dx-style-compatibility` when a package-status row loses the
style contract. This is source and receipt evidence only; live model streaming,
provider-specific theme review, accessibility QA, and browser visual proof
remain app-owned.

## dx-check Visibility

The launch assistant receipt exposes `dx.forge.package.dx_check_visibility`
for present, stale, missing-receipt, blocked, and unsupported-surface states.
It monitors the AI chat route, dashboard readiness helper, dashboard assistant
component, and launch assistant workflow.

The receipt also carries SHA-256 hashes for the lane-owned Forge AI SDK slice,
launch assistant source, this package document, and inspected upstream
stream/transport files. dx-check, Zed, and DX Studio can use those hashes to
mark a receipt stale without claiming provider-runtime proof. Shared launch
catalog metadata is checked for AI SDK visibility but is not hashed by this
package receipt, so unrelated package-lane catalog edits do not stale the AI
SDK receipt.

The shared package-status read model now consumes this receipt through
`aiSdkPackageVisibility` in
`examples/template/forge-package-status-read-model.ts`. It publishes the
`ai_sdk_*` dx-check metrics and registers the Zed/DX Studio surfaces
`ai-sdk:chat-route`, `ai-sdk:dashboard-readiness`,
`ai-sdk:dashboard-assistant-component`, and
`ai-sdk:launch-assistant-dashboard-workflow`.

Rust `dx check` now consumes the same package-status row through
`core/src/ecosystem/project_check/ai_sdk_dx_check.rs`. It emits
`ai_sdk_package_present`, `ai_sdk_receipt_present`, `ai_sdk_receipt_stale`,
`ai_sdk_missing_receipt`, `ai_sdk_blocked_surface`,
`ai_sdk_unsupported_surface`, `ai_sdk_hash_manifest_present`, and
`ai_sdk_hash_mismatch`, with findings for stale receipts, missing receipts,
blocked credential/runtime boundaries, unsupported surfaces, and stale
hash-backed materialized files. Upstream source hashes remain provenance
metadata; local app hash checks apply to front-facing materialized files through
the shared `file_hashes` helper so the AI SDK lane follows the same byte-level
receipt checker as other Forge package lanes while still skipping upstream,
`core/`, and `docs/` provenance hashes.
The package-owned Rust fixture
`ai_sdk_hash_mismatch_metric_and_finding_are_byte_derived` mutates a temporary
materialized launch assistant file and proves `ai_sdk_hash_mismatch` plus
`ai-sdk-hash-mismatch` flip together from bytes, not intent.

The shared DX Studio/check-panel AI SDK package row now consumes the same
package-status evidence through `core/src/ecosystem/dx_check_receipt.rs`. It
renders official `AI SDK` naming, upstream `ai` provenance, selected launch
assistant surfaces, receipt status, runtime limitations,
`ai_sdk_hash_manifest_present`, `ai_sdk_hash_mismatch`,
`ai_sdk_receipt_hash_refresh_current`,
`ai_sdk_receipt_hash_refresh_stale`,
`ai_sdk_receipt_hash_refresh_missing`,
`ai_sdk_dx_style_compatibility_present`, and
`ai_sdk_dx_style_compatibility_missing` in
`check_panel.view_model.package_lane_rows` without claiming live model streaming
or browser proof. The row filters `upstream:`, `core/`, and `docs/` provenance
hashes before comparing app-file bytes, while stale or missing
`receipt_hash_refresh` helper evidence marks the package row stale without
pretending `ai_sdk_hash_mismatch` came from anything except byte-derived app-file
checks.

The static /launch AI SDK package-lane marker is present in
`the static launch runtime template` so Zed and DX Studio can
discover `ai/vercel-ai` before `.dx/receipts/check/check-latest.json` exists.
The marker uses official `AI SDK` naming, upstream `ai` and
`G:/WWW/inspirations/vercel-ai` provenance, the launch assistant receipt path,
`data-dx-style-surface="ai-sdk"`, and `data-dx-token-scope="ai/vercel-ai"`;
its default state is `missing` / `missing-receipt` because runtime proof still
depends on app-owned provider credentials, gateway routing, safety policy,
persistence, rate limits, billing controls, and governed browser proof.

The generated-starter materialization guard now verifies that this same AI SDK
package-lane marker survives `tools/launch/materialize-www-template.ts`
output. generated static launch HTML retains the static row markers, and
generated `public/preview-manifest.json` scopes
`launch-runtime-dx-check-panel` to `ai/vercel-ai` so Zed and DX Studio can jump
from the package filter to the check panel without scraping raw HTML. The source
edit contract and Rust Studio manifest also index the AI SDK check-panel row
markers as SOURCE-ONLY discovery evidence.
Generated `public/preview-manifest.json` now also lists
`docs/packages/ai-sdk.source-guard-runbook.json` in root
`sourceGuardRunbookFixtures` and `/launch`
`routes[].sourceGuardRunbookFixtures`, with `AI SDK` naming,
`ai` provenance, `SOURCE-ONLY`, `runtimeProof: false`, and
`ai-sdk:receipt-hash-refresh` metadata.

The package-owned receipt hash helper
`examples/template/ai-sdk-receipt-hashes.ts` now checks or refreshes
the selected AI SDK launch assistant SHA-256 evidence across the workflow
receipt, `.dx/forge/package-status.json`, and the typed package-status read
model. It also tracks the package-owned source-guard runbook fixture
`docs/packages/ai-sdk.source-guard-runbook.json` as
`source_guard_runbook_fixture` and the shared preview-manifest materializer
`tools/launch/materialize-www-template.ts` as
`preview_manifest_materializer`, so stale Studio/Zed runbook metadata or
starter preview-manifest fixture emission is visible through the AI SDK receipt
freshness chain. Run
`node tools/launch/run-template-receipt-helper.js examples/template/ai-sdk-receipt-hashes.ts --check` for a
human-readable freshness check, `--check --json` for the
`dx.forge.package.receipt_hash_refresh` report, or `--write` after reviewing
source-owned changes. The JSON report now separates direct selected-file
freshness from mirror drift with `tracked_files`, `current_files`,
`stale_files`, `missing_files`, `stale_mirror_files`,
`missing_mirror_files`, and `mirror_problem_count`, so a
materializer-only drift can blame `tools/launch/materialize-www-template.ts`
without falsely marking the selected chat, dashboard, package doc, runbook, or
upstream transport source files stale. The helper reads local files plus the configured
`G:/WWW/inspirations/vercel-ai` upstream mirror for `upstream:` provenance
hashes; it does not run live AI SDK provider calls, read `AI_PROVIDER_API_KEY`
or `AI_GATEWAY_API_KEY`, install provider packages, or claim model streaming
runtime proof.

The targetable shared-reader fixture
`dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row` now writes a
temporary AI SDK package-status row and launch assistant receipt, verifies the
current `ai_sdk_receipt_hash_refresh_*` metrics, then flips only
`receipt_hash_refresh.stale_file_count` while selected source hashes stay
current. The stale-helper proof makes the row and receipt status stale through
`ai_sdk_receipt_hash_refresh_stale` while keeping `ai_sdk_hash_mismatch = 0`.
Run
`cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row --lib`
when shared Cargo compilation is calm; this fixture is SOURCE-ONLY and does not
claim provider credentials, gateway routing, model streaming, persistence, rate
limits, billing controls, browser visual proof, or runtime AI SDK execution.

The same stale-helper proof is now published for Studio/Zed discovery as
`ai-sdk-check-panel-helper-freshness` in `source_guard_index` and the `/launch`
`source_guard_runbook_index`. `docs/packages/ai-sdk.source-guard-runbook.json`
is the package-owned runbook fixture for that contract, including the exact
Cargo command, `ai_sdk_receipt_hash_refresh_stale`,
`ai_sdk_hash_mismatch` byte-derived evidence, `ai-sdk:receipt-hash-refresh`,
dx-style markers, app-owned provider boundaries, and SOURCE-ONLY runtime
limitations without claiming live model streaming proof.

## Verification

Narrow source guard:

```powershell
dx run --test .\benchmarks\vercel-ai-receipt-hash-refresh.test.ts
dx run --test .\benchmarks\vercel-ai-dashboard-workflow.test.ts
dx run --test .\benchmarks\vercel-ai-launch-visible-proof.test.ts
dx run --test .\benchmarks\vercel-ai-official-naming.test.ts
dx run --test .\benchmarks\vercel-ai-dx-check-visibility-receipt.test.ts
dx run --test .\benchmarks\vercel-ai-package-status-read-model.test.ts
dx run --test .\benchmarks\vercel-ai-rust-hash-helper.test.ts
dx run --test .\benchmarks\vercel-ai-dx-style-compatibility.test.ts
dx run --test .\benchmarks\vercel-ai-dx-check-package-lane-panel.test.ts
cargo test -q -p dx-www-compiler dx_check_reports_ai_sdk_package_status_visibility --lib
cargo test -q -p dx-www-compiler ai_sdk_hash_mismatch_metric_and_finding_are_byte_derived --lib
cargo test -q -p dx-www-compiler ai_sdk_dx_style_compatibility_missing_is_reported --lib
cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row --lib
```

The guards fail if the upstream API evidence, Forge metadata, generated
component, dashboard usage, materialized `/launch` assistant workflow markers,
DX icon markers, no-node-modules marker, or this package document disappears.
