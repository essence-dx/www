# DX Forge Launch Limitations

DX Forge is a source-owned package firewall. Its first public claim is narrow on purpose: it reduces install-time supply-chain blast radius by materializing reviewed source files, recording receipts, scoring risk, and refusing blind updates.

Forge defines reviewed import-planning surfaces for the modeled ecosystems in
`docs/forge-universal-importers.md`: npm, pip, cargo, Go, JSR, pub, Maven,
NuGet, Composer, RubyGems, SwiftPM, Hex, and CRAN. It is not universal package
compatibility yet, and it is not a package-manager replacement.

## What Forge Does Today

- Materializes curated source-owned packages such as `ui/button`, `ui/input`, `ui/textarea`, and `dx/icon/search` into editable project files.
- Writes `.dx/forge/source-manifest.json` with package id, version, generator, variant, license, integrity hash, and tracked file hashes.
- Writes reviewable receipts under `.dx/forge/receipts`.
- Writes package-facing docs under `.dx/forge/docs` explaining which files are owned, editable, and updated by Forge.
- Blocks package-manager lifecycle execution during Forge add/update flows.
- Classifies source-owned file traffic as green, yellow, or red.
- Prevents blind overwrites of local edits.
- Provides `dx check --strict-forge` for Forge release checks over stale receipts, missing rollback coverage, and red package traffic.
- Verifies local and R2 registry manifests against BLAKE3 package/file hashes before trusting hydrated package content.
- Treats package-status as a read model and never as authority; source manifests, package locks, receipts, trust policy, remote status, and current file hashes remain the authority chain.

## What Forge Does Not Replace Yet

- It does not ingest arbitrary packages from npm, pip, cargo, Go, JSR, pub,
  Maven, NuGet, Composer, RubyGems, SwiftPM, Hex, CRAN, or every other
  ecosystem.
- It does not let package-status alone prove materialization, lock, receipt, remote, license, advisory, or trust-policy claims.
- It does not replace package managers for existing React, Next.js, Svelte, Astro, or Node.js applications.
- It does not solve every transitive dependency risk in an app that still uses `node_modules`.
- It does not audit every browser API, SDK, auth client, payment SDK, editor, map, chart, or animation library.
- It does not provide a full multi-language registry for Rust, Python, Go, Dart,
  Java/Kotlin, .NET, PHP, Ruby, Swift, or other ecosystems yet.
- It does not replace cloud infrastructure, CI, secrets management, SBOM tooling, or runtime monitoring.
- It does not guarantee that source-owned code is safe after a developer edits it.

## Honest Launch Position

The initial Forge launch should be presented as:

> DX Forge reduces install-time supply-chain blast radius through source-owned packages, package scoring, receipts, rollback, strict Forge release checks, and optional registry publishing.

Do not present the initial Forge launch as:

- Universal package compatibility or a complete package-manager replacement.
- A guarantee that no supply-chain attack can reach a project.
- A drop-in migration path for every enterprise React/Next dependency graph.
- A reason to stop using lockfiles, provenance checks, secret scanning, CI hardening, or dependency review.

## Why This Still Matters

The current web ecosystem often hides critical application code behind opaque dependency folders and install-time execution. Forge attacks that specific failure mode by moving selected packages into visible, editable source, then making changes reviewable and reversible.

That is a real wedge. It is strongest first for curated UI, selected icons, template primitives, and small high-trust packages where source ownership is more valuable than dependency indirection.

## Expansion Bar

Forge should only expand beyond curated JS and UI compatibility packages when each new package path can prove:

- No install scripts or upstream lifecycle hooks are executed.
- Materialized files are deterministic and hash-addressed.
- Local edits are detected and never overwritten blindly.
- Updates produce reviewable green/yellow/red change sets.
- Rollback receipts exist for accepted updates.
- Registry manifests and blobs verify before materialization.
- Status rows reference existing source manifests, package locks, receipts, trust policy, and remote proof before they are presented as green.
- Public docs state the remaining limitations honestly.
