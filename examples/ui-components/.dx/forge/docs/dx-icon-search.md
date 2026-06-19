# DX Forge Package: `dx/icon/search`

- Variant: `default`
- Version: `0.1.0`
- Upstream: `@dx/forge-icons`
- Generator: `dx-forge/selected-icons`
- License: `MIT OR Apache-2.0`
- Provenance: `dx-forge-curated-registry` (verified: `no`)
- Advisory coverage: `curated-fixture` via `dx-forge-curated-advisory-fixture` (live: `no`, findings: `0`)
- License review: declared `MIT OR Apache-2.0` (reviewed: `no`)
- Last action: `UpdateWrite`
- Risk score: `100`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update dx/icon/search`.

## Package Metadata Review

- Provenance note: Curated DX Forge source metadata is recorded, but this is not SLSA or live upstream provenance yet.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this selected icon package, but it is not a live advisory feed.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/icons.ts` | `js/lib/icons.ts` | `824` | `d34ad7db752f79ff5c928858b99f048dca4c7222809fb64fa6f071e7a4d46a3e` |
| `components/icons/search.tsx` | `js/icons/search.tsx` | `496` | `fcca597c047e13aae8bfcb56867d78303704a9bcfcd1332b60650e0f82e0b7a7` |
| `components/icons/README.md` | `js/icons/README.md` | `196` | `a20e985338e0fa6ae5685a33fc1eb4ceb6b428d25026eb111cbc5120922ab4af` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-unchanged` | lib/icons.ts already matches the latest package. |
| `green` | `update-unchanged` | components/icons/search.tsx already matches the latest package. |
| `green` | `update-unchanged` | components/icons/README.md already matches the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `dx/icon/search` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Unchanged` | `lib/icons.ts` | `Accepted` |
| `green` | `Unchanged` | `components/icons/search.tsx` | `Accepted` |
| `green` | `Unchanged` | `components/icons/README.md` | `Accepted` |
