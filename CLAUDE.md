# Claude Instructions For DX WWW

Read `AGENTS.md` first. It is the canonical agent contract for this repository.
Read `README.md` second for the public framework overview and benchmark results.

Claude-specific rules:

- Do not create a separate Claude-only framework plan that conflicts with
  `AGENTS.md`.
- Keep claims evidence-backed. If a receipt says source-only, report it as
  source-only.
- Prefer focused source scans and targeted tests before broad builds.
- Use `dx check <project> --json` as the stable project-health surface after
  coherent WWW edits, and report its score/traffic without hand-patching
  receipts.
- Use `.ts` for Node tests, scripts, and framework logic, and `.tsx` for UI
  source. Do not add hand-authored `.js`, `.mjs`, or `.cjs` source unless the
  file is an explicitly documented generated/runtime artifact or compatibility
  shim.
- Preserve the source-owned WWW identity: React/Next-familiar authoring without
  hidden React DOM, Next runtime, Vite, Webpack, or template-local
  `node_modules`.
- Forge is Forge-first and no-node_modules by default. Do not run
  package-manager installs, create template-local `node_modules`, or add
  template-local package dependencies for Forge/package lanes; dependency
  installation is app-owned.
- Forge import is a materialization/scoring gate, not a fake package manager.
  Accepted snapshots under `lib/forge/<ecosystem>/<package>/` may resolve from
  clean package-name imports only when `.dx/forge/source-manifest.json` tracks
  the reviewed source files.
- Current local proof shape: WWW is expected to lead the fair-counter
  controlled local benchmark on both total bytes and median RPS. Hosted/provider
  benchmark publication is tracked separately.
- Production preview must serve all built App Router routes through
  `deploy-adapter.json`, keep devtools/hot reload out of HTML, and support
  public assets through root aliases such as `/logo.svg`.
- For `next/image` and `next/font`, describe the current support as
  static-safe/source-owned foundations, not full Next optimizer or font-service
  parity.
- Treat global stores as framework-owned WWW behavior: `lib/stores/*.ts` can
  export `store({ state, derived, effect, action })` modules that route units
  link across source files.
- When discussing Zustand, use the professional package lane name `State
  Management` and treat upstream `zustand` as provenance. Do not confuse that
  package lane with WWW-native `store()` global stores.
- React-style native events are the public event syntax. Quoted values such as
  `onClick="scale-up bg-accent"` are interaction-class commands; braced values
  such as `onClick={() => ...}` are logic that must lower safely or produce a
  diagnostic.
- Motion is DX Style-backed through `motion` / `dxMotion` metadata and named
  animation syntax, not a hidden Framer Motion runtime unless an adapter
  boundary is explicit.
- DX Style owns Tailwind-like `className` authoring, event-class strings,
  motion class strings, grouping syntax such as `hover:(bg-accent
  text-accent-foreground)`, and the atomic/custom CSS balance. Do not patch
  generated CSS by hand.
- Use the source-owned DX Icon path for first-party WWW icons: author
  `<Icon name="pack:check" />` and let the `icons(...)` config own generated
  files and runtime tags instead of importing npm icon packages.
- Public docs should use launch-grade wording such as `Verified Results`,
  `Performance Leadership`, `Framework Capabilities`, and `Benchmark
  Governance`. Do not reintroduce casual or internal status headings.
- Use domain capability names, not filler names. Avoid labels such as `ai-slop`,
  `v1`, `temp`, `final`, `final2`, `demo`, or `new` unless part of a real
  public versioned contract.

If this file and `AGENTS.md` disagree, follow `AGENTS.md`.
