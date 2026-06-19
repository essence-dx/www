# DX Forge Package: `shadcn/ui/badge`

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

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/badge`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry and Radix Slot primitive for the official UI Components Badge surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn badge launch slice, but it is not a live advisory feed and does not audit application status taxonomy, labels, or tone.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/slot.tsx` | `js/ui/slot.tsx` | `642` | `ba2a601a0e6ff61980dca81e9e81cf87c8be19d05cd09af3adefbbc7658786ff` |
| `components/ui/badge.tsx` | `js/ui/badge.tsx` | `1515` | `a553e933c645c9fbdf1382e65ac0bf06050ad3536e195fa29a10f73434de6786` |
| `components/ui/README.md` | `js/ui/README.md` | `425` | `f45104611f2dadb25f83f6c043fdc9709090e485afe4bfec2f6f914536139db1` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-unchanged` | lib/utils.ts already matches the latest package. |
| `green` | `update-unchanged` | components/ui/slot.tsx already matches the latest package. |
| `green` | `update-update` | components/ui/badge.tsx would be updated from the latest package. |
| `green` | `update-update` | components/ui/README.md would be updated from the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `shadcn/ui/badge` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Unchanged` | `lib/utils.ts` | `Accepted` |
| `green` | `Unchanged` | `components/ui/slot.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/badge.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/README.md` | `Accepted` |
