# DX Forge Package: `shadcn/ui/button`

- Variant: `default`
- Version: `0.1.0`
- Upstream: `shadcn-ui`
- Generator: `dx-forge/shadcn-ui`
- License: `MIT OR Apache-2.0`
- Provenance: `dx-forge-curated-registry` (verified: `no`)
- Advisory coverage: `curated-fixture` via `dx-forge-curated-advisory-fixture` (live: `no`, findings: `0`)
- License review: declared `MIT OR Apache-2.0` (reviewed: `no`)
- Last action: `AddWrite`
- Risk score: `100`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/button`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry and Radix Slot primitive for the official UI Components Button surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn-style button package, but it is not a live advisory feed.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/slot.tsx` | `js/ui/slot.tsx` | `642` | `ba2a601a0e6ff61980dca81e9e81cf87c8be19d05cd09af3adefbbc7658786ff` |
| `components/ui/button.tsx` | `js/ui/button.tsx` | `1976` | `dd3775a5f32b9a2e3f05cd9955042b68686b514ef88c814c64e47f79200f9afe` |
| `components/ui/README.md` | `js/ui/README.md` | `183` | `6a5633e2163aff242dff4dfbe3f2feb8dd906c4ac848464662de7e22a13e02ba` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `no-lifecycle-execution` | Forge add does not run npm install, lifecycle hooks, or upstream scripts. |
| `green` | `package-provenance-recorded` | Forge records package provenance source `dx-forge-curated-registry` but does not claim external provenance verification yet. |
| `green` | `package-advisory-boundary` | Forge records advisory coverage `curated-fixture` from provider `dx-forge-curated-advisory-fixture` with live coverage `no` and finding count `0`. |
| `green` | `package-license-review-boundary` | Forge records declared license `MIT OR Apache-2.0` with formal review `no`. |
