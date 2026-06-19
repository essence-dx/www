//! DX-owned generated-to-source map adapter inspired by Turbopack core.
//!
//! The adapter is intentionally data-only: it normalizes source-map segments and
//! resolves generated output positions back to original source positions without
//! linking or executing vendored Turbopack, Node/NAPI, or `node_modules`.

use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Schema for the DX-owned Turbopack-core-inspired source-map adapter.
pub const DX_NEXT_RUST_SOURCE_MAP_ADAPTER_SCHEMA: &str =
    "dx.nextRust.turbopackCore.sourceMapAdapter";

/// Stable format version for [`DxNextRustSourceMapAdapter`].
pub const DX_NEXT_RUST_SOURCE_MAP_ADAPTER_FORMAT: u16 = 1;

/// A generated-to-original source-map segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustSourceMapSegment {
    /// One-based generated output line.
    pub generated_line: u32,
    /// Zero-based generated output column.
    pub generated_column: u32,
    /// Project-relative original source path.
    pub source_path: String,
    /// One-based original source line.
    pub original_line: u32,
    /// Zero-based original source column.
    pub original_column: u32,
}

/// DX-owned source-map receipt derived from Turbopack core's `SourceMap` idea.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustSourceMapAdapter {
    /// Stable DX receipt schema.
    pub schema: &'static str,
    /// Stable numeric format version.
    pub format: u16,
    /// Vendored upstream crate that inspired the adapter.
    pub upstream_crate: &'static str,
    /// Upstream concept represented without executing upstream runtime code.
    pub upstream_concept: &'static str,
    /// Project-relative generated output path.
    pub generated_path: String,
    /// Deterministically ordered source paths represented by the segments.
    pub sources: Vec<String>,
    /// Deterministically ordered generated-to-original segments.
    pub segments: Vec<DxNextRustSourceMapSegment>,
    /// Number of distinct source files represented by this adapter.
    pub source_count: usize,
    /// Number of normalized segments represented by this adapter.
    pub segment_count: usize,
    /// Whether this remains an adapter boundary instead of native upstream runtime adoption.
    pub adapter_boundary: bool,
    /// Whether this exposes Turbopack as the public DX architecture.
    pub public_architecture: bool,
    /// Whether the vendored Turbopack runtime executed.
    pub turbopack_runtime_executed: bool,
    /// Whether project-local `node_modules` are required.
    pub node_modules_required: bool,
    /// Boundary explanation for receipts and status surfaces.
    pub boundary: &'static str,
}

/// A resolved original source position for a generated output position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustSourceMapLookup {
    /// One-based generated output line requested by the caller.
    pub generated_line: u32,
    /// Zero-based generated output column requested by the caller.
    pub generated_column: u32,
    /// Project-relative original source path.
    pub source_path: String,
    /// One-based resolved original source line.
    pub original_line: u32,
    /// Zero-based resolved original source column.
    pub original_column: u32,
    /// Whether the lookup matched an exact generated segment start.
    pub exact_segment: bool,
    /// Whether this remains an adapter boundary instead of native upstream runtime adoption.
    pub adapter_boundary: bool,
    /// Whether this exposes Turbopack as the public DX architecture.
    pub public_architecture: bool,
    /// Whether the vendored Turbopack runtime executed.
    pub turbopack_runtime_executed: bool,
    /// Whether project-local `node_modules` are required.
    pub node_modules_required: bool,
    /// Boundary explanation for receipts and status surfaces.
    pub boundary: &'static str,
}

/// A diagnostic-ready source-map location that carries both generated and source positions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustSourceMapDiagnosticLocation {
    /// Project-relative generated output path.
    pub generated_path: String,
    /// One-based generated output line requested by the caller.
    pub generated_line: u32,
    /// Zero-based generated output column requested by the caller.
    pub generated_column: u32,
    /// Project-relative original source path.
    pub source_path: String,
    /// One-based resolved original source line.
    pub source_line: u32,
    /// Zero-based resolved original source column.
    pub source_column: u32,
    /// Whether the lookup matched an exact generated segment start.
    pub exact_segment: bool,
    /// Source identifier suitable for DX diagnostic/code-frame receipts.
    pub code_frame_source: &'static str,
    /// Whether this remains an adapter boundary instead of native upstream runtime adoption.
    pub adapter_boundary: bool,
    /// Whether this exposes Turbopack as the public DX architecture.
    pub public_architecture: bool,
    /// Whether the vendored Turbopack runtime executed.
    pub turbopack_runtime_executed: bool,
    /// Whether project-local `node_modules` are required.
    pub node_modules_required: bool,
    /// Boundary explanation for receipts and status surfaces.
    pub boundary: &'static str,
}

/// Build a deterministic source-map adapter from generated-to-original segments.
///
/// Invalid segments with empty source paths or zero line numbers are discarded.
/// Remaining paths are normalized to forward slashes, sorted by generated
/// position, and exact duplicates are removed.
#[must_use]
pub fn dx_next_rust_source_map_adapter(
    generated_path: &str,
    segments: Vec<DxNextRustSourceMapSegment>,
) -> DxNextRustSourceMapAdapter {
    let mut segments = segments
        .into_iter()
        .filter_map(normalize_segment)
        .collect::<Vec<_>>();

    segments.sort_by(compare_segments);
    segments.dedup();

    let sources = segments
        .iter()
        .map(|segment| segment.source_path.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    DxNextRustSourceMapAdapter {
        schema: DX_NEXT_RUST_SOURCE_MAP_ADAPTER_SCHEMA,
        format: DX_NEXT_RUST_SOURCE_MAP_ADAPTER_FORMAT,
        upstream_crate: "turbopack/crates/turbopack-core",
        upstream_concept: "SourceMap",
        generated_path: normalize_path(generated_path),
        source_count: sources.len(),
        segment_count: segments.len(),
        sources,
        segments,
        adapter_boundary: true,
        public_architecture: false,
        turbopack_runtime_executed: false,
        node_modules_required: false,
        boundary: "DX-owned source-map adapter; vendored Turbopack core SourceMap is reference material only",
    }
}

/// Resolve a generated output position to its nearest original source position.
///
/// Lookups use the last segment whose generated position is before or equal to
/// the requested position. Same-line lookups carry the generated column delta to
/// the original column; cross-line lookups keep the segment's original column.
#[must_use]
pub fn dx_next_rust_source_map_lookup(
    adapter: &DxNextRustSourceMapAdapter,
    generated_line: u32,
    generated_column: u32,
) -> Option<DxNextRustSourceMapLookup> {
    if generated_line == 0 {
        return None;
    }

    let mut candidate = None;
    for segment in &adapter.segments {
        match compare_segment_to_position(segment, generated_line, generated_column) {
            Ordering::Less | Ordering::Equal => candidate = Some(segment),
            Ordering::Greater => break,
        }
    }

    let segment = candidate?;
    let same_line = segment.generated_line == generated_line;
    let original_column = if same_line {
        segment
            .original_column
            .saturating_add(generated_column.saturating_sub(segment.generated_column))
    } else {
        segment.original_column
    };

    Some(DxNextRustSourceMapLookup {
        generated_line,
        generated_column,
        source_path: segment.source_path.clone(),
        original_line: segment.original_line,
        original_column,
        exact_segment: same_line && segment.generated_column == generated_column,
        adapter_boundary: adapter.adapter_boundary,
        public_architecture: adapter.public_architecture,
        turbopack_runtime_executed: adapter.turbopack_runtime_executed,
        node_modules_required: adapter.node_modules_required,
        boundary: adapter.boundary,
    })
}

/// Resolve a generated output position into a diagnostic-ready source location.
///
/// This is the build/dev diagnostics handoff: it wraps a lookup with the
/// generated output path and keeps the same adapter-boundary metadata.
#[must_use]
pub fn dx_next_rust_source_map_diagnostic_location(
    adapter: &DxNextRustSourceMapAdapter,
    generated_line: u32,
    generated_column: u32,
) -> Option<DxNextRustSourceMapDiagnosticLocation> {
    let lookup = dx_next_rust_source_map_lookup(adapter, generated_line, generated_column)?;

    Some(DxNextRustSourceMapDiagnosticLocation {
        generated_path: adapter.generated_path.clone(),
        generated_line: lookup.generated_line,
        generated_column: lookup.generated_column,
        source_path: lookup.source_path,
        source_line: lookup.original_line,
        source_column: lookup.original_column,
        exact_segment: lookup.exact_segment,
        code_frame_source: "dx.nextRust.sourceMapAdapter",
        adapter_boundary: lookup.adapter_boundary,
        public_architecture: lookup.public_architecture,
        turbopack_runtime_executed: lookup.turbopack_runtime_executed,
        node_modules_required: lookup.node_modules_required,
        boundary: lookup.boundary,
    })
}

fn normalize_segment(
    mut segment: DxNextRustSourceMapSegment,
) -> Option<DxNextRustSourceMapSegment> {
    segment.source_path = normalize_path(&segment.source_path);

    if segment.source_path.is_empty() || segment.generated_line == 0 || segment.original_line == 0 {
        return None;
    }

    Some(segment)
}

fn normalize_path(path: &str) -> String {
    path.trim().replace('\\', "/")
}

fn compare_segments(
    left: &DxNextRustSourceMapSegment,
    right: &DxNextRustSourceMapSegment,
) -> Ordering {
    left.generated_line
        .cmp(&right.generated_line)
        .then_with(|| left.generated_column.cmp(&right.generated_column))
        .then_with(|| left.source_path.cmp(&right.source_path))
        .then_with(|| left.original_line.cmp(&right.original_line))
        .then_with(|| left.original_column.cmp(&right.original_column))
}

fn compare_segment_to_position(
    segment: &DxNextRustSourceMapSegment,
    generated_line: u32,
    generated_column: u32,
) -> Ordering {
    segment
        .generated_line
        .cmp(&generated_line)
        .then_with(|| segment.generated_column.cmp(&generated_column))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment(
        generated_line: u32,
        generated_column: u32,
        source_path: &str,
        original_line: u32,
        original_column: u32,
    ) -> DxNextRustSourceMapSegment {
        DxNextRustSourceMapSegment {
            generated_line,
            generated_column,
            source_path: source_path.to_string(),
            original_line,
            original_column,
        }
    }

    #[test]
    fn source_map_adapter_sorts_and_deduplicates_segments() {
        let adapter = dx_next_rust_source_map_adapter(
            ".dx\\build\\app\\page.js",
            vec![
                segment(3, 9, "app\\page.tsx", 8, 4),
                segment(2, 0, "app/layout.tsx", 1, 0),
                segment(3, 9, "app/page.tsx", 8, 4),
                segment(0, 0, "app/ignored.tsx", 1, 0),
                segment(4, 0, "   ", 1, 0),
            ],
        );

        assert_eq!(adapter.generated_path, ".dx/build/app/page.js");
        assert_eq!(adapter.source_count, 2);
        assert_eq!(adapter.segment_count, 2);
        assert_eq!(adapter.sources, vec!["app/layout.tsx", "app/page.tsx"]);
        assert_eq!(adapter.segments[0].generated_line, 2);
        assert_eq!(adapter.segments[0].source_path, "app/layout.tsx");
        assert_eq!(adapter.segments[1].generated_line, 3);
        assert_eq!(adapter.segments[1].source_path, "app/page.tsx");
        assert!(adapter.adapter_boundary);
        assert!(!adapter.public_architecture);
        assert!(!adapter.turbopack_runtime_executed);
        assert!(!adapter.node_modules_required);
    }

    #[test]
    fn source_map_lookup_uses_nearest_previous_segment() {
        let adapter = dx_next_rust_source_map_adapter(
            ".dx/build/app/page.js",
            vec![
                segment(5, 12, "app/page.tsx", 10, 20),
                segment(5, 2, "app/page.tsx", 10, 4),
                segment(7, 0, "app/other.tsx", 1, 0),
            ],
        );

        let lookup =
            dx_next_rust_source_map_lookup(&adapter, 5, 8).expect("nearest same-line segment");
        assert_eq!(lookup.source_path, "app/page.tsx");
        assert_eq!(lookup.original_line, 10);
        assert_eq!(lookup.original_column, 10);
        assert!(!lookup.exact_segment);
        assert!(lookup.adapter_boundary);
        assert!(!lookup.turbopack_runtime_executed);
        assert!(!lookup.node_modules_required);

        let exact = dx_next_rust_source_map_lookup(&adapter, 5, 12).expect("exact segment");
        assert_eq!(exact.original_column, 20);
        assert!(exact.exact_segment);
    }

    #[test]
    fn source_map_lookup_rejects_positions_before_first_segment() {
        let adapter = dx_next_rust_source_map_adapter(
            ".dx/build/app/page.js",
            vec![segment(10, 3, "app/page.tsx", 2, 1)],
        );

        assert!(dx_next_rust_source_map_lookup(&adapter, 0, 0).is_none());
        assert!(dx_next_rust_source_map_lookup(&adapter, 1, 0).is_none());
        assert!(dx_next_rust_source_map_lookup(&adapter, 10, 2).is_none());
        assert!(dx_next_rust_source_map_lookup(&adapter, 10, 3).is_some());
    }

    #[test]
    fn source_map_diagnostic_location_carries_generated_and_source_positions() {
        let adapter = dx_next_rust_source_map_adapter(
            ".dx/build/app/page.js",
            vec![
                segment(4, 0, "app/page.tsx", 9, 2),
                segment(4, 10, "app/page.tsx", 9, 20),
            ],
        );

        let location = dx_next_rust_source_map_diagnostic_location(&adapter, 4, 12)
            .expect("diagnostic location");

        assert_eq!(location.generated_path, ".dx/build/app/page.js");
        assert_eq!(location.generated_line, 4);
        assert_eq!(location.generated_column, 12);
        assert_eq!(location.source_path, "app/page.tsx");
        assert_eq!(location.source_line, 9);
        assert_eq!(location.source_column, 22);
        assert_eq!(location.code_frame_source, "dx.nextRust.sourceMapAdapter");
        assert!(!location.exact_segment);
        assert!(location.adapter_boundary);
        assert!(!location.public_architecture);
        assert!(!location.turbopack_runtime_executed);
        assert!(!location.node_modules_required);
        assert!(location.boundary.contains("reference material only"));
    }
}
