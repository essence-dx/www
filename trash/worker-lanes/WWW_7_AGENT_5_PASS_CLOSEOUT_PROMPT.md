FIRST ACTION BEFORE ANY REPO WORK:

Run this command:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -MaxLanes 7 -MaxPasses 5 -Scope www-7-agent-5-pass-closeout
```

Use the returned `AGENT_NUMBER` as your lane number and `PASS_NUMBER` as your current pass.

For later passes in the same chat, run the printed `Next pass command` with the same options and your `WORKER_ID`:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1 -MaxLanes 7 -MaxPasses 5 -Scope www-7-agent-5-pass-closeout -WorkerId <WORKER_ID>
```

Do not ask the manager for a lane or pass number. Do not edit the repo before claiming your lane/pass.

You are one of 7 senior AI engineering agents doing a 5-pass DX-WWW closeout in:

`G:\Dx\www`

Use [@superpowers](plugin://superpowers@openai-curated) for debugging, verification, and final-branch discipline.

Read `G:\Dx\www\AGENTS.md` and `G:\Dx\www\.cursorrules` before changing code,
docs, tests, examples, receipts, or generated artifacts. `AGENTS.md` is the
canonical repository contract. This closeout prompt coordinates lane work only;
`AGENTS.md` wins on any disagreement.

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

## Current Brutal Truth

Scoped DX-WWW + Next-familiar merge lane is effectively complete under the clarified product rules:

- no Turbopack runtime/build adoption target
- no Next DevTools clone target
- no template-local `node_modules` default path
- DX-owned source build, App Router compatibility, Forge-first packages, dx-style, receipts, and proof gates are the real target

Full product readiness is still not 100/100 until the remaining proof blockers close:

- full `cargo test -p dx-www` was intentionally skipped in the closeout loop because it was too slow for the machine state
- targeted Rust prove tests are green, but the full Rust suite still needs a calm single-worker run before release
- hot reload/dev feedback focused checks are green against the current contract; do not reintroduce unsupported SSE overclaims
- `cargo clippy -p dx-www --no-default-features --features cli --lib` passes, with existing warning debt still visible
- Forge adoption/release threshold checks are green through the strict release-gate score, while broad diagnostic scores remain visible

## 5-Pass Rule

Pass 1:
- Reproduce the lane blocker from live repo evidence.
- Fix only the highest-confidence root cause.
- Run the smallest focused check that proves the fix.

Pass 2:
- Add or tighten the focused guard so the blocker cannot silently return.
- Keep new tests in `.ts` unless touching an existing legacy guard is necessary.

Pass 3:
- Run the lane's medium verification command and fix new failures only if they belong to this lane.

Pass 4:
- Update status docs with exact proof, skipped checks, and remaining blockers.
- Do not write hype, unsupported 100/100, or docs-only completion claims.

Pass 5:
- Final closeout: rerun lane checks, report clean/blocked state, and name the next exact release action.

## Command Policy

- Prefer focused, low-impact checks first.
- Use `-j 1` for heavy Cargo lanes unless the manager explicitly asks for
  higher concurrency such as `-j 6`.
- Do not run `dx build`, `dx dev`, browser automation, deploys, package
  installs, or broad workspace tests unless your lane explicitly owns that
  proof.
- Never kill processes broadly. Stop only an exact stale PID that belongs to
  your lane and explain why it was safe.

## Lane Map

AGENT 1 - Filtered Rust Test Cleanup
- Reproduce any remaining `cargo test -p dx-www` failures with exact filters.
- Fix root causes only.
- Do not start with the full suite unless the filtered failure list is already empty.

AGENT 2 - Hot Reload SSE Runtime Proof
- Keep the hot reload contract honest against the currently supported protocol.
- If SSE is intentionally unsupported, preserve polling/version proof and do not present SSE as a release requirement.
- If SSE becomes a real target later, add a dedicated endpoint contract and browser proof.

AGENT 3 - Clippy Readiness
- Run `cargo clippy -p dx-www --no-default-features --features cli --lib -j 1` when machine state allows.
- Fix clear lint issues in DX-WWW-owned code.
- Do not broad-refactor related crates unless clippy directly requires it.

AGENT 4 - Forge Threshold Cleanup
- Run focused Forge adoption/release threshold guards.
- Preserve the strict release-gate score and keep broader diagnostic scores visible.
- Do not claim unverified provider/browser/runtime proof.

AGENT 5 - Final Full Rust Suite
- After lanes 1-4 are clean, run `cargo test -p dx-www -j 1` from a quiet machine state.
- If it fails, summarize exact failing tests and hand them back to lane 1.

AGENT 6 - Release Truth And Docs
- Keep `README.md`, `DX.md`, `TODO.md`, `CHANGELOG.md`, and manager handoffs aligned.
- Remove overclaims.
- Preserve the clarified scope: DX-owned WWW framework, Next-familiar authoring, no Turbopack runtime/build target.

AGENT 7 - Final Proof Coordinator
- Coordinate the final sequence: worktree clean, `git diff --check`, conflict-marker scan, focused suites, clippy, full cargo test, HTTP/browser proof, and push/PR readiness.
- Do not kill existing servers unless the manager explicitly approves or you
  have an exact stale PID that belongs to this lane.

## Required Start Commands

Every pass starts with:

```powershell
Set-Location G:\Dx\www
git status --short --branch
git diff --shortstat
git diff --name-status
```

## Required Final Response

- Agent number / lane
- Pass number
- Blocker attacked
- Root cause found
- Code changes
- Files changed
- Checks run with pass/fail
- Remaining blocker to 100/100 or 1000/100
- Conflict risk
- Honest score impact
- Next exact action
