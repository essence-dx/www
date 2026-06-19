# DX WWW Framework

Forge-first web framework with React-shaped TSX authoring, visible source files,
Rust-owned tooling, and no `node_modules` by default.

## Overview

DX WWW keeps the developer-facing project small and readable while the compiler owns the heavy runtime work. New projects use an App Router-shaped `app/`, `components/`, `lib/`, `server/`, `styles/`, and `public/` layout, plus Forge-managed source packages instead of a default dependency black box.

Legacy `.html` and `.tsx` files remain supported, and older `dx.config.toml` projects still load as a fallback. The default project contract is now the extensionless root `dx` file in DX Serializer LLM format.

## Deferred Polish Scope

These items are intentionally tracked as polish work, not active score caps for
the current DX-WWW launch score:

- Mobile Speed Index optimization.
- The current Clippy warning backlog.
- Old extra worktrees left by earlier workers.
- Further splitting of the 20,000+ line `dx-www/src/cli/mod.rs`.
- Gradual migration of legacy `.js`, `.cjs`, and `.mjs` benchmark/tooling files
  while new scripts prefer `.ts`.

## Features

- **React-shaped authoring**: Build with familiar components, props, events, and route files.
- **Single setup file**: Biome, DX-Style/Tailwind-compatible, shadcn-style, build, dev, and Forge policy live in `dx`.
- **Forge-owned packages**: Keep package source, receipts, manifests, trust
  policy, import plans, and docs visible under `.dx/forge/`.
- **Clean reviewed imports**: Materialized source snapshots under
  `lib/forge/<ecosystem>/<package>/` can be imported by package name when
  `.dx/forge/source-manifest.json` proves the reviewed files.
- **No default node_modules**: Fresh DX WWW apps run with source-owned files and Forge packages.
- **Adaptive output**: Compile static routes, micro-js interactions, and future Wasm targets.
- **Hot reload**: Development server hot reload is enabled by default.

## Quick Start

```bash
dx new my-app
cd my-app
dx dev
dx build
dx check . --json
dx www readiness --json --full
dx www agent-context --json --full
dx www docs-doctor --json
```

`dx www readiness --json --full` is intentionally strict: local matrices such as `route-handler-conformance-matrix.json`, `server-action-replay-ledger.json`, and `provider-adapter-smoke-matrix.json` are evidence, not hosted-release proof. The server-action replay ledger is hash-only local preview evidence; provider-hosted route-handler replay, distributed server-action replay, and multi-adapter smoke proof stay open until receipts say otherwise.

## Project Structure

```text
my-app/
├── dx                     # LLM-format setup and project contract
├── app/                   # React-shaped routes and layouts
├── components/            # Local and Forge-owned UI source
├── server/                # Actions, loaders, and endpoints
├── styles/                # DX-Style tokens and generated CSS-facing files
├── public/                # Static assets
└── .dx/                   # Generated machine cache plus Forge metadata
```

## Configuration

Create a root `dx` file in DX Serializer LLM format. `dx.config.toml` is still readable for older projects, but new projects use `dx`.

```dx
project(name=dx-www-template version=0.1.0 kind=www-app)

www(
   app_dir=app
   output_dir=.dx/www/output
)

dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)

style(
   mode=generated-css
   tokens=styles/theme.css
   generated_css=styles/generated.css
)

imports(
   map=.dx/imports/import-map.json
   barrel=components/auto-imports.ts
   declarations=.dx/imports/imports.d.ts
   scan_roots=components,composables,utils
   used_roots=app,components,lib,server,styles
   aliases=#imports,#components
   used_only=true
)

icons(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components/icons)

forge(policy=forge-first-no-node-modules)
check(score_scale=500 lighthouse=true)
```

`dx forge import <ecosystem> <package> --plan` scores and records evidence
without running package-manager installs or lifecycle scripts. `--write`
materializes only accepted source snapshots after `--from-plan` validates the
reviewed import plan.

`dx-www serializer dx` writes `.dx/serializer/dx.machine`. The root `dx` file is already the LLM-readable text source, so `.dx/serializer/` does not duplicate a `.llm` copy.

## License

MIT OR Apache-2.0
