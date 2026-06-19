# DX Forge Package: `shadcn/ui/field`

- Variant: `default`
- Version: `0.1.0`
- Upstream: `shadcn-ui`
- Generator: `dx-forge/shadcn-ui`
- License: `MIT OR Apache-2.0`
- Provenance: `dx-forge-curated-registry` (verified: `no`)
- Advisory coverage: `curated-fixture` via `dx-forge-curated-advisory-fixture` (live: `no`, findings: `0`)
- License review: declared `MIT OR Apache-2.0` (reviewed: `no`)
- Last action: `UpdateWrite`
- Risk score: `100`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/field`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry plus Radix Label and Separator primitives for the official UI Components Field surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn field launch slice, but it is not a live advisory feed and does not audit application form layout, validation copy, or error announcement policy.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/label.tsx` | `js/ui/label.tsx` | `458` | `159b0a0f2182491257a052818159734b53add410a74eed7a0c8f990da0ff9ac8` |
| `components/ui/separator.tsx` | `js/ui/separator.tsx` | `809` | `fce33cd494a500bdbbbd36398c0be4623f3c84900b41728c7f6c3a6ac9f8e849` |
| `components/ui/field.tsx` | `js/ui/field.tsx` | `5370` | `0221cc93b81801c177790c136b01fb25e459b0356075e8170f6bb35603e9d584` |
| `components/ui/README.md` | `js/ui/README.md` | `686` | `c806994969ae2271ae8f4c8f1ada11df6b8a92028249034ad10ce38f010412f4` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-unchanged` | lib/utils.ts already matches the latest package. |
| `green` | `update-unchanged` | components/ui/label.tsx already matches the latest package. |
| `green` | `update-unchanged` | components/ui/separator.tsx already matches the latest package. |
| `green` | `update-update` | components/ui/field.tsx would be updated from the latest package. |
| `green` | `update-update` | components/ui/README.md would be updated from the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `shadcn/ui/field` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Unchanged` | `lib/utils.ts` | `Accepted` |
| `green` | `Unchanged` | `components/ui/label.tsx` | `Accepted` |
| `green` | `Unchanged` | `components/ui/separator.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/field.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/README.md` | `Accepted` |
