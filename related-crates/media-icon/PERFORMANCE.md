# Media-Icon Evidence Boundary

This document tracks the current evidence boundary for `dx-icons`. It is not an
upstream comparison and it is not a global performance claim.

## Current Scope

- JSON icon packs remain the source of truth.
- Generated `.machine` files are local advisory caches.
- Cache readers validate schema, source fingerprints, bytecheck/rkyv payloads,
  and shape-specific invariants before adoption.
- Bad, stale, missing, or unsupported caches fall back to the current JSON or
  rebuilt in-memory path.

## Wave 5z Evidence Boundary

Wave 5z is receipt-backed performance instrumentation, not a global benchmark
claim and not an upstream comparison.

The tracked media-icon lane now has source contracts and receipt writers for:

- `media-icon-engine-startup.json`: local startup/cache-adoption timing when the
  startup receipt path is explicitly used. Normal `load_fast` behavior is not
  changed by the receipt writer.
- `media-icon-body-resolution.json`: one-icon pack-body machine resolution,
  JSON fallback resolution, and SVG string render timing for one pack/name.
- `media-icon-query-latency.json`: bounded warm-cache query samples plus
  top-result body resolution and SVG string render timing. It does not write
  exported files.
- `media-icon-existing-cache-readiness.json`: before/after readiness for the
  seven local `.machine` cache families when the index build path explicitly
  ensures them.

The useful technical proof is local and structural:

- JSON icon packs remain authoritative.
- Generated `.machine` files are advisory local caches.
- Runtime readers validate schema, source fingerprints, bytecheck/rkyv payloads,
  machine shape, and runtime metadata before adoption.
- The engine reuses one runtime source fingerprint across machine readers during
  startup.
- `pack-body.machine` is adopted for body resolution only; engine search does
  not consume pack-body bodies in this wave.
- Bad, stale, missing, unreadable, or unsupported caches fall back to the
  current JSON/raw-index or rebuilt in-memory path.

The receipts intentionally carry no-claim fields, including:

- `json_source_authoritative: true`
- `generated_machine_cache_only: true`
- `search_behavior_changed: false`
- `normal_search_behavior_changed: false`
- `full_icon_search_speed_claimed: false`
- `full_startup_search_render_proof_claimed: false`
- `full_render_proof_claimed: false`
- `upstream_baseline_measured: false`
- `faster_than_upstream_claimed: false`
- `same_machine_benchmark_required: true`

## Implemented Cache Families

The tracked `G:\Dx\www\related-crates\media-icon` lane currently has source
guards for these generated cache families:

- `manifest.machine`
- `catalog.machine`
- `prefix.machine`
- `perfect-hash.machine`
- `bloom.machine`
- `lowercase-cache.machine`
- `pack-body.machine`

Wave 5y also tightened the manifest source fingerprint so JSON file content
hashes are folded into the manifest source hash, and the readiness receipt uses
unique PID/time/counter temp names before rename.

## Open Proof

The same-machine upstream baseline has not been measured. Faster-than-upstream is not claimed. Full startup/search/render proof is still open.

Open items before any broad performance claim:

- Generate real same-machine local receipts for startup, body resolution, and
  bounded query latency.
- Run an upstream/fork baseline on the same machine with the same dataset and
  commands.
- Keep standalone `G:\Dx\icon` and `G:\Dx\media\icon` mirrors aligned only after
  the tracked `G:\Dx\www` lane is stable.
- Avoid carrying DX-WWW runtime scores into this crate.

Do not reuse DX-WWW runtime scores for media-icon. The DX-WWW runtime score is a
different product surface and does not prove media-icon search/cache speed.

## Useful Commands

Use lightweight source checks while this lane remains proof-first:

```powershell
node --test benchmarks\media-icon-claim-alignment-contract.test.ts
node --test benchmarks\media-icon-existing-cache-readiness-contract.test.ts
cargo check --manifest-path G:\Dx\www\Cargo.toml -p dx-icons --locked --no-default-features --features rayon --lib --bin build_index -j 1 --message-format=short --color never
```

Run heavier benchmarks only when the machine is idle and the result will be
recorded as a same-machine receipt with explicit dataset, command, checkout, and
hardware context.
