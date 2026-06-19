# Forge Import Security Model

DX Forge import is a review firewall. It can describe, quarantine, slice, and
materialize external package source, but it must not make a package-manager
cache, install tree, or status dashboard the authority for project source.
The currently modeled import-review ecosystems are listed in
`docs/forge-universal-importers.md`: npm, pip, cargo, Go, JSR, pub, Maven,
NuGet, Composer, RubyGems, SwiftPM, Hex, and CRAN. These are non-executing
review and materialization contracts, not universal package compatibility.

## Authority Chain

`.dx/forge/package-status.json` is a read model, not an authority artifact. It
is useful for editors, check panels, dashboards, and launch summaries, but every
claim in that file must be backed by referenced authority artifacts before a
materialization or trust claim can pass.

The authority chain is:

- `.dx/forge/source-manifest.json`: the project source ownership manifest.
- `.dx/forge/package-lock.json`: the generated lock derived from the source
  manifest.
- `.dx/forge/receipts/**`: import, add, update, rollback, remote sync, and
  package receipts.
- `.dx/forge/trust-policy.sr` or its generated `.machine` artifact when a trust
  policy is present.
- `.dx/forge/remote-status.json` and the referenced remote sync-plan receipt
  when remote or registry status is claimed.
- Current materialized files on disk, checked against the recorded hashes.

A package is allowed to show as present only when the status row agrees with the
source manifest, package lock, current receipts, applicable trust policy, remote
status if claimed, and materialized files. If any referenced authority file is
missing, stale, or contradictory, the claim must be downgraded to `stale`,
`missing receipt`, `blocked`, or `manual review`.

## Import Phases

The import lane remains non-executing by default:

1. Plan: describe the requested package, symbols, ecosystem, and next command.
2. Acquire: collect metadata or source into quarantine only when explicitly
   allowed.
3. Quarantine: keep acquired files outside importable app source.
4. Analyze: scan paths, scripts, native code, dynamic execution, license, and
   advisory declarations.
5. Slice: choose reviewed files or adapters.
6. Materialize: write only approved source-owned files.
7. Rewrite: update imports only after an accepted materialization receipt.

Materialized files without an accepted materialization receipt are treated as an
incomplete write boundary. They may exist on disk, but Forge must not call them
`source-owned` until the receipt path is present and coherent with the written
manifest, docs, import plan, serializer `.sr`, and machine cache.

`dx forge import` must not run `npm install`, `pip install`, `cargo add`,
`go get`, `deno add`, `jsr add`, `dart pub get`, `mvn install`,
`dotnet restore`, `composer install`, `bundle install`, `swift build`,
`mix deps.get`, `R CMD INSTALL`, package lifecycle hooks, setup hooks, build
scripts, or native binary loaders during the default review path.

## Hash Roles

BLAKE3 package and file integrity is for source package identity, registry blob
integrity, and cache verification. SHA-256 receipt freshness is for comparing
current project files with package receipts, dashboard receipts, and status read
models. These two hash roles are intentionally separate and should not be
substituted for each other in scoring, receipts, or launch claims.

## License And Advisory Boundaries

License and advisory data are declaration-only unless a receipt explicitly marks
them reviewed. A package can carry declared license text, advisory placeholders,
and upstream metadata without claiming legal approval or security clearance. A
reviewed claim needs the reviewer, source, timestamp, scope, and receipt path.

## Coherence Rules

The coherence gate fails or downgrades a package claim when:

- The package-status row references a missing source manifest, package lock,
  receipt, trust policy, remote status, or sync-plan receipt.
- The source manifest and package lock disagree on package id, version,
  materialized files, or integrity.
- A receipt claims files that are absent or whose SHA-256 freshness check is
  stale.
- A remote status claims readiness without the referenced sync-plan receipt.
- A trust policy allows materialization but the receipt has unresolved red or
  manual-review findings.
- A license or advisory claim is presented as reviewed without a review receipt.

This keeps Forge honest: dashboards can summarize, but authority comes from
matching source, locks, receipts, trust policy, remote proof, and current bytes.
