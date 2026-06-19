use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::types::DxDeliveryMode;

pub use crate::ecosystem::DxSourceManifest;

/// Versioned browser packet magic for the canonical DX packet envelope.
pub const DX_PACKET_MAGIC: [u8; 4] = *b"DXPK";

/// Initial canonical DX packet version.
pub const DX_PACKET_VERSION: u16 = 1;

/// Compiler-facing page graph that feeds fallback HTML and browser packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxPageGraph {
    /// Stable route id, for example `/pricing`.
    pub route_id: String,
    /// Optional source file path relative to the project root.
    pub source_path: Option<String>,
    /// Root component id.
    pub root_component_id: String,
    /// Component graph used by this page.
    pub components: DxComponentGraph,
    /// Style graph used by this page.
    pub styles: DxStyleGraph,
    /// Forge manifest hash or receipt hash associated with this page build.
    pub source_manifest_hash: Option<String>,
}

/// First-class compiler output for one route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxRouteUnit {
    /// Route unit schema version.
    pub version: u32,
    /// Stable route path.
    pub route: String,
    /// Project-relative route source path.
    pub source_path: String,
    /// Crawlable shell and style assets.
    pub shell: DxRouteShell,
    /// Typed component/style/source graph.
    pub graph: DxPageGraph,
    /// Typed local/page/global state graph.
    pub state: DxStateGraph,
    /// Browser packet reference, when a packet was emitted.
    pub packet: DxRoutePacketRef,
    /// Source ownership/provenance receipt for this route build.
    pub receipt: DxRouteReceipt,
    /// Adaptive runtime decision report.
    pub runtime_report: DxAdaptiveRuntimeReport,
}

/// Crawlable route shell emitted before optional runtime work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxRouteShell {
    /// Crawlable fallback HTML.
    pub fallback: DxFallbackHtml,
    /// Generated CSS hrefs used by the shell.
    pub style_hrefs: Vec<String>,
    /// Streaming strategy selected for the route shell.
    pub streaming_strategy: String,
    /// Bytes in the first streaming flush, if any.
    pub first_flush_bytes: usize,
    /// Whether any browser runtime is required.
    pub runtime_required: bool,
}

/// Browser packet reference for a route unit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxRoutePacketRef {
    /// Packet kind.
    pub kind: DxPacketKind,
    /// Encoded packet bytes.
    pub bytes: usize,
    /// Packet payload bytes.
    pub payload_bytes: u32,
    /// Number of packet sections.
    pub section_count: usize,
    /// Whether packet encoding round-tripped.
    pub roundtrip_matches: bool,
}

/// Route-build source ownership receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxRouteReceipt {
    /// Forge manifest hash associated with the route, when known.
    pub source_manifest_hash: Option<String>,
    /// Whether the route requires node_modules at runtime.
    pub node_modules_required: bool,
    /// Source-owned client-island ABI proof metadata for this route, when islands exist.
    pub client_island_abi: Option<DxRouteClientIslandAbiReceipt>,
    /// Local or Forge-owned source paths included in the route unit.
    pub source_paths: Vec<String>,
    /// Forge-owned package ids included in the route unit.
    pub forge_package_ids: Vec<String>,
}

/// Route-unit proof metadata for source-owned client islands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxRouteClientIslandAbiReceipt {
    /// Receipt schema for the route-local island ABI proof.
    pub schema: String,
    /// Schema revision.
    pub schema_revision: u32,
    /// Public directive style used by WWW authoring.
    pub directive_style_id: String,
    /// Core camelCase hydration directives guarded by the ABI.
    pub core_directives: Vec<String>,
    /// Whether the island runtime is compiler/source owned.
    pub source_owned_runtime: bool,
    /// Whether route islands require node_modules at runtime.
    pub node_modules_required: bool,
    /// Whether this route proof claims full React hydration.
    pub full_react_hydration: bool,
    /// Whether no-JS HTML fallback boundaries are required.
    pub no_js_fallback_required: bool,
    /// Number of route islands covered by this proof metadata.
    pub island_count: usize,
    /// Number of route islands covered by source-owned runtime.
    pub source_owned_island_count: usize,
    /// Number of explicit framework adapter islands.
    pub framework_adapter_count: usize,
    /// Number of `clientOnly` adapter islands.
    pub client_only_adapter_count: usize,
    /// Hydration strategies observed for this route's islands.
    pub hydration_strategies: Vec<String>,
    /// Browser proof status for this source-only route proof.
    pub browser_proof_status: String,
    /// Scope of the proof represented by this route metadata.
    pub proof_scope: String,
    /// Whether this route-level island proof is release-ready.
    pub release_ready: bool,
    /// Whether this proof supports fastest-world claims.
    pub fastest_world_claim: bool,
}

/// Default state visibility for a route graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Default)]
pub enum DxStateScope {
    /// State belongs to one component/client island by default.
    #[default]
    Local,
    /// State is explicitly lifted to the route/page.
    Page,
    /// State is explicitly shared beyond one route.
    Global,
}

/// Typed state graph compiled from React-shaped authoring.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxStateGraph {
    /// Default state scope. DX-WWW keeps state local unless source says otherwise.
    pub default_scope: DxStateScope,
    /// App-level stores compiled from DX-native `store({ ... })` authoring.
    pub global_stores: Vec<DxGlobalStore>,
    /// Mutable state slots.
    pub slots: Vec<DxStateSlot>,
    /// Derived/computed slots.
    pub derived_slots: Vec<DxDerivedStateSlot>,
    /// Event slots that mutate state or invoke actions.
    pub event_slots: Vec<DxStateEventSlot>,
    /// Effect slots that observe state.
    pub effects: Vec<DxStateEffectSlot>,
    /// Server action edges reachable from this route.
    pub server_actions: Vec<DxServerActionEdge>,
}

/// One mutable state slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxStateSlot {
    /// Stable slot id.
    pub id: String,
    /// Developer-facing state name.
    pub name: String,
    /// React-compatible setter name when lowered from useState.
    pub setter: Option<String>,
    /// Slot scope.
    pub scope: DxStateScope,
    /// Project-relative source path.
    pub source_path: String,
    /// Original initial-value source text.
    pub initial_source: String,
    /// Coarse value kind used by runtime selection and diagnostics.
    pub value_kind: String,
}

/// One derived/computed state slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxDerivedStateSlot {
    /// Stable slot id.
    pub id: String,
    /// Developer-facing derived name.
    pub name: String,
    /// Original expression source.
    pub expression: String,
    /// State slots referenced by this expression.
    pub dependencies: Vec<String>,
    /// Project-relative source path.
    pub source_path: String,
}

/// One event slot in the state graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxStateEventSlot {
    /// Stable event slot id.
    pub id: String,
    /// Project-relative source path.
    pub source_path: String,
    /// JSX element name.
    pub element: String,
    /// DOM event name.
    pub event: String,
    /// Handler expression.
    pub handler: String,
    /// State dependencies mentioned by the handler.
    pub state_dependencies: Vec<String>,
    /// Server action reached by this event, when known.
    pub action: Option<String>,
}

/// One effect slot that observes state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxStateEffectSlot {
    /// Stable effect id.
    pub id: String,
    /// Effect kind, for example `effect`.
    pub kind: String,
    /// Project-relative source path.
    pub source_path: String,
    /// State dependencies mentioned by the effect.
    pub dependencies: Vec<String>,
}

/// One app-level global store compiled from DX-native authoring.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxGlobalStore {
    /// Stable store id.
    pub id: String,
    /// Developer-facing store variable name.
    pub name: String,
    /// Project-relative source path.
    pub source_path: String,
    /// Mutable state slots owned by the store.
    pub slots: Vec<DxStateSlot>,
    /// Derived slots owned by the store.
    pub derived_slots: Vec<DxDerivedStateSlot>,
    /// Store actions.
    pub actions: Vec<DxGlobalStoreAction>,
    /// Store effects.
    pub effects: Vec<DxStateEffectSlot>,
}

/// One action exposed by a global store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxGlobalStoreAction {
    /// Stable action id.
    pub id: String,
    /// Developer-facing action name.
    pub name: String,
    /// Project-relative source path.
    pub source_path: String,
    /// Original action body or expression source.
    pub handler: String,
    /// Store state slots referenced by the action.
    pub state_dependencies: Vec<String>,
}

/// Edge from route/client state into a server action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxServerActionEdge {
    /// Stable edge id.
    pub id: String,
    /// Imported action symbol.
    pub action: String,
    /// Project-relative source path that imported or referenced the action.
    pub source_path: String,
    /// Import specifier source, when known.
    pub import_source: Option<String>,
    /// Related event id, when the action is bound to an event.
    pub event_id: Option<String>,
}

/// Runtime decision report attached to every route unit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxAdaptiveRuntimeReport {
    /// Selected route delivery mode.
    pub selected_mode: DxDeliveryMode,
    /// Tiny-static/no-JS route proof used by Readiness and public-byte reports.
    pub tiny_static_route_proof: DxTinyStaticRouteProof,
    /// Candidate modes considered with byte estimates.
    pub candidates: Vec<DxRuntimeCandidate>,
    /// Candidate modes rejected by the compiler.
    pub rejected_modes: Vec<DxRuntimeRejection>,
    /// Human-readable decision reasons.
    pub reasons: Vec<String>,
    /// Honest caveats for benchmark/report surfaces.
    pub warnings: Vec<String>,
}

/// Concrete evidence for static/no-JS route output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxTinyStaticRouteProof {
    /// Whether the compiler selected the tiny-static route mode.
    pub selected: bool,
    /// Stable output-mode label used in HTML/proof surfaces.
    pub output_mode: String,
    /// Browser JavaScript requirement for this route.
    pub js: String,
    /// Crawlable fallback HTML bytes.
    pub html_bytes: usize,
    /// Generated CSS bytes referenced by the route.
    pub css_bytes: usize,
    /// Public bytes required before optional evidence receipts.
    pub total_public_bytes: usize,
    /// Number of script tags in the emitted fallback shell.
    pub script_tag_count: usize,
    /// Whether route-level browser runtime is required.
    pub runtime_required: bool,
    /// Whether the route can render meaningful HTML/CSS with JavaScript disabled.
    pub no_js_capable: bool,
    /// Whether the fallback contains visible semantic content.
    pub meaningful_html: bool,
    /// Whether the fallback exposes a semantic document landmark.
    pub semantic_landmark_present: bool,
    /// Number of anchor elements available for no-JS navigation.
    pub link_count: usize,
    /// Number of forms available for progressive no-JS submission.
    pub form_count: usize,
    /// Whether route fallback metadata carries a title-like SEO signal.
    pub seo_title_present: bool,
    /// Count of simple accessibility signals found in the fallback HTML.
    pub accessibility_signal_count: usize,
    /// Whether browser APIs are required for the route's authored behavior.
    pub browser_api_required: bool,
    /// Honest parity status against Astro tiny-static benchmarks.
    pub astro_parity_status: String,
}

/// One runtime candidate considered by the compiler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxRuntimeCandidate {
    /// Candidate mode.
    pub mode: DxDeliveryMode,
    /// Estimated bytes for this candidate.
    pub estimated_bytes: usize,
    /// Whether this candidate was selected.
    pub selected: bool,
    /// Short reason for the estimate/selection.
    pub reason: String,
}

/// One rejected runtime candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxRuntimeRejection {
    /// Rejected mode.
    pub mode: DxDeliveryMode,
    /// Estimated bytes for this candidate.
    pub estimated_bytes: usize,
    /// Why the compiler rejected it.
    pub reason: String,
}

/// Component dependency graph with source-owned package provenance.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxComponentGraph {
    /// Component nodes.
    pub nodes: Vec<DxComponentNode>,
    /// Directed component dependency edges.
    pub edges: Vec<DxComponentEdge>,
}

/// One component in a page graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxComponentNode {
    /// Stable component id.
    pub id: String,
    /// Display or source name.
    pub name: String,
    /// Optional source-owned package id.
    pub package_id: Option<String>,
    /// Hash of the component template/source body.
    pub content_hash: String,
}

/// Directed dependency between two component ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxComponentEdge {
    /// Parent/importing component id.
    pub from: String,
    /// Child/imported component id.
    pub to: String,
}

/// Style graph extracted from tokens, classes, and generated CSS.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxStyleGraph {
    /// Source-owned design tokens.
    pub tokens: Vec<DxStyleToken>,
    /// Generated or referenced classes.
    pub classes: Vec<DxStyleClass>,
    /// Preferred browser delivery mode for styles.
    pub delivery: DxStyleDelivery,
}

/// One style token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxStyleToken {
    /// Token name.
    pub name: String,
    /// Token value.
    pub value: String,
}

/// One generated CSS class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxStyleClass {
    /// Class name.
    pub name: String,
    /// Minified CSS rule body.
    pub rule: String,
    /// Source hash for ownership/debugging.
    pub source_hash: Option<String>,
}

/// Browser-facing style delivery decision.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxStyleDelivery {
    /// Generated CSS file or inline critical CSS.
    #[default]
    GeneratedCss,
    /// Optional Binary Dawn sidecar.
    BinaryDawnSidecar,
    /// Future style patch packet.
    StylePatch,
}

/// SEO/accessibility fallback HTML emitted beside a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxFallbackHtml {
    /// Minified HTML.
    pub html: String,
    /// Byte length of the HTML string.
    pub bytes: usize,
    /// Whether the fallback is intended to be crawlable without runtime code.
    pub crawlable: bool,
    /// Accessibility warnings found while building fallback HTML.
    pub accessibility_warnings: Vec<String>,
}

impl DxFallbackHtml {
    /// Build a crawlable fallback record from HTML.
    pub fn crawlable(html: impl Into<String>) -> Self {
        let html = html.into();
        Self {
            bytes: html.len(),
            html,
            crawlable: true,
            accessibility_warnings: Vec::new(),
        }
    }
}

/// Canonical browser-delivered DX packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxPacket {
    /// Compatibility header.
    pub header: DxPacketHeader,
    /// Typed packet sections.
    pub sections: Vec<DxPacketSection>,
}

impl DxPacket {
    /// Create a packet and derive payload length plus payload hash.
    pub fn new(kind: DxPacketKind, sections: Vec<DxPacketSection>) -> Self {
        let mut hasher = blake3::Hasher::new();
        let mut payload_len = 0u32;
        for section in &sections {
            hasher.update(&section.bytes);
            payload_len = payload_len.saturating_add(section.bytes.len() as u32);
        }

        Self {
            header: DxPacketHeader {
                magic: DX_PACKET_MAGIC,
                version: DX_PACKET_VERSION,
                kind,
                flags: 0,
                header_len: DxPacketHeader::STRUCTURED_HEADER_LEN,
                payload_len,
                payload_hash: hasher.finalize().to_hex().to_string(),
            },
            sections,
        }
    }

    /// Whether this packet uses the supported canonical envelope.
    pub fn is_compatible(&self) -> bool {
        self.header.magic == DX_PACKET_MAGIC && self.header.version == DX_PACKET_VERSION
    }
}

/// Packet compatibility header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxPacketHeader {
    /// Distinct packet magic.
    pub magic: [u8; 4],
    /// Packet version.
    pub version: u16,
    /// Top-level packet kind.
    pub kind: DxPacketKind,
    /// Future flags for compression, signatures, dictionaries, and chunking.
    pub flags: u32,
    /// Structured header length used by the encoder.
    pub header_len: u16,
    /// Combined section payload bytes.
    pub payload_len: u32,
    /// BLAKE3 hash of all section byte payloads.
    pub payload_hash: String,
}

impl DxPacketHeader {
    /// Initial structured header size target before optional fields.
    pub const STRUCTURED_HEADER_LEN: u16 = DxPacketCodec::HEADER_LEN;
}

/// Top-level packet kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxPacketKind {
    /// Full route payload.
    Route,
    /// Shared template dictionary.
    TemplateDictionary,
    /// Repeated instance batch.
    InstanceBatch,
    /// Live update patch stream.
    PatchStream,
    /// Style reference or style patch.
    Style,
    /// Forge/source manifest reference.
    Manifest,
}

/// One typed packet section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxPacketSection {
    /// Section kind.
    pub kind: DxPacketSectionKind,
    /// Section byte encoding.
    pub encoding: DxPacketSectionEncoding,
    /// Raw section bytes.
    pub bytes: Vec<u8>,
    /// BLAKE3 hash of the raw section bytes.
    pub content_hash: String,
}

impl DxPacketSection {
    /// Create a section and derive its content hash.
    pub fn new(
        kind: DxPacketSectionKind,
        encoding: DxPacketSectionEncoding,
        bytes: Vec<u8>,
    ) -> Self {
        let content_hash = blake3::hash(&bytes).to_hex().to_string();
        Self {
            kind,
            encoding,
            bytes,
            content_hash,
        }
    }
}

/// Section kind inside a canonical packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxPacketSectionKind {
    /// Fallback HTML hash/reference.
    FallbackHtmlRef,
    /// Template slot payload.
    TemplateSlots,
    /// Columnar repeated slots.
    ColumnarSlots,
    /// Semantic data codec payload.
    SemanticCodec,
    /// Patch operations.
    PatchOps,
    /// Style graph or style artifact reference.
    StyleGraph,
    /// Streaming/deferred rendering plan.
    StreamingPlan,
    /// Forge source manifest reference.
    SourceManifest,
}

/// Byte encoding used by a packet section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxPacketSectionEncoding {
    /// UTF-8 JSON metadata.
    Json,
    /// Existing HTIP v2 payload.
    HtipV2,
    /// Existing delivery lab payload, wrapped inside the canonical envelope.
    DeliveryLab,
    /// Future compact canonical binary encoding.
    CanonicalBinary,
}

/// Binary encoder/decoder for the canonical `DXPK` packet envelope.
pub struct DxPacketCodec;

impl DxPacketCodec {
    /// Fixed packet header length.
    pub const HEADER_LEN: u16 = 52;
    /// Fixed per-section header length.
    pub const SECTION_HEADER_LEN: u16 = 40;

    /// Encode a packet into the canonical browser envelope.
    pub fn encode(packet: &DxPacket) -> Result<Vec<u8>, DxPacketCodecError> {
        if packet.header.magic != DX_PACKET_MAGIC {
            return Err(DxPacketCodecError::InvalidMagic {
                expected: DX_PACKET_MAGIC,
                found: packet.header.magic,
            });
        }
        if packet.header.version != DX_PACKET_VERSION {
            return Err(DxPacketCodecError::UnsupportedVersion {
                supported: DX_PACKET_VERSION,
                found: packet.header.version,
            });
        }

        let section_count = u16::try_from(packet.sections.len()).map_err(|_| {
            DxPacketCodecError::TooManySections {
                count: packet.sections.len(),
            }
        })?;
        let payload_len = packet_payload_len(&packet.sections)?;
        let payload_hash = packet_payload_hash_bytes(&packet.sections);

        let section_headers_len = usize::from(Self::SECTION_HEADER_LEN)
            .checked_mul(packet.sections.len())
            .ok_or(DxPacketCodecError::LengthOverflow)?;
        let mut out = Vec::with_capacity(
            usize::from(Self::HEADER_LEN)
                .saturating_add(section_headers_len)
                .saturating_add(payload_len as usize),
        );

        out.extend_from_slice(&DX_PACKET_MAGIC);
        put_u16(&mut out, DX_PACKET_VERSION);
        out.push(packet_kind_code(packet.header.kind));
        out.push(0);
        put_u32(&mut out, packet.header.flags);
        put_u16(&mut out, Self::HEADER_LEN);
        put_u16(&mut out, section_count);
        put_u32(&mut out, payload_len);
        out.extend_from_slice(&payload_hash);

        for section in &packet.sections {
            let expected_hash = section_hash_bytes(&section.bytes);
            if section.content_hash != hash_hex(expected_hash) {
                return Err(DxPacketCodecError::SectionHashMismatch {
                    expected: hash_hex(expected_hash),
                    found: section.content_hash.clone(),
                });
            }

            out.push(section_kind_code(section.kind));
            out.push(section_encoding_code(section.encoding));
            put_u16(&mut out, 0);
            put_u32(&mut out, section.bytes.len() as u32);
            out.extend_from_slice(&expected_hash);
            out.extend_from_slice(&section.bytes);
        }

        Ok(out)
    }

    /// Decode and validate a canonical browser packet.
    pub fn decode(bytes: &[u8]) -> Result<DxPacket, DxPacketCodecError> {
        let mut reader = PacketReader::new(bytes);
        let magic = reader.read_array::<4>()?;
        if magic != DX_PACKET_MAGIC {
            return Err(DxPacketCodecError::InvalidMagic {
                expected: DX_PACKET_MAGIC,
                found: magic,
            });
        }

        let version = reader.read_u16()?;
        if version != DX_PACKET_VERSION {
            return Err(DxPacketCodecError::UnsupportedVersion {
                supported: DX_PACKET_VERSION,
                found: version,
            });
        }

        let kind = packet_kind_from_code(reader.read_u8()?)?;
        let _reserved = reader.read_u8()?;
        let flags = reader.read_u32()?;
        let header_len = reader.read_u16()?;
        if header_len != Self::HEADER_LEN {
            return Err(DxPacketCodecError::InvalidHeaderLength {
                expected: Self::HEADER_LEN,
                found: header_len,
            });
        }

        let section_count = reader.read_u16()?;
        let declared_payload_len = reader.read_u32()?;
        let declared_payload_hash = reader.read_array::<32>()?;

        let mut sections = Vec::with_capacity(usize::from(section_count));
        let mut payload_hasher = blake3::Hasher::new();
        let mut actual_payload_len = 0u32;

        for _ in 0..section_count {
            let section_kind = section_kind_from_code(reader.read_u8()?)?;
            let encoding = section_encoding_from_code(reader.read_u8()?)?;
            let _reserved = reader.read_u16()?;
            let section_len = reader.read_u32()?;
            let declared_section_hash = reader.read_array::<32>()?;
            let section_bytes = reader.read_vec(section_len as usize)?;

            let actual_section_hash = section_hash_bytes(&section_bytes);
            if actual_section_hash != declared_section_hash {
                return Err(DxPacketCodecError::SectionHashMismatch {
                    expected: hash_hex(declared_section_hash),
                    found: hash_hex(actual_section_hash),
                });
            }

            payload_hasher.update(&section_bytes);
            actual_payload_len = actual_payload_len
                .checked_add(section_len)
                .ok_or(DxPacketCodecError::LengthOverflow)?;
            sections.push(DxPacketSection {
                kind: section_kind,
                encoding,
                bytes: section_bytes,
                content_hash: hash_hex(actual_section_hash),
            });
        }

        if actual_payload_len != declared_payload_len {
            return Err(DxPacketCodecError::PayloadLengthMismatch {
                expected: declared_payload_len,
                found: actual_payload_len,
            });
        }

        let actual_payload_hash = *payload_hasher.finalize().as_bytes();
        if actual_payload_hash != declared_payload_hash {
            return Err(DxPacketCodecError::PayloadHashMismatch {
                expected: hash_hex(declared_payload_hash),
                found: hash_hex(actual_payload_hash),
            });
        }

        if !reader.is_finished() {
            return Err(DxPacketCodecError::TrailingBytes {
                count: reader.remaining(),
            });
        }

        Ok(DxPacket {
            header: DxPacketHeader {
                magic,
                version,
                kind,
                flags,
                header_len,
                payload_len: actual_payload_len,
                payload_hash: hash_hex(actual_payload_hash),
            },
            sections,
        })
    }
}

/// Errors produced while encoding or decoding canonical packets.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DxPacketCodecError {
    /// Packet magic does not match the canonical envelope.
    #[error("invalid DX packet magic: expected {expected:?}, found {found:?}")]
    InvalidMagic {
        /// Expected magic bytes.
        expected: [u8; 4],
        /// Found magic bytes.
        found: [u8; 4],
    },
    /// Packet version is not supported by this codec.
    #[error("unsupported DX packet version: supported {supported}, found {found}")]
    UnsupportedVersion {
        /// Supported version.
        supported: u16,
        /// Found version.
        found: u16,
    },
    /// Header length does not match this codec.
    #[error("invalid DX packet header length: expected {expected}, found {found}")]
    InvalidHeaderLength {
        /// Expected fixed header length.
        expected: u16,
        /// Found header length.
        found: u16,
    },
    /// Packet has more sections than the v1 header can store.
    #[error("too many DX packet sections: {count}")]
    TooManySections {
        /// Section count.
        count: usize,
    },
    /// A declared payload length does not match decoded bytes.
    #[error("DX packet payload length mismatch: expected {expected}, found {found}")]
    PayloadLengthMismatch {
        /// Declared payload bytes.
        expected: u32,
        /// Decoded payload bytes.
        found: u32,
    },
    /// A declared payload hash does not match decoded bytes.
    #[error("DX packet payload hash mismatch")]
    PayloadHashMismatch {
        /// Declared payload hash.
        expected: String,
        /// Computed payload hash.
        found: String,
    },
    /// A section hash does not match decoded bytes.
    #[error("DX packet section hash mismatch")]
    SectionHashMismatch {
        /// Declared or expected section hash.
        expected: String,
        /// Found or computed section hash.
        found: String,
    },
    /// The packet ended before a complete field could be read.
    #[error("truncated DX packet: needed {needed} bytes at offset {offset}, total {total}")]
    Truncated {
        /// Required bytes for the read.
        needed: usize,
        /// Read offset.
        offset: usize,
        /// Total packet bytes.
        total: usize,
    },
    /// Unknown packet kind code.
    #[error("unknown DX packet kind code {code}")]
    UnknownPacketKind {
        /// Raw code.
        code: u8,
    },
    /// Unknown section kind code.
    #[error("unknown DX packet section kind code {code}")]
    UnknownSectionKind {
        /// Raw code.
        code: u8,
    },
    /// Unknown section encoding code.
    #[error("unknown DX packet section encoding code {code}")]
    UnknownSectionEncoding {
        /// Raw code.
        code: u8,
    },
    /// The packet had bytes after the last declared section.
    #[error("DX packet has {count} trailing bytes")]
    TrailingBytes {
        /// Number of trailing bytes.
        count: usize,
    },
    /// Length math overflowed.
    #[error("DX packet length overflow")]
    LengthOverflow,
}

fn packet_payload_len(sections: &[DxPacketSection]) -> Result<u32, DxPacketCodecError> {
    sections.iter().try_fold(0u32, |acc, section| {
        let len =
            u32::try_from(section.bytes.len()).map_err(|_| DxPacketCodecError::LengthOverflow)?;
        acc.checked_add(len)
            .ok_or(DxPacketCodecError::LengthOverflow)
    })
}

fn packet_payload_hash_bytes(sections: &[DxPacketSection]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    for section in sections {
        hasher.update(&section.bytes);
    }
    *hasher.finalize().as_bytes()
}

fn section_hash_bytes(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

fn hash_hex(hash: [u8; 32]) -> String {
    blake3::Hash::from_bytes(hash).to_hex().to_string()
}

fn put_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn packet_kind_code(kind: DxPacketKind) -> u8 {
    match kind {
        DxPacketKind::Route => 1,
        DxPacketKind::TemplateDictionary => 2,
        DxPacketKind::InstanceBatch => 3,
        DxPacketKind::PatchStream => 4,
        DxPacketKind::Style => 5,
        DxPacketKind::Manifest => 6,
    }
}

fn packet_kind_from_code(code: u8) -> Result<DxPacketKind, DxPacketCodecError> {
    match code {
        1 => Ok(DxPacketKind::Route),
        2 => Ok(DxPacketKind::TemplateDictionary),
        3 => Ok(DxPacketKind::InstanceBatch),
        4 => Ok(DxPacketKind::PatchStream),
        5 => Ok(DxPacketKind::Style),
        6 => Ok(DxPacketKind::Manifest),
        _ => Err(DxPacketCodecError::UnknownPacketKind { code }),
    }
}

fn section_kind_code(kind: DxPacketSectionKind) -> u8 {
    match kind {
        DxPacketSectionKind::FallbackHtmlRef => 1,
        DxPacketSectionKind::TemplateSlots => 2,
        DxPacketSectionKind::ColumnarSlots => 3,
        DxPacketSectionKind::SemanticCodec => 4,
        DxPacketSectionKind::PatchOps => 5,
        DxPacketSectionKind::StyleGraph => 6,
        DxPacketSectionKind::SourceManifest => 7,
        DxPacketSectionKind::StreamingPlan => 8,
    }
}

fn section_kind_from_code(code: u8) -> Result<DxPacketSectionKind, DxPacketCodecError> {
    match code {
        1 => Ok(DxPacketSectionKind::FallbackHtmlRef),
        2 => Ok(DxPacketSectionKind::TemplateSlots),
        3 => Ok(DxPacketSectionKind::ColumnarSlots),
        4 => Ok(DxPacketSectionKind::SemanticCodec),
        5 => Ok(DxPacketSectionKind::PatchOps),
        6 => Ok(DxPacketSectionKind::StyleGraph),
        7 => Ok(DxPacketSectionKind::SourceManifest),
        8 => Ok(DxPacketSectionKind::StreamingPlan),
        _ => Err(DxPacketCodecError::UnknownSectionKind { code }),
    }
}

fn section_encoding_code(encoding: DxPacketSectionEncoding) -> u8 {
    match encoding {
        DxPacketSectionEncoding::Json => 1,
        DxPacketSectionEncoding::HtipV2 => 2,
        DxPacketSectionEncoding::DeliveryLab => 3,
        DxPacketSectionEncoding::CanonicalBinary => 4,
    }
}

fn section_encoding_from_code(code: u8) -> Result<DxPacketSectionEncoding, DxPacketCodecError> {
    match code {
        1 => Ok(DxPacketSectionEncoding::Json),
        2 => Ok(DxPacketSectionEncoding::HtipV2),
        3 => Ok(DxPacketSectionEncoding::DeliveryLab),
        4 => Ok(DxPacketSectionEncoding::CanonicalBinary),
        _ => Err(DxPacketCodecError::UnknownSectionEncoding { code }),
    }
}

struct PacketReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> PacketReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], DxPacketCodecError> {
        let bytes = self.read_bytes(N)?;
        let mut out = [0u8; N];
        out.copy_from_slice(bytes);
        Ok(out)
    }

    fn read_u8(&mut self) -> Result<u8, DxPacketCodecError> {
        Ok(self.read_bytes(1)?[0])
    }

    fn read_u16(&mut self) -> Result<u16, DxPacketCodecError> {
        Ok(u16::from_le_bytes(self.read_array::<2>()?))
    }

    fn read_u32(&mut self) -> Result<u32, DxPacketCodecError> {
        Ok(u32::from_le_bytes(self.read_array::<4>()?))
    }

    fn read_vec(&mut self, len: usize) -> Result<Vec<u8>, DxPacketCodecError> {
        Ok(self.read_bytes(len)?.to_vec())
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], DxPacketCodecError> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or(DxPacketCodecError::LengthOverflow)?;
        if end > self.bytes.len() {
            return Err(DxPacketCodecError::Truncated {
                needed: len,
                offset: self.offset,
                total: self.bytes.len(),
            });
        }
        let bytes = &self.bytes[self.offset..end];
        self.offset = end;
        Ok(bytes)
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }
}
