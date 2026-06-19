# Next.js / Turbopack Rust Vendor Snapshot

This directory contains a quarantined source snapshot from the upstream Next.js
repository for DX-WWW integration work.

Upstream source:

- Repository: `vercel/next.js`
- Local source path at import time: `G:\WWW\inspirations\nextjs`
- Commit: `f3f56ecec2f3f8cefa0f0a1323ea406740251d5c`
- Branch at import time: `canary`
- Imported on: 2026-05-23
- License file: `license.nextjs.md`

Imported Rust groups:

- `crates/next-code-frame`
- `crates/next-custom-transforms`
- `turbopack/crates/turbo-persistence`
- `turbopack/crates/turbo-tasks`
- `turbopack/crates/turbo-tasks-auto-hash-map`
- `turbopack/crates/turbo-tasks-backend`
- `turbopack/crates/turbo-tasks-bytes`
- `turbopack/crates/turbo-tasks-env`
- `turbopack/crates/turbo-tasks-fetch`
- `turbopack/crates/turbo-tasks-fs`
- `turbopack/crates/turbo-tasks-fuzz`
- `turbopack/crates/turbo-tasks-hash`
- `turbopack/crates/turbo-tasks-macros`
- `turbopack/crates/turbo-tasks-macros-tests`
- `turbopack/crates/turbo-tasks-malloc`
- `turbopack/crates/turbo-tasks-testing`
- `turbopack/crates/turbopack-core`
- `turbopack/crates/turbopack-ecmascript`
- `turbopack/crates/turbopack-css`
- `turbopack/crates/turbopack-image`
- `turbopack/crates/turbopack-mdx`
- `turbopack/crates/turbopack-dev-server`
- `turbopack/crates/turbopack-ecmascript-hmr-protocol`
- `turbopack/crates/turbopack-resolve`

DX-WWW integration policy:

- Public CLI, docs, dev overlay, and diagnostics must use DX-WWW branding.
- Upstream provenance and license notices must remain available internally.
- These vendored files are not the trusted DX-WWW runtime core.
- `dx-www-browser-micro`, `dx-www-browser`, `dx-www-packet`,
  `dx-www-binary`, `dx-www-morph`, `dx-serializer`, `dx-style`, and
  `related-crates/style`, and `dx-www-server` remain authoritative.
- Protected DX boundaries include `browser-micro`, `browser`, `packet`, `binary`,
  `morph`, `serializer`, `dx-style / related-crates/style`, the Rust server,
  Forge/source receipts, dx-check, Zed surfaces, and Studio surfaces.
- `next-napi-bindings`, `turbopack-nodejs`, and full `next-core` were not
  imported as core dependencies in this phase.
- `next-core`, `next-napi-bindings`, `turbopack-nodejs`, `React/RSC`,
  `Node/NAPI`, `Turborepo`, `node_modules`, and Node resolver defaults are
  excluded as DX-WWW core foundations.
