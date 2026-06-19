# DX Icon Search

Receipt-backed Rust icon search components for DX.

## Run

cargo run --release --bin search_cli

## Features

- **FST-based prefix search** - Indexed prefix lookup
- **Zero-copy rkyv metadata** - No deserialization overhead
- **Fuzzy matching** - Typo tolerance with Levenshtein distance
- **LZ4 compression** - Fast decompression for network transfer
- **WASM support** - Browser-oriented build support
- **Multi-strategy search** - Exact, prefix, and fuzzy matching
- **Smart caching** - LRU cache for repeated queries

## Architecture

```
┌─────────────────────────────────────────────────┐
│  TIER 1: FST Index (~1MB)                       │
│  - Finite State Transducer for prefix search   │
│  - O(k) lookup where k = query length          │
└─────────────────────────────────────────────────┘
           ↓
┌─────────────────────────────────────────────────┐
│  TIER 2: rkyv Metadata (~2MB)                   │
│  - Zero-copy archived data                      │
│  - Direct memory access, no parsing             │
└─────────────────────────────────────────────────┘
```

## Usage

### Build Index

```bash
cargo run --bin build_index
```

### CLI Search

```bash
cargo run --bin search_cli
```

### WASM Build

```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/dx_icon_search.wasm --out-dir pkg
```

## Evidence Boundary

- JSON icon packs remain the source of truth.
- Generated `.machine` files are advisory local caches.
- Same-machine upstream baseline proof has not been measured yet.
- Faster-than-upstream is not claimed.

## License

MIT
