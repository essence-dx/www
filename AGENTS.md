# DX WWW Agent Contract

This file is the canonical agent-facing guide for `G:\Dx\www`.

All agents should read this file before changing code, docs, tests, examples,
or receipts in this repository. Tool-specific files such as `CLAUDE.md`,
`GEMINI.md`, `.cursorrules`, `.cursor/rules/dx-www.mdc`, and
`.github/copilot-instructions.md` are lightweight bridges back to this
contract; they must not become separate sources of truth.

## Start Here

1. Verify the repo root and branch with `git status --short --branch`.
2. Read `README.md` for the public framework overview and benchmark results.
3. Read this file for framework rules and work habits.
4. Use `rg` / `rg --files` for source scans.
5. Prefer focused checks before broad builds or heavy suites.

Current branch in the main working copy is usually `features`, but agents must
verify it. Do not assume `dev`, `main`, or any lane branch without checking.

## Framework Snapshot

Treat this section as orientation, not as a fresh release certificate:

- `dx www readiness --json --full` and `dx check examples/template --json`
  are the receipt-backed release/readiness authorities. Refresh them before
  repeating a numeric score.
- Recent local evidence has shown WWW leading the controlled fair-counter tiny
  route in bytes and median throughput. Quote the exact RPS/byte numbers only
  from a current receipt or command output.
- Benchmark results are controlled local evidence. Hosted/provider-wide
  performance publication is tracked separately.
- Template production preview must prove that the default `/` route is listed
  in `deploy-adapter.json`, that devtools/scripts are absent from production
  HTML, and that public assets work from both `/public/<asset>` and root public
  aliases such as `/logo.svg`. State-runtime and island browser proof routes
  must be materialized from `examples/template/proof-routes/` before expecting
  `/state-runtime` or `/islands` in preview output.
- The `dx-www` CLI strict clippy gate is clean with `-D warnings`; keep it that
  way before calling framework work complete.

Always refresh these numbers from command output or receipts before presenting
them as current. Do not repeat old perfect-score or branch claims unless they
are verified in the current turn.

## Framework Identity

DX WWW is the DX ecosystem web framework:

- React/Next-familiar `.tsx` authoring.
- App Router-shaped `app/` routes.
- Rust-owned CLI, dev server, build, check, and readiness tooling.
- Source-owned TSX lowering and runtime behavior.
- dx-style generated CSS and checks.
- DX icons.
- Forge package lanes, source manifests, locks, receipts, trust policy, and
  package docs.
- `.dx/*` evidence, serializer `.sr` receipts, and generated `.machine`
  contracts.
- DX Env Firewall for typed, scoped, redacted environment capabilities.

WWW is not Next.js internally. It must not hide React DOM, Turbopack, Vite,
Webpack, or template-local `node_modules` behind source-owned claims.

## Current Authoring Contract

Keep these newer WWW capabilities explicit in docs, examples, and agent work:

- Global state is framework-owned. Use `store({ state, derived, effect, action })`
  for WWW-native app stores, usually under `lib/stores/`, and let route units
  link those stores across imported source files.
- `State Management` is the Forge package lane for upstream Zustand provenance.
  Treat that as a source-owned compatibility/package lane, not as the internal
  WWW state runtime.
- React-style events are first-class. Quoted event values such as
  `onClick="bg-red-500 scale-up"` are interaction-class commands; braced values
  such as `onClick={() => counterStore.increment(counterStore)}` are logic that
  must lower safely or produce a diagnostic.
- DX Style owns Tailwind-like `className` authoring, event-class strings,
  motion class strings, grouped syntax such as `hover:(bg-accent
  text-accent-foreground)`, and the balance between generated atomic utilities
  and authored custom CSS. Do not hand-edit generated CSS.
- DX Icon is first-party. Author icons through `<Icon name="pack:check" />` or
  configured source/runtime tags and let `dx icons sync/check` own generated
  wrappers. Do not import npm icon packages in official WWW starters.
- Forge imports are package materialization gates, not package-manager installs.
  Reviewed `npm`, `jsr`, `pip`, `cargo`, `go`, `pub`, `maven`, `nuget`,
  `composer`, `gem`, `swift`, `hex`, and `cran` packages can be planned,
  scored, sliced, bridged, rejected, or materialized according to evidence.
- Accepted source snapshots live under `lib/forge/<ecosystem>/<package>/` and
  are tracked in `.dx/forge/source-manifest.json`. WWW resolves clean
  package-name imports such as `import { Vector3 } from "three"` only when that
  source manifest points to reviewed Forge-owned files.

## Public API Stability

Use `docs/api/versioning.md` as the release-facing public API stability
contract. Use `docs/dx-www-developer-contract.md` as the source of truth for
project shape and authoring rules.

Stable public surfaces include:

- `dx new`, `dx dev`, `dx build`, `dx check`, `dx www agent-context`,
  `dx style`, `dx icons`, `dx imports`, and `dx env`.
- `app/`, `components/`, `composables/`, `utils/`, `lib/`, `lib/stores/`,
  `server/`, `styles/`, `public/`, `.dx/`, and the root `dx` config.
- TSX route files, React-style DOM event names, camelCase island directives,
  DX-native state primitives, source-owned assets, and `.dx` receipts.

React hook-like compatibility syntax is not a separate runtime truth. It must
lower exactly to DX-native `state`, `derived`, `effect`, and `action` semantics
or fail with a precise diagnostic. Never document silent no-op hooks as a
feature.

## Public Project Structure

The official WWW starter contract is:

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

Folder responsibilities:

| Path | Purpose |
| --- | --- |
| `app/` | Route tree, pages, layouts, route handlers, metadata, and user-editable route surfaces. |
| `app/page.tsx` | Root route. Keep it thin; move reusable UI and logic out. |
| `app/layout.tsx` | Shared document/app shell and metadata defaults. |
| `app/api/**/route.ts` | API and server route handlers. |
| `components/` | Reusable TSX components and generated `components/auto-imports.ts`. |
| `composables/` | Auto-importable helper/composable functions. |
| `lib/` | App utilities, domain helpers, adapters, data helpers, and shared types. |
| `lib/stores/` | Framework-owned global store modules using `store`, `state`, `derived`, `effect`, and `action`. |
| `server/` | Server-only helpers and app-owned backend boundaries. |
| `styles/` | Authored tokens, generated CSS, and global stylesheet entry. |
| `public/` | Static public assets. |
| `dx` | Extensionless human-facing project config. |
| `.dx/` | Generated output, receipts, serializer contracts, import maps, and evidence. |

## DX Env Firewall

WWW owns env safety at the framework level. Treat `.env` as a short-lived editable
viewport, not the durable source of truth.

```text
dx env lock --password-env DX_ENV_PASSWORD
dx env open --password-env DX_ENV_PASSWORD --ttl-seconds 180
dx env reconcile --password-env DX_ENV_PASSWORD
dx env check --json
dx env agent-context --json
```

Rules:

- Durable local env values belong in encrypted `.dx/env/local.sr` with generated
  `.dx/env/local.machine`.
- `.dx/env/env.d.ts` is the typed contract for `dx/env` imports.
- `.dx/receipts/env/check-latest.json` is a redacted read model; it may contain
  key names, scopes, capabilities, hashes, freshness, and paths, but never raw
  values.
- Agents may use `dx env agent-context --json` for names/scopes/missing status.
  They must not ask for or print secret values.
- Browser/client code must not receive server-only env values. Leak prevention
  belongs in framework checks, not template convention.

Rules:

- `app/` is the public route authoring truth.
- Do not present `pages/`, `.dxob`, `runtime-pages`, or old launch-runtime
  folders as current app authoring surfaces.
- Keep root-visible fixture paths precise: fixtures are not starter app roots.
- Generated `.dx/` files are machine output. Fix generators or source inputs,
  then regenerate.

## The `dx` Config

The extensionless `dx` file is the visible project config. Keep it small,
readable, and developer-friendly.

The current starter owns:

```text
www(app_dir=app output_dir=.dx/www/output)
dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true server_mode=auto)
style(tokens=styles/theme.css generated_css=styles/generated.css)
imports(map=.dx/imports/import-map.json barrel=components/auto-imports.ts declarations=.dx/imports/imports.d.ts)
icons(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components/icons)
forge(policy=forge-first-no-node-modules)
check(score_scale=500 lighthouse=true)
```

Durable generated state belongs in `.dx/*` receipts, `.sr` serializer receipts,
and generated `.machine` contracts. Do not invent new durable JSON formats when
the serializer contract should own the state.

## Routes

WWW routes are discovered from `app/`.

```text
app/page.tsx                 -> /
app/about/page.tsx           -> /about
app/blog/[slug]/page.tsx     -> /blog/:slug
app/docs/[...parts]/page.tsx -> /docs/*
app/api/health/route.ts      -> /api/health
```

Page files export a default TSX function. Layout files wrap child routes.
Route handlers export HTTP functions such as `GET`, `POST`, `HEAD`, and
`OPTIONS`.

Safe static metadata can be merged from layouts and pages. Dynamic request-time
metadata, cookies/headers, arbitrary async Server Components, and full RSC
semantics remain bounded unless a specific source-owned implementation and
receipt proves them.

## Rendering Model

WWW uses capability-based delivery tiers:

| Tier | When used | Output goal |
| --- | --- | --- |
| Static/no-JS | HTML, CSS, links, forms, metadata, and assets only. | No client runtime for that route. |
| Micro runtime | Supported DX-native state, events, actions, or effects. | Minimal source-owned JS, not React DOM. |
| Islands | A component needs delayed or conditional interactivity. | Lazy source-owned island chunk. |
| Adapter boundary | A React/Svelte/other framework component is explicit. | Honest adapter surface, not hidden runtime adoption. |

A route that does not need JavaScript should not get JavaScript just because it
is authored in TSX. A route that needs interactivity should receive the smallest
supported runtime surface.

## TSX Authoring

Supported patterns include:

- Intrinsic HTML/SVG elements.
- Static attributes, `className`, simple `style`, booleans, strings, numbers,
  and expressions the compiler can reason about.
- Layout/page composition.
- Known route/search parameter projection.
- Static-safe `next/link`, `next/image`, `next/script`, and `next/font`
  foundations.
- DX-owned state/event/island/motion metadata where behavior can be proven.
- Framework-owned global store modules imported from `lib/stores/*.ts`.

Unsupported React behavior must not become a silent no-op. Lower it exactly,
diagnose it, or require an explicit adapter island.

## Next-Familiar Primitives

These are source-owned compatibility primitives. They are not complete Next
runtime clones.

### `next/link`

Static-safe `next/link` imports lower `<Link href="/path">` to source-owned
`<a>` output with DX framework markers. Full Next router prefetch/cache
semantics are tracked as compatibility work.

### `next/image`

`next/image` support exists for static-safe usage.

What WWW does:

- Detects default or named `Image` imports from `next/image`.
- Lowers supported `<Image />` usage to `<img>`.
- Preserves static `src`, `alt`, `width`, and `height`.
- Converts `priority` into eager loading and high fetch priority metadata.
- Adds `decoding`, `loading`, `data-nimg`, and DX image boundary markers.
- Handles `fill` by removing fixed dimensions and adding fill-style metadata.

Current support boundary:

- Hosted image optimization service.
- Remote loader parity.
- Complete `srcset` generation for every loader.
- Full Next image cache/provider behavior.

### `next/font/google` And `next/font/local`

`next/font` support exists as a source-owned foundation.

What WWW does:

- Detects `next/font/google` and `next/font/local` imports.
- Requires module-scope calls assigned to `const` bindings for safe lowering.
- Records loader, binding, call count, module scope, const assignment, CSS
  variable intent, and compatibility issues.
- Maps `font.className` and `font.variable` expressions into source-owned class
  and inline style metadata on static output.
- Emits DX font markers for receipts.

Current support boundary:

- Downloading Google font binaries.
- Hosted font cache.
- Complete Next font manifest parity.
- Full cross-provider font optimization.

### `next/script`

Static-safe `next/script` lowers to `<script>` with strategy metadata. Full
Next lifecycle callbacks, worker strategy, and ordering semantics are tracked as
compatibility work.

## State, Stores, Events, And Actions

WWW is DX-native here, not React-runtime dependent.

Public primitives:

- `state()` for local reactive values.
- `derived()` for computed values.
- `effect()` for supported side-effect boundaries.
- `action()` for event and mutation semantics.
- `store()` for framework-owned global stores.

React-style event names are the public authoring style:

```tsx
<button onClick={action(() => count.set(count.get() + 1))}>Add</button>
<input onInput={action((event) => name.set(event.currentTarget.value))} />
```

Global stores live in normal source files, usually under `lib/stores/`:

```tsx
export const counterStore = store({
  count: state(0),
  doubled: derived((self) => self.count * 2),
  log: effect((self) => console.log(self.count)),
  increment: action((self) => {
    self.count += 1;
  }),
});
```

WWW scans `app/`, `components/`, and `lib/stores/` source so route units can
link imported global store slots and actions across files. This is the default
framework state path; do not replace it with template-only Zustand-style
packages for first-party WWW behavior.

If the Forge package catalog exposes upstream Zustand provenance, refer to it
with the professional lane name `State Management`. Keep that package lane
explicit and separate from WWW-native `store()` global stores so agents do not
mistake a compatibility/provenance package for the framework state runtime.

Event lowering is intentionally split by attribute shape:

```tsx
<button onClick="scale-up bg-accent">Save</button>
<button onClick={() => counterStore.increment(counterStore)}>Add</button>
```

- A quoted string event value is an interaction-class command. It lowers to
  `data-dx-on-click-class` and the source-owned binder applies those classes
  without creating a state dispatch.
- A braced expression event value is logic. It lowers through the state/action
  runtime when WWW can prove it safely.
- Unsupported event logic must be diagnostic-only or require an explicit
  adapter boundary. Do not silently no-op.

The native DOM event catalog is generated from the local MDN browser compat
data mirror and recorded in `.dx/receipts/readiness/native-events-latest.*`.
As of the latest receipt in this checkout, the compiler catalog contains
`321` MDN event names, with `missing=0` and `extra=0`. Refresh that receipt
before quoting the number as current.

`useState` may remain as compatibility sugar only when WWW can lower it exactly
to a compiler-owned state slot. Advanced React hooks such as arbitrary
`useEffect`, `useReducer`, `useContext`, transitions, and full React scheduling
must diagnose or require an adapter boundary until truly implemented.

## Motion

WWW motion is source-owned and DX Style-backed by default.

Authoring forms:

```tsx
<button
  motion="button(transition-transform duration-200) animation-pop(opacity:0..1,scale:0.96..1)"
  onClick="scale-up"
>
  Save
</button>
```

Current behavior:

- `motion` and `dxMotion` attributes lower to DX motion metadata such as
  `data-dx-motion`, `data-dx-motion-engine`, and `data-dx-motion-class`.
- dx-style scans `motion=`, `dxMotion=`, and `data-dx-motion-class=`.
- Named animation syntax such as
  `animation-pop(opacity:0..1,scale:0.96..1)` generates the keyframes and
  animation rule through DX Style.
- Group syntax remains owned by dx-style, so motion can stay compact in TSX
  while the generated CSS remains inspectable.

Motion docs should identify Forge Motion packages and React/Svelte adapter
islands as explicit adapter boundaries rather than hidden Framer Motion or React
Motion runtimes.

## Islands

WWW island directives use camelCase props:

```tsx
<Counter clientLoad />
<Chart clientVisible={{ rootMargin: "200px" }} />
<Editor clientIdle={{ timeout: 1200 }} />
<ReactWidget clientOnly="react" />
<SvelteWidget clientOnly="svelte" />
```

Supported directive names:

- `clientLoad`
- `clientVisible`
- `clientIdle`
- `clientOnly`
- `clientMedia`
- `clientInteraction`

Rules:

- Source-owned islands are the default.
- Every island route should preserve meaningful no-JS fallback HTML.
- `clientOnly="react"` or `clientOnly="svelte"` is an explicit adapter
  boundary.
- Astro-style `client:load` syntax is intentionally not the WWW public style.

## Auto Imports

WWW has Nuxt-like auto-import infrastructure with explicit receipts.

Default scan roots:

```text
components
composables
utils
```

Generated artifacts:

```text
components/auto-imports.ts
.dx/imports/import-map.json
.dx/imports/imports.d.ts
.dx/imports/sync.sr
.dx/imports/check.sr
```

Aliases:

```text
#imports
#components
```

Rules:

- The generated barrel must stay visible and reviewable.
- IDE declarations live at `.dx/imports/imports.d.ts`.
- `used_only=true` keeps imports scoped to used symbols.
- `dx check` and `dx build` can fail stale import maps.

Useful commands:

```powershell
dx imports sync
dx imports check
```

## Styles

The public stylesheet entry is `styles/globals.css`.

```text
styles/theme.css      # authored tokens
styles/generated.css  # dx-style generated output
styles/globals.css    # imports both
```

Rules:

- Import `styles/globals.css` from `app/layout.tsx`.
- Put tokens in `styles/theme.css`.
- Let dx-style own `styles/generated.css`.
- Avoid hardcoded colors when a token exists.
- Do not patch generated CSS by hand unless the generator contract explicitly
  requires it.
- Treat DX Style as the Tailwind-like class authoring layer for WWW, not as a
  separate CSS framework bolted onto the app.
- Atomic utility classes and authored custom CSS are both first-class. Use
  atomic classes for common layout, color, motion, and interaction tokens; keep
  app-specific component structure in normal CSS classes when that is clearer.
- Keep source class strings readable with dx-style grouping syntax:

```tsx
<section className="grid gap-4 hover:(bg-accent text-accent-foreground) md:(grid-cols-2 gap-6)">
  <button onClick="scale-up bg-accent">Save</button>
</section>
```

- Grouped class names expand through dx-style into inspectable generated CSS.
  Invalid groups should surface diagnostics such as `dx-grouping-error:*`;
  never silently drop broken styling input.
- Configured helper aliases such as `cn`, `clsx`, `classNames`, `tw`, `dx`, and
  `dxStyle` should lower into the same dx-style token pipeline when the local
  config supports them.
- Event class strings, motion class strings, and ordinary `className` strings
  share the same styling vocabulary. Do not invent a parallel interaction CSS
  format.

Useful commands:

```powershell
dx style build --json
dx style check --json
```

## Icons

WWW uses DX Icon as the first-party icon path. Do not install or import npm icon
packs in first-party WWW starters when the icon can be expressed through DX
Icon.

The starter uses a source-owned icon component under
`components/icons/icon.tsx`, so normal TSX can stay compact:

```tsx
<Icon name="pack:check" />
```

The `dx` config owns:

```text
icons(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components/icons)
```

Rules:

- Use the configured `Icon` component in TSX for app-authored icons.
- Preserve `source_tag=icon` and `runtime_tag=dx-icon` semantics for generated
  and runtime icon surfaces.
- Keep generated icon components under the configured `generated_dir`.
- Missing icons must remain visible through explicit missing-icon diagnostics or
  markers such as `data-dx-icon-missing`; do not replace them with empty spans.
- If an external pack is needed, bring it through the DX Icon/Forge provenance
  path instead of adding a template-local package dependency.

Useful command:

```powershell
dx icons sync --json
```

## Dev Server

Use:

```powershell
dx dev --host 127.0.0.1 --port 3000
```

Server modes:

| Mode | Use |
| --- | --- |
| `auto` | Default. Chooses the runtime from project shape. |
| `axum` | Full path for hot reload, devtools, API routes, server sources, and richer dev behavior. |
| `may-minihttp` | Tiny TCP responder for simple static projects with hot reload and devtools disabled. |

Devtools and hot reload are development-only. They must not appear in
production `dx build` output.

## Devtools

DX Devtools are framework-level and dev-only.

Responsibilities:

- Inspect DOM/source markers.
- Show route/source/status information.
- Show diagnostics and error overlays.
- Preview style edits without mutating source.
- Apply style edits only when the source target is known and safe.
- Stay entirely out of production builds.

Devtools endpoints live under `/_dx/devtools/*` and must remain dev-only.

## Build Output

Use:

```powershell
dx build
```

Build output goes to:

```text
.dx/www/output
```

Production output should separate:

- Public deployable bytes.
- Evidence/proof receipts.
- Internal generated contracts.

Devtools, hot reload clients, dev endpoints, and hidden production file
abstractions must not leak into deployable output.

Production preview serves only paths proven by `deploy-adapter.json`. Public
assets are recorded under `public/` in the deploy contract and should also be
reachable through root public aliases such as `/logo.svg`, matching dev-server
and web-framework expectations.

## Forge And Package Receipts

Forge package lanes must clearly distinguish source-owned support from live
provider/browser proof.

Forge is Forge-first and no-node_modules by default. Do not run package-manager
installs, create template-local `node_modules`, or add template-local package
dependencies just to make a Forge lane look green. Dependency installation is
app-owned; WWW should record provenance, receipts, generated contracts, and
explicit adapter boundaries.

Never patch scores directly.

If Forge goes yellow because of stale hashes, run the owning helper:

```powershell
node tools\launch\run-template-receipt-helper.js examples/template\<name>-receipt-hashes.ts --write
```

Then verify:

```powershell
dx check examples/template --json
dx www agent-context --json
```

Run helpers sequentially when they touch
`examples/template/.dx/forge/package-status.json`; parallel writes can corrupt
intermediate JSON.

## Check, Readiness, And Receipts

Useful commands:

```powershell
dx check . --json
dx www readiness --json --full
dx www agent-context --json --full
```

Receipt paths:

```text
.dx/receipts/check/
.dx/receipts/readiness/
.dx/serializer/
.dx/imports/
.dx/www/output/
```

Rules:

- If a receipt is stale, regenerate through the owning command.
- Durable DX state should use `.sr` and generated `.machine` contracts.
- Browser, provider, CDN, and hosted benchmark evidence require their own
  receipts beyond source-only checks.

## Plain HTML And Non-WWW Projects

The long-term DX ecosystem goal is that `dx style`, `dx icons`, `dx check`,
Forge, and related tools can help WWW, Next, React, Svelte, Astro, and plain
HTML projects.

Inside WWW itself:

- Keep route authoring under `app/`.
- Use `public/` for static public assets.
- Do not pretend arbitrary `.html` files are full WWW routes unless route
  discovery and build output explicitly support them.

## WebAssembly

WWW recognizes `.wasm` assets and records primitive proof for source-owned Wasm
boundaries.

Current bounded claims:

- `.wasm` and `.wasm.gz` can be immutable runtime assets.
- Precompressed Wasm metadata can carry content encoding and MIME details.
- WebAssembly Bridge package receipts expose app-owned generated-Wasm
  boundaries.

Not yet claimed:

- Automatic Rust-to-Wasm or arbitrary language-to-Wasm app build pipeline.
- Browser execution proof for arbitrary app-owned Wasm modules.
- Hosted provider performance/security proof.

## Verification Defaults

Use focused checks after coherent edits.

Docs-only or agent-file edits:

```powershell
node --test benchmarks\dx-www-current-status-docs.test.ts
git diff --check
```

Framework source edits:

```powershell
node --test <focused-test>.ts
cargo fmt --check
cargo check -p dx-www --no-default-features --features cli --bin dx-www -j 1 --message-format=short
git diff --check
```

Production hardening, warning-clean, and 100/100 claims are gated by strict clippy:

```powershell
cargo clippy -p dx-www --no-default-features --features cli --bin dx-www -j 1 -- -D warnings
```

Do not call a production-hardening lane green until this exact target passes,
or report the blocker with the failed command and current output.

Template/check edits:

```powershell
node --test benchmarks\www-template-score-gate.test.ts benchmarks\www-template-forge-reality.test.ts benchmarks\www-forge-package-status-read-model.test.ts benchmarks\dx-www-agent-context-command.test.ts benchmarks\www-template-source-honesty.test.ts
dx check examples/template --json
dx www agent-context --json
git diff --check
```

Standalone worker command policy:

- Default to read-only inspection and small focused commands.
- After edits, run at most one targeted check that directly covers the touched
  files or lane unless the manager asks for broader proof.
- Use `node --check`, focused `node --test <file>.ts`, scoped
  `git diff --check`, targeted rustfmt, or `cargo check ... -j 1` when Rust was
  touched and compile proof is necessary.
- Do not run `dx build`, `dx dev`, start or stop servers, browser automation,
  package installs, deploys, broad benchmark suites, full workspace tests, full
  `cargo test`, or combined readiness gates unless the manager explicitly
  requests it for the current run or the lane has an active proof lease from
  the coordinator.
- Cargo concurrency defaults to `-j 1`. Never use `-j6`, `-j 6`, or higher
  parallelism unless the manager explicitly requests that exact concurrency for
  the current run.

Use heavier `cargo test`, broad benchmark suites, browser automation, or release
builds only when the manager asks, the lane owns that proof, or an active proof
lease explicitly covers the command.

## Development Rules

- Read current code and receipts before changing architecture.
- Preserve user-facing behavior unless the user asks to change it.
- Do real implementation, not placeholder UI, synthetic receipts, or decorative wrappers.
- Keep files cohesive and maintainable; avoid mega-files and pointless sprawl.
- Use domain capability names, not filler names. Avoid labels such as `ai-slop`,
  `v1`, `temp`, `final`, `final2`, `demo`, or `new` unless the name is part of
  a real public versioned contract. Name files, tests, receipts, and lanes after
  the behavior they prove.
- Use `.ts` for Node scripts, tests, and framework logic, and `.tsx` for UI
  source. Do not add hand-authored `.js`, `.mjs`, or `.cjs` source unless the
  file is an explicitly documented generated/runtime artifact or compatibility
  shim. When touching legacy JavaScript-owned code, prefer migrating the edited
  surface to `.ts`/`.tsx` as part of the same coherent change.
- Do not delete old worktrees, external folders, or generated artifacts without
  ownership confirmation.
- On Windows, avoid destructive shell-built filesystem commands.
- Before claiming completion, report exactly what passed, failed, or was
  skipped.

## Public Communication

When reporting status or writing public docs, include:

- Exact command run.
- Pass/fail result.
- Current score and traffic when relevant.
- Active blockers from `dx www agent-context --json` when relevant.
- Whether evidence is source-only, browser proof, provider proof, build proof,
  installed binary proof, or benchmark proof.

Use professional product language. Prefer headings such as `Verified Results`,
`Performance Leadership`, `Framework Capabilities`, and `Benchmark Governance`.
Avoid casual or internal status labels in public-facing documentation.

Full release validation requires more than source guards. Performance
leadership claims should travel with the benchmark source: controlled local
benchmark, browser benchmark, real-world LCP sample, published benchmark, or
hosted/provider benchmark.

## Tool-Specific Bridges

Tool-specific instruction files in this repo must stay short:

- `CLAUDE.md`
- `GEMINI.md`
- `.cursorrules`
- `.cursor/rules/dx-www.mdc`
- `.github/copilot-instructions.md`

They should tell that tool to read this `AGENTS.md` and `README.md` first, then
summarize only tool-specific behavior. If those files disagree with this file,
this file wins.
