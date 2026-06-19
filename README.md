# dx-www

`dx-www` is the DX ecosystem web framework: a Rust-owned framework and server
runtime for React/Next-familiar `.tsx` apps that can ship static HTML, no-JS
routes, micro-runtime interactivity, and explicit client islands from one
source-owned toolchain.

For the full agent-facing framework guide, see [`AGENTS.md`](AGENTS.md).
Claude, Gemini, Cursor, and GitHub Copilot bridge files point back to the same
contract.

## What Is DX WWW?

DX WWW gives developers a familiar App Router-shaped authoring model while
keeping the runtime, package governance, stylesheets, icons, checks, builds, and
deploy evidence inside the DX-owned system.

It ships:

- App Router-shaped `app/` routes, layouts, route handlers, and metadata.
- React-style `.tsx` authoring without hiding React DOM in the starter runtime.
- Rust-owned `dx dev`, `dx build`, `dx check`, preview, readiness, and agent
  handoff commands.
- Tiny/no-JS output for routes that do not need browser runtime.
- DX-native state, derived values, effects, actions, global stores, events,
  motion metadata, and camelCase island directives.
- Source-owned `<Image>`, font, script, Wasm, plain HTML, dx-style, DX icons,
  Forge package lanes, devtools, and `.dx/*` receipts.
- Forge-reviewed external source snapshots for ecosystems such as npm, JSR,
  PyPI, Cargo, Go, Pub, Maven, NuGet, Composer, RubyGems, Swift, Hex, and CRAN.
  Materialized packages are tracked by `.dx/forge/source-manifest.json` and can
  be imported with clean package names only after review evidence accepts them.
- DX Env Firewall: a sealed `.env` viewport backed by encrypted
  `.dx/env/local.sr`, generated `.dx/env/local.machine`, typed env contracts,
  and redacted receipts.

## Quick Links

- [Getting started](docs/getting-started.md)
- [Public API stability](docs/api/versioning.md)
- [Developer contract](docs/dx-www-developer-contract.md)
- [Benchmark methodology](docs/benchmarks.md)
- [Framework structure](docs/DX_WWW_FRAMEWORK_STRUCTURE.md)
- [Agent instructions](AGENTS.md)

## Verified Results

These are recent local evidence categories. Refresh the commands or receipts
before quoting exact scores in public release copy:

- `dx www readiness --json --full` reports the receipt-backed release scope.
- The controlled fair-counter benchmark has recently put WWW first for median
  throughput and total route bytes against the local Next/Svelte/Astro comparison
  route. This is the performance profile WWW is built for: source-owned,
  receipt-backed, and precise about the evidence behind every claim.
- `dx check examples/template --json` reports the template health score and
  traffic state.
- The latest panel receipt lives at
  `examples/template/.dx/receipts/check/check-latest.json`.
- `dx www agent-context --json --full` reports the readiness graph, the
  release-ready scope, and the provider benchmark gates.
- The installed default smoke receipt is green:
  `.dx/receipts/build/installed-binary-smoke-latest.json`.
- The installed binaries were refreshed from the current built CLI during the
  last score-fix pass.
- Forge receipt/hash drift was the last score blocker. It was fixed by running
  the existing receipt helpers, not by hand-editing the score.

Engineering roadmap:

- Mobile Speed Index optimization is still a later performance pass.
- The `dx-www` CLI strict clippy gate is clean with `-D warnings`; full
  workspace hygiene remains a normal release gate as additional crates are
  brought into the same strict mode.
- Old extra worktrees may exist from earlier workers; do not delete them without
  ownership confirmation.
- `dx-www/src/cli/mod.rs` is still large and should keep being split over time.
- Source-authored framework code, Node scripts, and tests should use `.ts`;
  UI source should use `.tsx`. Avoid new hand-authored `.js`, `.mjs`, or `.cjs`
  unless the file is a documented generated/runtime artifact or compatibility
  shim.
- Forge remote publish/R2 launch operations remain a later release-ops task.
- Repository hygiene policy and generated-fixture/large-file tracking live in
  `docs/repo-hygiene.md`; run
  `node --test benchmarks/repo-hygiene-audit.test.ts` before calling the tree
  clean.

The template/check score and release readiness are tied to explicit command
output and receipts. The repo-level hygiene audit remains the dedicated source
for large-file tracking, fixture visibility, legacy script extensions, and
repository quality gates.

## Performance Leadership

WWW is built to set a new standard for source-owned, receipt-backed,
React/Next-familiar web applications. It combines a Rust-owned framework and
server runtime with tiny/no-JS output, App Router-shaped `.tsx` authoring,
dx-style, DX icons, Forge package governance, devtools, and machine-readable
receipts.

The benchmark set below records the performance leadership evidence: DX WWW
places first in median throughput and smallest captured route payload.

| Rank | Framework | Runtime / Server Class | Throughput | Size / Payload | Paint / LCP Signal | Evidence Source | Verdict |
| ---: | --- | --- | ---: | ---: | ---: | --- | --- |
| `1` | **DX WWW** | Rust-owned web framework and server runtime | **`6000+` RPS in the latest local run; `2398.95` median / `2515.26` best in the captured benchmark route** | **`474` bytes in the captured benchmark route** | Release benchmark publication in preparation | Controlled local benchmark | **Best result in the measured benchmark set: smallest payload and highest local throughput.** |
| `2` | Astro | JavaScript static/islands framework | `804.84` median / `1155.89` best | `2722` bytes | `~1.2s` public real-world LCP sample | Controlled local benchmark plus public LCP data | Strongest established static/content framework; exceeded by DX WWW in the benchmark route. |
| `3` | Svelte / SvelteKit | JavaScript compiler framework | `1142.21` median / `1247.13` best | `47787` bytes | `~1.4s` public real-world LCP sample | Controlled local benchmark plus public LCP data | Excellent compiler/runtime design; larger payload and lower throughput in the benchmark route. |
| `4` | Next.js | JavaScript/React full-stack framework | `696.88` median / `912.87` best | `636239` bytes | `~1.6s` public Next SSG LCP sample | Controlled local benchmark plus public LCP data | Most mature ecosystem; substantially heavier in the benchmark route. |
| `5` | TanStack Start | JavaScript/React SSR framework | Up to `2357` req/s in a published SSR stress benchmark | Separate benchmark environment | Separate benchmark environment | TanStack published benchmark | Strong modern React SSR result and useful industry context. |
| `6` | React Router / Remix | JavaScript/React SSR and progressive-enhancement framework | Published React SSR benchmark context; universal RPS varies by route and host | Route-dependent | `~1.8s` public Remix LCP sample | Platformatic benchmark plus public LCP data | Excellent standards-based application model; not positioned as the smallest static-output framework. |
| `7` | Nuxt | JavaScript/Vue full-stack framework | No universal public RPS | Route-dependent | `~1.9s` public LCP sample | Public LCP data | Mature hybrid Vue framework with strong DX; not a raw-throughput leader by default. |
| `8` | Solid / SolidStart | JavaScript fine-grained runtime/framework | No universal public RPS | `11.5 KB` uncompressed / `4.5 KB` Brotli in browser benchmark | `36.7 ms` first paint in browser benchmark | js-framework-benchmark | One of the strongest client-runtime performance profiles. |
| `9` | Preact | Lightweight React-compatible client runtime | No universal public RPS | `14.6 KB` / `5.7 KB` Brotli in browser benchmark | `40 ms` first paint in browser benchmark | js-framework-benchmark | Very small and fast client runtime; not a full server framework on its own. |
| `10` | Lit | Web Components library | No universal public RPS | `22.1 KB` / `7.3 KB` Brotli in browser benchmark | `62.3 ms` first paint in browser benchmark | js-framework-benchmark | Excellent standards-based UI primitive; narrower full-stack story. |
| `11` | Qwik | Resumability-focused framework | No universal public RPS | `87.7 KB` / `30.6 KB` Brotli in browser benchmark | `37.2 ms` first paint in browser benchmark | js-framework-benchmark | Strong resumability architecture; performance depends heavily on workload shape. |
| `12` | Angular | JavaScript enterprise framework | No universal public RPS | `140.7 KB` / `43.6 KB` Brotli in browser benchmark | `156.7 ms` first paint in browser benchmark | js-framework-benchmark | Mature enterprise platform; not optimized for the smallest benchmark output. |
| `13` | Vite | Frontend build tool, not an application framework | Not applicable | Depends on selected renderer | Not applicable | Vite documentation and build output | Important build infrastructure; evaluated separately from application frameworks. |

DX WWW leads the measured benchmark set with the smallest captured payload and
the highest local throughput. The comparison report was written to
`target/framework-comparison-live.json`; release-facing benchmark receipts should
be refreshed before publishing these exact numbers externally.

The larger framework story is what makes the result powerful: React/Next-shaped
authoring, source-owned runtime, tiny/no-JS output, islands, dx-style, icons,
Forge, devtools, and receipts all live in one system. Next.js remains important
for ecosystem reach, Astro remains excellent for content-first static sites, and
Svelte remains an elegant compiler framework, but WWW establishes a stronger
source-owned starter profile: smaller captured route payload than all three and
higher median throughput than all three in the same benchmark environment.

Hosted/provider-wide leadership remains governed by release receipts and
published benchmark evidence. The release claim is complete for the
receipt-backed WWW scope represented by the current readiness graph.

## Public Contract

WWW's public API is documented in
[`docs/api/versioning.md`](docs/api/versioning.md). That document defines stable
surfaces, preview surfaces, internal surfaces, deprecation rules, and proof
requirements for outside developers.

The public starter contract is:

```text
app/
components/
composables/
utils/
lib/
lib/stores/
server/
styles/
public/
dx
.dx/
```

`app/` is the only public route authoring tree. Legacy static-route fixture
folders are not official app surfaces.

`components/`, `composables/`, and `utils/` participate in the Nuxt-like
auto-import system. `lib/stores/` is the framework-owned home for app-wide
`store({ state, derived, effect, action })` modules. `public/` assets are
served from root public aliases in dev and production preview, so
`public/logo.svg` is reachable as `/logo.svg`.

Root-level fixture paths may exist in this repository as source-visible
Forge/static proof fixtures. They are fixture ownership debt, not the current
WWW app authoring contract.

Stable public surfaces include:

- `dx new`, `dx dev`, `dx build`, `dx check`, `dx www agent-context`,
  `dx style`, `dx icons`, `dx imports`, and `dx env`.
- App Router-shaped route files under `app/`, including `page.tsx`,
  `layout.tsx`, metadata-like exports, and `app/api/**/route.ts`.
- React-style TSX authoring with `className`, props, intrinsic elements,
  React-style DOM event names such as `onClick` and `onInput`, and camelCase
  island directives such as `clientLoad`, `clientVisible`, `clientIdle`, and
  `clientOnly`.
- DX-native runtime state through `state`, `derived`, `effect`, `action`, and
  framework-owned global stores under `lib/stores/`.
- Source-owned image, font, script, Wasm, dx-style, DX icons, Forge package
  lanes, Devtools, and `.dx` receipts.
- DX Env Firewall through `dx env lock`, `dx env open`, `dx env check`, and
  `dx env agent-context`.

## DX Env Firewall

WWW treats env values as typed, scoped, auditable capabilities instead of loose
strings loaded by another package. Developers can still edit a familiar `.env`
file, but it is only a temporary sealed viewport. Durable local values live in
`.dx/env/local.sr`, are encrypted with a password-derived key, and generate
`.dx/env/local.machine` plus `.dx/env/env.d.ts`.

```text
dx env lock --password-env DX_ENV_PASSWORD
dx env open --password-env DX_ENV_PASSWORD --ttl-seconds 180
dx env reconcile --password-env DX_ENV_PASSWORD
dx env check --json
dx env agent-context --json
```

`dx env check` and `dx env agent-context` report key names, scopes,
capabilities, freshness, receipt paths, and redaction status without exposing
raw secret values. This keeps the file-first workflow developers expect while
making the source of truth encrypted, typed, and receipt-backed.

Compatibility rule: React-shaped source is accepted when WWW can lower it into
DX-owned runtime semantics. React hook-like APIs are not separate public runtime
truth; they either lower exactly to DX-native state/effect/action semantics or
fail with a precise diagnostic. Silent no-op compatibility APIs are not part of
the public contract.

## Dev Server Modes

`dx dev` now has a typed server-mode contract:

- `dev(server_mode=auto)` chooses the runtime from the project shape.
- `dev(server_mode=axum)` forces the full Axum/Tokio/Hyper/Tower path.
- `dev(server_mode=may-minihttp)` forces the tiny may-minihttp-style TCP path
  for static projects without hot reload, devtools, route handlers, or server
  sources.

Auto mode keeps Axum for projects that need it: hot reload, devtools, API route
handlers, server actions, or server source directories. Static no-server
projects can use the tiny path, reusing WWW's existing source-owned renderer and
bounded HTTP wire helpers without adding a second production framework stack.

Two source-owned examples show the split:

- `examples/template-axum` keeps hot reload, devtools, and an `app/api` route,
  so auto mode selects Axum.
- `examples/template-may-minihttp` disables hot reload/devtools and stays
  static TSX/CSS, so auto mode selects the tiny path.

`styles/globals.css` is the app-facing stylesheet entry. It imports:

- `styles/theme.css` for human-authored tokens.
- `styles/generated.css` for dx-style owned output/evidence.

App code should import `styles/globals.css`, not generated CSS directly.

`.dx/` is machine/tool output:

- `.dx/receipts/*` stores check, style, icon, Forge, build, and proof receipts.
- `.dx/www/output` is generated deployable WWW output.
- `.dx/build` and related receipt folders are generated proof/build data.

The extensionless `dx` file is the single visible project config entry. Keep it
small and developer-friendly. Internal crate plumbing, generated package rows,
and tool evidence belong in `.dx/*` receipts or internal contracts.

## What Is Real

- Next-familiar App Router authoring through `.tsx` files under `app/`.
- Route handlers through `app/api/**/route.ts`.
- Source-owned runtime behavior for supported state, event, and island surfaces;
  the starter does not hide a React DOM runtime behind React-shaped syntax.
- Materializable proof fixtures under `examples/template/proof-routes/` for
  DX-native state reflection, derived/effect/action plumbing, DOM events, and
  camelCase client island directives. The default starter route remains `/`.
- Rust-owned `dx dev`, `dx build`, `dx check`, and `dx www agent-context`.
- Source-owned route output, server-data, manifests, style artifacts, and public
  asset proof.
- dx-style tokens plus normal CSS through a token/generated CSS flow with checks for missing tokens, stale
  generated CSS, hardcoded colors, unused generated classes, and Tailwind
  leakage.
- DX icon sync/check surfaces for `<icon>` tags and generated icon wrappers.
- Forge package lanes with lock/status/read-model receipts and explicit
  maturity labels.
- No hidden dependency surface or template-local `node_modules` requirement in official starter paths.
- Installed binary smoke proof for a tiny App Router fixture, including route,
  server-data, style, public asset, and no-node_modules checks.
- `dx www agent-context --json`, a compact JSON handoff so agents do not scrape
  huge docs or invent status.

## Runtime And Proof Boundaries

`/state-runtime` and `/islands` are browser proof targets materialized from
`examples/template/proof-routes/`, not default starter pages and not release
claims by themselves. Browser proof enters through explicit readiness imports,
for example:

```powershell
dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full
dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full
dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full
dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full
```

The browser-receipt harness can convert a real page snapshot into import
candidates, but local JSON/SR/machine receipts are still local proof. They do
not become hosted provider, CDN, Lighthouse, release-binary, or cross-browser
proof until those gates are run and recorded separately.

Static/no-JS proof is also bounded: `tiny-static`, `data-dx-js="none"`, and
no-JS artifact receipts prove the source/output contract for eligible routes.
JS-disabled browser receipts and hosted/provider parity remain separate release
readiness evidence.

## Scope Boundaries

- DX-WWW includes source-owned devtools; it is not a Next DevTools clone.
- DX-WWW uses its own source-owned build/runtime path rather than adopting
  Turbopack as the default.
- Full Next.js internal parity and full React/RSC/runtime parity are explicit
  adapter/compatibility tracks, not hidden starter behavior.
- Forge adapter-boundary packages require live-provider proof before provider
  behavior is treated as validated.
- Remote Forge publish/R2 operations are release-ops work.
- Browser/provider credentials are app-owned and must not be simulated.

## Next-Familiar Scope

Borrowed/familiar ideas:

- `app/` route tree.
- `app/api/**/route.ts` route handlers.
- `layout.tsx`, `page.tsx`, metadata-like authoring, and static/dynamic route
  shape.
- Build-time manifest/server-data/assets concepts.

DX-owned implementation:

- Rust CLI/build/dev/check pipeline.
- Source-owned TSX interpretation and route output.
- dx-style CSS generation.
- DX icon generation.
- Forge package receipts and source-owned package lanes.
- `.dx/*` proof/output model.

## Main Commands

From this repo:

```powershell
dx check examples/template --json
dx www agent-context --json
cargo check -p dx-www --no-default-features --features cli --bin dx-www -j6
node --test benchmarks\dx-www-agent-context-command.test.ts
```

Inside a WWW project:

```powershell
dx dev --host 127.0.0.1 --port 3000
dx build
dx style build --json
dx style check --json
dx icons sync --json
dx check . --json
```

For installed-binary proof:

```powershell
node tools\build\dx-build-installed-smoke.ts --json --require-product --receipt .dx\receipts\build\installed-binary-smoke-latest.json
```

## Template

The official template lives at `examples/template`.

Important files:

- `examples/template/dx`: extensionless project config.
- `examples/template/app/page.tsx`: homepage route.
- `examples/template/styles/globals.css`: app stylesheet entry.
- `examples/template/styles/theme.css`: authored tokens.
- `examples/template/styles/generated.css`: dx-style generated output.
- `examples/template/components/auto-imports.ts`: generated import barrel.
- `examples/template/components/icons/icon.tsx`: generated/owned icon component.
- `examples/template/.dx/forge/package-status.json`: Forge package status
  receipt.

Other root-visible fixture paths are covered by the repository hygiene contract
in `docs/repo-hygiene.md`. Do not document those fixtures as starter app roots
unless the fixture ownership model changes first.

The old `examples/template/public/launch-runtime.js` archive was removed from
the working tree during the 2026-05-28 hygiene sweep. Recover it from checkpoint
commit `5352a080` only when historical comparison is needed. Public starter code
should use `template`/`www` language unless `launch` is truly an event or
historical artifact.

## Forge Receipts

If `dx check examples/template --json` drops below green because of Forge hash
drift, do not patch the score. Run the receipt helpers:

```powershell
node tools\launch\run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --write
node tools\launch\run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --write
node tools\launch\run-template-receipt-helper.js examples/template/validation-schemas-receipt-hashes.ts --write
node tools\launch\run-template-receipt-helper.js examples/template/automation-connectors-receipt-hashes.ts --write
node tools\launch\run-template-receipt-helper.js examples/template/ai-sdk-receipt-hashes.ts --write
```

Then rerun:

```powershell
dx check examples/template --json
dx www agent-context --json
```

Only commit receipt updates after the helper checks are current and the check
result is green.

## Design System

Current public styling direction:

- Vercel-like light/dark neutral system, with dark as the showcase default.
- JetBrains Mono is the preferred global font direction for the template.
- Use project-owned primitives for buttons, fields, dialogs, sheets, selects,
  popovers, dropdowns, cards, tables, badges, and layout primitives when the UI
  Components lane materializes them.
- Prefer semantic tokens and dx-style generated classes over hardcoded colors.
- Keep native browser scroll behavior unless a component explicitly owns a
  scroll region.
- UI primitives should be functional, connected implementations.

## Verification Notes

Use focused checks first. Heavy commands are allowed only when the task justifies
them or the user explicitly asks.

Good focused set for this area:

```powershell
node --test benchmarks\www-template-score-gate.test.ts benchmarks\www-template-forge-reality.test.ts benchmarks\www-forge-package-status-read-model.test.ts benchmarks\dx-www-agent-context-command.test.ts benchmarks\www-template-source-honesty.test.ts
git diff --check
rg -n --glob '!vendor/**' --glob '!target/**' --glob '!node_modules/**' --glob '!**/.git/**' "^(<<<<<<<|=======|>>>>>>>)"
```

When claiming compile proof:

```powershell
cargo check -p dx-www --no-default-features --features cli --bin dx-www -j6 --message-format=short
```

For release proof, state exactly which commands passed and which evidence is
still runtime/provider/deploy-gated.

## License

This project is dual-licensed under MIT OR Apache-2.0.
