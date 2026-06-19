# Gemini Instructions For DX WWW

Read `AGENTS.md` first. It is the canonical agent contract for this repository.
Read `README.md` second for the public framework overview and benchmark results.

Gemini-specific rules:

- Keep responses and patches grounded in local source evidence.
- Do not overclaim browser, provider, CDN, or global benchmark proof.
- Current same-machine fair-counter proof should rank WWW first on bytes and
  median RPS when fresh receipts confirm it. Hosted/provider benchmark
  publication is tracked separately.
- Production preview must be driven by `deploy-adapter.json`: the default `/`
  starter route should resolve there, devtools must be absent, and public assets
  such as `/logo.svg` must work. State-runtime and island proof routes are
  materializable fixtures, not default starter pages.
- Keep WWW route authoring centered on `app/`; do not revive `pages/` or
  `.dxob` language as public framework guidance.
- Keep official WWW starters free of template-local `node_modules`; package
  provenance belongs in Forge/source-owned lanes.
- Forge is Forge-first and no-node_modules by default. Do not run
  package-manager installs, create template-local `node_modules`, or add
  template-local package dependencies for Forge/package lanes; dependency
  installation is app-owned.
- Forge import is a materialization/scoring gate, not a fake package manager.
  Accepted snapshots under `lib/forge/<ecosystem>/<package>/` may resolve from
  clean package-name imports only when `.dx/forge/source-manifest.json` tracks
  the reviewed source files.
- Treat `.dx/*` as generated evidence. Fix source/generators before generated
  outputs.
- Use the current WWW state model: `state`, `derived`, `effect`, `action`, and
  `store` are framework-owned primitives, with global store modules usually in
  `lib/stores/`.
- `State Management` is the professional Forge package lane name for upstream
  Zustand provenance. Keep that separate from WWW-native `store()` global
  stores.
- Keep React-style event authoring precise: quoted `onClick="class names"`
  applies interaction classes; braced `onClick={() => ...}` lowers logic only
  when the compiler can prove it.
- Describe motion as DX Style-backed `motion` / `dxMotion` metadata and named
  animation generation, not a hidden React animation runtime.
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
- Prefer focused checks and `git diff --check` for docs-only changes. Use
  `dx check <project> --json` for project-health proof after coherent WWW
  edits.

If this file and `AGENTS.md` disagree, follow `AGENTS.md`.
