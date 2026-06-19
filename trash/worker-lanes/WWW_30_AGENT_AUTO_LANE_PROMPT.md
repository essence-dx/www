FIRST ACTION BEFORE ANY REPO WORK:

Run this command:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1
```

Use the returned `AGENT_NUMBER` as your lane number and `PASS_NUMBER` as your current pass.

For pass 2 and pass 3 in this same chat, run the `Next pass command` printed by the script. That command includes your `WORKER_ID`, keeps the same lane, and automatically increments the pass number. Do not ask the manager for a lane or pass number. Do not edit the repo before claiming your lane/pass.

If the command says all 30 lanes are claimed, stop and report that the current assignment round is full.

You are one of 30 senior AI engineering agents finishing DX-WWW in:

G:\Dx\www

Use [@superpowers](plugin://superpowers@openai-curated) when useful.

Read `G:\Dx\www\AGENTS.md` and `G:\Dx\www\.cursorrules` before changing code,
docs, tests, examples, receipts, or generated artifacts. `AGENTS.md` is the
canonical contract. This lane prompt is an execution guide only; if it
disagrees with `AGENTS.md`, follow `AGENTS.md`.

Use `README.md` for the public overview, `docs/dx-www-developer-contract.md`
for project shape and authoring rules, `docs/api/versioning.md` for public API
stability, and current code, receipts, and command output for numeric or
release claims. Treat `DX.md` as historical launch notes unless a task
explicitly asks for it.

## Current WWW Authoring Contract

- WWW is source-owned, Rust-backed, React/Next-familiar authoring.
- Framework-owned global state uses `store({ state, derived, effect, action })`
  modules, usually under `lib/stores/`.
- `State Management` is the Forge package lane for upstream Zustand provenance;
  keep it separate from WWW-native `store()` global stores.
- Quoted event values such as `onClick="bg-red-500 scale-up"` are
  interaction-class commands; braced event values must lower safely or produce
  a diagnostic.
- DX Style owns Tailwind-like `className`, grouped syntax, motion/event classes,
  generated CSS, and the atomic/custom CSS balance. Do not hand-edit generated
  CSS.
- DX Icon is first-party. Use `<Icon name="pack:check" />` through
  `icons(component=Icon source_tag=icon runtime_tag=dx-icon ...)`; do not
  import npm icon packages in official WWW starters.
- Devtools and hot reload are dev-only and must not ship in production output.

You are allowed to think as deeply as needed. Think hard, then code decisively. The target is 100/100 professional production readiness: clean architecture, clean Rust proof, real `dx build`, real dev proof, reduced warnings, curated worktree, no unsupported claims, and a codebase another strong engineer can continue confidently.

This prompt will be used for 3 passes across all 30 agents. Every pass must materially improve the repo.

Historical seed status, not current proof:
- Overall score: 84/100, not complete.
- App is running at http://127.0.0.1:3000, PID 18900.
- `git diff --check`: passed, LF/CRLF warnings only.
- Conflict-marker scan excluding `vendor`, `target`, `node_modules`: passed.
- `dx-www/src/cli/mod.rs`: 22,886 lines, below 40k.
- `cargo check -p dx-style --lib -j 1`: passed.
- `cargo check -p dx-www --no-default-features --features cli --bin dx-www -j 1`: passed with 62 warnings.
- `cargo build -p dx-www --no-default-features --features cli --bin dx-www -j 1`: passed with 62 warnings.
- Real `dx build` in `examples/template`: passed, 15 routes and `.dx/build` output.
- `dx dev --host 127.0.0.1 --port 3000`: passed, running PID 18900.
- HTTP `/`: 200.
- HTTP `/_dx/hot-reload/version?resource=route%3A%2F`: passed with `dx.hot-reload.poll`.
- HTTP `/favicon.svg`: 200 image/svg+xml.
- Focused suites: 245/245 passing.

Do not repeat the seed status as current truth. Refresh branch, dirty state,
processes, receipts, and proof commands from the live checkout before reporting
or scoring anything.

Remaining blockers:
1. Worktree is chaotic: 568 dirty entries and huge diff.
2. Rust compile/build emits 62 warnings.
3. Evidence is still too source-guard-heavy for 95+.
4. No browser screenshot or interactive overlay recovery proof.
5. Generated `.dx/build`, receipts, and `.tmp` artifacts need cleanup/ignore/commit decisions.
6. Full `cargo test` / `cargo clippy` has not been run after curation.

Architecture contract:
- DX-WWW is a DX-owned WWW framework with Next-familiar authoring.
- Do NOT implement a Next.js DevTools clone.
- Do NOT implement real Turbopack runtime/build adoption.
- Turbopack/Next internals are reference/provenance only.
- Keep Rust/WASM/tiny-runtime, Forge-first, no-node_modules-default discipline.
- Keep basic DX-owned hot reload and error overlay only.
- No Tailwind runtime/CDN/PostCSS requirement in normal DX paths.
- No synthetic receipts, unsupported scores, or docs-only completion.
- No broad rewrites.
- Do not restore the deleted junk root file named `-`.
- Do not reset, revert, checkout, pull, or overwrite other agents.

3-PASS RULE:
Pass 1:
- Fix the biggest blocker in your lane.
- Prefer real code changes over status reporting.
- If your exact blocker is already fixed, move to the nearest remaining blocker in your lane.

Pass 2:
- Harden the lane: edge cases, diagnostics, stale artifact behavior, platform behavior, and maintainability.
- Reduce warning count, dirty noise, or proof gaps where relevant.

Pass 3:
- Production proof and cleanup.
- Run the strongest focused check for your lane.
- Report exact remaining risk.
- Do not claim 100/100 unless the evidence supports it.

Start every pass after claiming lane/pass:
1. `Set-Location G:\Dx\www`
2. Run:
   - `git status --short --branch`
   - `git diff --shortstat`
3. Inspect your lane files and current dirty state.
4. Check what changed since your previous pass.
5. Code meaningfully.
6. Test minimally but honestly.

Testing rules:
- Think more. Code more. Do not burn the pass on broad testing unless your lane owns proof.
- New tests should be `.ts` by default.
- Do not create new `.cjs` or `.mjs` tests.
- You may edit an existing `.cjs`/`.mjs` test only for a small stale-guard fix and must explain why.
- Do not add dependencies just to force `.ts`.
- Use focused checks, not broad suites, unless assigned.
- Heavy Cargo/build/browser lanes are agents 1, 5, 10, 11, 12, 13, 14, 26, 27, 28, and 30. Other agents run focused checks only.
- Prefer low-impact commands. Use `-j 1` for heavy Cargo lanes unless the
  manager explicitly asks for higher concurrency such as `-j 6`.
- Do not run `dx build`, `dx dev`, browser automation, deploys, package
  installs, or broad workspace tests unless your lane explicitly owns that
  proof.
- Never kill processes broadly. Stop only an exact stale PID that belongs to
  your lane and explain why it was safe.

LANE MAP:

AGENT 1 - Process And Proof Coordinator
Goal: keep heavy proof from clogging the machine.
Tasks:
- Inspect cargo/rustc/node/dx-www processes.
- Identify stale processes without killing active worker work blindly.
- Coordinate when cargo/check/build/browser proof should run.
- Ensure port 3000 process status is known.
Checks:
- process list
- disk space
- git status
- HTTP probe if server is running.

AGENT 2 - Worktree Ownership Map
Goal: turn 568 dirty entries into understandable ownership groups.
Tasks:
- Group dirty files by domain: dx-www core, dx-style, Forge, build-smoke, template, docs, generated, vendor/reference.
- Detect overlapping risky edits.
- Produce a commit/quarantine plan.
- Do not stage or delete unless explicitly safe.
Checks:
- `git status --short`
- `git diff --name-status`
- unmerged file scan.

AGENT 3 - Generated Artifact Cleanup Plan
Goal: decide what `.dx/build`, receipts, `.tmp`, reports, and generated outputs should be committed, ignored, or removed.
Tasks:
- Identify accidental generated artifacts.
- Update ignore rules only if clearly correct.
- Do not delete source-owned fixtures or proof files.
Checks:
- artifact inventory
- git status diff before/after.

AGENT 4 - Commit Slice Planner
Goal: create coherent release-control slices without reverting anyone.
Tasks:
- Propose focused commit groups.
- Separate build/runtime, dx-style, Forge, docs, generated proof.
- Flag files that must not be committed.
- Do not commit unless explicitly asked by manager.
Checks:
- name-status summary
- conflict/unmerged scan.

AGENT 5 - Rust Warnings: dx-www CLI
Goal: reduce warnings in `dx-www` CLI modules.
Tasks:
- Fix unused imports, dead code, unused fields, unreachable helpers.
- Do not remove behavior.
- Prefer deleting only clearly dead code from your lane.
Checks:
- focused cargo check or rustc output for touched modules.

AGENT 6 - Rust Warnings: App Router Modules
Goal: reduce warnings in App Router execution/build modules.
Tasks:
- Clean warning causes in metadata, request props, render plan, source render, server data.
- Preserve 59/59 App Router behavior.
Checks:
- focused App Router tests
- cargo check if practical.

AGENT 7 - Rust Warnings: Build Source Engine
Goal: reduce warnings in source graph/build artifact modules.
Tasks:
- Fix unused code/imports in source_engine, route outputs, manifests, receipts.
- Preserve real `dx build`.
Checks:
- focused build checks.

AGENT 8 - Rust Warnings: Dev/Hot Reload/Diagnostics
Goal: reduce warnings in dev server, hot reload, diagnostics, overlay code.
Tasks:
- Clean warning causes.
- Preserve hot reload protocol and error UX.
Checks:
- hot reload/dev feedback focused tests.

AGENT 9 - Rust Warnings: Route Handlers/Resolver
Goal: reduce warnings in route-handler and resolver paths.
Tasks:
- Clean warning causes in request behavior, module linker, resolver config.
- Preserve route-handler/resolver suites.
Checks:
- route-handler and resolver focused tests.

AGENT 10 - Clippy Prep
Goal: make `cargo clippy` feasible.
Tasks:
- Inspect likely clippy failures in touched areas.
- Fix obvious lint issues without huge refactors.
- Do not run full clippy repeatedly.
Checks:
- focused clippy if practical, otherwise cargo check for touched crate.

AGENT 11 - Black-Box E2E Build Test
Goal: add one real black-box E2E test for `dx build`.
Tasks:
- Build a tiny app fixture.
- Verify manifest/assets/server-data/routes.
- Make it `.ts`, not `.cjs`/`.mjs`.
- Keep it deterministic and not huge.
Checks:
- focused new `.ts` test.

AGENT 12 - Black-Box Dev Server Test
Goal: add dev-server black-box proof.
Tasks:
- Start or probe `dx dev` safely.
- Verify route 200, hot reload endpoint, favicon/static asset.
- Avoid port conflicts.
- Use `.ts` for new test.
Checks:
- focused dev proof.

AGENT 13 - Browser Render Proof
Goal: get real browser/screenshot proof if tooling is available.
Tasks:
- Open `/`, `/dashboard`, `/login`, `/launch` if present.
- Capture screenshot proof or explain tool limitation.
- Check visible DX-WWW content, no blank page.
Checks:
- Browser automation if available, otherwise HTTP fallback.

AGENT 14 - Diagnostics Overlay Recovery Proof
Goal: prove basic DX-owned overlay/error recovery.
Tasks:
- Create or use a small failing fixture.
- Verify error response/overlay data/code frame.
- Verify recovery after fix if practical.
Checks:
- focused diagnostics/browser proof.

AGENT 15 - Real dx build Artifact Audit
Goal: make `.dx/build` production evidence trustworthy.
Tasks:
- Inspect real template build outputs.
- Verify 15 routes, assets, CSS, source maps/hash, server-data, route handler evidence.
- Fix missing/stale artifact proof.
Checks:
- focused installed-smoke/build artifact tests.

AGENT 16 - No-node_modules Boundary Proof
Goal: protect Forge-first/no-node default path.
Tasks:
- Scan build/dev/template paths for `node_modules` assumptions.
- Ensure generated proof says `node_modules_required=false` only when true.
- Fix misleading claims.
Checks:
- no-node guards.

AGENT 17 - App Router Black-Box Fixtures
Goal: add runtime-style proof for App Router behavior.
Tasks:
- Tiny app fixtures for nested layouts, dynamic params, route groups, metadata, not-found/loading/error.
- Prefer `.ts` tests.
- Do not claim full Next parity.
Checks:
- focused App Router black-box tests.

AGENT 18 - Route Handler Black-Box Fixtures
Goal: prove request/response behavior beyond source guards.
Tasks:
- Tiny fixtures for GET/POST, JSON/text body, headers, cookies, search params, HEAD.
- Keep unsupported factories honest.
Checks:
- focused route-handler tests.

AGENT 19 - Hot Reload Live Edit Proof
Goal: prove edit-save-update behavior.
Tasks:
- Verify page/style/asset/route edit events.
- Improve stale recovery or event accuracy if weak.
- Do not claim Turbopack HMR.
Checks:
- focused hot reload test or HTTP proof.

AGENT 20 - Diagnostics UX Cleanup
Goal: make errors professional.
Tasks:
- Improve unsupported class/build/route diagnostics.
- Ensure messages include source path, line/column/code frame where possible.
- Remove vague failure wording.
Checks:
- diagnostics tests.

AGENT 21 - Resolver/Module Linker Production Edges
Goal: harden source-first resolver.
Tasks:
- `@/`, src/app, local TS/TSX, package exports, config extends, external boundaries.
- Add missing `.ts` proof if needed.
Checks:
- resolver focused tests.

AGENT 22 - Template Product Runtime QA
Goal: make the default template feel production-clean.
Tasks:
- Verify `/`, `/dashboard`, `/login`, `/launch` via server/HTTP/browser if available.
- Fix broken links, broken assets, obvious runtime text/visual errors.
- Do not add marketing filler.
Checks:
- HTTP/browser proof.

AGENT 23 - Receipts Truth Cleanup
Goal: remove or fix stale/misleading receipts.
Tasks:
- Scan receipts/read models for stale claims.
- Keep only real executable proof.
- Do not delete useful provenance.
Checks:
- JSON parse and focused receipt tests.

AGENT 24 - Temp/Cache Ignore Hygiene
Goal: clean accidental cache/temp churn.
Tasks:
- Identify `.tmp`, cache, build byproducts, local-only generated noise.
- Add ignore rules only for true local artifacts.
- Do not hide real source outputs accidentally.
Checks:
- git status before/after.

AGENT 25 - Docs Truth And Score Honesty
Goal: keep docs/status aligned with runtime truth.
Tasks:
- Remove overclaims.
- Update only necessary docs with verified facts.
- Avoid docs-only completion.
Checks:
- source claim scan if available.

AGENT 26 - Focused Cargo Tests
Goal: run and fix focused cargo tests after curation.
Tasks:
- Run relevant package tests with low concurrency.
- Fix real failures.
- Do not run full workspace loop repeatedly.
Checks:
- focused `cargo test -p dx-www ... -j 1` or scoped tests.

AGENT 27 - Production Readiness Gate
Goal: create/update one reliable readiness gate.
Tasks:
- Bundle the strongest current proof into one command/script/test.
- Include cargo check, focused Node tests, dx build smoke, HTTP proof where safe.
- Keep it practical on Windows.
Checks:
- readiness gate dry run if feasible.

AGENT 28 - Windows Server/Port Management
Goal: make dev server lifecycle reliable.
Tasks:
- Detect existing server on port 3000.
- Avoid duplicate server starts.
- Add safer process/status reporting if needed.
- Do not kill unrelated processes.
Checks:
- process + HTTP probes.

AGENT 29 - Security/Supply Chain Boundary
Goal: protect against accidental runtime supply-chain drift.
Tasks:
- Scan for CDN, npm runtime assumptions, public schema-suffix overclaims, package.json misuse, next/turbo active adoption.
- Fix or mark boundaries.
Checks:
- focused source guards.

AGENT 30 - Final Integration Scorer
Goal: provide the real final score after all lanes.
Tasks:
- Run final checks after other agents settle.
- Apply hard caps honestly.
- Do not guess.
Checks:
- `git diff --check`
- conflict scan
- cargo check/build if safe
- real `dx build`
- focused suite summary
- HTTP/browser proof
- warning count
- dirty worktree count.

Common final checks for every agent:
- `git diff --check`
- conflict-marker scan excluding `vendor`, `target`, `node_modules`, `.git`
- one focused lane check
- if Rust changed, practical cargo/rustfmt check or explain skip
- if JS/TS changed, `node --check` or focused `node --test`
- new tests must be `.ts`

Final response format:
- Agent number / lane
- Pass number: 1/3, 2/3, or 3/3
- 84-to-100 blocker attacked
- Root cause found
- Real code changes
- Files changed
- Checks run with pass/fail
- Warning count impact if relevant
- Worktree hygiene impact if relevant
- What remains not 100/100
- Conflict risk with other agents
- Honest score impact
- Next exact action for the next pass
