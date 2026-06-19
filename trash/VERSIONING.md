
# Versioning Guide

This document describes the versioning strategy for dx-www and when to bump version numbers.

## Forge Launch Status Matrix - 2026-05-22

| Surface | Launch readiness | Evidence | Boundary |
| --- | --- | --- | --- |
| Package manager | Partial but real | `examples/template/.dx/forge/package-lock.json`, `.dx/forge/package-status.json`, package-add receipts, cache manifests, rollback anchors, safety archive receipts, and generated `forge-package-status-read-model.ts` cover `shadcn/ui/button`, `state/zustand`, and `tanstack/query` as source-owned slices. | No public package registry, dependency solver, or npm replacement claim yet. |
| VCS | Partial but real | `flow/crates/forge` exposes init/add/commit/status/log/diff/checkout paths over content-addressed storage, and the launch template now emits `.dx/forge/vcs-status.json` plus `.dx/forge/receipts/vcs/launch-template-snapshot.json` for a 66-file code/media/package inventory. | The launch-template snapshot is a deterministic status receipt, not a full Forge commit graph, branch UX, or remote collaboration proof. |
| Multi-remote | Partial but receipt-backed | Package status receipts define five remotes, and the launch template now emits `.dx/forge/remote-status.json` plus `.dx/forge/receipts/remotes/launch-template-sync-plan.json` for local filesystem, Git-compatible, S3-compatible, database-backed, and custom Forge adapter providers. | Only the local filesystem provider is launch-executable in this source-only proof; Git/S3/database/custom adapters are configured boundaries and network sync is not claimed. |
| Media versioning | Proof path | The launch template tracks `runtime-assets/favicon.svg` through `.dx/forge/media-status.json`, `.dx/forge/media-cache/.../favicon.svg`, and `.dx/forge/receipts/media/launch-template-favicon-restore.json` with hash, chunk map, preview receipt, dedupe key, cache copy, and restore plan; Forge crate has structure-aware chunker modules for MP4, EXR, UAsset, and CSP. | One SVG fixture does not prove broad Git LFS replacement yet. |
| DX-WWW / dx-check / Zed receipts | Real read surface | `forge-package-status-read-model.ts` turns `.dx/forge/package-status.json` into typed package, remote sync, media restore/cache, VCS snapshot, safety/archive, dx-check, and Zed receipt surfaces consumed by `forge-package-status.ts`; `dx check` source now reads package lock, media, media-status, remote definition, remote-status, cache, rollback, and VCS snapshot metrics. | Zed GPUI is still future UI consumption, not a native panel implementation; archive receipts are local cache/receipt restore inputs, not remote rollback execution. |

## Semantic Versioning

dx-www follows Semantic Versioning 2.0.0:
```
MAJOR.MINOR.PATCH ```


## When to Bump Versions



### MAJOR Version (Breaking Changes)


Increment the MAJOR version when you make incompatible API changes: -Removing public types, functions, or modules -Changing function signatures (parameters, return types) -Changing struct field types or removing fields -Changing enum variants -Changing trait definitions -Changing the HTIP binary format in incompatible ways -Changing the delta patch format in incompatible ways -Removing or renaming feature flags Examples: -`parse()` now returns `Result<Ast, Error>` instead of `Option<Ast>` -Removing `TreeShaker::shake_module()` in favor of `TreeShaker::shake()` -Changing `DeltaPatch` struct layout


### MINOR Version (New Features)


Increment the MINOR version when you add functionality in a backward-compatible manner: -Adding new public types, functions, or modules -Adding new optional struct fields (with defaults) -Adding new enum variants (if non-exhaustive) -Adding new trait methods with default implementations -Adding new feature flags -Adding new CLI commands or options -Performance improvements Examples: -Adding `Parser::parse_with_options()` -Adding `TreeShakeStats::modules_affected` field -Adding `io_uring` feature flag


### PATCH Version (Bug Fixes)


Increment the PATCH version when you make backward-compatible bug fixes: -Fixing incorrect behavior -Fixing panics or crashes -Fixing memory leaks -Fixing security vulnerabilities -Documentation fixes -Internal refactoring (no API changes) Examples: -Fixing parser crash on malformed input -Fixing delta patch corruption on large files -Fixing memory leak in reactor


## Version Bump Process



### 1. Determine Version Change


Review all changes since the last release:
```bash
git log v0.1.0..HEAD --oneline ```
Categorize changes as MAJOR, MINOR, or PATCH.

### 2. Update Cargo.toml

Update the workspace version in the root `Cargo.toml`:
```toml
[workspace.package]
version = "0.2.0" # Updated version ```


### 3. Update CHANGELOG.md


Move items from `[Unreleased]` to a new version section:
```markdown

## [0.2.0] - 2026-02-15

### Added

- New feature X

### Fixed

- Bug Y
```


### 4. Create Git Tag


```bash
git add -A git commit -m "Release v0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main --tags ```

### 5. Publish to crates.io

The release workflow will automatically publish when a version tag is pushed.

## Pre-release Versions

For pre-release versions, use suffixes: -Alpha: `0.2.0-alpha.1` -Beta: `0.2.0-beta.1` -Release Candidate: `0.2.0-rc.1`

## Workspace Versioning

All crates in the workspace share the same version number. This simplifies dependency management and ensures compatibility. Individual crates inherit the version from `workspace.package`:
```toml
[package]
name = "dx-core"
version.workspace = true ```


## Breaking Change Policy


Before making breaking changes: -Deprecate the old API in a MINOR release -Document the migration path -Remove the deprecated API in the next MAJOR release This gives users time to migrate their code.
