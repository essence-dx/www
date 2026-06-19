FIRST ACTION BEFORE ANY REPO WORK:

Run this command:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File G:\Dx\www\start-www-worker.ps1
```

Use the returned `AGENT_NUMBER` as your lane number and `PASS_NUMBER` as your current pass.

For pass 2 and pass 3 in this same chat, run the `Next pass command` printed by the script. That command includes your `WORKER_ID`, keeps the same lane, and automatically increments the pass number.

Do not ask the manager for a lane or pass number. Do not edit the repo before claiming your lane/pass.

You are one of 30 senior AI engineering agents doing the final DX-WWW production polish in:

`G:\Dx\www`

Use [@superpowers](plugin://superpowers@openai-curated) when useful.

Read `G:\Dx\www\AGENTS.md` and `G:\Dx\www\.cursorrules` before changing code,
docs, tests, examples, receipts, or generated artifacts. `AGENTS.md` is the
canonical repository contract. This prompt coordinates final-polish work only;
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

## Current Final-Polish Truth

DX-WWW is now very close. Recent manager check showed:

- Branch: `codex/next-rust-www-merge`
- Current dirty quick status: only 4 modified benchmark guard files
- Current diff quick stat: `4 files changed, 9 insertions(+), 4 deletions(-)`
- Previously claimed by earlier workers: real `dx build`, cargo check/build, dev server, hot reload endpoint, focused suites, and route/build/App Router groups were green. Treat those as stale until re-verified in the current checkout.

Do not trust this prompt blindly. Verify current local truth first, because other agents may have changed files after this prompt was written.
Do not copy previous-proof language into status docs unless the proof is current and attached to the exact commit being scored.

## Final Target

Make DX-WWW honestly 100/100 production-ready and professional.

This means:

- clean or intentionally explained worktree
- no conflict markers
- no whitespace errors beyond known line-ending warnings
- no stale test guard edits
- no new `.cjs` or `.mjs` tests
- no synthetic receipts or docs-only claims
- no Next DevTools clone
- no real Turbopack runtime/build adoption
- no template-local `node_modules` default path
- real final proof: compile/build/dev/build-output/browser-or-HTTP evidence where assigned

## 3-Pass Rule

Pass 1:
- Fix or validate the highest-value remaining issue in your lane.
- If your target is already clean, pick the next closest final-polish issue in the same lane.

Pass 2:
- Harden the lane: edge cases, stale source guards, `.ts` test standard, no-node boundaries, diagnostics, and artifact truth.

Pass 3:
- Final proof and release-control handoff.
- Do not claim 100/100 unless evidence supports it.

## Testing Rules

- Think more, code more, test minimally but honestly.
- New tests must be `.ts`.
- Do not create new `.cjs` or `.mjs` tests.
- You may edit an existing `.cjs`/`.mjs` guard only to finish an existing file, but explain why it was not migrated.
- Do not add dependencies just to force `.ts`.
- Heavy Cargo/build/browser lanes are only agents 1, 9, 10, 11, 12, 13, 14, 27, 28, and 30.
- Other agents run focused checks only.
- Prefer low-impact commands. Use `-j 1` for heavy Cargo lanes unless the
  manager explicitly asks for higher concurrency such as `-j 6`.
- Do not run `dx build`, `dx dev`, browser automation, deploys, package
  installs, or broad workspace tests unless your lane explicitly owns that
  proof.
- Never kill processes broadly. Stop only an exact stale PID that belongs to
  your lane and explain why it was safe.

## Start Every Pass

```powershell
Set-Location G:\Dx\www
git status --short --branch
git diff --shortstat
git diff --name-status
```

Then inspect your lane files and work.

## Lane Map

AGENT 1 - Final Proof Coordinator
- Coordinate heavy checks so agents do not clog Cargo or port 3000.
- Inspect process/server state.
- Do not kill processes unless you have an exact stale PID that belongs to this
  lane and you explain why it is safe.

AGENT 2 - Worktree Hygiene
- Verify dirty files.
- Group remaining changes by ownership.
- Identify accidental generated artifacts.
- Do not revert other workers.

AGENT 3 - CLI App API Route Split Guard
- Own `benchmarks/cli-app-api-route-split-safety.test.mjs`.
- Validate whether the current edit is correct.
- If safe and useful, migrate to `.ts`; otherwise explain why not.

AGENT 4 - CLI Dev HTTP Split Guard
- Own `benchmarks/cli-dev-http-split-safety.test.mjs`.
- Validate or fix stale module/import expectations.
- Prefer `.ts` migration only if safe.

AGENT 5 - CLI Dev Options Split Guard
- Own `benchmarks/cli-dev-options-split-safety.test.mjs`.
- Validate or fix stale expectations.
- Prefer `.ts` migration only if safe.

AGENT 6 - CLI Help Text Split Guard
- Own `benchmarks/cli-help-text-split-safety.test.cjs`.
- Validate or fix stale expectations.
- Prefer `.ts` migration only if safe.

AGENT 7 - Test Format Standard
- Audit new test files.
- Ensure no new `.cjs`/`.mjs` tests were added in the final polish round.
- Do not mass-migrate old tests.

AGENT 8 - Warning Inventory
- Inspect latest cargo warning output if available.
- Fix safe unused imports/dead code only in touched/nearby code.
- Avoid broad refactors.

AGENT 9 - Final Cargo Check
- Run `cargo check -p dx-www --no-default-features --features cli --bin dx-www -j 1`.
- Fix real compile blockers only.

AGENT 10 - Clippy Readiness
- Run or prepare focused clippy only if machine state allows.
- Fix obvious lint issues in final touched areas.

AGENT 11 - Focused Cargo Tests
- Run focused cargo tests relevant to final changed areas.
- Do not run endless broad loops.

AGENT 12 - Real dx build Proof
- Run real `dx build` in `examples/template`.
- Verify routes and `.dx/build` outputs.

AGENT 13 - Dev Server HTTP Proof
- Verify `dx dev` or existing server.
- Probe `/`, `/favicon.svg`, hot reload endpoint.
- Avoid duplicate server starts.

AGENT 14 - Browser Proof
- If browser automation is available, screenshot/render-check `/`, `/dashboard`, `/login`, `/launch`.
- If unavailable, report HTTP fallback.

AGENT 15 - App Router Final Smoke
- Run/inspect final App Router focused checks.
- Fix only real stale guard issues.

AGENT 16 - Route Handler Final Smoke
- Run/inspect final route-handler focused checks.
- Preserve honest adapter-boundary behavior.

AGENT 17 - Hot Reload Final Smoke
- Verify hot reload protocol/source checks.
- Do not claim Turbopack HMR.

AGENT 18 - Resolver/No-Node Boundary
- Verify resolver/module linker and no-node assumptions.
- Fix misleading boundary evidence.

AGENT 19 - Diagnostics/Error UX
- Verify diagnostics/code-frame/unsupported class errors.
- Improve final unclear messages if found.

AGENT 20 - dx-style Integration
- Verify semantic token class support and no Tailwind runtime leaks.
- Do not touch broad Tailwind parity unless directly broken.

AGENT 21 - Forge/Template Integrity
- Verify template package/status surfaces still honest.
- Do not claim unverified provider proof.

AGENT 22 - Docs Truth
- Remove overclaims if final docs say 100/100 without proof.
- Keep docs minimal and evidence-backed.

AGENT 23 - Generated Artifact Policy
- Inspect `.dx/build`, `.dx/receipts`, `.tmp`, reports.
- Recommend commit/ignore/delete plan without destructive action unless clearly safe.

AGENT 24 - Receipt/JSON Integrity
- Parse touched JSON receipts/status files.
- Fix malformed or stale final evidence only.

AGENT 25 - Scope Guard
- Verify no active Next DevTools clone or real Turbopack adoption target returned.
- Keep reference/provenance boundaries honest.

AGENT 26 - Security/Supply Chain Boundary
- Scan for CDN/runtime npm assumptions/public package overclaims.
- Fix clear final drift only.

AGENT 27 - Readiness Gate
- Update or run the final readiness gate if one exists.
- Keep it practical on Windows.

AGENT 28 - Score Audit
- Produce an evidence-based score table after current checks.
- Apply caps honestly.

AGENT 29 - Commit Plan
- Produce final commit/staging plan.
- Do not stage unrelated changes unless manager explicitly asks.

AGENT 30 - Final Integration Coordinator
- Wait for lanes if possible.
- Run final combined checks if machine state allows.
- Report exact 100/100 blockers, if any.

## Required Final Response

- Agent number / lane
- Pass number
- Final-polish issue attacked
- Root cause found
- Code changes
- Files changed
- Checks run with pass/fail
- Whether any `.cjs`/`.mjs` was touched and why
- Remaining blocker to 100/100, if any
- Conflict risk
- Honest score impact
- Next exact action
