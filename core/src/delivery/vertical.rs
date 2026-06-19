use serde::{Deserialize, Serialize};

use crate::binary_compiler::{
    BinaryCompiler, BindingType, CompilerError, CompilerResult, ComponentType, DxObjectBinary,
    SlotType, deserialize_dxob, serialize_dxob,
};
use crate::dx_parser::{BlockType, parse_dx_file};

use super::contract::{
    DxPacket, DxPacketCodec, DxPacketKind, DxPacketSection, DxPacketSectionEncoding,
    DxPacketSectionKind,
};
use super::html::{DxOptimizedHtml, optimize_generated_html};
use super::vertical_interaction::{DxVerticalInteractionProof, compile_vertical_interaction};
use super::vertical_render::{DxVerticalAstComponent, render_fallback_document};

/// Component source used by the Phase 1 vertical-slice proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxVerticalComponentSource {
    /// Component name without the `.tsx` extension.
    pub name: String,
    /// Raw `.tsx` source.
    pub source: String,
}

/// Source inputs for one route-level DX-WWW proof build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxVerticalSliceInput {
    /// Route path, such as `/` or `/pricing`.
    pub route: String,
    /// Raw `.html` page source.
    pub page_source: String,
    /// Source-owned component files available to the page.
    pub components: Vec<DxVerticalComponentSource>,
}

/// Compiled proof for a `.html` route plus its source-owned components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalSliceProof {
    /// Route path that was compiled.
    pub route: String,
    /// Compiled page object.
    pub page: DxObjectBinary,
    /// Compiled component objects.
    pub components: Vec<DxObjectBinary>,
    /// Crawlable fallback HTML generated from the compiled template output.
    pub fallback: DxOptimizedHtml,
    /// Compiler-derived tiny runtime proof when the route has supported interaction.
    pub interaction: Option<DxVerticalInteractionProof>,
    /// Binary compiler packet proof decoded back into a logical page model.
    pub packet: DxVerticalPacketProof,
    /// Canonical browser-delivered packet proof decoded through the `DXPK` envelope.
    pub browser_packet: DxVerticalBrowserPacketProof,
    /// Component tags referenced by the page that were not supplied.
    pub missing_components: Vec<String>,
}

/// Decoded binary packet proof for the page object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalPacketProof {
    /// Packet format label.
    pub format: String,
    /// Raw encoded packet bytes.
    pub bytes: usize,
    /// Decoded page/component name.
    pub decoded_name: String,
    /// Decoded component type.
    pub decoded_component_type: ComponentType,
    /// Decoded template tree summary.
    pub decoded_templates: Vec<DxVerticalDecodedTemplate>,
    /// Decoded literal string table entries.
    pub decoded_strings: Vec<String>,
    /// Decoded binding summary.
    pub decoded_bindings: Vec<DxVerticalDecodedBinding>,
    /// Number of decoded event handlers.
    pub decoded_event_count: usize,
    /// CSS classes decoded from the packet.
    pub decoded_css_classes: Vec<String>,
    /// Whether the serialized fields round-trip back to the compiled object.
    pub roundtrip_matches: bool,
}

/// Decoded canonical browser packet proof for the route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalBrowserPacketProof {
    /// Packet format label.
    pub format: String,
    /// Encoded `DXPK` bytes.
    pub bytes: usize,
    /// Decoded packet kind.
    pub decoded_kind: DxPacketKind,
    /// Number of decoded sections.
    pub section_count: usize,
    /// Decoded payload bytes.
    pub payload_bytes: u32,
    /// Decoded section summaries.
    pub decoded_sections: Vec<DxVerticalBrowserPacketSectionProof>,
    /// Whether the canonical packet decoded back to the emitted packet.
    pub roundtrip_matches: bool,
    /// Raw encoded packet used by CLI write paths.
    #[serde(skip, default)]
    pub encoded: Vec<u8>,
}

/// Decoded canonical browser packet section summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalBrowserPacketSectionProof {
    /// Section kind.
    pub kind: DxPacketSectionKind,
    /// Section encoding.
    pub encoding: DxPacketSectionEncoding,
    /// Raw section byte length.
    pub bytes: usize,
    /// BLAKE3 content hash.
    pub content_hash: String,
}

/// Decoded template and slot tree from the emitted binary packet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalDecodedTemplate {
    /// Template id.
    pub id: u16,
    /// Static HTML with dynamic slot placeholders.
    pub html: String,
    /// Slots belonging to this template.
    pub slots: Vec<DxVerticalDecodedSlot>,
}

/// Decoded slot metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalDecodedSlot {
    /// Slot id.
    pub id: u16,
    /// Slot kind.
    pub slot_type: SlotType,
    /// Logical template path.
    pub path: String,
}

/// Decoded binding metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxVerticalDecodedBinding {
    /// Binding id.
    pub id: u32,
    /// Source expression.
    pub expression: String,
    /// Template id targeted by the binding.
    pub template_id: u16,
    /// Slot id targeted by the binding.
    pub slot_id: u16,
    /// Binding kind.
    pub binding_type: BindingType,
    /// State or prop dependencies used by the binding.
    pub dependencies: Vec<String>,
}

/// Compile one `.html` page plus local `.tsx` component sources into a route proof.
///
/// This is intentionally narrow. It gives Phase 1 a real product-facing entry
/// point while leaving full routing, Forge receipts, hydration, and packet
/// serving as explicit follow-up work.
pub fn compile_vertical_slice(input: DxVerticalSliceInput) -> CompilerResult<DxVerticalSliceProof> {
    let page_ast = parse_dx_file(&input.page_source, Some(BlockType::Page))
        .map_err(|error| CompilerError::Parse(error.to_string()))?;
    let component_asts = parse_components(&input.components)?;
    let components = compile_components(&input.components)?;
    let mut page_compiler = BinaryCompiler::new();
    let page = page_compiler.compile_source(
        &input.page_source,
        route_name(&input.route),
        Some(BlockType::Page),
    )?;
    let missing_components = missing_component_tags(&page, &components);
    let interaction = compile_vertical_interaction(&page_ast, &component_asts);
    let fallback = optimize_generated_html(&render_fallback_document(
        &input.route,
        &page_ast,
        &component_asts,
        interaction.as_ref().map(|output| output.runtime.clone()),
    ));
    let packet = compile_vertical_packet_proof(&page)?;
    let browser_packet = compile_vertical_browser_packet_proof(&input.route, &page, &fallback)?;

    Ok(DxVerticalSliceProof {
        route: input.route,
        page,
        components,
        fallback,
        interaction: interaction.map(|output| output.proof),
        packet,
        browser_packet,
        missing_components,
    })
}

fn compile_vertical_packet_proof(page: &DxObjectBinary) -> CompilerResult<DxVerticalPacketProof> {
    let encoded = serialize_dxob(page);
    let decoded = deserialize_dxob(&encoded)?;

    Ok(DxVerticalPacketProof {
        format: "dxob-v1".to_string(),
        bytes: encoded.len(),
        decoded_name: decoded.name.clone(),
        decoded_component_type: decoded.component_type,
        decoded_templates: decoded
            .templates
            .iter()
            .map(|template| DxVerticalDecodedTemplate {
                id: template.id,
                html: template.html.clone(),
                slots: template
                    .slots
                    .iter()
                    .map(|slot| DxVerticalDecodedSlot {
                        id: slot.id,
                        slot_type: slot.slot_type,
                        path: slot.path.clone(),
                    })
                    .collect(),
            })
            .collect(),
        decoded_strings: decoded.strings.clone(),
        decoded_bindings: decoded
            .bindings
            .iter()
            .map(|binding| DxVerticalDecodedBinding {
                id: binding.id,
                expression: binding.expression.clone(),
                template_id: binding.template_id,
                slot_id: binding.slot_id,
                binding_type: binding.binding_type,
                dependencies: binding.dependencies.clone(),
            })
            .collect(),
        decoded_event_count: decoded.events.len(),
        decoded_css_classes: decoded.css_classes.clone(),
        roundtrip_matches: packet_roundtrip_matches(page, &decoded),
    })
}

fn compile_vertical_browser_packet_proof(
    route: &str,
    page: &DxObjectBinary,
    fallback: &DxOptimizedHtml,
) -> CompilerResult<DxVerticalBrowserPacketProof> {
    let fallback_bytes = fallback.html().as_bytes();
    let fallback_hash = blake3::hash(fallback_bytes).to_hex().to_string();
    let fallback_profile = fallback.profile();
    let fallback_ref = serde_json::json!({
        "route": route,
        "html_hash": fallback_hash,
        "html_bytes": fallback_bytes.len(),
        "delivery_mode": fallback_profile.delivery_mode.as_str(),
        "crawlable": true,
    });
    let route_manifest = serde_json::json!({
        "route": route,
        "name": page.name,
        "component_type": format!("{:?}", page.component_type),
        "string_count": page.strings.len(),
        "binding_count": page.bindings.len(),
        "event_count": page.events.len(),
        "css_class_count": page.css_classes.len(),
    });
    let sections = vec![
        DxPacketSection::new(
            DxPacketSectionKind::FallbackHtmlRef,
            DxPacketSectionEncoding::Json,
            serde_json::to_vec(&fallback_ref)
                .map_err(|error| CompilerError::Parse(error.to_string()))?,
        ),
        DxPacketSection::new(
            DxPacketSectionKind::TemplateSlots,
            DxPacketSectionEncoding::CanonicalBinary,
            encode_vertical_template_slots(page)?,
        ),
        DxPacketSection::new(
            DxPacketSectionKind::SourceManifest,
            DxPacketSectionEncoding::Json,
            serde_json::to_vec(&route_manifest)
                .map_err(|error| CompilerError::Parse(error.to_string()))?,
        ),
    ];
    let packet = DxPacket::new(DxPacketKind::Route, sections);
    let encoded =
        DxPacketCodec::encode(&packet).map_err(|error| CompilerError::Parse(error.to_string()))?;
    let decoded =
        DxPacketCodec::decode(&encoded).map_err(|error| CompilerError::Parse(error.to_string()))?;
    let decoded_sections = decoded
        .sections
        .iter()
        .map(|section| DxVerticalBrowserPacketSectionProof {
            kind: section.kind,
            encoding: section.encoding,
            bytes: section.bytes.len(),
            content_hash: section.content_hash.clone(),
        })
        .collect::<Vec<_>>();

    Ok(DxVerticalBrowserPacketProof {
        format: "dxp-v1".to_string(),
        bytes: encoded.len(),
        decoded_kind: decoded.header.kind,
        section_count: decoded.sections.len(),
        payload_bytes: decoded.header.payload_len,
        decoded_sections,
        roundtrip_matches: decoded == packet,
        encoded,
    })
}

fn encode_vertical_template_slots(page: &DxObjectBinary) -> CompilerResult<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(b"DXVT1");
    put_varint(&mut out, page.templates.len() as u64);
    for template in &page.templates {
        put_varint(&mut out, template.id as u64);
        put_bytes(&mut out, template.html.as_bytes());
        put_varint(&mut out, template.slots.len() as u64);
        for slot in &template.slots {
            put_varint(&mut out, slot.id as u64);
            out.push(vertical_slot_kind_code(slot.slot_type));
            put_bytes(&mut out, slot.path.as_bytes());
        }
    }
    Ok(out)
}

fn vertical_slot_kind_code(slot_type: SlotType) -> u8 {
    match slot_type {
        SlotType::Text => 1,
        SlotType::Attribute => 2,
        SlotType::Element => 3,
        SlotType::Children => 4,
    }
}

fn put_varint(out: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        out.push(((value as u8) & 0x7f) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

fn put_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    put_varint(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

fn compile_components(
    sources: &[DxVerticalComponentSource],
) -> CompilerResult<Vec<DxObjectBinary>> {
    let mut compiled = Vec::with_capacity(sources.len());

    for component in sources {
        let mut compiler = BinaryCompiler::new();
        compiled.push(compiler.compile_source(
            &component.source,
            &component.name,
            Some(BlockType::Component),
        )?);
    }

    Ok(compiled)
}

fn parse_components(
    sources: &[DxVerticalComponentSource],
) -> CompilerResult<Vec<DxVerticalAstComponent>> {
    sources
        .iter()
        .map(|component| {
            let file = parse_dx_file(&component.source, Some(BlockType::Component))
                .map_err(|error| CompilerError::Parse(error.to_string()))?;
            Ok(DxVerticalAstComponent {
                name: component.name.clone(),
                file,
            })
        })
        .collect()
}

fn route_name(route: &str) -> &str {
    if route == "/" {
        "index"
    } else {
        route
            .trim_matches('/')
            .rsplit('/')
            .next()
            .filter(|name| !name.is_empty())
            .unwrap_or("route")
    }
}

fn missing_component_tags(page: &DxObjectBinary, components: &[DxObjectBinary]) -> Vec<String> {
    let mut missing = Vec::new();

    for name in referenced_component_names(page) {
        if !components.iter().any(|component| component.name == name) && !missing.contains(&name) {
            missing.push(name);
        }
    }

    missing
}

fn packet_roundtrip_matches(expected: &DxObjectBinary, decoded: &DxObjectBinary) -> bool {
    expected.name == decoded.name
        && expected.component_type == decoded.component_type
        && expected.strings == decoded.strings
        && templates_match(expected, decoded)
        && bindings_match(expected, decoded)
        && events_match(expected, decoded)
        && expected.css_classes == decoded.css_classes
}

fn templates_match(expected: &DxObjectBinary, decoded: &DxObjectBinary) -> bool {
    expected.templates.len() == decoded.templates.len()
        && expected
            .templates
            .iter()
            .zip(&decoded.templates)
            .all(|(expected, decoded)| {
                expected.id == decoded.id
                    && expected.html == decoded.html
                    && expected.slots.len() == decoded.slots.len()
                    && expected
                        .slots
                        .iter()
                        .zip(&decoded.slots)
                        .all(|(expected, decoded)| {
                            expected.id == decoded.id
                                && expected.slot_type == decoded.slot_type
                                && expected.path == decoded.path
                        })
            })
}

fn bindings_match(expected: &DxObjectBinary, decoded: &DxObjectBinary) -> bool {
    expected.bindings.len() == decoded.bindings.len()
        && expected
            .bindings
            .iter()
            .zip(&decoded.bindings)
            .all(|(expected, decoded)| {
                expected.id == decoded.id
                    && expected.expression == decoded.expression
                    && expected.template_id == decoded.template_id
                    && expected.slot_id == decoded.slot_id
                    && expected.binding_type == decoded.binding_type
                    && expected.dependencies == decoded.dependencies
            })
}

fn events_match(expected: &DxObjectBinary, decoded: &DxObjectBinary) -> bool {
    expected.events.len() == decoded.events.len()
        && expected
            .events
            .iter()
            .zip(&decoded.events)
            .all(|(expected, decoded)| {
                expected.id == decoded.id
                    && expected.event_type == decoded.event_type
                    && expected.handler == decoded.handler
            })
}

fn referenced_component_names(page: &DxObjectBinary) -> Vec<String> {
    let mut names = Vec::new();

    for template in &page.templates {
        let mut rest = template.html.as_str();
        while let Some(index) = rest.find("data-component=\"") {
            let value_start = index + "data-component=\"".len();
            let Some(value_end) = rest[value_start..].find('"') else {
                break;
            };
            let name = rest[value_start..value_start + value_end].to_string();
            if !names.contains(&name) {
                names.push(name);
            }
            rest = &rest[value_start + value_end..];
        }
    }

    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::delivery::DxDeliveryMode;

    const BUTTON_CP: &str = r#"
<script lang="rust">
struct Props {
    label: String,
}
</script>

<component>
    <button class="inline-flex items-center rounded-md px-3 py-2 text-sm font-medium" on:click={increment}>
        {label}
    </button>
</component>
"#;

    const INDEX_PG: &str = r#"
<script lang="rust">
let count = 0;

fn increment() {
    count += 1;
}
</script>

<page>
    <main class="mx-auto max-w-2xl p-6">
        <h1 class="text-3xl font-semibold">DX-WWW vertical slice</h1>
        <p class="mt-2 text-sm text-muted-foreground">Count: {count}</p>
        <Button label="Increase" />
    </main>
</page>
"#;

    #[test]
    fn compiles_page_component_and_fallback_html() {
        let proof = compile_vertical_slice(DxVerticalSliceInput {
            route: "/".to_string(),
            page_source: INDEX_PG.to_string(),
            components: vec![DxVerticalComponentSource {
                name: "Button".to_string(),
                source: BUTTON_CP.to_string(),
            }],
        })
        .expect("vertical slice compiles");

        assert_eq!(proof.page.name, "index");
        assert_eq!(proof.components.len(), 1);
        assert!(proof.missing_components.is_empty());
        assert!(proof.fallback.html().starts_with("<!doctype html>"));
        assert!(proof.fallback.html().contains("DX-WWW vertical slice"));
        assert!(proof.fallback.html().contains("<button"));
        assert!(proof.fallback.html().contains("Increase"));
        assert!(!proof.fallback.html().contains("data-dx-fallback-text"));
        assert!(proof.interaction.is_some());
        assert_eq!(proof.packet.format, "dxob-v1");
        assert!(proof.packet.bytes > 0);
        assert!(proof.packet.roundtrip_matches);
        assert_eq!(proof.browser_packet.format, "dxp-v1");
        assert!(proof.browser_packet.bytes > 0);
        assert_eq!(proof.browser_packet.decoded_kind, DxPacketKind::Route);
        assert_eq!(proof.browser_packet.section_count, 3);
        assert!(proof.browser_packet.roundtrip_matches);
        assert!(proof.browser_packet.encoded.starts_with(b"DXPK"));
        assert_eq!(proof.packet.decoded_name, "index");
        assert!(!proof.packet.decoded_templates.is_empty());
        assert!(
            proof
                .packet
                .decoded_strings
                .iter()
                .any(|value| value.contains("DX-WWW vertical slice"))
        );
        assert_eq!(
            proof.fallback.profile().delivery_mode,
            DxDeliveryMode::MicroJs
        );
    }

    #[test]
    fn reports_missing_source_owned_components() {
        let proof = compile_vertical_slice(DxVerticalSliceInput {
            route: "/".to_string(),
            page_source: INDEX_PG.to_string(),
            components: Vec::new(),
        })
        .expect("page still compiles");

        assert_eq!(proof.missing_components, vec!["Button".to_string()]);
        assert!(proof.fallback.html().contains("data-component=\"Button\""));
    }

    #[test]
    fn renders_nested_source_owned_component_tree() {
        const CARD_CP: &str = r#"
<component>
    <article class="rounded-md border p-4">
        <h2>{title}</h2>
        <slot />
    </article>
</component>
"#;

        const PAGE_PG: &str = r#"
<page>
    <main>
        <Card title="Launch proof">
            <p>Nested content is source-owned.</p>
        </Card>
    </main>
</page>
"#;

        let proof = compile_vertical_slice(DxVerticalSliceInput {
            route: "/launch".to_string(),
            page_source: PAGE_PG.to_string(),
            components: vec![DxVerticalComponentSource {
                name: "Card".to_string(),
                source: CARD_CP.to_string(),
            }],
        })
        .expect("nested vertical slice compiles");

        let html = proof.fallback.html();
        assert!(html.contains("<main><article"));
        assert!(html.contains("<h2>Launch proof</h2>"));
        assert!(html.contains("<p>Nested content is source-owned.</p>"));
        assert!(proof.missing_components.is_empty());
    }

    #[test]
    fn proves_micro_js_counter_interaction_from_source() {
        const PAGE_PG: &str = r#"
<script lang="rust">
let mut count = 0;

fn increment() {
    count += 1;
}

fn decrement() {
    count -= 1;
}

fn reset() {
    count = 0;
}
</script>

<page>
    <main>
        <p>Count: {count}</p>
        <button on:click={decrement}>Decrease</button>
        <button on:click={increment}>Increase</button>
        <button on:click={reset}>Reset</button>
    </main>
</page>
"#;

        let proof = compile_vertical_slice(DxVerticalSliceInput {
            route: "/counter".to_string(),
            page_source: PAGE_PG.to_string(),
            components: Vec::new(),
        })
        .expect("counter vertical slice compiles");

        let interaction = proof.interaction.expect("micro interaction proof");
        let html = proof.fallback.html();
        assert_eq!(interaction.delivery_mode, DxDeliveryMode::MicroJs);
        assert_eq!(interaction.state_name, "count");
        assert_eq!(interaction.program.actions.len(), 3);
        assert!(html.contains(r#"id="dx-state-count">0</span>"#));
        assert!(html.contains("dx-action-increment"));
        assert!(html.contains("<script>"));
        assert!(html.contains("textContent=c"));
    }
}
