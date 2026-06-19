# DX WWW Repository Hygiene

This repository is allowed to be broad, but it should not look accidental. The
active tree should separate source, fixtures, generated state, and historical
archives clearly enough that a worker can scan it without drowning in stale app
copies or local run output.

## 12-Flaw Hygiene Scorecard

The hygiene scorecard is the repo-level debt map for this hardening pass. It is
not the same thing as the template `dx check` score: a project fixture can report
green while the repository still carries structural or proof debt.

The stable scorecard ids are:

- `cli-mod-large`: tracks whether `dx-www/src/cli/mod.rs` exceeds the
  current production-file budget.
- `public-framework-tools-large`: `dx-www/src/cli/public_framework_tools.rs`
  must stay below the current production-file budget.
- `source-render-large`: `dx-www/src/cli/app_router_execution/source_render.rs`
  must stay below the current production-file budget.
- `dx-check-receipt-large`: tracks whether
  `core/src/ecosystem/dx_check_receipt.rs` exceeds the current production-file
  budget.
- `forge-registry-large`: tracks whether `core/src/ecosystem/forge_registry.rs`
  exceeds the current production-file budget.
- `project-check-large`: tracks whether `core/src/ecosystem/project_check.rs`
  exceeds the current production-file budget.
- `devtools-runtime-large`: `dx-www/src/cli/devtools/assets/runtime.ts` must
  stay a small manifest while Rust serves the ordered `assets/runtime/*.ts`
  fragments as one injected runtime.
- `devtools-style-ops-large`: `dx-www/src/cli/devtools/style_ops.rs` must stay
  below the production-file budget; this pass moved the Rust tests into
  `style_ops_tests.rs`.
- `devtools-css-large`: `dx-devtools/styles/devtools.css` must stay a small
  public import wrapper over the split `styles/devtools/*.css` fragments.
- `source-visible-fixtures`: source-visible fixture roots must be owned in
  `docs/hygiene/source-visible-fixtures.md`.
- `legacy-script-extensions`: every remaining `.js`, `.cjs`, and `.mjs` file
  must be converted or owned in `docs/hygiene/legacy-script-extensions.md` as
  `runtime`, `vendor`, `fixture`, or `generated-proof`.
- `readiness-overclaim-risk`: docs and receipts must keep distinguishing a
  closed hygiene scorecard from product, browser, provider, or launch readiness.

`ok` in the hygiene audit means the active tree is free of hygiene blockers and
scorecard debt, including junk files, stale generated run output, unowned
fixtures, unowned legacy scripts, or deprecated copies that should not be
present. It does not mean the repo is 100/100.

`readyFor100` is the hygiene field for a repo-wide hygiene 100 claim. It is true
only when the 12 hygiene scorecard items have no blockers or debt. It is not a
codebase/product/browser/provider readiness proof; runtime proof must stay in
its own receipts until DX publishes a combined readiness contract.

The machine-readable readiness object is scoped with
`claimScope: "repo-hygiene-scorecard"` and keeps `codebaseReady`,
`productReady`, `browserReady`, `providerReady`, and `launchReady` false. Those
fields guard against treating a clean hygiene audit as a full codebase, product,
or launch-readiness certificate.

## Fixed In The 2026-05-28 Sweep

- Removed root junk: `NONE`, `NUL`, root cargo logs, Devtools server logs, and
  the zero-byte root `index.html`.
- Removed ignored local run output: `.codex-tmp/`, `.tmp/`,
  `.dx/codex-run/`, `.dx/run/`, `.dx/build/`, `.dx/style/`,
  `.dx/serializer/`, `.dx/launch/`, `.dx/adoption-package-review/`, and
  `.dx/adoption-update-rehearsal/`.
- Removed ignored proof/build scratch output: `artifacts/` and
  `target-codex-readiness-gate/`.
- Removed deprecated example copies from the active tree:
  `examples/deprecated-1/`, `examples/deprecated-2/`, `examples/depricated-3/`,
  and `examples/onboard-deprecated-recovery-copy/`.
- Removed the `trash/legacy-launch-runtime/` archive. Recover it from checkpoint
  commit `5352a080` only if historical comparison is needed.
- Added `tools/hygiene/audit-repo-hygiene.ts` and
  `benchmarks/repo-hygiene-audit.test.ts` to keep the fixed blockers from
  returning.

## Canonical Source Areas

- `dx-www/`: the CLI, dev server, build/runtime implementation, Devtools
  framework integration, and App Router execution path.
- `core/`: reusable compiler, delivery, and ecosystem checks.
- `server/`: standalone server crate surfaces.
- Root crates such as `a11y/`, `auth/`, `binary/`, `browser/`, `cache/`,
  `dom/`, `form/`, `guard/`, `interaction/`, `morph/`, `packet/`, `query/`,
  `reactor/`, `rtl/`, `sched/`, `state/`, and `sync/`: workspace member crates.
- `related-crates/`: companion crates that still live inside this repo.
- `benchmarks/`: executable proof and regression tests.
- `tools/`: repo-owned automation, proof, migration, and hygiene tools.
- `examples/template/`: the current app template contract.
- `examples/onboard/`: source-owned package/status panels used by the template
  and onboarding proofs.

## Tracked Fixture Exceptions

These paths still look generated, but tests read them directly today. Do not
delete or move them casually; first update the tests and the generator contract.
Each active root must also stay listed in
`docs/hygiene/source-visible-fixtures.md`.

- `.dx/template-app-browser-preview/`
- `.dx/receipts/`
- `components/`
- `lib/`
- `pages/`
- `public/`
- `dx-devtools/.dx/`

The root `components/`, `lib/`, `pages/`, and `public/` directories are
source-visible Forge/static proof fixtures. They are not current WWW app
authoring roots and should not be presented as the public App Router contract.
The current authoring root remains `examples/template/app/` for the official
template, and generated proof fixtures must be named as fixtures when referenced
from docs, tests, or receipts.

The long-term direction is to move active fixtures into a named fixture root or
regenerate them inside tests, leaving root `.dx` for source-owned receipts only.

## Serializer Boundary

The old in-repo `related-crates/serializer/` tree has been removed from this
repository. `dx-serializer` now resolves from the sibling `G:\Dx\serializer`
workspace path. Treat that as an intentional migration boundary: do not recreate
`related-crates/serializer/` unless the workspace dependency is deliberately
vendor-restored.

## Closed Scorecard Items

The current hygiene audit reports all 12 scorecard items as passing. The large
source files are under their configured line budgets, the Devtools split files
remain under budget, and the source-visible fixture and legacy-extension items
are now contract-owned instead of silently allow-listed.

- `devtools-style-ops-large`: Rust tests now live in
  `dx-www/src/cli/devtools/style_ops_tests.rs`, leaving
  `dx-www/src/cli/devtools/style_ops.rs` under budget.
- `devtools-runtime-large`: the injected runtime is served by Rust from ordered
  `dx-www/src/cli/devtools/assets/runtime/*.ts` fragments; `runtime.ts` is now a
  manifest so developers do not have to work inside a mega asset.
- `devtools-css-large`: the standalone Devtools app keeps
  `dx-devtools/styles/devtools.css` as its import entry, backed by ordered
  `dx-devtools/styles/devtools/*.css` fragments.
- `source-visible-fixtures`: active generated-proof and fixture roots are owned
  in `docs/hygiene/source-visible-fixtures.md` with owner, source, and
  removal-gate fields.
- `legacy-script-extensions`: all current `.js`, `.cjs`, and `.mjs` files are
  converted or covered by `docs/hygiene/legacy-script-extensions.md` as
  `runtime`, `vendor`, `fixture`, or `generated-proof`.
- `readiness-overclaim-risk`: the hygiene audit can report `readyFor100: true`
  only when blockers and scorecard debt are zero. That is still a hygiene claim,
  not a codebase, launch, provider, browser, or product-readiness claim.

## Legacy Extension Contract

The top-level benchmark migration remains closed:

- All `benchmarks/*.test.mjs` guards are now `benchmarks/*.test.ts` guards, and
  `benchmarks/repo-hygiene-audit.test.ts` prevents that extension from returning
  in the benchmark suite.
- All top-level `benchmarks/*.test.cjs` guards are now `benchmarks/*.test.ts`
  guards with a small `createRequire(import.meta.url)` bridge where they still
  depend on CommonJS helper modules.

Remaining legacy extensions are not hidden. They are explicitly owned as
runtime, vendor, fixture, or generated-proof surfaces in
`docs/hygiene/legacy-script-extensions.md`. New source-visible `.js`, `.cjs`, or
`.mjs` files must either be converted or added to that contract with a real
migration gate.

## Verification

Use `node --test benchmarks/repo-hygiene-audit.test.ts` to verify the executable
scorecard. Use
`node --experimental-strip-types tools/hygiene/audit-repo-hygiene.ts --json` to
inspect the current machine-readable audit output.

When the audit reports `readyFor100: true`, read it narrowly: the hygiene
scorecard is closed. Codebase readiness, product runtime, browser automation,
provider coverage, release quality, and launch readiness still need their own
receipts.

## Non-Scorecard Backlog

These are not part of the closed 12-item hygiene scorecard. They are listed so
future "codebase 100" work does not confuse a clean hygiene audit with total
repository perfection.

- `integrations/n8n-nodes-base/`: huge third-party reference corpus; needs a
  separate vendor/reference ownership policy before it can be called clean.
- `vendor/next-rust/`: large upstream mirror; keep it behind explicit vendor
  boundaries and receipts instead of treating it as DX-owned source.
- `flow/`: adjacent product/workspace inside this repo; needs a workspace
  ownership decision before broad codebase-readiness claims.
- `.dx/`: checked-in proof state is owned, but still needs count, age, and
  retention budgets in a future audit lane.
- `examples/`: proof/demo fixtures are useful, but heavy conversion fixtures and
  binary assets need their own fixture-budget policy.
- root docs such as `DX.md`, `TODO.md`, and `CHANGELOG.md`: useful history, but
  still large enough to deserve a future docs consolidation lane.
