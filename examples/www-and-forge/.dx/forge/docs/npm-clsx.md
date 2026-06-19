# DX Forge Package: `npm/clsx`

- Variant: `default`
- Version: `0.0.0-dx-reviewed-adapter`
- Upstream: `npm:clsx`
- Generator: `dx-forge/npm-reviewed-source-slice`
- License: `MIT`
- Provenance: `dx-forge-npm-reviewed-source-slice` (verified: `no`)
- Advisory coverage: `missing` via `none` (live: `no`, findings: `0`)
- License review: declared `MIT` (reviewed: `no`)
- Last action: `TrackWrite`
- Risk score: `95`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update npm/clsx`.

## Package Metadata Review

- Provenance note: Reviewed npm adapter/source slice; Forge did not run npm install or lifecycle scripts.
- Advisory note: Reviewed npm source slices do not have live advisory coverage attached by Forge yet.
- License review note: License is recorded from reviewed npm metadata only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/forge/npm/clsx.ts` | `lib/forge/npm/clsx.ts` | `910` | `7dc7e7b672f4dcf5e1698fa0b0c5baa8e4d3277855a674b9d8e3905a7428516b` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `reviewed-npm-source-slice` | Forge records this npm package as a reviewed source-owned adapter slice, not an installed node_modules dependency. |
| `green` | `no-package-install` | Forge did not run npm, pnpm, yarn, bun, or any package-manager install command. |
| `green` | `no-lifecycle-execution` | Forge did not run npm lifecycle hooks, postinstall scripts, or upstream generated install artifacts. |
