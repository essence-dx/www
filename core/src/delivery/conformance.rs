use serde::{Deserialize, Serialize};

use super::app_route::{
    DxReactAppRouteInput, DxReactComponentSource, DxReactStyleSource, compile_react_app_route,
};
use super::tsx_ast::parse_tsx_module;
use super::types::DxDeliveryMode;

/// Summary for one strict Next App Router conformance fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxNextConformanceFixture {
    /// Stable fixture name.
    pub name: String,
    /// Route pattern or concrete route path.
    pub route: String,
    /// Selected DX-WWW delivery mode.
    pub delivery_mode: DxDeliveryMode,
    /// Whether the fixture requires node_modules at runtime.
    pub node_modules_required: bool,
    /// State slots compiled for this fixture.
    pub state_slots: usize,
    /// Event slots compiled for this fixture.
    pub event_slots: usize,
    /// Server action edges compiled for this fixture.
    pub server_action_edges: usize,
    /// Metadata objects extracted from the route source.
    pub metadata_entries: usize,
    /// Fallback HTML bytes.
    pub fallback_bytes: usize,
    /// DXPK packet bytes.
    pub packet_bytes: usize,
}

/// Fixture suite proving the route compiler can compare against common App Router shapes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxNextConformanceReport {
    /// Fixture suite version.
    pub version: u32,
    /// Whether every fixture is strict no-node_modules.
    pub strict_no_node_modules: bool,
    /// Fixture summaries.
    pub fixtures: Vec<DxNextConformanceFixture>,
}

/// Build the deterministic strict Next App Router conformance fixture report.
pub fn next_app_router_conformance_report() -> DxNextConformanceReport {
    let fixtures = next_conformance_inputs()
        .into_iter()
        .map(compile_fixture)
        .collect::<Vec<_>>();
    let strict_no_node_modules = fixtures
        .iter()
        .all(|fixture| !fixture.node_modules_required);

    DxNextConformanceReport {
        version: 1,
        strict_no_node_modules,
        fixtures,
    }
}

fn compile_fixture(
    (name, input): (&'static str, DxReactAppRouteInput),
) -> DxNextConformanceFixture {
    let metadata_entries = metadata_entries(&input);
    let proof = compile_react_app_route(input).expect("compile conformance fixture");
    DxNextConformanceFixture {
        name: name.to_string(),
        route: proof.route.clone(),
        delivery_mode: proof.delivery_mode,
        node_modules_required: proof.route_unit.receipt.node_modules_required,
        state_slots: proof.route_unit.state.slots.len(),
        event_slots: proof.route_unit.state.event_slots.len(),
        server_action_edges: proof.route_unit.state.server_actions.len(),
        metadata_entries,
        fallback_bytes: proof.fallback.bytes,
        packet_bytes: proof.packet.bytes,
    }
}

fn metadata_entries(input: &DxReactAppRouteInput) -> usize {
    let route = usize::from(
        parse_tsx_module(&input.route_source_path, &input.route_source)
            .metadata
            .is_some(),
    );
    route
        + input
            .segments
            .iter()
            .filter(|segment| {
                parse_tsx_module(&segment.source_path, &segment.source)
                    .metadata
                    .is_some()
            })
            .count()
}

fn next_conformance_inputs() -> Vec<(&'static str, DxReactAppRouteInput)> {
    vec![
        (
            "static-page",
            fixture_input(
                "/",
                "app/page.tsx",
                "export default function Page() { return <main><h1>Home</h1></main>; }\n",
                Vec::new(),
            ),
        ),
        (
            "client-island",
            fixture_input(
                "/counter",
                "app/counter/page.tsx",
                "import { Counter } from '../../components/Counter';\nexport default function Page() { return <Counter />; }\n",
                vec![counter_component()],
            ),
        ),
        (
            "form-server-action",
            fixture_input(
                "/contact",
                "app/contact/page.tsx",
                "import { submitContact } from '../../server/actions';\nexport default function Page() { return <form action={submitContact}><button type=\"submit\">Send</button></form>; }\n",
                Vec::new(),
            ),
        ),
        (
            "dynamic-route",
            fixture_input(
                "/blog/[slug]",
                "app/blog/[slug]/page.tsx",
                "export default function Page({ params }) { return <article><h1>{params.slug}</h1></article>; }\n",
                Vec::new(),
            ),
        ),
        (
            "metadata-image-font",
            fixture_input(
                "/brand",
                "app/brand/page.tsx",
                "export const metadata = { title: \"Brand\", description: \"DX-WWW\", alternates: { canonical: \"/brand\" } };\nexport default function Page() { return <main><img src=\"/public/d-logo.svg\" alt=\"D\" /><h1>Brand</h1></main>; }\n",
                Vec::new(),
            ),
        ),
    ]
}

fn fixture_input(
    route: &str,
    route_source_path: &str,
    route_source: &str,
    components: Vec<DxReactComponentSource>,
) -> DxReactAppRouteInput {
    DxReactAppRouteInput {
        route: route.to_string(),
        route_source_path: route_source_path.to_string(),
        route_source: route_source.to_string(),
        segments: Vec::new(),
        components,
        styles: vec![DxReactStyleSource {
            source_path: "styles/tokens.css".to_string(),
            source: ":root { --dx-bg: #000; --dx-fg: #fff; }\n".to_string(),
        }],
        source_manifest_hash: Some("conformance-fixture".to_string()),
    }
}

fn counter_component() -> DxReactComponentSource {
    DxReactComponentSource {
        name: "Counter".to_string(),
        source_path: "components/Counter.tsx".to_string(),
        source: r#""use client";
import { useState } from "react";

export function Counter() {
  const [count, setCount] = useState(0);
  return <button type="button" onClick={() => setCount(count + 1)}>{count}</button>;
}
"#
        .to_string(),
        package_id: None,
    }
}
