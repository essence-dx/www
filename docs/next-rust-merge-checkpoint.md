# DX-WWW Next Rust Merge Checkpoint

Date: 2026-05-23

This checkpoint freezes the current DX-WWW workspace before importing selected
Next.js and Turbopack Rust internals.

Current branch: `codex/unified-dx-root-cli`

Upstream state before checkpoint:

- Ahead of `origin/codex/unified-dx-root-cli` by 76 commits.
- Behind `origin/codex/unified-dx-root-cli` by 0 commits.
- Pending worktree entries before checkpoint: 939.
- Modified entries: 288.
- Deleted entries: 162.
- Untracked entries: 489.

Purpose:

- Preserve the current DX-WWW runtime, Forge, source-engine, and template work.
- Create a professional rollback point before vendoring Next.js/Turbopack Rust
  build internals.
- Mark the next branch as an integration branch where build or workspace checks
  may temporarily break while the copied code is quarantined, wrapped, and
  rebranded for DX-WWW.

Runtime guard:

- Public template/config surfaces describe the runtime through professional
  layers: `dx-www`, `dx-www-html`, `dx-www-js`, `dx-www-wasm`,
  `dx-www-browser`, `dx-www-protocol`, and `dx-www-server`.
- Protected implementation crates still remain authoritative where exact crate
  identity matters: `dx-www-browser-micro`, `dx-www-browser`, `dx-www-packet`,
  `dx-www-binary`, `dx-www-morph`, `dx-serializer`, `dx-style`,
  `related-crates/style`, and `dx-www-server`.
- Forge/source receipts, dx-check, Zed surfaces, and Studio surfaces remain
  DX-owned source evidence surfaces.
- React, RSC, Node, NAPI, Turborepo, `node_modules`, `next-core`,
  `next-napi-bindings`, `turbopack-nodejs`, and Node resolver defaults are not
  promoted to the trusted core runtime by this checkpoint.
