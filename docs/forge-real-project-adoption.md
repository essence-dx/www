# DX Forge Real-Project Adoption Path

This guide is the reproducible developer path for proving DX Forge in a clean, app-shaped project. It is intentionally local and secret-free: it uses the checked-in CLI, materializes source-owned packages, generates the public Forge evidence routes, writes review artifacts, and verifies no `node_modules` folder is created.

Run every command from `G:\WWW`.

## What This Proves

- Forge can materialize the current curated launch packages as editable local source.
- Forge writes manifests, receipts, docs, scorecards, and release proof that a reviewer can inspect.
- The public Forge routes can be generated from local evidence without package install scripts, R2 credentials, browser plugins, or a full workspace build.
- The local adoption path proves no `node_modules` is created by Forge commands.

## What This Does Not Prove

- Forge is not a universal npm replacement today.
- This is not a full framework benchmark against production Astro, Svelte, HTMX, Next.js, WordPress, or enterprise React apps.
- The curated advisory rows are fixture metadata, not a live vulnerability-feed integration.
- R2 registry publishing is opt-in and must use environment variables; this guide never reads or prints `CLOUDFLARE_R2_` secrets.

## 1. Start From A Clean Checkout

```powershell
git status --short
cargo fmt --manifest-path .\www\Cargo.toml -p dx-www -- --check
cargo check --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www
```

Do not continue if the checkout has unrelated dirty files or the binary does not typecheck.

## 2. Generate A Clean Adoption Project

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge adoption-smoke --project .\.dx\adoption-app --format markdown --output .\.dx\adoption-app\.dx\forge\adoption-smoke\adoption-smoke.md --fail-under 90
```

Expected result:

- A clean app-shaped project exists at `.dx\adoption-app`.
- Forge launch packages are materialized as source-owned files.
- `pages\forge-adoption.html` references `shadcn/ui/button`, `shadcn/ui/card`, `shadcn/ui/input`, `shadcn/ui/textarea`, `dx/icon/search`, `auth/better-auth`, `auth/better-auth`, `supabase/client`, `db/drizzle-sqlite`, and `migration/static-site` together as the first app-shaped adoption fixture.
- Public Forge routes are generated under `.dx\adoption-app\public`.
- Adoption evidence is written under `.dx\adoption-app\.dx\forge\adoption-smoke`.
- No `.dx\adoption-app\node_modules` directory exists.

## 3. Verify The Project Score

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- check .\.dx\adoption-app --strict-forge --format markdown --fail-under 90
```

This checks project structure, Forge receipts, package docs, rollback coverage, package risk, and security scoring. Treat red Forge package traffic, missing package docs, stale receipts, or a new `node_modules` folder as release blockers.

## 4. Generate The Adoption Report

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge adoption-report --project .\.dx\adoption-app --format markdown --output .\.dx\adoption-app\.dx\forge\adoption-smoke\adoption-report.md --fail-under 90
```

This is the read-only report to hand to another developer. It summarizes the existing project structure, source-owned package IDs, receipts, package docs, public route artifacts, release-bundle verification, strict Forge status, and no-`node_modules` proof.

## 5. Generate The Public Adoption Route

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- prove vertical --fixture forge-adoption --out .\.dx\adoption-app\public --write --format markdown
```

This writes a static `/forge/adoption` route from the real adoption report model, plus `forge/adoption.html`, `forge/adoption.dxp`, `forge/adoption.claims.json`, and the shared `proof.json` summary. The route must keep the same honest scope as the adoption report: local project evidence, no universal npm replacement claim, no live-customer adoption claim, and no `node_modules`.

## 6. Measure Public Route Browser Smoke

```powershell
node .\benchmarks\measure-forge-adoption-browser-smoke.ts
```

This writes `benchmarks\reports\forge-adoption-browser-smoke.json` and `.md`. The default lane creates a fresh local `.dx\adoption-browser-smoke` app through `dx forge adoption-smoke`, serves the generated public routes, verifies headings, links, static/no-runtime shape, DXPK/proof/claims artifacts, no `node_modules`, HTTP timing, and local Chrome timing when Chrome or Edge is available.

## 7. Rehearse Package Updates

```powershell
node .\benchmarks\measure-forge-package-update-rehearsal.ts
```

This writes `benchmarks\reports\forge-package-update-rehearsal.json` and `.md`. The default lane creates a fresh local `.dx\adoption-update-rehearsal` app and proves that green package updates write safely, yellow local edits block by default and require review, reviewed yellow edits produce receipts, red package states stay quarantined, rollback restores from Forge receipts, and no `node_modules` is created.

## 8. Review Source-Owned Package Evidence

```powershell
node .\benchmarks\measure-forge-source-owned-package-review.ts
```

This writes `benchmarks\reports\forge-source-owned-package-review.json` and `.md`. The report creates a clean local `.dx\adoption-package-review` app, verifies package docs and receipts, checks curated advisory placeholders for the registry packages, runs `dx forge verify-package --all`, folds in the yellow-edit and rollback rehearsal, and proves no `node_modules` is created.

## 9. Build And Verify The Release Bundle

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --project .\.dx\adoption-app --out .\.dx\adoption-app\.dx\forge\adoption-smoke\release-bundle --format markdown --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --verify .\.dx\adoption-app\.dx\forge\adoption-smoke\release-bundle --format markdown --fail-under 90
```

The first command assembles the publishable Forge evidence bundle. The second command re-validates the existing bundle hashes and route artifacts without regenerating source files.

The default release bundle intentionally stays on the stable six-route operator surface. When the promoted route comparison and release history already include `/forge/adoption`, add `--include-adoption` to assemble and verify the opt-in adoption-inclusive bundle.

## 10. Refresh Release-Readiness Trend Evidence

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-trend --write-history --format markdown --fail-under 90
```

This compares the latest public bundle, medium-route fixture, large-content fixture, and Forge trust-policy score against the checked-in trend history.

## Expected Artifacts

The adoption smoke project should contain:

- `.dx\forge\adoption-smoke\forge-smoke.json`
- `.dx\forge\adoption-smoke\adoption-smoke.md`
- `.dx\forge\adoption-smoke\adoption-report.md`
- `.dx\forge\adoption-smoke\release-bundle\forge-release-manifest.json`
- `.dx\forge\source-manifest.json`
- `.dx\forge\receipts`
- `.dx\forge\docs`
- `pages\forge-adoption.html`
- `public\forge.html`
- `public\forge\scorecard.html`
- `public\forge\ci.html`
- `public\forge\evidence.html`
- `public\forge\releases.html`
- `public\forge\changelog.html`

The repository-level trend command should update or verify:

- `benchmarks\reports\forge-release-readiness-trend.json`
- `benchmarks\reports\forge-release-readiness-trend.md`

## Review Checklist

- Confirm the adoption smoke score is at least `90`.
- Confirm `dx check --strict-forge` passes for `.dx\adoption-app`.
- Confirm `dx forge release-bundle --verify` passes against the generated release bundle.
- Confirm `.dx\adoption-app\node_modules` does not exist.
- Confirm public route copy stays within the honest boundaries in `docs\forge-launch-limitations.md`.
- Confirm no public artifact contains secret marker strings such as `CLOUDFLARE_R2_`, `DX_FORGE_R2_LIVE`, `SECRET_ACCESS_KEY`, or `R2_SECRET`.

## Honest Public Claim

The claim this path supports is:

> DX Forge reduces install-time supply-chain blast radius for curated packages by materializing editable source files, blocking lifecycle scripts on the Forge path, writing reviewable receipts, and producing local release proof with no `node_modules` creation.

It does not support stronger claims until live advisory integrations, broader package ingestion, real customer projects, and full framework benchmark suites exist.
