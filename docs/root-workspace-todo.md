# DX-WWW Binary Web Billion-Dollar Roadmap

> Historical roadmap, not the current 30-agent release-readiness score.
> The current 30-agent worker checkpoint is 84/100, blocked by 62 Rust warnings,
> chaotic worktree state, source-guard-heavy evidence, missing browser screenshot
> and overlay recovery proof, generated artifact curation, and no post-curation
> full `cargo test` / `cargo clippy` run.

This TODO is the working plan for turning the current DX-WWW binary-web research into a real platform: source-owned packages, Forge-governed updates, tiny binary/static web output, a creator workflow, and a profitable cloud layer.

The target is not to claim victory with demo numbers. The target is to build one repeatable system that can fairly beat bloated WordPress, Webflow/Wix-style sites, and average React/Next deployments on payload size, startup cost, deployment simplicity, and source ownership.

## 0. Current Source Map

Primary staging folder:

- `G:\WWW`

Current DX-WWW binary web copy:

- `G:\WWW\www`
- Existing strengths: binary protocol crates, packet layer, browser/WASM runtime experiments, shadcn-style source component registry, demos, docs, benchmarks.
- Current status: `cargo metadata` and `cargo check --workspace` now pass after repairing missing path crates, support-crate layout, and the demo WASM artifact.

Current Flow reference snapshot copied into this folder:

- `flow\src` from `G:\Workspaces\flow\src`
- `flow\crates\flow-browser-core` from `G:\Workspaces\flow\crates\flow-browser-core`
- `flow\crates\forge` from `G:\Workspaces\flow\forge`
- `flow\crates\serializer` from `G:\Workspaces\flow\serializer`
- `flow\docs` from `G:\Workspaces\flow\docs`

Focused integration references copied into this folder:

- `integrations\flow-forge` from `G:\Workspaces\flow\forge`
- `integrations\flow-serializer` from `G:\Workspaces\flow\serializer`
- `integrations\flow-forge-bridge` from `G:\Workspaces\flow\src\forge_bridge`

Existing copied support crates:

- `related-crates\style`
- `G:\Dx\serializer`
- `related-crates\security`
- `related-crates\media-icon`
- `related-crates\markdown`
- `www\related-crates\*` Cargo-facing copies of the WWW-local support crates.

Important distinction:

- Flow Forge is currently the stronger VCS/storage/sync engine.
- Archived tool-tui Forge is the stronger DX package-orchestration idea source: green/yellow/red traffic, source-owned package variants, file-save/LSP triggers, tool ordering, and editable package updates.
- DX-WWW is the web compiler/runtime target.

The product becomes powerful only when these become one clean pipeline instead of separate experiments.

## 1. Non-Negotiable Product Thesis

Build DX-WWW as:

1. A source-owned package system that ingests npm-style packages/components and materializes only the needed files into the project.
2. A Forge-governed update system that tracks generated/owned files, classifies updates as green/yellow/red, and makes upgrades diffable and reversible.
3. A binary/static-first web compiler that emits tiny HTML fallback, tiny WASM/runtime, binary packets, binary style artifacts, and cacheable static assets.
4. A creator and hosting layer that makes production sites smaller, safer, cheaper, and easier to operate than common WordPress/React deployments.

Do not start by trying to replace every npm package, every Next.js app, or AWS. Start by winning the profitable site/creator layer, then expand.

## 2. Phase 0 - Make The Workspace Honest

- [x] Convert `G:\WWW` into a clean git repo before deeper edits.
- [x] Add a top-level `README.md` explaining the staged folders and which source is authoritative.
- [x] Add `.gitignore` for `target`, `node_modules`, `.next`, `out`, benchmark output, and local server logs.
- [x] Fix `www\Cargo.toml` workspace members:
  - [x] Either restore the missing `debug` crate or remove `debug` and `dx-www-debug` references.
  - [x] Restore the expected `icon` path or change dependencies to point to `related-crates\media-icon`.
  - [x] Normalize `dx-style`, `dx-security`, and `markdown` paths to staged `www\related-crates` folders, with `dx-serializer` promoted to the root `G:\Dx\serializer` crate.
  - [x] Fix `cli\Cargo.toml` path references that point at old nested locations.
- [x] Run `cargo metadata --no-deps --format-version 1` until it succeeds.
- [x] Run targeted `cargo check -p dx-www-browser --target wasm32-unknown-unknown`, `cargo check -p dx-www-browser-micro --target wasm32-unknown-unknown`, `cargo check -p dx-www-demo`, and then broader workspace checks.
- [x] Generate the local uncompressed `dx_www_client.wasm` and `dx_www_client_tiny_opt.wasm` files from the archived `.wasm.gz` artifacts so the demo URL resolves.
- [x] Document every remaining placeholder honestly in a `STATUS.md`.

Acceptance:

- `cargo metadata` works.
- Core crates typecheck.
- No demo or benchmark is presented as real framework proof unless it uses the actual compiler/runtime path.

## 3. Phase 1 - One Real Vertical Slice

Build one tiny but complete DX-WWW page that proves the system:

Progress:

- [x] Add a compiler-facing vertical-slice proof API that accepts `.html` page source plus local `.tsx` component sources, compiles them with the existing DX parser/binary compiler, reports missing source-owned components, and emits optimized crawlable fallback HTML.
- [x] Wire the proof API to a real CLI/dev-server smoke path instead of test-only usage.
- [x] Replace structural fallback stitching with a real nested page tree renderer.
- [x] Add source-owned Forge manifest/receipt integration for the component inputs.
- [x] Add runtime interaction proof for the counter/todo path without hand-written demo glue.
- [x] Decode the emitted binary packet back into the expected logical page tree.
- [x] Measure real bytes transferred and browser timing for this exact vertical slice.

Input:

- `.html` page file
- `.tsx` component file
- DX component import such as `dx Button`
- DX style tokens/classes
- A minimal interactive counter or todo list

Pipeline:

- Parse source with one chosen parser path.
- Resolve DX-owned components.
- Emit project-owned component files.
- Store package/component metadata in Forge.
- Compile page/component/style into one DX object model.
- Serialize to a single canonical packet format.
- Serve HTML fallback plus binary packet.
- Hydrate or patch with the browser-micro WASM runtime.
- Measure real bytes transferred and real browser timing.

Acceptance:

- Page works without hand-written demo glue.
- The counter/todo interaction is powered by the compiled runtime path, not a vanilla JS simulation.
- HTML fallback is crawlable.
- The binary packet can be decoded back into the expected logical page tree.
- The same source can rebuild deterministically.

## 4. Phase 2 - Unify The Wire Format

Current issue:

- DX-WWW has multiple binary stories: HTIP v1-style binary crate, packet v2-style crate, DXOB-style demo artifacts, binary style formats, and serializer experiments.

Active 100-point set:

- [x] Audit every currently shipped binary page/style/update format and document which layer owns it.
- [x] Choose the browser-delivered `dxp` packet boundary and separate it from compiler-only `dxob`.
- [x] Define typed `DxPageGraph`, `DxComponentGraph`, `DxStyleGraph`, `DxPacket`, `DxFallbackHtml`, and `DxSourceManifest` contracts.
- [x] Implement one canonical encoder/decoder pair with compatibility/version headers.
- [x] Update `dx prove vertical` to emit and decode the canonical packet alongside fallback HTML.
- [x] Add a browser/runtime fixture that applies the canonical packet to the fallback DOM.
- [x] Re-run the real vertical proof measurement against the canonical packet path.

Decision:

- [x] Choose one canonical `dxp` packet model for runtime page/component/style updates.
- [x] Keep `dxob` as a build artifact only if it has a clear compiler-side role.
- [ ] Use Flow serializer ideas for compact metadata/config/tool schemas where it fits.
- [x] Use binary packet format for browser runtime payloads, not generic LLM serializer text.

Required interfaces:

- `DxPageGraph`: canonical compiler IR.
- `DxComponentGraph`: component dependency graph with source-owned package provenance.
- `DxStyleGraph`: extracted tokens/classes/styles.
- `DxPacket`: browser-delivered binary page/update packet.
- `DxFallbackHtml`: SEO/accessibility fallback.
- `DxSourceManifest`: Forge-owned map from generated files to package/version/variant/provenance.

Acceptance:

- One encoder.
- One decoder.
- One compatibility/version header.
- Round-trip tests for every packet kind.
- Payload measurements always include runtime, dictionaries, decompressor, and metadata overhead.

## 5. Phase 3 - Forge-Owned Package System

This is the node_modules replacement wedge.

Core idea:

- Use package ecosystems as upstream material.
- Materialize only the needed source files into the project.
- Let Forge track ownership, provenance, version, local edits, and update safety.

Active 100-point set:

- [x] Promote `dx forge add shadcn/ui/button` into a polished `dx add ui/button` developer path while preserving explicit `dx forge` commands.
- [x] Connect Forge package materialization to the vertical proof so a page can import a source-owned package without hand-supplied component files.
- [x] Record package provenance, upstream version, content hashes, license, and generated file map in the same receipts used by the proof path.
- [x] Add update traffic classification for local edits against source-owned files: green for clean, yellow for edited, red for incompatible/security-sensitive changes.
- [x] Add one fixture that proves locally edited source-owned package files are not overwritten silently.
- [x] Add a package materialization smoke test proving no `node_modules`, lifecycle hooks, or upstream scripts are created or executed.
- [x] Re-run vertical measurement with a Forge-materialized component package in the route.

Next 100-point set:

- [x] Implement `dx update ui/button --dry-run` as a Forge change-set preview that compares current source-owned files to the curated registry package.
- [x] Implement `dx update ui/button --write` for green updates only, with yellow/red requiring explicit review flags.
- [x] Write reviewable update receipts that include before/after hashes, per-file traffic, and accepted/rejected decisions.
- [x] Add a local-edit merge fixture proving yellow files are never replaced by a blind update.
- [x] Add a red update fixture for missing/security-sensitive files and block writes by default.
- [x] Surface Forge update status in `dx check` with package counts for clean, edited, missing, and blocked files.
- [x] Re-run the vertical Forge-package measurement after the update flow and record the before/after payload and browser timing.

Next 100-point set:

- [x] Add explicit review-plan output for yellow/red Forge updates without enabling blind writes.
- [x] Implement a first selected-icon package vertical: `dx add icon search` materializes only one icon and its local helper files.
- [x] Extend source-owned package metadata with generator name, variant, last accepted update, and rollback receipt reference.
- [x] Add a package variant/fork model so local source-owned packages can be named, shared, and updated independently.
- [x] Teach `dx check` to report Forge update age, stale package counts, and rollback coverage.
- [x] Reduce warning debt in touched serializer/security public APIs and document the currently justified unsafe helpers.
- [x] Re-run the Forge-package vertical measurement after the next package vertical and compare package count, bytes, and browser timing.

Next 100-point set:

- [x] Add a combined Forge vertical route that materializes `ui/button` and `icon/search` together and records package count, file count, bytes, and browser timing.
- [x] Preserve named historical benchmark snapshots automatically instead of only overwriting `vertical-proof-measurement.*`.
- [x] Add a Forge rollback smoke command that restores source-owned files from a prior receipt.
- [x] Add a strict Forge launch gate to `dx check` that fails on stale receipts, missing rollback coverage, or red package traffic.
- [x] Add registry integrity verification for local and R2 manifests before materialization.
- [x] Generate package-facing docs for materialized files so users understand what is owned, editable, and updated by Forge.
- [x] Add a public launch limitations page that states exactly what Forge does and does not replace in npm yet.

Next 100-point set:

- [x] Add `dx forge doctor` as a single launch-readiness command that runs strict checks, registry integrity checks, receipt coverage, and package docs checks.
- [x] Add a real temp-project launch smoke that runs `dx add ui/button --write`, `dx add icon search --write`, `dx check --strict-forge`, and verifies no `node_modules`.
- [x] Add package docs coverage checks to `dx check` so missing `.dx/forge/docs` pages lower the Forge section score.
- [x] Add a docs regeneration command for existing Forge manifests without rewriting package source files.
- [x] Add a release proof report that bundles strict check output, registry verification, rollback coverage, and latest vertical benchmark history.
- [x] Add one public DX-WWW site route generated from the real compiler path that presents Forge without overclaiming npm replacement.
- [x] Re-run the combined Forge vertical benchmark after the launch-readiness command and store the historical snapshot.

Next 100-point set:

- [x] Add `dx add auth/better-auth` as a source-owned Forge package that writes only the required auth route, callback handler, config, and environment example files.
- [x] Add package docs and receipts for `auth/better-auth` that clearly explain required env vars, OAuth redirect URLs, and what Forge owns.
- [x] Add `dx check` auth-package validation for missing env examples, unsafe redirect defaults, and stale auth receipts.
- [x] Add fixture tests proving `dx add auth/better-auth --dry-run` writes nothing and `--write` creates no `node_modules`.
- [x] Add a yellow-review acceptance flow for source-owned update previews with local edits, recording explicit human approval in the receipt.
- [x] Add a red-package quarantine report that explains blocked update files without writing or deleting user source.
- [x] Add a public Forge package scorecard report covering `ui/button`, `icon/search`, and `auth/better-auth` with honest launch boundaries.

Required features:

- [x] `dx add ui/button` installs editable source, not opaque dependencies.
- [x] `dx add auth/better-auth` installs only the required auth files/config.
- [x] `dx add icon search` installs only selected icons.
- [x] `dx update ui/button` produces a Forge change set, not a blind package replacement.
- [x] Green update: no local edits and safe patch-level change, auto-apply allowed.
- [x] Yellow update: local edits or minor behavior change, human review required.
- [x] Red update: destructive change, security-sensitive change, or incompatible major change, manual resolution required.
- [x] Package metadata records upstream, version, hash, license, files, variant, generator, and last accepted update.
- [x] Local package variants can be forked, named, shared, and updated independently.

Next 100-point set:

- [x] Add `dx forge scorecard --project <path>` mode that merges the public registry scorecard with local manifest, docs, rollback, and `dx check` evidence.
- [x] Add a public `/forge/scorecard` compiler-generated route that renders the package scorecard from real scorecard data.
- [x] Add provenance, advisory, and license-review placeholder fields to Forge package metadata without claiming live vulnerability coverage.
- [x] Add package scorecard history snapshots so score drift can be tracked across releases.
- [x] Add `dx forge verify-package <id>` for one-package integrity, docs, update, rollback, and scorecard checks.
- [x] Add package scorecard evidence to `dx forge evidence` so launch reports include package scorecard status.
- [x] Run a temp-project launch smoke for `ui/button`, `icon/search`, `auth/better-auth`, `dx check --strict-forge`, `dx forge doctor`, `dx forge scorecard`, and no `node_modules`.

Next 100-point set:

- [x] Add `dx forge verify-package --all` to verify every materialized Forge package and every launch registry package in one command.
- [x] Add JSON schema/golden fixtures for Forge manifests, receipts, doctor reports, scorecards, and release proof.
- [x] Preserve release-proof history snapshots beside scorecard history so launch evidence can be compared across releases.
- [x] Add a first-class `dx forge smoke` command that runs the temp-project launch path outside unit tests.
- [x] Add Markdown snapshot coverage for public Forge scorecard/evidence output so launch copy stays honest and stable.
- [x] Add update-simulation fixtures for `icon/search` and `auth/better-auth` so non-button package updates prove receipts, traffic, and rollback behavior.
- [x] Generate a compact public Forge launch page from the real release-proof and scorecard models.

Next 100-point set:

- [x] Add stable snapshot coverage for the compact `/forge` launch page so release-proof, scorecard, and honest-boundary copy cannot drift silently.
- [x] Add a `dx forge launch-page` convenience command that wraps the compiler fixture, writes `/forge`, and reports the generated HTML, DXPK, runtime, manifest, and receipt paths.
- [x] Add package detail sections or routes for the curated Forge launch packages from the same scorecard model.
- [x] Add a public claims manifest that records every launch-page claim, its source model field, and its verification status.
- [x] Add a launch smoke that runs `dx forge smoke`, `dx forge evidence`, `dx forge scorecard`, and the compact `/forge` page generation in one temp project.
- [x] Add CI-friendly JSON output for the launch smoke with fail-under support and no secret/environment dependency.
- [x] Re-run and record the latest compact `/forge` route payload and browser timing against the current vertical proof benchmark path.

Next 100-point set:

- [x] Add a production delivery budget gate for compact `/forge` decoded bytes, Brotli bytes, HTTP median, and Chrome load-event timing using benchmark history.
- [x] Add a static/no-runtime delivery mode for routes with no interactions, while preserving canonical DXPK proof artifacts for benchmark mode.
- [x] Split the public `/forge` evidence model so non-critical claim detail can be loaded separately from the first route payload.
- [x] Add a focused `/forge` comparison report across static/no-runtime, DXPK-runtime, and package vertical modes.
- [x] Surface the latest `/forge` payload and browser timing inside `dx forge evidence` and `dx forge scorecard`.
- [x] Add launch-page accessibility, SEO, and claims-manifest smoke checks for headings, links, and verified claim consistency.
- [x] Add CI docs for the smoke and benchmark sequence without requiring R2 secrets, browser plugins, or a full workspace build.

Next 100-point set:

- [x] Add a `dx forge ci` artifact command that runs the secret-free Forge smoke path and writes reviewable CI artifacts in one directory.
- [x] Add CI workflow templates for GitHub Actions and generic PowerShell runners without requiring a full workspace build.
- [x] Add a release-readiness badge JSON output derived from smoke, evidence, scorecard, and latest `/forge` benchmark status.
- [x] Add configurable launch-budget thresholds from `dx.config.toml` while preserving the current safe environment-variable overrides.
- [x] Add failure-triage Markdown generation from smoke findings, launch-page quality findings, and budget-gate failures.
- [x] Add CLI fixture tests proving CI artifact mode creates no `node_modules`, does not require R2 env vars, and writes only expected artifact files.
- [x] Add a public `/forge/ci` route generated from the same CI evidence model so the launch site can explain how Forge is verified.

Next 100-point set:

- [x] Add schema snapshot fixtures for `forge-smoke`, `forge-triage`, `forge-readiness-badge`, and `/forge/ci` claims so public CI evidence stays contract-stable.
- [x] Add `dx forge ci --format markdown` summary output that links every generated artifact and route path consistently.
- [x] Add `dx forge ci --verify-artifacts <dir>` to validate an existing CI artifact bundle without re-running smoke.
- [x] Add launch-budget failure fixtures that prove `/forge/ci` renders failing readiness, quality, and budget states honestly.
- [x] Add docs wiring for publishing `forge-readiness-badge.json` and `/forge/ci` artifacts from GitHub Actions without secrets.
- [x] Add artifact history retention and cleanup policy docs for repeated Forge CI runs.
- [x] Re-run compact `/forge`, `/forge/scorecard`, and `/forge/ci` route measurements and record the comparison in benchmark history.

Next 100-point set:

- [x] Add a public `/forge/evidence` index route that links the launch page, scorecard, CI evidence, readiness badge, claims manifests, evidence models, and benchmark comparisons without shipping a client runtime.
- [x] Add a `dx forge public-evidence` command that exports the same evidence map as terminal, JSON, and Markdown for CI summaries.
- [x] Add route-size budget gates for `/forge/scorecard` and `/forge/ci`, not only `/forge`.
- [x] Add fixture tests proving every public evidence link points to an existing artifact and no secret markers are present.
- [x] Add a release-notes generator that summarizes Forge CI readiness, package scorecard status, route measurements, and honest launch limitations.
- [x] Verify the GitHub Pages publish bundle after generation so missing `forge-readiness-badge.json`, clean-route `/forge/ci/index.html`, claims, or DXPK artifacts fail CI before deploy.
- [x] Re-run the public evidence portal measurements and record the comparison in benchmark history.

Next 100-point set:

- [x] Add `dx forge release-dashboard --project <path>` that verifies CI artifacts, Pages publish bundle, release notes, public evidence links, and latest route comparison in one command.
- [x] Add JSON/schema snapshot coverage for Pages bundle verification, release notes, public evidence export, and public route comparison.
- [x] Add Markdown snapshot coverage for `/forge/evidence` and the four-route public comparison so public launch copy cannot drift silently.
- [x] Add a docs-first public launch checklist tying `dx forge ci`, `dx forge release-notes`, Pages publishing, and benchmark comparison into one operator flow.
- [x] Add a `dx forge public-evidence --verify <dir>` mode that validates an exported public evidence directory without regenerating routes.
- [x] Add CI workflow wiring for the release-dashboard command while keeping the default path secret-free and local/offline.
- [x] Re-run the full public launch evidence suite and record the release-dashboard result plus route comparison in benchmark history.

Next 100-point set:

- [x] Promote Forge public release history recording into a first-class `dx forge release-history` command instead of relying on the Node benchmark helper.
- [x] Add JSON/schema and Markdown snapshot coverage for public release history so dashboard score, route totals, and findings cannot drift silently.
- [x] Add release-regression checks for dashboard score drops, route payload growth, missing public routes, and failed route budgets.
- [x] Generate a public `/forge/releases` route from release history, with compact static output and no runtime asset.
- [x] Wire release-history verification into `scripts/ci/forge-ci.ps1` and `.github/workflows/forge-ci.yml` without secrets or `node_modules`.
- [x] Add docs for preserving and publishing release-history artifacts during public launch reviews.
- [x] Re-run the public release-history route measurement and record the `/forge/releases` result in the route comparison.

Active 100-point set:

- [x] Add a `dx forge release-bundle --project <path> --out <dir>` command that assembles `/forge`, `/forge/scorecard`, `/forge/ci`, `/forge/evidence`, `/forge/releases`, badges, claims, route comparisons, and release history into one verified publish folder.
- [x] Add `dx forge release-bundle --verify <dir>` so operators can validate a prepared public bundle without regenerating routes.
- [x] Add checksum/signature-style manifest output for every public launch artifact, including DXPK packets, claims, evidence JSON, release history, and route comparison reports.
- [x] Wire `/forge/releases` into the secret-free Pages preview bundle so CI can publish the full public evidence surface, not only `/forge/ci`.
- [x] Add Markdown and JSON snapshot coverage for the release-bundle manifest and Pages preview shape.
- [x] Make release-history regression thresholds configurable for expected route additions versus accidental payload growth.
- [x] Add a public launch changelog generator that turns release-history records into human launch notes without overclaiming.

Next active 100-point set: Forge launch-changelog distribution and public-review polish.

- [x] Wire `dx forge launch-changelog` into `dx forge release-bundle` so release bundles include reviewable Markdown plus machine-readable JSON.
- [x] Add a static `/forge/changelog` route generated from launch-changelog evidence with crawlable HTML, DXPK proof, claims JSON, and no runtime asset.
- [x] Extend the secret-free Pages preview bundle to include `/forge/changelog` beside `/forge/ci` and `/forge/releases`.
- [x] Add release-dashboard coverage for launch changelog readiness without turning the generator into an overclaiming marketing gate.
- [x] Add golden schema and Markdown snapshot coverage for launch changelog JSON, Markdown, Pages preview shape, and release-bundle manifest additions.
- [x] Update `scripts/ci/forge-ci.ps1` and `.github/workflows/forge-ci.yml` to preserve changelog artifacts in the normal evidence packet.
- [x] Re-run public route comparison with `/forge/changelog` and record the new route budget honestly.

Next active 100-point set: Forge production launch hardening and DX-WWW credibility evidence.

- [x] Add a `dx forge release-review --project <path>` command that joins release-dashboard, release bundle manifest, launch changelog, public route comparison, and release history into one human signoff report.
- [x] Add JSON/schema and Markdown snapshot coverage for the six-route public comparison and `/forge/changelog` launch-budget evidence.
- [x] Add a CI guard that fails if public route comparison drops below the required public surface or loses `/forge/changelog`.
- [x] Add a temp-project launch bundle smoke that runs `dx forge release-bundle`, `--verify`, release-dashboard, launch-changelog, and route comparison validation without secrets or `node_modules`.
- [x] Add public launch handoff docs for reviewing claims, changelog, bundle manifest hashes, route budgets, and Pages artifacts before publishing.
- [x] Add one honest competitor-evidence fixture comparing DX-WWW static public routes against minimal Astro/Svelte/Next static baselines without claiming broad framework replacement.
- [x] Reduce warning debt touched by the release/benchmark path, especially noisy serializer rustdoc warnings that obscure real CI failures.

Next active 100-point set: Forge real-project adoption and credible benchmark breadth.

- [x] Add a real-project Forge adoption smoke that creates a clean temp app, materializes launch packages, generates `/forge`, `/forge/scorecard`, `/forge/ci`, `/forge/evidence`, `/forge/releases`, and `/forge/changelog`, then verifies no `node_modules`.
- [x] Add a medium-route benchmark fixture with repeated cards, forms, route links, and static evidence so DX-WWW can be compared against Astro, Svelte, HTMX, and Next on more than tiny public pages.
- [x] Add a large-content benchmark fixture that stresses repeated sections and source-owned package metadata while keeping first-route payload budgets honest.
- [x] Add a Forge trust-policy file and CLI report that explains allowed packages, blocked lifecycle shapes, advisory placeholders, license review state, and package-owner responsibilities.
- [x] Add package advisory metadata fixtures for the current curated packages so public reports can distinguish "no live advisory integration yet" from missing package-review data.
- [x] Add a release-readiness trend report that compares the latest public bundle, medium route, large route, and package trust-policy scores against previous snapshots.
- [x] Add launch documentation for the real-project adoption path, including exact commands, expected artifacts, and honest claims that developers can reproduce locally.

Completed 100-point set: Forge real-app integration and live benchmark proof.

- [x] Add a source-owned example app fixture that uses `ui/button`, `icon/search`, and `auth/better-auth` together in one app route with generated package docs and strict Forge checks.
- [x] Add a real-app adoption report command that summarizes project structure, source-owned packages, receipts, package docs, release-bundle status, route artifacts, and no-`node_modules` proof for an existing project path.
- [x] Add a framework comparison harness that can optionally run installed Astro, Svelte, HTMX, and Next baselines while clearly separating live builds from deterministic static-floor fixtures.
- [x] Add browser-level smoke evidence for the example app public routes, including route load timing, static/no-runtime status, links, headings, and artifact availability.
- [x] Add a package update rehearsal for the example app proving green updates apply, yellow edits require review, red changes are quarantined, and rollback coverage remains intact.
- [x] Add a public `/forge/adoption` route generated from the real adoption report, with exact commands, artifact links, and honest launch boundaries.
- [x] Add CI artifact coverage for the example app adoption report and `/forge/adoption` route without requiring secrets, R2, `node_modules`, or a full workspace build.

Next active 100-point set: Forge adoption release channel and benchmark hardening.

- [x] Add `/forge/adoption` to the public route comparison lane with honest route-budget tracking and no live-customer adoption claim.
- [x] Add optional release-bundle coverage for `/forge/adoption` while keeping the default six-route release bundle stable for existing operators.
- [x] Add golden JSON schema and Markdown snapshot coverage for `forge-adoption-report` and `/forge/adoption` claims/proof artifacts.
- [x] Add a docs-first adoption launch checklist that ties `dx forge ci`, adoption smoke, adoption report, Pages preview, and benchmark evidence into one reproducible flow.
- [x] Add an adoption-route browser benchmark that compares `/forge/adoption` against deterministic Astro/Svelte/HTMX/Next static-floor fixtures without installing packages.
- [x] Add a source-owned package fixture review for the example app that verifies docs, receipts, rollback, advisory placeholders, and local-edit yellow states together.
- [x] Reduce CI runtime and warning noise in the Forge adoption lane so the secret-free artifact path stays practical on low-resource machines.

Completed 100-point set: Forge public beta developer onboarding and release discipline (100/100).

- [x] Add a `dx forge init-app` onboarding command that creates the clean adoption app, package docs, scorecard, and first strict check in one developer-friendly flow.
- [x] Add a public beta quickstart route and Markdown guide that starts from `dx forge init-app`, then shows `dx forge ci`, source-owned package review, and no-`node_modules` evidence.
- [x] Add a source-owned package gallery report for the curated launch packages with file maps, editable ownership boundaries, advisory placeholders, and update/rollback status.
- [x] Add a release candidate gate that combines CI artifacts, Pages verification, route comparison, source-owned review, static competitor evidence, and secret-marker scans into one pass/fail artifact.
- [x] Add a minimal migration guide for shadcn/ui users that maps `shadcn add button` expectations to Forge materialization, receipts, and local ownership.
- [x] Add beta telemetry-free diagnostics that summarize local machine/tool versions, Cargo cache use, command durations, and skipped optional browser checks without collecting secrets.
- [x] Add final public beta launch copy review that forbids universal npm/framework replacement claims while highlighting verified source-owned package security and static-route performance.

Completed 100-point set: Forge advisory provenance and package trust expansion (100/100).

- [x] Add a source-owned package provenance report that joins registry manifests, receipt hashes, accepted update receipts, license metadata, and rollback coverage into one `dx forge provenance` artifact.
- [x] Add offline advisory metadata ingestion for curated Forge packages so package-gallery and scorecard can distinguish placeholder review from real advisory coverage without network requirements.
- [x] Add signed-release-manifest preparation that separates current BLAKE3 integrity verification from future publisher identity and records explicit unsigned/signed status.
- [x] Expand curated shadcn coverage beyond `button` with a second source-owned component package and migration-guide support for that package.
- [x] Add a package trust regression fixture that mutates provenance, license, advisory, and receipt data to prove red/yellow/green trust decisions cannot silently drift.

Completed 100-point set: Forge signed publishing and hosted beta operations (100/100).

- [x] Add cryptographic publisher-key and signature verification for signed release manifests so `signed` status means verified identity, not only declared metadata.
- [x] Add a hosted-registry publish/pull smoke that exercises the R2/local-registry boundary with dry-run evidence and no required secrets.
- [x] Add a beta install/adoption script that can bootstrap a clean project from a release bundle, verify trust-regression/provenance gates, and leave a reviewable artifact trail.
- [x] Add a hosted package-gallery index artifact for public docs so adopters can inspect packages, trust signals, advisories, and migration guides without running the CLI first.
- [x] Add a release-operations report that links signed manifests, trust-regression, release-candidate, CI artifacts, and public evidence into one shipping gate.

Completed 100-point set: Forge hosted release promotion and public beta publishing polish (100/100).

- [x] Add first-class publisher key creation/signing commands so operators can sign release manifests without test-only helpers.
- [x] Wire hosted package-gallery artifacts into release bundles and release-operations promotion checks.
- [x] Add a public beta publish-plan report that maps Pages/R2/local artifacts, cache headers, rollback inputs, and no-secret requirements before deployment.
- [x] Add an installed-binary upgrade smoke that verifies a beta app can move from one signed release bundle to the next without losing local source-owned edits.
- [x] Add CI/docs wiring for release-operations and publish-plan artifacts so the hosted beta path is reproducible without local tribal knowledge.

Completed 100-point set: Forge beta distribution installability and operator UX (100/100).

- [x] Add a release bundle inspector command that summarizes signed manifest status, hosted artifacts, rollback inputs, cache policy, and package-gallery coverage from an existing bundle.
- [x] Add a downloaded beta artifact verifier that validates a release bundle plus Pages/R2 evidence without rebuilding locally.
- [x] Add operator-friendly triage for release-operations and publish-plan failures grouped by missing artifact, secret risk, cache policy, rollback readiness, and no-`node_modules` boundary.
- [x] Add portable CI snippets for GitHub Actions, local PowerShell, and generic runners that replay the same beta promotion evidence path.
- [x] Add an installability benchmark snapshot comparing beta install/upgrade time and artifact size against npm/shadcn baselines without running package installs.

Completed 100-point set: Forge beta trust packaging and adoption analytics (100/100).

- [x] Split large Forge CLI report builders into focused modules for maintainability without changing command contracts.
  - [x] Extract portable CI snippets report builder/renderers into `www\dx-www\src\cli\forge_ci_snippets.rs` with module-boundary coverage.
  - [x] Extract release triage report builder/renderers into `www\dx-www\src\cli\forge_release_triage.rs` with module-boundary coverage.
  - [x] Extract release bundle inspector report builder/renderers into `www\dx-www\src\cli\forge_release_bundle_inspect.rs` with module-boundary coverage.
- [x] Add signed snippet/report provenance so generated CI templates and benchmark snapshots can be verified as release artifacts.
  - [x] Add optional publisher-key signing for generated CI snippet provenance manifests.
  - [x] Add signed Ed25519 sidecar provenance for installability benchmark snapshots.
- [x] Add installability trend history for beta install/upgrade snapshots across releases.
  - [x] Preserve per-run installability snapshots under `forge-installability-history\snapshots`.
  - [x] Render install/upgrade time deltas and trend status in JSON and Markdown history indexes.
- [x] Add an operator dashboard summary that joins release triage, beta artifact verification, CI snippets, and installability snapshots into one review artifact.
  - [x] Add `dx forge operator-dashboard` JSON/Markdown output with joined release, beta, CI, installability, and trend checks.
  - [x] Add fixture-backed operator coverage for status, score, target counts, signed CI provenance, install/upgrade timings, and no-`node_modules` behavior.
- [x] Add a fixture-backed WordPress/static-site migration package example with honest scope, source-owned files, and no package installs.

Completed 100-point set: Forge migration adoption proof and hosted conversion workflow (100/100).

- [x] Add a static migration audit command that reads a local exported HTML/WordPress fixture, reports pages, assets, redirects, metadata, dynamic gaps, and unsafe HTML review requirements without running package installs.
- [x] Add migration package verification that checks `migration/static-site` docs, receipts, asset mapping notes, manual review warnings, source-owned files, and no-`node_modules` behavior together.
- [x] Add a hosted migration-gallery artifact that shows the static migration package, supported scope, manual gaps, package evidence, and payload comparison boundaries for public beta users.
- [x] Add a reproducible migrated-route benchmark fixture comparing a scoped static migrated page against WordPress-style and Next.js-style baselines without broad framework replacement claims.
- [x] Split the remaining adoption/migration report builders into focused modules after the migration audit command establishes the stable report contracts.

Completed 100-point set: Forge static migration conversion pipeline and preview proof (100/100).

- [x] Add `dx forge migrate-static-page --input <html-or-export-dir> --route <route> --project <path> --dry-run|--write` that turns one audited static page into source-owned migration files without running package installs.
- [x] Add a migrated-asset manifest that records source URL, copied target path, hash, byte size, cache hint, alt-text review state, and unresolved media gaps.
- [x] Add an unsafe-HTML policy gate that blocks scripts, event handlers, embeds, forms, and shortcode leftovers unless the migration output records an explicit manual-review decision.
- [x] Add a hosted migrated-route preview artifact that links the migration audit, generated source files, asset manifest, benchmark fixture, and manual-review warnings for public beta reviewers.
- [x] Add an end-to-end temp-project smoke that runs audit -> migrate-static-page -> verify-package -> migrated-route-benchmark -> package-gallery with no `node_modules`.

Next active 100-point set: Forge static migration production hardening and beta adoption (60/100).

- [x] Add multi-page static export migration planning that batches routes, slugs, metadata, and per-route review state before writes.
- [x] Add edit-preservation checks for regenerated migrated routes so local changes become yellow review state instead of silent overwrites.
- [x] Add asset copy/optimization execution that can materialize reviewed local assets into public output with cache policy evidence.
- [ ] Add public preview bundle publishing that collects migrated route previews into a single static review portal.
- [ ] Add CI snippets/docs for the static migration smoke so teams can run it before beta releases without repeated full builds.

Completed 100-point set: DX-WWW React-Familiar Developer Contract (100/100).

- [x] Add a project-contract check that makes the React-familiar `app`, `components`, `server`, `styles`, and no-`node_modules` default visible in `dx check`.
- [x] Add a strict starter app fixture that uses the recommended folders, one React-shaped route, one interactive local component, one server action, one style token file, and Forge-owned packages without `node_modules`.
- [x] Add `dx forge import npm <package> --plan` as a report-only import gate that never installs or executes package lifecycle scripts.
- [x] Add IDE/dev-server hint artifacts for missing source-owned packages without silently writing files on save.
- [x] Update onboarding docs and quickstart output so new DX-WWW developers see the React-familiar, source-owned project model first.

The 100/100 labels in the historical sets below are scoped source/contract
completion notes, not current live browser, runtime, or full-release proof.

Completed 100-point set: DX-WWW React compiler adoption and reviewed Forge import materialization (100/100).

- [x] Add a Next-familiar no-`node_modules` `dx new` template surface with `app/` route files, route-handler shape, local components, server loaders/actions, style tokens, public assets, `tsconfig.json`, DX JSX intrinsics, and a Forge template manifest.
- [x] Let the dev server render the new React-shaped starter route as a no-`node_modules` template preview while true TSX compilation is being built.
- [x] Compile the React-shaped `app/` starter route into the canonical DX page graph instead of only proving the project contract.
- [x] Map a small supported React-shaped interaction subset, such as stateful buttons and events, into adaptive static/micro-JS output.
- [x] Emit production build artifacts for compiled `app/` routes: fallback HTML, canonical DXPK packet, and page-graph JSON, still without `node_modules`.
- [x] Promote `dx forge import npm <package> --plan` into a reviewed materialization flow for one small ESM package or adapter, still blocking lifecycle scripts.
- [x] Surface project-contract hints in the dev server and future LSP path without automatic source writes on file save.
- [x] Add benchmark evidence for the React-shaped starter across static output, micro-JS interaction output, and Forge-owned package boundaries.

Next 100-point set: DX-WWW True TSX/App Router Runtime Parity (100/100).

- [x] Replace heuristic TSX route/component extraction with a real JSX/TSX syntax lowering layer that preserves familiar props, imports, children, fragments, text, and event attributes.
- [x] Compose `app/` route segments with `layout.tsx`, `loading.tsx`, `error.tsx`, and `not-found.tsx` semantics instead of only compiling a single `page.tsx`.
- [x] Compile route handlers, server loaders, and server actions into explicit DX-WWW server contracts with safe request/response serialization.
- [x] Add a client-component boundary model for `"use client"` files that selects static, micro-JS, or WASM/runtime delivery from measured complexity.
- [x] Expand import resolution to local aliases, Forge-owned files, and reviewed npm adapters while keeping no-`node_modules` strict mode as the default.
- [x] Extend style and metadata compilation for global CSS, token files, route metadata, canonical links, and asset references.
- [x] Add parity benchmarks against a matching Next.js starter route with honest caveats, no fake ecosystem/user-count claims, and reproducible local evidence.

Completed 100-point set: DX-WWW Production Next-Parity Compiler And Hosting Readiness (100/100).

- [x] Replace regex-backed metadata/import extraction with a proper TS/TSX AST pass that preserves spans, diagnostics, side-effect imports, and type-only imports.
- [x] Execute `app/api/**/route.ts` handlers in `dx dev` through a safe DX-WWW server runtime with typed request/response serialization.
- [x] Add a server-action invocation protocol for compiled client events, including CSRF/session hooks and replay-safe action receipts.
- [x] Support nested dynamic route segments, route groups, optional catchalls, params/searchParams, and route-local metadata inheritance.
- [x] Lower React-shaped TSX into the DX template graph deeply enough to preserve component props, children, fragments, conditional branches, lists, and keyed updates.
- [x] Add editor/LSP diagnostics for contract violations, import resolution, client/server boundary mistakes, and Forge package provenance.
- [x] Add a deploy adapter contract for DX-WWW hosting: immutable assets, cache headers, health checks, build manifest signing, and rollback metadata.

Completed 100-point set: DX-WWW Hosted Platform And Production Runtime Expansion (100/100).

- [x] Add a local `dx preview --production-contract` path that serves the build output exactly through the deploy-adapter contract.
- [x] Add signed build-manifest promotion with local Ed25519 keys and verification before hosted release.
- [x] Add runtime request methods/body parsing for route handlers beyond GET, still through the safe source-owned interpreter.
- [x] Add server-action POST endpoints in `dx dev` and preview using the compiled action protocol receipts.
- [x] Add a hosting provider adapter fixture for DX-WWW cloud/Vercel-like deployment metadata without requiring a provider account.
- [x] Add rollback verification that can compare two build directories and prove the previous immutable assets are still restorable.
- [x] Add production observability hooks for health, ready, route timing, packet byte sizes, and server-action receipt counts without collecting user secrets.

Next 100-point set: DX-WWW True Next Runtime Replacement Push (100/100).

- [x] Add true TSX App Router execution fixtures for nested layouts, page props, loading, error, not-found, route groups, and metadata without relying on `node_modules`.
- [x] Add client-island compilation for `"use client"` components with event handlers, keyed updates, and deterministic micro-js output.
- [x] Add server component/data-loader evaluation for async pages and loaders through the source-owned interpreter.
- [x] Add route handler parity for common Next-style request/response helpers, redirects, cookies API shims, and safe headers.
- [x] Add CSS/module/style token lowering for component-scoped styles and generated CSS-facing files.
- [x] Add streaming/deferred rendering proof points for partial HTML flush and resumable islands where DX-WWW can beat default Next output.
- [x] Add a Next project migration fixture that compiles a small App Router app into DX-WWW output with zero `node_modules` at runtime.
- [x] Add parity score evidence comparing DX-WWW output against a matching Next fixture on routes, bytes, hydration, server actions, and security posture.

Next 100-point set: DX-WWW Production Next Compatibility And Forge Hosting Beta (100/100).

- [x] Add a `dx migrate next --plan` command that inventories a real Next App Router project, reports unsupported APIs, and maps each migration step to source-owned DX-WWW files.
- [x] Add `next/link`, `next/image`, `next/headers`, `next/cookies`, and `next/navigation` adapter fixtures with strict no-`node_modules` runtime proofs.
- [x] Add a hosted preview contract that turns `.dx/build` plus Forge receipts into a Vercel-like account-free deployment bundle.
- [x] Add parity fixtures for dynamic routes, route groups, metadata files, middleware-like redirects, and mixed static/server output.
- [x] Add a Forge hosting manifest that records cache headers, rollback inputs, signed manifests, observability endpoints, and provider portability in one release gate.
- [x] Add developer-facing diagnostics that explain exactly why a Next project cannot yet compile under strict DX-WWW and how to fix each issue.

Next 100-point set: DX-WWW TSX App Router Compiler And Strict Next Parity (92/100).

- [x] Replace the `pages/index.html` compatibility fallback in fresh apps with first-class TSX App Router route output.
- [x] Add nested layout/template compilation for route groups, dynamic segments, loading, error, and not-found boundaries.
- [x] Add typed server-action request/response runtime validation with source-owned receipts and replay-safe error reporting.
- [x] Add client-island hydration event wiring for React-shaped `useState`, props, forms, and explicit dynamic imports.
- [x] Promote compiled App Router routes into first-class Route Unit contracts with shell, graph, state, packet, receipt, and runtime-report sections.
- [x] Add the State Graph ABI for local state slots, derived slots, event slots, effects, and server-action edges while keeping state local by default.
- [x] Add adaptive runtime decision reports that explain selected, candidate, and rejected static/micro-js/server-fragment/wasm delivery modes with byte estimates.
- [x] Add LLM-friendly `dx check --project-contract` standards for source file size, barrel files, dynamic imports, client/server boundary mistakes, and Forge provenance gaps.
- [ ] Add metadata, image, font, script, redirect, and headers adapters as Forge-owned compiler intrinsics with strict diagnostics.
- [x] Add a Next conformance fixture suite that compares DX-WWW output against App Router fixtures without `node_modules`.

Completed feature slice: DX-WWW Route, Page, And State Compilation Standard (100/100).

Forge responsibilities:

- Track generated files as first-class objects.
- Preserve user edits.
- Diff package updates file-by-file.
- Roll back failed updates.
- Provide update receipts.
- Expose a library API that DX-WWW, editor extensions, and future DX cloud can call.

Do not claim this is safer than npm until:

- provenance is stored,
- licenses are tracked,
- vulnerability/advisory metadata can be attached,
- generated code changes are reviewable,
- and supply-chain checks run before materialization.

## 6. Phase 4 - DX-WWW Compiler And Runtime

Compiler responsibilities:

- Parse `.html`, `.tsx`, `.lyt`, and supported TS/TSX/JS/JSX entrypoints.
- Resolve components from DX registry, local variants, and project source.
- Extract style rules into binary style payloads and fallback CSS.
- Split server-only, client-only, and static code.
- Emit HTML fallback, packet payloads, and optional tiny WASM runtime.
- Generate source maps and debug metadata.

Runtime responsibilities:

- Decode packets.
- Clone templates.
- Patch text/attrs/classes.
- Attach event handlers.
- Manage small reactive state.
- Defer or skip WASM where no interactivity is needed.
- Never ship JS/WASM that the page does not need.

Acceptance:

- Static page ships zero client runtime.
- Small interactive page ships tiny runtime plus packet.
- Rich page ships only the feature slices it uses.
- Browser tests verify real DOM behavior.
- Accessibility and SEO fallback tests pass.

## 6.1. Phase 4A - Adaptive Runtime Size Supremacy

Problem found by fair benchmarks:

- DX-WWW currently beats Next.js, Svelte, and HTMX on the minimal counter payload.
- Astro still beats current DX-WWW on that same tiny page because Astro emits one static HTML file with a tiny inline script, while DX-WWW ships HTML plus WASM.
- This is not a failure of the source-owned runtime idea. It means DX-WWW must
  become runtime-adaptive instead of always-WASM.

Rule:

- DX-WWW is source-owned and runtime-adaptive, not WASM-first.
- The compiler must choose the smallest correct delivery mode for each page, component, and interaction.

Required delivery modes:

1. `mode=static`
   - Emits HTML and CSS only.
   - No JavaScript.
   - No WASM.
   - Must beat Astro on pages with no interactivity.

2. `mode=micro-js`
   - Emits tiny generated JavaScript for trivial interactions where JS is smaller than the WASM runtime.
   - Examples: counter, toggle, tabs, accordion, modal open/close, copy button.
   - Must beat Astro/Svelte on very small interactive pages.

3. `mode=wasm-core`
   - Emits the smallest WASM runtime only when packet decoding, binary patching, state machines, or repeated UI updates make WASM cheaper.
   - Must use feature flags so unused host imports, template cache, event systems, and debug paths disappear.

4. `mode=wasm-split`
   - Ships tiny boot runtime first.
   - Lazy-loads optional slices such as router, forms, validation, rich text, charts, animation, auth, and data cache.
   - Must beat Next/Svelte/HTMX on medium interactive apps.

5. `mode=server-fragment`
   - Uses server-rendered fragments for HTMX-like workflows when that is smaller or safer than client state.
   - Must keep the same DX-WWW component/source ownership model.

Compiler selection algorithm:

- Estimate static HTML/CSS bytes.
- Estimate generated micro-JS bytes.
- Estimate WASM runtime plus packet plus dictionaries.
- Estimate server-fragment runtime plus endpoint overhead.
- Pick the smallest mode that satisfies the page's interaction contract.
- Write the chosen mode and byte estimates into a build manifest.

Required size work:

- [ ] Add `wasm-opt` / Binaryen optimization path when available.
- [ ] Add `twiggy` or equivalent WASM size attribution.
- [ ] Split `dx-www-browser` into feature-gated runtime crates/modules.
- [ ] Remove panic formatting, debug strings, unused allocator paths, unused host imports, and unused template/cache paths from tiny builds.
- [ ] Test `wee_alloc`, no allocator, and custom bump allocation for tiny runtime builds.
- [ ] Add `panic = "abort"`, LTO, codegen-units=1, opt-level `z`/`s`, and strip-symbol release profiles.
- [ ] Generate a micro-JS fallback for tiny DOM interactions.
- [ ] Emit static-only pages with zero runtime.
- [ ] Add a byte-budget CI gate: static page <= Astro, tiny interaction <= Astro or Svelte, app island <= Svelte, rich app <= Next by a large margin.

Acceptance:

- DX-WWW beats Astro on static pages by emitting zero runtime and equally compact HTML.
- DX-WWW beats Astro/Svelte on tiny interactions by choosing micro-JS when WASM is larger.
- DX-WWW beats Svelte/HTMX/Next on medium interactive pages by switching to binary packets and WASM only when the amortized runtime win is real.
- Every benchmark records the chosen runtime mode, all runtime bytes, and why the compiler selected it.

## 6.2. Phase 4B - Template/Data ABI Breakthrough

Result from `benchmarks\binary-web-lab`:

- Generic rkyv object graphs are excellent for zero-copy access but are not automatically the smallest browser payload.
- `memmap2` helps server/build/edge startup for large cached packets, but browsers still receive bytes through fetch/streams.
- The strongest size lever is separating reusable UI structure from changing data slots.

Working hypothesis:

- The "one piece" is not raw serialization.
- The one piece is a DX Template/Data ABI:
  - Component templates are compiled once.
  - Static structure is interned and cached.
  - Repeated UI ships only slot values, row data, patch ops, and event bindings.
  - The browser runtime clones templates and applies compact typed slots.

Benchmark signal:

- On a dashboard-like 1200-row page:
  - HTML string: `203,444` raw / `3,912` Brotli.
  - JSON graph: `547,068` raw / `10,870` Brotli.
  - rkyv graph: `339,048` raw / `35,062` Brotli.
  - DX dictionary packet: `68,827` raw / `6,384` Brotli.
  - DX template/data packet: `43,628` raw / `2,675` Brotli.

Required architecture:

- [ ] Define `DxTemplate` as the stable binary representation of component structure.
- [ ] Define `DxSlotSchema` for typed dynamic values: text, number, boolean, class toggle, attr, child range, event action.
- [ ] Define `DxInstanceBatch` for repeated cards, rows, list items, forms, nav items, and CMS blocks.
- [ ] Define `DxPatchStream` for runtime updates using compact opcodes.
- [ ] Cache templates by content hash so repeated pages reuse them.
- [ ] Let Forge track template provenance and source-owned package versions.
- [ ] Use rkyv for build cache and server-side immutable metadata where zero-copy access matters.
- [ ] Use the custom DX template/data ABI for browser transfer where byte size matters most.
- [ ] Add browser decode benchmarks: template clone, slot apply, list patch, table update, form validation.

Acceptance:

- DX-WWW beats Astro on repeated content/components after templates are cached.
- DX-WWW beats Svelte on medium repeated UI because Svelte must ship compiled JS per behavior while DX ships data slots.
- DX-WWW beats HTMX on repeated server fragments by shipping compact patches instead of repeated HTML.
- DX-WWW beats Next.js by avoiding React runtime/RSC/client-island overhead for common creator/business app screens.

## 6.3. Phase 4C - Game-Changing Delivery Modes

New lab:

- `benchmarks\binary-web-lab` now tests columnar data, semantic codecs, viewport packets, and patch streams.
- Report: `benchmarks\reports\binary-web-lab.md`.

Result:

- The next breakthrough is not "binary instead of text" by itself.
- The breakthrough is "compiler knows the UI/data meaning and chooses the smallest valid delivery mode."

Measured signals:

- Dashboard full data:
  - DX template slots: `43,628` raw / `2,675` Brotli.
  - DX columnar slots: `21,655` raw / `3,544` Brotli.
  - DX semantic codec: `22` raw / `26` Brotli.
- Initial viewport for a 1200-row table:
  - DX viewport-40 packet: `638` raw / `199` Brotli.
- 12-row live update:
  - HTML row fragments: `1,187` raw / `147` Brotli.
  - JSON cell patch: `1,152` raw / `163` Brotli.
  - DX cell patch: `145` raw / `134` Brotli.
- 600-row bulk update:
  - HTML row fragments: `102,020` raw / `2,512` Brotli.
  - JSON range op: `74` raw / `66` Brotli.
  - DX range op: `10` raw / `14` Brotli.

Caveat:

- Semantic codecs are powerful but not universal. They only apply when data has detectable shape: ranges, enums, repeated prefixes, generated labels, counters, dates, prices, status sets, CMS blocks, table schemas, or route patterns.
- This is still valuable because business websites and dashboards are full of exactly those shapes.

Required architecture:

- [ ] Add a `DxDeliveryPlanner` that can choose `static`, `micro-js`, `template-slots`, `columnar-slots`, `semantic-codec`, `patch-stream`, `range-op`, and `server-fragment`.
- [ ] Add columnar encoders for repeated component slots.
- [ ] Add semantic detectors for numeric ranges, repeated prefixes, enum columns, date sequences, price/currency columns, route lists, and CMS collection fields.
- [ ] Add `DxPatchOp` opcodes for text, attr, class, enum, number, insert, remove, move, range-set, and formula-update.
- [ ] Add viewport-first delivery for large lists/tables with progressive background fill.
- [ ] Add template dictionaries that can be cached by browser/CDN and reused across pages.
- [ ] Add browser-side benchmarks for applying these packets to real DOM templates.
- [ ] Add fallback rules so unsupported or unsafe semantic compression degrades to normal template slots.

Strategic claim to prove:

- Next.js, React, Svelte, Astro, and HTMX mostly optimize how developers author UI.
- DX-WWW must optimize the actual meaning of the UI at compile time and ship the cheapest representation per surface.
- If implemented correctly, this is the path to "websites as optimized products," not just another JavaScript framework.

## 6.4. Phase 4D - DX-Style CSS-First Delivery

Decision from the 2026-05-15 architecture review:

- Browser delivery should be CSS-first by default.
- Binary style should stay as a compiler/cache/analysis/update artifact until it proves a real browser win over native CSS delivery.
- DX-Style should beat Tailwind first by generating smaller source-owned CSS, not by forcing a binary CSS runtime onto every page.

Why:

- Native CSS is already the browser's fastest styling runtime.
- Binary CSS only wins if the binary payload plus decoder plus apply cost is smaller and faster than normal CSS after Brotli.
- The current `www\build\src\style.rs` Binary Dawn loader is still a stub, so binary style cannot be the default production browser path yet.
- DX-WWW already wins the tiny fair counter by using static HTML plus micro JS. Adding a style decoder to that path would make it worse.

Required architecture:

- [ ] Make generated/minified CSS the default emitted browser artifact.
- [ ] Keep Binary Dawn/other binary style formats available for build cache, diffing, offline analysis, repeated theme patches, and future runtime experiments.
- [ ] Add a style delivery planner that chooses `css-file`, `critical-inline-css`, `css-modules`, `binary-style-cache`, or `style-patch` per page.
- [ ] Benchmark binary style with all overhead counted: binary file, decoder runtime, apply time, style recalculation, cache behavior, gzip, and Brotli.
- [ ] Only enable binary style in production when the planner proves it beats generated CSS for that page shape.
- [ ] Keep source-owned classes/tokens editable and inspectable, similar to the shadcn model.

Acceptance:

- Static and tiny interactive pages ship normal CSS or inline critical CSS, not binary style runtime.
- Repeated themes and live style updates may use binary patches only if measured faster/smaller.
- DX-Style can honestly claim "Tailwind-class DX with smaller generated CSS" before claiming "binary CSS runtime beats CSS."

## 7. Phase 5 - Creator, CMS, And Hosting Layer

This is the likely first money path.

Product target:

- Businesses, agencies, freelancers, ecommerce landing pages, docs sites, dashboards, and slow WordPress/Webflow/Wix replacements.

Required platform pieces:

- [ ] DX site creator that outputs DX-WWW projects.
- [ ] Visual page/block/component registry.
- [ ] CMS fields compiled into static/binary output.
- [ ] Forms, auth, analytics, redirects, image optimization, deployment previews, rollbacks.
- [ ] Hosted build cache and profile reports.
- [ ] CDN/static deployment with edge function escape hatch.
- [ ] Profiler that shows exact savings against a Next/WordPress baseline.

Initial pricing wedge:

- Free local compiler and small projects.
- Paid hosting/profiling/creator.
- Paid team registry/private variants.
- Paid agency workflow.
- Later enterprise package-governance features.

Do not compete with AWS directly. Build the profitable layer above AWS, Cloudflare, R2, object storage, and edge/CDN providers.

## 8. Phase 6 - Benchmark Lab

Benchmarks must be fair or they are useless.

Compare against:

- A generated Next.js App Router static page.
- A good Next.js App Router interactive page with a small client island.
- A WordPress-like content page with plugin-style overhead.
- A dashboard page with forms, table, filters, modals, and charts.
- A DX-WWW equivalent generated from the same content and interactions.

Metrics:

- Cold build time.
- Incremental rebuild time.
- HTML bytes.
- JS bytes.
- WASM bytes.
- CSS bytes.
- Binary packet bytes.
- Gzip and Brotli transferred bytes.
- Requests count.
- Time to first byte.
- First contentful paint.
- Largest contentful paint.
- Total blocking time.
- Hydration/runtime startup time.
- Memory after interactive.
- CPU time for first interaction.

Rules:

- Count every required runtime byte.
- Count dictionaries, decompressors, metadata, and shims.
- Test production builds only.
- Use the same content and UX.
- Use localhost first, then hosted CDN later.
- Publish raw artifacts and scripts.

Acceptance:

- `benchmarks\reports\latest.md` contains reproducible numbers.
- Every number links to the exact output folder.
- If DX-WWW loses a case, document why and fix the architecture rather than hiding the result.

## 9. Phase 7 - Inspiration And Reference Mining

Reference repos are stored under:

- `inspirations\`

Use them for architecture study, not copying blindly.

Planned references:

- Supabase: product/platform monorepo, CLI, dashboard, local developer experience.
- Drizzle: TypeScript ORM packaging, docs, migrations, schema DX.
- TanStack Query: async state/cache model.
- TanStack Table: headless composable UI logic.
- TanStack Router: type-safe routing and data loading.
- TanStack Form: form state and validation.
- TanStack Store: small reactive store ideas.
- TanStack DB: client data layer ideas.
- TanStack Virtual: high-performance list virtualization.
- TanStack Start: full-stack app framework comparison.

Extraction tasks:

- [ ] Create one note per reference repo.
- [ ] Identify package boundaries, CLI patterns, docs structure, tests, examples, and release workflow.
- [ ] Extract ideas into DX-WWW design docs only after understanding them.
- [ ] Never paste third-party code into DX source without license review and explicit purpose.

## 10. Immediate Work Queue

1. Make the staged workspace self-contained.
2. Fix Cargo metadata and the missing crate paths.
3. Run a static server for the existing demo and record current demo truth.
4. Create a Next.js baseline under benchmarks.
5. Run the first payload-size comparison.
6. Replace simulated DX demo paths with one real compiler/runtime vertical slice.
7. Add Forge-owned package manifest format.
8. Add green/yellow/red package update classification.
9. Add source-owned component registry v0.
10. Add honest benchmark report generation.

## 11. Anti-Scope

Do not do these first:

- Replace all npm packages.
- Replace all of React/Next.js.
- Replace AWS.
- Support every browser API.
- Support every enterprise package.
- Build a giant all-in-one crate.
- Use fake demos as proof.
- Benchmark against bad Next.js code and call it a win.

The first win is narrower:

DX-WWW should make real business websites and creator-built sites dramatically smaller, faster, safer to own, and easier to host.

## 12. Current Honest Status

As of this TODO:

- The big idea is strong.
- The staged source base is promising.
- The current DX-WWW workspace is not yet self-contained.
- The current demos contain useful artifacts but are not yet proof of a complete framework.
- The Flow Forge implementation is the best foundation for versioning/sync.
- The archived Forge docs are the best foundation for source-owned package orchestration.
- The Flow serializer is the best current compact serializer reference.
- The next milestone is not more claims. It is one real vertical slice and one fair benchmark.
