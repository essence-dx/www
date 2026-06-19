# DX-WWW Wire Format Audit

Last updated: 2026-05-16

This audit is the Phase 2 ownership map for every binary page, style, update, and serializer format currently shipped or staged in DX-WWW. Its job is to keep browser-delivered packets, compiler build artifacts, style cache artifacts, and general serializers from collapsing into one ambiguous "binary web" format.

## Boundary Rules

1. Browser-delivered route and update payloads must move toward one canonical `dxp` packet envelope.
2. `dxob` is a compiler/build artifact unless a future decision explicitly promotes a subset into the browser packet ABI.
3. Generated/minified CSS remains the default browser style delivery path. Binary style formats are cache, analysis, or measured opt-in patch artifacts.
4. Serializer machine formats are for storage, config, tooling, build cache, or server-side metadata. They are not the route packet ABI.
5. Prototype formats without a magic, version, and compatibility story must be wrapped by `dxp` before they are browser contracts.
6. Magic-byte collisions are not allowed in new public formats. Existing collisions must be quarantined before we claim a stable ABI.

## Inventory

| Format | Current Owner | Header Or Magic | Payload Role | Browser Delivered Today | Status |
| --- | --- | --- | --- | --- | --- |
| HTIP v1 binary | `www/binary` | `DXB1`, version `1`, signed header, template/string/opcode sections | Early signed browser stream and DOM operation proof | Yes, in legacy/tests paths | Legacy runtime proof. Keep as compatibility evidence, not the next public ABI. |
| HTIP v2 packet | `www/packet`, `www/core/src/codegen.rs`, `www/browser/src/stream_reader.rs` | 16-byte header with `0x4458` (`DX`), version `2`, flags/counts/payload size; chunk headers are 5 bytes | Current packet-layer stream, chunked template/state/wasm/patch transport | Partially, through browser stream reader and tests | Strongest existing runtime packet base. Candidate payload source for canonical `dxp`, but needs a clearer envelope and type contracts. |
| Core DXOB compiler object | `www/core/src/binary_compiler.rs` | `DXOB`, version `1`, component type, name, string table, templates, bindings, events, CSS metadata | Compiler-side component/page object and vertical proof round-trip | No, except proof reporting | Keep as compiler artifact. It should feed `DxPageGraph`/`DxComponentGraph`, then emit `dxp` for browsers. |
| Legacy CLI DXOB object | `www/dx-www/src/cli/mod.rs` | `DXOB`, `u16` version, flags, section offsets and lengths, payload hash | Standalone compile wrapper binary output | No | Quarantine and migrate. It reuses `DXOB` magic with an incompatible layout. |
| Delivery lab template slots | `www/core/src/delivery/encoding.rs` | `DXT1` | Template slot payload experiment | No | Good payload idea. Needs decoder and `dxp` envelope before product use. |
| Delivery lab columnar slots | `www/core/src/delivery/encoding.rs` | `DXC1` | Columnar repeated-slot payload experiment | No | Good medium/big UI size lever. Needs typed schema and `dxp` envelope. |
| Delivery lab semantic codec | `www/core/src/delivery/encoding.rs` | `DXS1` | Semantic compression for ranges, enums, repeated prefixes, and shaped data | No | High-upside but shape-specific. Must degrade safely to normal slots. |
| Delivery lab patch stream | `www/core/src/delivery/encoding.rs` | `DXP1` | Compact update patch stream experiment | No | Keep as payload experiment. Avoid using `DXP1` as the future envelope magic to prevent naming confusion. |
| Browser delta patch | `www/browser/src/lib.rs`, `www/binary/src/delta.rs` | `DXDL` | Delta patch payload | Partially in browser tests/runtime code | Can inform `DxPatchStream`, but should become a packet kind under canonical `dxp`. |
| Binary Dawn style file | `www/related-crates/style/src/binary/dawn.rs`, `www/browser/src/style_loader.rs` | `DXBD`, version `1`, entry table, checksum | Binary CSS storage/runtime experiment | Optional sidecar only | Keep measured opt-in. Generated CSS remains default browser output. |
| Binary style ID stream | `www/related-crates/style/src/binary/api.rs` | No versioned envelope; combo `0xFF` or varint IDs | Compact style selection command | No | Internal experiment. Needs a versioned style-patch packet before browser use. |
| Server binary fragment | `www/core/src/server_component.rs` | 8-byte header: template id, slot count, total size | Server component template/slot fragment | No | Prototype payload. Needs magic/version or `dxp` wrapping. |
| Streaming SSR chunk | `www/core/src/streaming.rs` | 5-byte header: chunk type, target slot, payload length | Progressive template/data/activate chunks | No | Prototype payload. Could become a `dxp` chunk kind. |
| LiveView binary patch | `www/core/src/liveview.rs` | 4-byte header: target, op, value length | Small DOM update operation | No | Useful op vocabulary. Needs typed `DxPatchOp` and packet framing. |
| DXB packer container | `www/core/src/packer.rs` | `DX`, version `1`; original artifact+WASM mode or HTIP-only mode flag | Bundle/container artifact around templates, HTIP, runtime metadata | No direct browser ABI | Treat as package/container experiment, not page packet ABI. |
| Server stream wrapper | `www/server/src/stream.rs` | Chunked wrapper using `DX\0\0`, version, signature bytes | Server-side stream framing | Not the vertical proof path | Reconcile after `dxp` envelope is selected. |
| DX-Machine serializer header | `serializer/src/machine/header.rs` | `ZD`, version `1`, flags | General serializer/storage header | No | Keep for storage/config/tooling. Not route delivery. |
| DX-Machine serde compat | `serializer/src/machine/serde_compat.rs` | `ZD`, version `1`, flags `0x01`, payload length, reserved bytes | Serde-compatible binary value encoding | No | Keep separate from browser packet ABI. Note header mismatch with the compact machine header. |

## Ownership Decision

The next public browser contract should be a new canonical `dxp` envelope, not raw HTIP, raw DXOB, raw delivery lab payloads, or raw serializer data.

Recommended ownership:

- `dxob`: compiler/build artifact for parsed page and component structure.
- `dxp`: browser-delivered route, template/data, patch, and update packet.
- `dxbd`: optional binary style cache or style patch artifact when measured smaller/faster than CSS.
- `dxm`/DX-Machine: serializer and storage layer for build cache, Forge metadata, config, and server-side immutable metadata.
- `.dxb`: bundle/container artifact only if it remains useful after `dxp` exists.

## Canonical `dxp` Direction

The canonical packet now starts with the `DXPK` envelope. Version 1 uses one fixed header followed by typed section headers and section payload bytes:

| Field | Size | Notes |
| --- | ---: | --- |
| Magic | 4 | `DXPK`, distinct from `DXOB`, `DXB1`, `DXT1`, `DXC1`, `DXS1`, and `DXP1`. |
| Version | 2 | Little-endian `u16`, currently `1`. |
| Packet kind | 1 | Route, template dictionary, instance batch, patch stream, style, or manifest. |
| Reserved | 1 | Reserved for alignment/future flags. |
| Flags | 4 | Compression, signatures, dictionaries, and chunking hooks. |
| Header length | 2 | Fixed `52` bytes in v1. |
| Section count | 2 | Number of typed sections. |
| Payload length | 4 | Sum of raw section payload bytes. |
| Payload hash | 32 | BLAKE3 over raw section payload bytes in order. |

Each section then carries kind, encoding, reserved bytes, payload length, a 32-byte BLAKE3 section hash, and the raw section bytes. The decoder rejects bad magic, unsupported versions, unknown kind/encoding codes, wrong lengths, hash mismatches, truncation, and trailing bytes.

The `dxp` payload can then reuse the best existing ideas:

- HTIP v2 chunking and opcodes for runtime stream structure.
- `DXT1`, `DXC1`, `DXS1`, and `DXP1` payload experiments as internal section encodings.
- LiveView patch op vocabulary as a starting point for `DxPatchOp`.
- Binary Dawn as an optional style sidecar reference, not default CSS delivery.
- DX-Machine/rkyv only for build cache and server-side metadata where zero-copy access matters.

## Immediate Blockers

1. `DXOB` currently names two incompatible layouts: the core compiler object and the legacy standalone CLI object.
2. The vertical proof currently decodes compiler-side DXOB, not a browser-stable packet.
3. Delivery lab encoders have useful signals but no matching product decoder/envelope.
4. Several prototype payloads have no magic/version/compatibility header.
5. Binary style exists, but CSS-first remains the correct default until the planner proves a per-page binary win with decoder cost included.
6. Client/runtime code can recognize HTIP v1/v2 paths, but the product needs one `dxp` contract that owns future compatibility.

## Next Implementation Order

1. Define typed `DxPageGraph`, `DxComponentGraph`, `DxStyleGraph`, `DxPacket`, `DxFallbackHtml`, and `DxSourceManifest` contracts.
2. Implement a minimal `dxp` envelope encoder/decoder with route and patch packet kinds.
3. Convert the vertical proof from "fallback HTML plus decoded DXOB report" to "fallback HTML plus emitted/decoded `dxp` route packet".
4. Add round-trip tests for every packet kind before changing browser runtime behavior.
5. Leave Binary Dawn and DX-Machine outside the browser page ABI unless a benchmark proves they belong in a specific packet kind.
