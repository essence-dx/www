# DX Forge Package: `pip/requests`

- Variant: `default`
- Version: `source-dir-snapshot`
- Upstream: `pip:requests`
- Generator: `dx-forge/pip-external-source-snapshot`
- License: `unreviewed`
- Provenance: `dx-forge-pip-external-source-snapshot` (verified: `no`)
- Advisory coverage: `missing` via `none` (live: `no`, findings: `0`)
- License review: declared `unreviewed` (reviewed: `no`)
- Last action: `TrackWrite`
- Risk score: `90`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update pip/requests`.

## Package Metadata Review

- Provenance note: External ecosystem source was materialized into Forge-owned files without running package-manager install or lifecycle commands.
- Advisory note: External source snapshots do not have live advisory coverage attached by Forge yet.
- License review note: License is recorded from inspected package metadata only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/forge/pip/requests/__init__.py` | `lib/forge/pip/requests/__init__.py` | `40` | `c28218c36762c676349c7b49c8e5ef4c3a9db6ac268ae1d55593baf61910cc40` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `external-source-snapshot` | Forge records this package as a source-owned external ecosystem snapshot, not as a package-manager install. |
| `green` | `no-package-install` | Forge did not run npm, pip, cargo, go, or any package-manager install command. |
| `green` | `no-lifecycle-execution` | Forge did not run lifecycle hooks, build scripts, setup hooks, or upstream generated install artifacts. |
