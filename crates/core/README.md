# dx-www compiler core

`dx-www-compiler` is the source-analysis and delivery-planning crate for the
DX WWW framework. It parses TSX/JSX, discovers component and state surfaces,
classifies runtime boundaries, and produces internal delivery artifacts used by
the Rust-owned WWW build/dev/check pipeline.

This crate is not the public framework contract by itself. Public WWW starter
apps are described by the extensionless `dx` config, an `app/` route tree,
`components/`, `styles/`, `public/`, server files, generated output, and
`.dx/*` receipts.

## Current Role

- Parse TSX/JSX source, with OXC support when enabled.
- Analyze components, event handlers, state reads, and delivery complexity.
- Split static markup from dynamic bindings where the compiler can do so safely.
- Feed source-owned render, state, and runtime contracts.
- Keep legacy binary-format experiments isolated for compatibility tests.

## What This Crate Does Not Claim

- It is not a full React runtime.
- It is not a full Next.js compiler.
- It does not make binary route output the default public browser contract.
- Legacy packet/object experiments must stay behind explicit compatibility
  boundaries until promoted by the wire-format audit.

## Quick Start

```rust
use dx_compiler::{analyze_tsx, can_compile};
use std::path::Path;

let entry = Path::new("app/page.tsx");

if can_compile(entry) {
    let (metrics, variant) = analyze_tsx(entry, false)?;
    println!("Runtime: {:?}, components: {}", variant, metrics.component_count);
}

# Ok::<(), anyhow::Error>(())
```

## Main Modules

- `parser`: TSX/JSX parsing and source extraction.
- `delivery`: source-owned route, state, and runtime delivery contracts.
- `splitter`: static template and binding separation.
- `analyzer`: complexity analysis and runtime selection.
- `codegen`: internal delivery code generation.
- `binary_compiler`: legacy/internal binary object experiment.

For the current public framework story, start with the root `README.md`,
`dx-www/src/lib.rs`, and `docs/architecture.md`.
