# www Developer Contract

www should feel familiar to React and Next.js developers without inheriting the opaque `node_modules` default. The public developer model is React-shaped, source-owned, forge-governed, and compiler-owned behind the scenes.

## Default Project Shape

Use this layout for strict www apps:

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
.dx/forge/
```

- `app/` owns routes, layouts, and route-local UI.
- `components/` owns visible reusable UI source.
- `composables/` and `utils/` participate in framework-owned auto-imports.
- `lib/` owns app library code; `lib/stores/` owns framework-level global
  stores.
- `server/` owns loaders, actions, endpoint helpers, and server-only code.
- `styles/` owns DX-Style tokens, generated CSS-facing files, and editable style contracts.
- `public/` owns public assets served from root aliases.
- `dx` is the extensionless project config.
- `.dx/` owns generated output, receipts, serializer `.sr` artifacts, and
  `.machine` contracts.
- `.dx/forge/` owns package manifests, receipts, docs, provenance, and rollback evidence.
- `forge/` may be used for project-facing package docs or review artifacts when a team wants them outside `.dx/`.

## New Project Template

`dx new <name>` creates the strict App Router-shaped starter by default:

- `app/layout.tsx` and `app/page.tsx` as the visible route shell.
- `components/`, `lib/`, `server/`, `styles/`, and `public/` as project-facing source roots.
- `styles/theme.css`, `styles/generated.css`, and `styles/globals.css` for DX-Style-owned tokens, generated CSS, and the app stylesheet entry.
- The root `dx` file owns dev, build, imports, icons, style, Forge, and check contracts; the starter does not require `tailwind.config.*`, PostCSS config files, `npm install`, or `node_modules`.
- `public/logo.svg`, `public/icon.svg`, `public/favicon.svg`, `.dx/forge/template-manifest.json`, `.dx/forge/source-manifest.json`, package docs, and receipts.

Route handlers such as `app/api/health/route.ts`, server actions under `server/`, richer component primitives, and adapter-boundary islands are framework capabilities, but docs should not claim that every minimal starter emits every optional file.

Route-handler conformance receipts are local deploy-contract evidence today. `route-handler-conformance-matrix.json` may prove discovered GET/HEAD/OPTIONS/405 expectations for a build, but provider-hosted route-handler replay remains a release-readiness gap until the same contract passes against real adapters.

A local server-action replay ledger is hash-only local evidence for request/action replay behavior. It is not a distributed replay store, provider-backed persistence layer, hosted adapter proof, or release-readiness substitute until provider receipts explicitly prove that path.

The starter runs with `dx dev` and does not require `npm install` or `node_modules`. The dev path now compiles the starter through the React-shaped App Route compiler slice, producing a canonical `DxPageGraph`, crawlable fallback HTML, JS interaction output for the supported state/button subset, and a round-tripped `DXPK` packet.

`dx build` emits production artifacts for compiled `app/` routes:

- `.dx/www/output/app/index.html`
- `.dx/www/output/app/index.dxpk`
- `.dx/www/output/app/page-graph.json`
- `.dx/www/output/server-contracts.json`
- `.dx/www/output/import-resolution.json`
- `.dx/www/output/manifest.json` with `app_routes_compiled` and `node_modules_required: false`

This is still a supported subset, not full Next.js runtime parity. The next milestone is broader TSX/App Router semantics and reviewed npm materialization.

## Authoring Model

www accepts React-shaped source first: TSX-style components, props, events,
imports, and file organization that existing React developers recognize.
DX-native files such as `.html`, `.tsx`, and `.lyt` remain supported where they
are the smaller or clearer source format.

The compiler remains responsible for choosing the smallest correct delivery mode: static HTML/CSS, generated JS, wasm core, split wasm, or server fragments.

## Stable Public API

The public API stability policy lives in [`docs/api/versioning.md`](api/versioning.md).
The stable developer-facing surface is:

- Commands: `dx new`, `dx dev`, `dx build`, `dx check`,
  `dx www agent-context`, `dx style`, `dx icons`, `dx imports`, and
  `dx env`.
- Project folders: `app/`, `components/`, `composables/`, `utils/`, `lib/`,
  `lib/stores/`, `server/`, `styles/`, `public/`, `.dx/`.
- Config: the extensionless root `dx` file.
- TSX route files: `app/**/page.tsx`, `app/**/layout.tsx`, and
  `app/api/**/route.ts`.
- TSX syntax: `className`, props, intrinsic elements, React-style DOM events
  such as `onClick`, quoted event-class commands such as
  `onClick="scale-up bg-accent"`, braced event logic that lowers only when the
  compiler can prove it, and camelCase island directives such as `clientLoad`.
- Runtime state: DX-native `state`, `derived`, `effect`, `action`, and global
  stores under `lib/stores/`.
- Styling: DX Style-owned `className`, motion/event class strings, grouped
  tokens such as `hover:(bg-accent text-accent-foreground)`, generated CSS, and
  authored custom CSS. Do not hand-edit generated CSS.
- Icons: first-party DX Icon authoring through `<Icon name="pack:check" />`
  and the root `icons(...)` config, not starter-local npm icon packages.
- Proof: `.dx/receipts/**`, `.sr` serializer receipts, `.machine` contracts,
  `.dx/www/output`, and `dx www agent-context --json`.

React hook-like APIs are not a separate runtime surface. `useState`,
`useEffect`, `useMemo`, `useRef`, and similar compatibility syntax may exist
only when the compiler can lower the pattern exactly to DX-native semantics.
Unsupported patterns must fail with precise diagnostics. Silent no-op hooks are
not allowed.

Full React, Next.js, Svelte, or other runtimes belong behind explicit adapter or
island boundaries with documented payload and receipt behavior.

## Allowed Source Files

Strict apps should keep hand-authored and forge-owned source in visible project files:

- `app/**/*.{tsx,ts,html,lyt}`
- `components/**/*.{tsx,ts}`
- `composables/**/*.ts`
- `utils/**/*.ts`
- `lib/**/*.ts`
- `server/**/*.{ts,rs}`
- `styles/**/*.{css,dx.css,tokens.css}`
- `.dx/forge/**/*.json`, `.dx/forge/**/*.md`, and receipt/provenance artifacts written by forge.

Generated caches, opaque dependency folders, and install artifacts are not part of the strict contract.

JavaScript source can be part of migration or adapter projects, but new strict
WWW starters and framework-authored scripts should prefer `.ts` and `.tsx`.

## Import Resolution

www import resolution should stay predictable:

- Relative imports resolve to visible local files first.
- `app/`, `components/`, `server/`, and `styles/` remain the recommended project-facing roots.
- forge-owned package imports resolve only to files recorded in `.dx/forge/source-manifest.json`.
- React-shaped compiler intrinsics, such as the familiar component/event model, are compiler responsibilities in strict mode.
- Bare npm package specifiers are not the strict default unless they have an explicit forge import plan or a reviewed adapter.

## forge Boundaries

forge owns package boundaries through manifests, receipts, docs, provenance, rollback references, and update traffic. A file can be local-editable and still forge-owned, but forge must be able to prove:

- which package wrote it,
- which version or variant it came from,
- which files are editable source,
- which updates are green/yellow/red,
- and how rollback or manual review should work.

Local components under `components/local/` remain normal project source. Curated or imported package files should live under visible target folders such as `components/ui/`, `lib/`, `styles/`, `server/`, or package-specific folders, with their ownership recorded by forge.

## Package Rule

Strict www apps do not rely on `node_modules` by default.

forge materializes reviewed source-owned packages into visible project files. External package compatibility should go through an explicit forge import gate so package code can be inspected, scored, approved, and tracked before it becomes part of the app.

Normal package-manager compatibility can exist for non-strict projects and migration work, but it is not the default www contract.

## External Package Import Bridge

Use the report-only import gate before materializing external package source:

```powershell
dx forge import npm react --plan
dx forge import cargo serde --plan --format markdown
dx forge import pub vector_math --plan
```

The planning command must not install packages, create `node_modules`, or execute lifecycle scripts. It reports the review needed for provenance, exports, dependency graph, license/advisory state, runtime entrypoints, and materialization scope.

Future materialization should write reviewed source-owned files or adapters intentionally. IDE and dev-server hints may suggest missing packages, but the planning surface should not silently auto-write files on save.

## Checks

Use project-contract checks while building strict www apps:

```powershell
dx check . --project-contract
dx check . --strict-project-contract
dx check . --project-contract --hints-output .dx/forge/hints/project-contract-hints.json
```

- `--project-contract` adds a report section for the developer contract.
- `--strict-project-contract` fails on red contract findings such as a `node_modules` folder in a strict app.
- The contract report counts React-shaped source, DX-native source, forge-owned files, editable local components, unmanaged vendor files, and forge metadata.
- `--hints-output` writes an IDE/dev-server/LSP-friendly JSON hint artifact, but it does not write source files or enable save-time package generation.

This is not universal package compatibility. It is the default www app contract for source-owned, visible, reviewable web projects.
