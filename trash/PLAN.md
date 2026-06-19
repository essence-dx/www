# DX WWW Release Readiness Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` or `superpowers:executing-plans` to implement this plan task-by-task. Release readiness is the active goal. The completed auto-import plan remains below for history.

**Readiness score:** **99/100 local receipt-backed release readiness**. `dx www readiness --json --full` is allowed to report `release_ready=true` for the local WWW release scope. Hosted/provider benchmark publication remains a separate release track.

**Active detailed plan:** `docs/superpowers/plans/2026-06-01-dev-server-mode-selection.md`

**Immediate execution order:**

- [x] Reset the readiness plan and separate implementation depth from proof maturity.
- [x] Stabilize the docs/onboarding receipt and agent-context blocker slice with focused Node tests.
- [x] Rebuild the local `dx-www` binary from current source.
- [x] Generate release-readiness local receipts through `dx www readiness --write-receipts --json --full`.
- [x] Refresh `examples/template/.dx/receipts/check/check-latest.json` through `dx check examples/template --json`.
- [x] Verify focused docs/readiness/agent-context tests and targeted Rust receipt test.
- [x] Verify `cargo check -j 6 -p dx-www --no-default-features --features cli --bin dx-www --message-format=short`.
- [x] Add typed `dx dev` server modes: `auto`, `axum`, and `may-minihttp`.
- [x] Keep Axum for hot reload, devtools, route handlers, and server sources.
- [x] Add the tiny may-minihttp-style dev path for static projects that do not
  need Axum-only capabilities.
- [x] Add `examples/template-axum` and `examples/template-may-minihttp` as
  source-owned template examples with `.ts`/`.tsx` app-side code and no
  `.js`, `.cjs`, or `.mjs` template scripts.
- [x] Run the focused tiny-static/public-vs-evidence source guard. This is still not live browser/Astro parity proof.
- [x] Import the latest controlled local tiny-route benchmark receipt into canonical `.dx` readiness proof:
  WWW is rank 1 at `3084.37` median RPS and also smallest first response at
  `474` bytes for `/fair-counter`.
- [x] Expose `first_response_bytes` in the readiness and agent-context benchmark
  read models so the smallest-byte claim comes from benchmark fixtures, not
  manual README text.
- [x] Promote the current local proof graph to release-ready for the WWW scope.
- [ ] Continue hosted/provider and cross-browser proof as post-release hardening, not as blockers to the current local release claim.
- [ ] Keep broader hosted/provider speed leadership tied to replayable provider receipts.

**Benchmark governance rule:** `READINESS_CURRENT_HONEST_SCORE = 99` is scoped to local receipt-backed WWW release readiness. The controlled local benchmark can state that WWW wins the measured route; hosted/provider leadership requires separate receipts.

---

# World Integration Adapter Suggestions

> These are framework-level suggestions discovered from `examples/world`. They are not automatic Rust changes yet. The example should prove the shape first; then we can decide which pieces belong in WWW, Forge, DX Env Firewall, or app templates.

## Goal

Make WWW connect cleanly to the production web stack without hardcoding providers into the framework. The first implementation lives in `examples/world` as provider contracts, env contracts, route contracts, redacted readiness status, receipt expectations, and a TypeScript live-connection runner for the top three targets in every `WORLD.md` category.

## Suggested Framework Additions

- [ ] Add a stable `WorldProviderCard` contract with `package_id`, category, provider, env vars, secret scopes, runtime boundaries, route handlers, receipt paths, replay commands, rollback plan, docs path, and next action.
- [ ] Standardize the provider adapter contract so every provider declares safe probe shape, env scope, receipt schema, side-effect policy, and promotion rules before it can enter `dx world check`.
- [ ] Add `dx world check` as a receipt helper after `examples/world` stabilizes. It should read provider cards, run preview checks, import live-provider receipts, and never print secret values.
- [ ] Extend DX Env Firewall with provider-aware env templates for the top three targets in each `WORLD.md` category, including `NEXT_PUBLIC_*` declarations only when browser exposure is explicit.
- [ ] Add provider route-handler conformance receipts: `GET` readiness, `HEAD` parity where useful, `OPTIONS`, explicit `405`, JSON errors, redaction, idempotency for writes, and webhook signature verification.
- [ ] Add preview/live promotion rules: every provider starts as `preview-only`, moves to `configured-readiness` when env is present, and becomes `live-validated` only after a replayable provider receipt exists.
- [ ] Add `.sr` and `.machine` receipt schemas for world connections: `dx.forge.world.connection`, `schema`, `migration`, `auth`, `security`, `webhook`, `payments`, `storage`, `search`, `vector`, `ai`, `queue`, `workflow`, `deploy`, `provider-live-proof`, and `preview-only`.
- [ ] Add a redacted `dx world agent-context --json` surface so AI workers can see provider names, missing/current env status, route contracts, and next actions without seeing values.
- [ ] Add Studio/devtools world panels only after provider receipts exist: provider cards, missing env, route replay, webhook proof, live validation state, and stale receipt reasons.
- [ ] Keep package installation app-owned. WWW and Forge should expose contracts and verified package paths; they should not force `node_modules` or provider lock-in inside the minimal template.
- [ ] Promote the `examples/world/lib/world/connections` runner shape into a `dx world check` command after the route-handler runtime can execute imported server helpers safely.
- [ ] Add a provider probe registry contract that records endpoint class, read/write risk, env scopes, timeout, retry policy, redaction policy, and receipt schema before a probe can run.
- [ ] Promote Firebase Firestore into a real provider receipt path with three modes: public-rules REST proof, Firebase Auth ID-token proof, and server-owned Admin/service-account proof. Never claim CRUD from Web config alone when rules return 403.
- [ ] Promote the Neon HTTP SQL CRUD smoke into a reusable database provider receipt: model server-only connection strings, temporary table cleanup, row-level write proof, and no-secret output as a first-class `dx world check database neon` command.
- [ ] Add a Supabase database CRUD promotion path that is separate from Storage CRUD: require an existing exposed table contract, a DX migration receipt, or a provider management/database credential before claiming table create/read/update/delete proof.
- [ ] Promote the Turso/libSQL HTTP pipeline CRUD smoke into a reusable database provider receipt with token redaction, temporary table cleanup, and a read-only `SELECT 1` preflight before mutating proof.

## Route Handler Runtime Gaps

- [ ] `imported-helper-execution`: route handlers need source-owned execution for imported TypeScript helpers before `examples/world/app/api/**/route.ts` can call shared provider probe modules instead of returning inline preview payloads.
- [ ] `server-only-env-injection`: route handlers need redacted server-only env presence from DX Env Firewall so providers can distinguish missing config from configured readiness without exposing secret values or browser-public keys accidentally.
- [ ] `route-handler-fetch`: route handlers need bounded `fetch` support for read-only provider probes, including timeout, method, header, body, redirect, and redaction rules that can be represented in receipts.
- [ ] `provider-receipt-import`: WWW needs a provider receipt import command that promotes a provider from configured readiness to live validation only after replayable evidence exists, while generated `.dx/receipts/world/**` artifacts remain uncommitted by default.
- [ ] `provider-side-effect-policy`: route handlers and provider adapters need an explicit no-mutation mode for checks so account creation, deploys, payments, messages, emails, queue sends, and writes cannot happen accidentally.
- [ ] `server-only-fetch-redaction`: provider route receipts must redact request headers, response bodies, query values, and env-derived host labels while preserving enough evidence for replay.
- [ ] `supabase-table-crud-schema-proof`: Supabase project API keys can prove Storage CRUD and query existing exposed REST tables, but schema/table creation needs a migration or database-level credential. WWW should model this as a separate schema-proof receipt instead of pretending every API key can create tables.

## Category Coverage To Preserve

- Database: PostgreSQL, Neon, Turso/libSQL.
- ORM/query tooling: DX ORM / Forge database, Drizzle, Prisma.
- Authentication: DX Auth / Forge auth, Better Auth, Clerk.
- Authorization: PostgreSQL RLS, OpenFGA, Casbin.
- Payments: Stripe, Lemon Squeezy, Paddle.
- Storage/media/CDN: AWS S3, Cloudflare R2, Vercel Blob.
- Search: Meilisearch, Typesense, Algolia.
- Vector and AI data: pgvector, Pinecone, MongoDB Atlas Vector Search.
- AI/runtime: OpenAI, Anthropic, Google Gemini.
- Realtime: WebSocket/SSE, Supabase Realtime, Ably.
- Queues/workflows: Cloudflare Queues, Upstash QStash, Temporal.
- Cache/config/rate limits: Redis/Valkey, Upstash Redis, Cloudflare KV.
- Analytics: PostHog, Plausible, Vercel Analytics.
- Observability: OpenTelemetry, Sentry, Datadog.
- Content/CMS: Content collections/MDX, Sanity, Strapi.
- Notifications: Resend, Twilio, Firebase Cloud Messaging.
- Deployment: Vercel, Cloudflare, Fly.io.
- Security/provenance: DX Env Firewall, GitHub Advanced Security, Sigstore.
- Team tools: GitHub, Linear, Notion.
- Product surface: FormatJS/Intl, Google Maps, React Hook Form shape.

## Decision Boundary

- Add to WWW framework level when the behavior is universal: env scoping, redaction, route-handler conformance, receipts, replay commands, and source maps.
- Keep at Forge/package level when the behavior is provider-specific: SDK calls, webhook formats, pricing objects, buckets, indexes, queues, workflow clients, and dashboard links.
- Keep at template/example level when it is only education or a provider catalog view.

---

# Completed DX WWW Auto-Import Production Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade `dx imports` from a readable component/import receipt helper into a Nuxt-grade, type-safe, production-gated auto-import system for DX WWW projects.

**Architecture:** Keep auto-import state source-owned and visible. The extensionless `dx` config defines import policy, `.dx/imports/import-map.json` stores machine-readable import metadata, `components/auto-imports.ts` remains the readable barrel, and generated declaration files provide IDE/type support without requiring app-local `node_modules`.

**Tech Stack:** Rust `dx-www` CLI, TSX App Router source scanning, `.sr` receipts, `.dx/imports/*` artifacts, generated TypeScript declarations, focused `node --test` and Rust unit/CLI tests.

---

## Task 1: Scan Components, Composables, And Utils

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Modify: `dx-www/src/config.rs`
- Test: `benchmarks/dx-imports-auto-import-scan.test.ts`

- [x] Add config defaults for scanning `components/`, `composables/`, and `utils/`.
- [x] Scan top-level exports and safe nested exports according to explicit config.
- [x] Classify entries as `component`, `composable`, `utility`, `styleHelper`, or `forgePackage`.
- [x] Preserve source path, import path, export names, default export name, and kind in `.dx/imports/import-map.json`.
- [x] Keep scans deterministic by sorting paths and exports.

## Task 2: Generate Type Declarations

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Modify: `dx-www/src/cli/help_text.rs`
- Create: generated project artifact `.dx/imports/imports.d.ts`
- Test: `benchmarks/dx-imports-dts-generation.test.ts`

- [x] Generate `.dx/imports/imports.d.ts` during `dx imports sync`.
- [x] Declare globals for auto-imported components, composables, and utilities.
- [x] Declare module aliases for `#imports` and `#components`.
- [x] Include source comments that point back to the real source file.
- [x] Make `dx imports check` fail when the declaration file is stale.

## Task 3: Preserve IDE Types

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Modify: `examples/template/dx`
- Modify: `examples/template/components/auto-imports.ts`
- Test: `benchmarks/dx-imports-ide-types.test.ts`

- [x] Emit declaration shapes that TypeScript language servers can read without runtime imports.
- [x] Keep generated types narrow enough for completion and hover hints.
- [x] Avoid `any` unless the source export cannot be resolved safely.
- [x] Add a receipt field that reports typed, untyped, and skipped entries.
- [x] Document skipped entries with exact source paths and reasons.

## Task 4: Include Only Used Imports

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Modify: `dx-www/src/cli/app_router_semantics.rs`
- Test: `benchmarks/dx-imports-used-only.test.ts`

- [x] Scan TSX route/component usage before writing the public barrel.
- [x] Include only symbols referenced by app routes, components, layouts, templates, or configured server/client entrypoints.
- [x] Keep a separate full catalog in `.dx/imports/import-map.json` for diagnostics.
- [x] Report unused discovered exports as advisory, not as public barrel entries.
- [x] Ensure production output never pulls unused auto-import symbols into route artifacts.

## Task 5: Support Explicit `#imports` Alias

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Modify: `dx-www/src/cli/app_router_execution/source_render.rs`
- Modify: `dx-www/src/config.rs`
- Test: `benchmarks/dx-imports-explicit-alias.test.ts`

- [x] Resolve `#imports` to the generated auto-import barrel.
- [x] Resolve `#components` to generated component exports.
- [x] Keep explicit imports valid even when automatic injection is disabled.
- [x] Expose alias resolution in route/source-render diagnostics.
- [x] Fail with a precise diagnostic if an alias points to a stale or missing generated artifact.

## Task 6: Fail Stale Maps In Check And Build

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Modify: `dx-www/src/cli/mod.rs`
- Modify: `dx-www/src/cli/app_router_runtime_command.rs`
- Test: `benchmarks/dx-imports-check-build-gate.test.ts`

- [x] Make `dx imports check` compare barrel, JSON map, declarations, and `.sr` receipt.
- [x] Make `dx check` surface stale import-map failures as framework readiness blockers.
- [x] Make `dx build` fail before route output if auto-import artifacts are stale.
- [x] Provide one recovery command: `dx imports sync`.
- [x] Keep all stale diagnostics source-owned, exact, and machine-readable.

## Task 7: Receipts And Verification

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Create or update: `.dx/imports/sync.sr`
- Create or update: `.dx/imports/check.sr`
- Test: `benchmarks/dx-imports-receipts.test.ts`

- [x] Record scan roots, entry counts, used counts, stale status, generated files, and source hashes.
- [x] Keep legacy JSON receipts only as compatibility outputs.
- [x] Use `.sr` as the durable DX state surface.
- [x] Add conflict-marker and `git diff --check` verification to the final pass.
- [x] Nuxt-grade auto-import parity is gated on IDE type proof and build-gate proof.

## Final Verification

- [x] Focused Node guards: `node --test benchmarks\dx-imports-auto-import-scan.test.ts benchmarks\dx-imports-dts-generation.test.ts benchmarks\dx-imports-ide-types.test.ts benchmarks\dx-imports-used-only.test.ts benchmarks\dx-imports-explicit-alias.test.ts benchmarks\dx-imports-check-build-gate.test.ts benchmarks\dx-imports-receipts.test.ts benchmarks\public-framework-contract.test.ts benchmarks\public-framework-tools.test.ts benchmarks\dx-run-extension-list-orchestrator.test.ts benchmarks\dx-dev-extension-toolchain-hook.test.ts benchmarks\dx-devtools-framework-integration.test.ts`
- [x] Rust format and compile: `cargo fmt --check`, `cargo check -p dx-www --bin dx-www`.
- [x] Release binary: `cargo build --manifest-path G:\Dx\www\Cargo.toml -p dx-www --bin dx-www --release --locked -j6`.
- [x] Fresh starter proof for the auto-import lane: `dx-www.exe new www`, `dx-www.exe build`, `dx-www.exe check` scored 100/green. This is not a readiness release-readiness claim; readiness no-JS/browser/provider proof receipts still decide that separately.
- [x] Production Devtools leak scan: no `devtools`, `/_dx/devtools`, `style-preview`, or `style-apply` markers in fresh `dx build` output.
