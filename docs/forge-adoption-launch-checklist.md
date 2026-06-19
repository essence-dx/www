# DX Forge Adoption Launch Checklist

This checklist is the adoption-first release path for proving DX Forge on a clean app-shaped project before promoting public launch copy. It ties `dx forge ci`, adoption smoke, adoption report, Pages preview, and benchmark evidence into one reproducible flow.

Run commands from `G:\WWW`.

Use this document when the release question is: "Can another developer reproduce the Forge adoption proof locally without `node_modules`, secrets, or a full framework migration claim?"

Use `docs\forge-public-beta-quickstart.md` first when the reviewer needs the shorter developer-facing beta path from `dx forge init-app` to `/forge/quickstart` before running the full adoption launch checklist.

## Launch Boundary

This checklist proves:

- a clean project can materialize source-owned packages through Forge;
- `pages\forge-adoption.html` references `shadcn/ui/button`, `shadcn/ui/card`, `dx/icon/search`, `auth/better-auth`, `animation/motion`, and `migration/static-site` together;
- Forge writes source manifests, receipts, package docs, adoption reports, claims, DXPK packets, and proof summaries;
- Pages preview artifacts verify before publishing;
- benchmark evidence exists for the local adoption route and package-update rehearsal;
- no `node_modules` folder is created by the Forge adoption path.

This checklist does not prove:

- universal npm, cargo, pip, or package-manager replacement;
- live customer adoption;
- enterprise migration safety;
- full Astro, Svelte, HTMX, Next.js, WordPress, or React benchmark dominance.

## 1. Preflight

Start from a clean working tree and run lightweight Rust checks:

```powershell
git status --short
cargo fmt --manifest-path .\www\Cargo.toml -p dx-www -- --check
cargo check --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www
```

Do not continue with unrelated dirty files, compile errors, local `node_modules`, or secret-environment assumptions such as `CLOUDFLARE_R2_`, `DX_FORGE_R2_LIVE`, `R2_SECRET`, or `SECRET_ACCESS_KEY`.

## 2. Generate The Clean Adoption App

Run the adoption smoke against a disposable local project:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge adoption-smoke --project .\.dx\adoption-app --format markdown --output .\.dx\adoption-app\.dx\forge\adoption-smoke\adoption-smoke.md --fail-under 90
```

Expected project evidence:

- `.dx\adoption-app\dx.config.toml`
- `.dx\adoption-app\pages\forge-adoption.html`
- `.dx\adoption-app\.dx\forge\source-manifest.json`
- `.dx\adoption-app\.dx\forge\receipts`
- `.dx\adoption-app\.dx\forge\docs`
- `.dx\adoption-app\.dx\forge\adoption-smoke\forge-smoke.json`
- `.dx\adoption-app\.dx\forge\adoption-smoke\release-bundle\forge-release-manifest.json`
- `.dx\adoption-app\public\forge.html`
- `.dx\adoption-app\public\forge\scorecard.html`
- `.dx\adoption-app\public\forge\ci.html`
- `.dx\adoption-app\public\forge\evidence.html`
- `.dx\adoption-app\public\forge\releases.html`
- `.dx\adoption-app\public\forge\changelog.html`

The app must not contain `.dx\adoption-app\node_modules`.

## 3. Verify The App Structure

Run strict Forge checks against the generated app:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- check .\.dx\adoption-app --strict-forge --format markdown --fail-under 90
```

Review the output for package docs coverage, rollback coverage, source-owned package counts, and absence of red Forge traffic.

## 4. Generate The Adoption Report

Write the human report from the existing adoption project:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge adoption-report --project .\.dx\adoption-app --format markdown --output .\.dx\adoption-app\.dx\forge\adoption-smoke\adoption-report.md --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge adoption-report --project .\.dx\adoption-app --format json --output .\.dx\adoption-app\.dx\forge\adoption-smoke\adoption-report.json --fail-under 90 --quiet
```

The report must summarize:

- project structure;
- source-owned package IDs;
- receipt count;
- package docs status;
- strict `dx check` score;
- public route artifacts;
- release-bundle verification;
- no-`node_modules` proof;
- honest scope boundaries.

## 5. Review Source-Owned Package Fixture Evidence

Run the joined package review:

```powershell
node .\benchmarks\measure-forge-source-owned-package-review.ts
```

The report must pass at `90` or higher and show docs, receipts, curated advisory placeholders, `dx forge verify-package --all`, local-edit yellow review, rollback rehearsal, and no-`node_modules` proof together.

## 6. Generate The Public Adoption Route

Generate `/forge/adoption` from the real adoption report model:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- prove vertical --fixture forge-adoption --out .\.dx\adoption-app\public --write --format markdown
```

Expected route artifacts:

- `.dx\adoption-app\public\forge\adoption.html`
- `.dx\adoption-app\public\forge\adoption.dxp`
- `.dx\adoption-app\public\forge\adoption.claims.json`
- `.dx\adoption-app\public\proof.json`
- `.dx\adoption-app\pages\forge\adoption.html`

The route must stay static/no-runtime unless a later TODO explicitly changes the adoption route strategy.

## 7. Run The CI Artifact And Pages Preview Lane

Generate the secret-free CI artifacts and Pages preview:

```powershell
.\scripts\ci\forge-ci.ps1 -ArtifactDir .\.dx\ci -PagesDir .\.dx\forge-pages -FailUnder 90
```

Then verify both folders without regenerating them:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge ci --verify-artifacts .\.dx\ci --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge ci --verify-pages .\.dx\forge-pages --fail-under 90
```

Required adoption artifacts:

- `.dx\ci\forge-adoption-smoke.json`
- `.dx\ci\forge-adoption-report.json`
- `.dx\ci\forge-adoption-report.md`
- `.dx\ci\forge-adoption-page.html`
- `.dx\ci\forge\adoption.html`
- `.dx\ci\forge\adoption\index.html`
- `.dx\ci\forge\adoption.claims.json`
- `.dx\ci\forge\adoption.dxp`
- `.dx\ci\forge\adoption.proof.json`
- `.dx\forge-pages\forge\adoption.html`
- `.dx\forge-pages\forge\adoption\index.html`
- `.dx\forge-pages\forge\adoption.claims.json`
- `.dx\forge-pages\forge\adoption.dxp`
- `.dx\forge-pages\forge\adoption.proof.json`

The Pages verifier must pass before any public preview is promoted.

## 8. Refresh Adoption Benchmark Evidence

Run local adoption browser evidence:

```powershell
node .\benchmarks\measure-forge-adoption-browser-smoke.ts
```

Run package-update rehearsal evidence:

```powershell
node .\benchmarks\measure-forge-package-update-rehearsal.ts
```

Refresh the public route comparison that includes `/forge/adoption`:

```powershell
$env:DX_VERTICAL_BUDGET_GATE = "1"
node .\benchmarks\measure-vertical-proof.ts
Remove-Item Env:\DX_VERTICAL_BUDGET_GATE
```

Review:

- `benchmarks\reports\forge-adoption-browser-smoke.json`
- `benchmarks\reports\forge-adoption-browser-smoke.md`
- `benchmarks\reports\forge-package-update-rehearsal.json`
- `benchmarks\reports\forge-package-update-rehearsal.md`
- `benchmarks\reports\forge-public-route-comparison.json`
- `benchmarks\reports\forge-public-route-comparison.md`

The benchmark lane must not run package installs. Static-floor framework comparison belongs in `benchmarks\compare-forge-static-competitors.ts`; live framework rows belong only in the opt-in `DX_FORGE_LIVE_FRAMEWORKS=1` harness.

## 9. Assemble The Adoption-Inclusive Bundle

Use the adoption-inclusive bundle only after the route comparison and release history already include `/forge/adoption`:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --project . --out .\.dx\forge-release-bundle-adoption --include-adoption --format markdown --fail-under 90
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --verify .\.dx\forge-release-bundle-adoption --include-adoption --format markdown --fail-under 90
```

Do not replace the default six-route operator bundle with the adoption-inclusive bundle until a reviewer explicitly promotes the adoption evidence lane.

To prove the bundle can bootstrap a clean beta app with one reviewable trail, run:

```powershell
cargo run --manifest-path .\www\Cargo.toml -p dx-www --bin dx-www -- forge beta-install --project .\.dx\forge-beta-app --release-bundle .\.dx\forge-release-bundle-adoption --artifacts .\.dx\forge-beta-app\.dx\forge\beta-install --write --format markdown --output .\.dx\forge-beta-app\.dx\forge\beta-install\beta-install.md --fail-under 90
```

This writes `.dx\forge-beta-app\.dx\forge\beta-install\forge-beta-install.ps1`, release-bundle verification, provenance, trust-regression, adoption-report, copied public route artifacts, and no-`node_modules` evidence.

## 10. Review Signoff

Before public copy is approved, record:

| Check | Result | Evidence |
| --- | --- | --- |
| Adoption smoke passed | pass/fail | `.dx\adoption-app\.dx\forge\adoption-smoke\forge-smoke.json` |
| Adoption report passed | pass/fail | `.dx\adoption-app\.dx\forge\adoption-smoke\adoption-report.md` |
| Strict Forge check passed | pass/fail | `dx check .\.dx\adoption-app --strict-forge` |
| Pages preview verified | pass/fail | `.dx\forge-pages\forge\adoption\index.html` |
| Browser smoke measured | pass/fail | `benchmarks\reports\forge-adoption-browser-smoke.md` |
| Package update rehearsal passed | pass/fail | `benchmarks\reports\forge-package-update-rehearsal.md` |
| Source-owned package review passed | pass/fail | `benchmarks\reports\forge-source-owned-package-review.md` |
| Bundle beta install passed | pass/fail | `.dx\forge-beta-app\.dx\forge\beta-install\forge-beta-install.json` |
| No secrets or `node_modules` | pass/fail | `dx forge ci --verify-artifacts`, `dx forge ci --verify-pages`, and local app inspection |

Approve only if every row is `pass` and the copy keeps the honest scope: local reproducible adoption evidence, source-owned packages, reviewable receipts, and no universal package-manager or full-framework replacement claim.
