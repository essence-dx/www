# GitHub Copilot Instructions For DX WWW

Read `AGENTS.md` first. It is the canonical agent contract for this repository.
Read `README.md` second for the public framework overview and benchmark results.

Copilot-specific rules:

- Keep route authoring centered on `app/`.
- Do not revive `pages/`, `.dxob`, or old launch-runtime language as current
  WWW framework guidance.
- Keep WWW source-owned: React/Next-familiar authoring without hidden React DOM,
  Next runtime, Vite, Webpack, or template-local `node_modules`.
- Forge is Forge-first and no-node_modules by default. Do not run
  package-manager installs, create template-local `node_modules`, or add
  template-local package dependencies for Forge/package lanes; dependency
  installation is app-owned.
- Forge import is a materialization/scoring gate, not a fake package manager.
  Accepted snapshots under `lib/forge/<ecosystem>/<package>/` may resolve from
  clean package-name imports only when `.dx/forge/source-manifest.json` tracks
  the reviewed source files.
- Keep performance claims tied to evidence. The current fair-counter target is
  WWW first on local bytes and median RPS when fresh benchmark receipts prove
  it; hosted/provider benchmark publication is tracked separately.
- Production preview follows `deploy-adapter.json`: built routes must resolve,
  devtools/hot reload must be absent, and public assets should work from root
  aliases such as `/logo.svg`.
- Use the framework-owned state contract: `state`, `derived`, `effect`,
  `action`, and `store`, with app-wide stores under `lib/stores/`.
- `State Management` is the professional Forge package lane name for upstream
  Zustand provenance. Keep that separate from WWW-native `store()` global
  stores.
- React-style event attributes are first-class. Quoted values are
  interaction-class commands; braced values are logic that must lower safely or
  diagnose.
- Motion is DX Style-backed through `motion` / `dxMotion` metadata and named
  animation generation. Do not assume a hidden Framer/React runtime.
- DX Style owns Tailwind-like `className`, event-class strings, motion class
  strings, grouped tokens such as `hover:(bg-accent text-accent-foreground)`,
  and the atomic/custom CSS balance. Do not hand-edit generated CSS.
- Use first-party DX Icon for WWW icons: author `<Icon name="pack:check" />`
  through the `icons(...)` config and run `dx icons sync/check`; do not import
  npm icon packages in official starters.
- Public docs should use launch-grade wording such as `Verified Results`,
  `Performance Leadership`, `Framework Capabilities`, and `Benchmark
  Governance`. Do not reintroduce casual or internal status headings.
- Use domain capability names, not filler names. Avoid labels such as `ai-slop`,
  `v1`, `temp`, `final`, `final2`, `demo`, or `new` unless part of a real
  public versioned contract.
- Treat `.dx/*` as generated evidence and receipts. Do not hand-patch scores or
  proof claims.
- Use `dx check <project> --json` as the stable project-health proof after
  coherent WWW edits.
- Prefer focused TypeScript tests, `cargo fmt --check`, and `git diff --check`
  before broad builds.

If this file and `AGENTS.md` disagree, follow `AGENTS.md`.
