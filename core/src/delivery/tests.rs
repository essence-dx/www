use super::*;
use std::collections::BTreeMap;

use crate::analyzer::{ComplexityMetrics, RuntimeVariant, StateComplexity};
use crate::splitter::{SlotDef, SlotType, Template};

fn template() -> Template {
    Template {
        id: 1,
        html: "<tr><td><!--SLOT_0--></td><td><!--SLOT_1--></td></tr>".to_string(),
        slots: vec![
            SlotDef {
                slot_id: 0,
                slot_type: SlotType::Text,
                path: vec![0, 0],
            },
            SlotDef {
                slot_id: 1,
                slot_type: SlotType::Text,
                path: vec![0, 1],
            },
        ],
        hash: "rowhash".to_string(),
    }
}

fn metrics() -> ComplexityMetrics {
    ComplexityMetrics {
        component_count: 1,
        total_state_vars: 1,
        total_props: 0,
        total_hooks: 1,
        event_handler_count: 1,
        max_component_depth: 1,
        has_async_logic: false,
        has_effects: false,
        total_jsx_nodes: 8,
        state_complexity: StateComplexity::Low,
    }
}

fn runtime_env(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(name, value)| ((*name).to_string(), (*value).to_string()))
        .collect()
}

#[test]
fn route_handler_endpoint_accepts_next_familiar_route_extensions() {
    for (source_path, route_path) in [
        ("app/api/status/route.tsx", "/api/status"),
        ("src/app/api/status/route.ts", "/api/status"),
        ("app/api/(internal)/status/route.tsx", "/api/status"),
        ("app/api/@admin/status/route.jsx", "/api/status"),
        ("app/api/widgets/[id]/route.js", "/api/widgets/42"),
        (
            "app/api/files/[...path]/route.jsx",
            "/api/files/reports/may",
        ),
    ] {
        let source = DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: source_path.to_string(),
            source: r#"export function GET(request) {
  return Response.json({
    ok: true,
    method: request.method,
    path: request.path,
  }, { status: 200 });
}
"#
            .to_string(),
        };

        let contract = compile_react_server_contracts(std::slice::from_ref(&source));
        assert_eq!(contract[0].endpoint.as_deref(), Some(route_path));

        let response = execute_react_route_handler(
            &source,
            DxReactRouteHandlerRequest {
                method: "GET".to_string(),
                path: route_path.to_string(),
                headers: BTreeMap::new(),
                body: serde_json::Value::Null,
                route_params: BTreeMap::new(),
                search_params: BTreeMap::new(),
                runtime_env: BTreeMap::new(),
            },
        )
        .expect("execute Next-familiar route handler extension");

        assert_eq!(response.status, 200);
        assert_eq!(response.body["path"], route_path);
        assert_eq!(response.execution_model, "source-owned-safe-interpreter");
        assert!(!response.lifecycle_scripts_executed);
    }
}

#[test]
fn route_handler_endpoint_match_ignores_search_and_fragment_without_mutating_request_path() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/status/route.tsx".to_string(),
        source: r#"export function GET(request) {
  return Response.json({
    ok: true,
    method: request.method,
    path: request.path,
  }, { status: 200 });
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/status?preview=1#details".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([("preview".to_string(), "1".to_string())]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("match route handler when request path carries query and fragment");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["path"], "/api/status?preview=1#details");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn route_handler_endpoint_match_accepts_absolute_request_url_without_mutating_request_path() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/status/route.tsx".to_string(),
        source: r#"export function GET(request) {
  return Response.json({
    ok: true,
    method: request.method,
    path: request.path,
  }, { status: 200 });
}
"#
        .to_string(),
    };

    let request_url = "https://example.test/api/status?preview=1#details";
    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: request_url.to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([("preview".to_string(), "1".to_string())]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("match route handler when request path is an absolute URL");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["path"], request_url);
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn route_handler_helper_interpreter_accepts_absolute_request_url_for_database_readiness() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/database-api/readiness/route.ts".to_string(),
        source:
            r#"import { createDatabaseApiReadinessResponse } from "@/server/database-api/readiness";

export function GET() {
  return createDatabaseApiReadinessResponse();
}
"#
            .to_string(),
    };

    let request_url = "https://example.test/api/database-api/readiness?source=dx#ready";
    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: request_url.to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([("source".to_string(), "dx".to_string())]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("source-owned helper interpreter accepts absolute request URL");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-database-api-readiness-interpreter"
    );
    assert_eq!(response.body["route"], "/api/database-api/readiness");
    assert_eq!(
        response
            .headers
            .get("x-dx-database-api-readiness")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
}

#[test]
fn react_route_handler_runtime_accepts_http_json_route_policy() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/v1/translate/route.ts".to_string(),
        source: r#"import { createDxHttpJsonRouteResponse } from "@/lib/server/http-json-route";

export async function GET(request: Request) {
  return createDxHttpJsonRouteResponse(request, {
    target: "https://translate.googleapis.com/translate_a/single",
    method: "GET",
    allowedOrigins: ["https://translate.googleapis.com"],
    requiredSearchParams: ["q", "tl"],
    cache: "no-store",
  });
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/v1/translate?q=tenali+rama&tl=en".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("execute HTTP JSON route policy");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-http-json-route-policy-interpreter"
    );
    assert_eq!(response.body["schema"], "dx.www.http_json_route_policy");
    assert_eq!(response.body["route"], "/api/v1/translate");
    assert_eq!(
        response.body["targetOrigin"],
        "https://translate.googleapis.com"
    );
    assert_eq!(response.body["upstreamRequestAllowed"], true);
    assert_eq!(response.body["proxyRequestForwarded"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["requiredSearchParams"][0], "q");
    assert_eq!(response.body["requiredSearchParams"][1], "tl");
    assert_eq!(response.body["searchParams"]["q"], "tenali rama");
    assert_eq!(
        response
            .headers
            .get("x-dx-http-json-route")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
}

#[test]
fn react_route_handler_runtime_accepts_http_json_route_policy_from_runtime_env() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/v1/translate/route.ts".to_string(),
        source: r#"import { createDxHttpJsonRouteResponse } from "@/lib/server/http-json-route";

export async function GET(request: Request) {
  return createDxHttpJsonRouteResponse(request, {
    target: "https://translate.googleapis.com/translate_a/single",
    method: "GET",
    requiredSearchParams: ["q", "tl"],
    cache: "no-store",
  });
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/v1/translate?q=tenali+rama&tl=en".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[(
                "DX_HTTP_JSON_ALLOWED_ORIGINS",
                "https://translate.googleapis.com",
            )]),
        },
    )
    .expect("execute HTTP JSON route policy with env allowlist");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["allowedOriginsConfigured"], true);
    assert_eq!(
        response.body["configuredAllowedOrigins"][0],
        "https://translate.googleapis.com"
    );
    assert_eq!(response.body["upstreamRequestAllowed"], true);
    assert_eq!(response.body["proxyRequestForwarded"], false);
}

#[test]
fn react_route_handler_runtime_rejects_http_json_route_missing_query() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/v1/translate/route.ts".to_string(),
        source: r#"import { createDxHttpJsonRouteResponse } from "@/lib/server/http-json-route";

export function GET(request: Request) {
  return createDxHttpJsonRouteResponse(request, {
    target: "https://translate.googleapis.com/translate_a/single",
    method: "GET",
    allowedOrigins: ["https://translate.googleapis.com"],
    requiredSearchParams: ["q", "tl"],
  });
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/v1/translate?q=tenali+rama".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("execute HTTP JSON route policy with missing query");

    assert_eq!(response.status, 400);
    assert_eq!(response.body["ok"], false);
    assert_eq!(
        response.body["policyStatus"],
        "missing-required-search-params"
    );
    assert_eq!(response.body["missingSearchParams"][0], "tl");
    assert_eq!(response.body["proxyRequestForwarded"], false);
}

#[test]
fn react_route_handler_runtime_accepts_http_json_factory_export() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/v1/translate/route.ts".to_string(),
        source: r#"import { createDxHttpJsonRoute } from "@/lib/server/http-json-route";

export const GET = createDxHttpJsonRoute({
  target: "https://translate.googleapis.com/translate_a/single",
  method: "GET",
  allowedOrigins: ["https://translate.googleapis.com"],
  requiredSearchParams: ["q", "tl"],
  cache: "no-store",
});
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/v1/translate?q=tenali+rama&tl=en".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("execute HTTP JSON factory export route policy");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["route"], "/api/v1/translate");
    assert_eq!(response.body["recognizedFactory"], "createDxHttpJsonRoute");
    assert_eq!(response.body["upstreamRequestAllowed"], true);
    assert_eq!(response.body["proxyRequestForwarded"], false);
}

#[test]
fn react_app_route_compiles_starter_into_page_graph_fallback_and_dxpk() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/".to_string(),
        route_source_path: "app/page.tsx".to_string(),
        route_source: r#"import { WelcomeCard } from "../components/local/WelcomeCard";
import { recordWelcomeView } from "../server/actions";
import { loadHomeMetrics } from "../server/loaders";

export default async function Page() {
  const metrics = await loadHomeMetrics();

  return (
    <main className="dx-shell">
      <WelcomeCard metrics={metrics} onView={recordWelcomeView} />
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "WelcomeCard".to_string(),
            source_path: "components/local/WelcomeCard.tsx".to_string(),
            source: r#""use client";

import { useState } from "react";

export function WelcomeCard({ metrics, onView }: WelcomeCardProps) {
  const [viewCount, setViewCount] = useState(0);
  return (
    <section className="dx-card">
      <p className="dx-eyebrow">DX-WWW starter</p>
      <h1>React-shaped source, Forge-owned packages, zero node_modules by default.</h1>
      <p>Start with familiar component files while DX-WWW keeps packages visible.</p>
      <button className="dx-action" type="button" onClick={() => setViewCount(viewCount + 1)}>
        Record server action
      </button>
      <p className="dx-count">Local interactions: {viewCount}</p>
    </section>
  );
}
"#
            .to_string(),
            package_id: None,
        }],
        styles: vec![DxReactStyleSource {
            source_path: "styles/tokens.css".to_string(),
            source:
                ":root { --dx-bg: #f8fafc; --dx-fg: #0f172a; }\n.dx-shell { min-height: 100vh; }\n"
                    .to_string(),
        }],
        source_manifest_hash: None,
    })
    .expect("compile app route");

    assert_eq!(proof.page_graph.route_id, "/");
    assert_eq!(
        proof.page_graph.source_path.as_deref(),
        Some("app/page.tsx")
    );
    assert_eq!(proof.page_graph.root_component_id, "app/page");
    assert!(
        proof
            .page_graph
            .components
            .nodes
            .iter()
            .any(|node| node.id == "component/WelcomeCard")
    );
    assert!(
        proof
            .page_graph
            .components
            .edges
            .iter()
            .any(|edge| edge.from == "app/page" && edge.to == "component/WelcomeCard")
    );
    assert_eq!(proof.delivery_mode, DxDeliveryMode::MicroJs);
    assert!(proof.fallback.crawlable);
    assert!(proof.fallback.html.contains("React-shaped source"));
    assert!(proof.fallback.html.contains("data-dx-runtime=\"js\""));
    assert!(proof.packet.roundtrip_matches);
    assert_eq!(proof.packet.decoded_kind, DxPacketKind::Route);
    assert!(proof.packet.section_count >= 3);
}

#[test]
fn react_jsx_lowering_preserves_imports_props_fragments_children_text_and_events() {
    let lowered = lower_react_jsx_source(
        "components/local/WelcomeCard.tsx",
        r#"import DefaultButton, { Icon as SearchIcon, Badge } from "../ui";

export function WelcomeCard({ title, children, onView }) {
  return (
    <>
      <section className="dx-card" data-kind="welcome">
        <SearchIcon aria-label="Search" />
        <h1>{title}</h1>
        <Badge tone="green">Ready</Badge>
        <button type="button" onClick={onView}>
          Record view
        </button>
        {children}
      </section>
    </>
  );
}
"#,
    );

    assert_eq!(lowered.source_path, "components/local/WelcomeCard.tsx");
    assert!(lowered.imports.iter().any(|import| {
        import.source == "../ui"
            && import.default.as_deref() == Some("DefaultButton")
            && import
                .specifiers
                .iter()
                .any(|specifier| specifier.imported == "Icon" && specifier.local == "SearchIcon")
    }));
    assert!(lowered.has_fragment);
    assert_eq!(
        lowered
            .elements
            .iter()
            .find(|element| element.name == "section")
            .and_then(|element| element.attribute("className")),
        Some("dx-card")
    );
    assert_eq!(
        lowered
            .elements
            .iter()
            .find(|element| element.name == "section")
            .and_then(|element| element.attribute("data-kind")),
        Some("welcome")
    );
    assert!(
        lowered
            .elements
            .iter()
            .any(|element| element.name == "SearchIcon")
    );
    assert!(lowered.text_nodes.iter().any(|text| text == "Ready"));
    assert!(lowered.text_nodes.iter().any(|text| text == "Record view"));
    assert!(
        lowered
            .expressions
            .iter()
            .any(|expression| expression == "title")
    );
    assert!(
        lowered
            .expressions
            .iter()
            .any(|expression| expression == "children")
    );
    assert!(lowered.event_attributes.iter().any(|event| {
        event.element == "button" && event.name == "onClick" && event.expression == "onView"
    }));
}

#[test]
fn react_jsx_lowering_preserves_conditionals_lists_and_keyed_updates() {
    let lowered = lower_react_jsx_source(
        "components/local/Feed.tsx",
        r#"export function Feed({ ready, items }) {
  return (
    <section>
      {ready ? <p>Ready</p> : <p>Waiting</p>}
      <ul>
        {items.map((item) => (
          <li key={item.id}>{item.label}</li>
        ))}
      </ul>
    </section>
  );
}
"#,
    );

    assert!(lowered.conditional_branches.iter().any(|branch| {
        branch.condition == "ready"
            && branch.when_true.contains("<p>Ready</p>")
            && branch
                .when_false
                .as_deref()
                .is_some_and(|value| value.contains("<p>Waiting</p>"))
    }));
    assert!(lowered.list_iterations.iter().any(|iteration| {
        iteration.source == "items"
            && iteration.item_binding == "item"
            && iteration.key_expression.as_deref() == Some("item.id")
    }));
    assert!(
        lowered
            .keyed_update_hints
            .iter()
            .any(|hint| { hint.element == "li" && hint.expression.as_deref() == Some("item.id") })
    );
}

#[test]
fn react_jsx_lowering_accepts_bom_and_unicode_before_jsx() {
    let lowered = lower_react_jsx_source(
        "app/page.tsx",
        "\u{feff}// caf\u{00e9}: S\u{00e3}o Paulo\nexport default function Page() {\n  return <main><h1>Ready</h1></main>;\n}\n",
    );

    assert!(
        lowered
            .elements
            .iter()
            .any(|element| element.name == "main")
    );
    assert!(lowered.text_nodes.iter().any(|text| text == "Ready"));
    assert!(lowered.diagnostics.is_empty());
}

#[test]
fn tsx_module_ast_preserves_import_spans_metadata_side_effects_type_only_and_diagnostics() {
    let ast = parse_tsx_module(
        "app/docs/page.tsx",
        r#"import "../styles/global.css";
import type { Metadata } from "next";
import React, { type ReactNode, useState as useReactState } from "react";
import * as Icons from "dx/icon/search";

export const metadata = {
  title: "DX Docs",
  description: "Source-owned docs route.",
  alternates: {
    canonical: "/docs",
  },
};

import { Broken } from ;
"#,
    );

    assert_eq!(ast.source_path, "app/docs/page.tsx");
    assert_eq!(ast.imports.len(), 4);

    let global_css = ast
        .imports
        .iter()
        .find(|import| import.source == "../styles/global.css")
        .expect("side-effect css import");
    assert!(global_css.side_effect_only);
    assert_eq!(global_css.span.line, 1);
    assert_eq!(global_css.span.column, 1);

    let metadata_import = ast
        .imports
        .iter()
        .find(|import| import.source == "next")
        .expect("metadata type import");
    assert!(metadata_import.type_only);
    assert!(metadata_import.specifiers.iter().all(|specifier| {
        specifier.type_only && specifier.imported == "Metadata" && specifier.local == "Metadata"
    }));

    let react_import = ast
        .imports
        .iter()
        .find(|import| import.source == "react")
        .expect("react import");
    assert_eq!(react_import.default.as_deref(), Some("React"));
    assert!(react_import.specifiers.iter().any(|specifier| {
        specifier.imported == "ReactNode" && specifier.local == "ReactNode" && specifier.type_only
    }));
    assert!(react_import.specifiers.iter().any(|specifier| {
        specifier.imported == "useState"
            && specifier.local == "useReactState"
            && !specifier.type_only
    }));

    let icon_import = ast
        .imports
        .iter()
        .find(|import| import.source == "dx/icon/search")
        .expect("namespace import");
    assert_eq!(icon_import.namespace.as_deref(), Some("Icons"));

    let metadata = ast.metadata.expect("metadata object");
    assert_eq!(metadata.title.as_deref(), Some("DX Docs"));
    assert_eq!(
        metadata.description.as_deref(),
        Some("Source-owned docs route.")
    );
    assert_eq!(metadata.canonical.as_deref(), Some("/docs"));
    assert!(
        ast.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "tsx-import-missing-source"
                && diagnostic.span.line == 14)
    );
}

#[test]
fn tsx_module_ast_ignores_import_keywords_inside_regex_literals() {
    let ast = parse_tsx_module(
        "components/content/markdown.tsx",
        r#"export function auditDxMarkdownSource(source: string) {
  const mdxExecutionPattern = /(^|\n)\s*(import|export)\s+|<[A-Z][A-Za-z0-9]*\b|\{[^}\n]+}/g;

  return source.matchAll(mdxExecutionPattern);
}
"#,
    );

    assert!(ast.imports.is_empty());
    assert!(
        ast.diagnostics.is_empty(),
        "regex keywords should not produce import diagnostics: {:?}",
        ast.diagnostics
    );
}

#[test]
fn tsx_module_ast_ignores_regex_literals_after_expression_keywords() {
    let ast = parse_tsx_module(
        "components/content/markdown.tsx",
        r#"export function hasMdxExecution(source: string) {
  if (source.length === 0) return /^(import|export)\s+/m.test(source);

  const hasImport = (value: string) => /(^|\n)\s*import\s+/.test(value);
  return hasImport(source);
}
"#,
    );

    assert!(ast.imports.is_empty());
    assert!(
        ast.diagnostics.is_empty(),
        "regex literals after return/arrow should not produce import diagnostics: {:?}",
        ast.diagnostics
    );
}

#[test]
fn react_app_route_links_aliased_imported_components_from_lowered_jsx() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/alias".to_string(),
        route_source_path: "app/alias/page.tsx".to_string(),
        route_source: r#"import { WelcomeCard as Hero } from "../../components/local/WelcomeCard";

export default function Page() {
  return (
    <main>
      <Hero />
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "WelcomeCard".to_string(),
            source_path: "components/local/WelcomeCard.tsx".to_string(),
            source: r#"export function WelcomeCard() {
  return <section><h1>Alias route works</h1></section>;
}
"#
            .to_string(),
            package_id: Some("local/welcome-card".to_string()),
        }],
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile alias route");

    assert!(
        proof
            .page_graph
            .components
            .edges
            .iter()
            .any(|edge| edge.from == "app/page" && edge.to == "component/WelcomeCard")
    );
    assert!(
        proof
            .page_graph
            .components
            .nodes
            .iter()
            .any(|node| node.id == "component/WelcomeCard"
                && node.package_id.as_deref() == Some("local/welcome-card"))
    );
}

#[test]
fn react_app_route_composes_layout_chain_and_route_boundaries() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/dashboard".to_string(),
        route_source_path: "app/dashboard/page.tsx".to_string(),
        route_source: r#"import { MetricsPanel } from "../../components/local/MetricsPanel";

export default function Page() {
  return (
    <main>
      <h1>Dashboard home</h1>
      <MetricsPanel />
    </main>
  );
}
"#
        .to_string(),
        segments: vec![
            DxReactAppSegmentSource {
                kind: DxReactAppSegmentKind::Layout,
                source_path: "app/layout.tsx".to_string(),
                source: r#"import { AppShell } from "../components/local/AppShell";

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body>
        <AppShell>{children}</AppShell>
      </body>
    </html>
  );
}
"#
                .to_string(),
            },
            DxReactAppSegmentSource {
                kind: DxReactAppSegmentKind::Layout,
                source_path: "app/dashboard/layout.tsx".to_string(),
                source: r#"export default function DashboardLayout({ children }) {
  return <section className="dashboard-layout">{children}</section>;
}
"#
                .to_string(),
            },
            DxReactAppSegmentSource {
                kind: DxReactAppSegmentKind::Loading,
                source_path: "app/dashboard/loading.tsx".to_string(),
                source: r#"export default function Loading() {
  return <p>Loading dashboard</p>;
}
"#
                .to_string(),
            },
            DxReactAppSegmentSource {
                kind: DxReactAppSegmentKind::Error,
                source_path: "app/dashboard/error.tsx".to_string(),
                source: r#""use client";

export default function Error({ reset }) {
  return <button type="button" onClick={reset}>Retry dashboard</button>;
}
"#
                .to_string(),
            },
            DxReactAppSegmentSource {
                kind: DxReactAppSegmentKind::NotFound,
                source_path: "app/dashboard/not-found.tsx".to_string(),
                source: r#"export default function NotFound() {
  return <h1>Dashboard missing</h1>;
}
"#
                .to_string(),
            },
        ],
        components: vec![
            DxReactComponentSource {
                name: "AppShell".to_string(),
                source_path: "components/local/AppShell.tsx".to_string(),
                source: r#"export function AppShell({ children }) {
  return <div className="app-shell">{children}</div>;
}
"#
                .to_string(),
                package_id: Some("local/app-shell".to_string()),
            },
            DxReactComponentSource {
                name: "MetricsPanel".to_string(),
                source_path: "components/local/MetricsPanel.tsx".to_string(),
                source: r#"export function MetricsPanel() {
  return <p>Live metrics</p>;
}
"#
                .to_string(),
                package_id: None,
            },
        ],
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile app route with app router segments");

    assert_eq!(proof.page_graph.root_component_id, "app/layout");
    assert!(
        proof
            .page_graph
            .components
            .nodes
            .iter()
            .any(|node| { node.id == "app/dashboard/loading" && node.name == "Loading" })
    );
    assert!(
        proof
            .page_graph
            .components
            .nodes
            .iter()
            .any(|node| { node.id == "app/dashboard/error" && node.name == "Error" })
    );
    assert!(
        proof
            .page_graph
            .components
            .nodes
            .iter()
            .any(|node| { node.id == "app/dashboard/not-found" && node.name == "NotFound" })
    );
    assert!(
        proof
            .page_graph
            .components
            .edges
            .iter()
            .any(|edge| { edge.from == "app/layout" && edge.to == "app/dashboard/layout" })
    );
    assert!(
        proof
            .page_graph
            .components
            .edges
            .iter()
            .any(|edge| { edge.from == "app/dashboard/layout" && edge.to == "app/page" })
    );
    assert!(
        proof.page_graph.components.edges.iter().any(|edge| {
            edge.from == "app/dashboard/layout" && edge.to == "app/dashboard/loading"
        })
    );
    assert!(
        proof
            .page_graph
            .components
            .edges
            .iter()
            .any(|edge| { edge.from == "app/layout" && edge.to == "component/AppShell" })
    );
    assert!(
        proof
            .page_graph
            .components
            .edges
            .iter()
            .any(|edge| { edge.from == "app/page" && edge.to == "component/MetricsPanel" })
    );
    assert_eq!(proof.delivery_mode, DxDeliveryMode::MicroJs);
    assert!(proof.fallback.html.contains("Dashboard home"));
    assert!(proof.fallback.html.contains("data-dx-layouts=\"2\""));
    assert!(
        proof
            .fallback
            .html
            .contains("data-dx-boundaries=\"loading,error,not-found\"")
    );
}

#[test]
fn react_server_contracts_capture_route_handlers_loaders_and_actions() {
    let contracts = compile_react_server_contracts(&[
        DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/health/route.ts".to_string(),
            source: r#"export function GET() {
  return {
    ok: true,
    runtime: "dx-www",
  };
}
"#
            .to_string(),
        },
        DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/session/route.ts".to_string(),
            source: r#"export const POST = async (request: Request) => {
  const body = await request.json();
  return Response.json({ ok: true, email: body.email });
};
"#
            .to_string(),
        },
        DxReactServerSource {
            kind: DxReactServerSourceKind::Loader,
            source_path: "server/loaders.ts".to_string(),
            source: r#"export async function loadHomeMetrics() {
  return {
    routes: 1,
    packages: 0,
  };
}
"#
            .to_string(),
        },
        DxReactServerSource {
            kind: DxReactServerSourceKind::Action,
            source_path: "server/actions.ts".to_string(),
            source: r#"export async function recordWelcomeView() {
  return { ok: true, source: "dx-www-server-action" };
}
"#
            .to_string(),
        },
    ]);

    let route = contracts
        .iter()
        .find(|contract| contract.source_path == "app/api/health/route.ts")
        .expect("route contract");
    assert_eq!(route.endpoint.as_deref(), Some("/api/health"));
    assert_eq!(route.execution_model, "analyze-only");
    assert!(!route.lifecycle_scripts_executed);
    assert!(route.exports.iter().any(|export| {
        export.name == "GET"
            && export.http_method.as_deref() == Some("GET")
            && export.response_serialization == "json-object"
    }));
    let session_route = contracts
        .iter()
        .find(|contract| contract.source_path == "app/api/session/route.ts")
        .expect("session route contract");
    assert_eq!(session_route.endpoint.as_deref(), Some("/api/session"));
    assert!(session_route.exports.iter().any(|export| {
        export.name == "POST"
            && export.http_method.as_deref() == Some("POST")
            && export.async_export
            && export.request_serialization == "web-request"
            && export.response_serialization == "json-response"
    }));

    let loader = contracts
        .iter()
        .find(|contract| contract.source_path == "server/loaders.ts")
        .expect("loader contract");
    assert_eq!(loader.kind, DxReactServerSourceKind::Loader);
    assert!(loader.exports.iter().any(|export| {
        export.name == "loadHomeMetrics"
            && export.async_export
            && export.response_serialization == "json-object"
    }));

    let action = contracts
        .iter()
        .find(|contract| contract.source_path == "server/actions.ts")
        .expect("action contract");
    assert_eq!(action.kind, DxReactServerSourceKind::Action);
    assert!(action.exports.iter().any(|export| {
        export.name == "recordWelcomeView"
            && export.async_export
            && export.request_serialization == "json-value"
    }));
}

#[test]
fn react_route_handler_runtime_serializes_returned_object_without_lifecycle() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/status/route.ts".to_string(),
            source: r#"export function GET() {
  return {
    ok: true,
    runtime: "dx-www",
    count: 3,
    status: 202,
  };
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/status".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("route response");

    assert_eq!(response.status, 202);
    assert_eq!(response.content_type, "application/json; charset=utf-8");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["runtime"], "dx-www");
    assert_eq!(response.body["count"], 3);
}

#[test]
fn react_route_handler_runtime_keeps_request_json_aliases_as_body_reads() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const body = await request.json();
  return {
    ok: true,
    payload: body,
    name: body.name,
    count: body.count,
    status: 201,
  };
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "name": "Ada",
                "count": 2,
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request json body");

    assert_eq!(response.status, 201);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["payload"]["name"], "Ada");
    assert_eq!(response.body["name"], "Ada");
    assert_eq!(response.body["count"], 2);
}

#[test]
fn react_route_handler_runtime_serializes_response_json_body_alias() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const body = await request.json();
  return Response.json(body, {
    status: 201,
    headers: {
      "x-dx-json-alias": "source-owned-safe-interpreter",
    },
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "name": "Ada",
                "count": 2,
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("Response.json body alias");

    assert_eq!(response.status, 201);
    assert_eq!(response.content_type, "application/json; charset=utf-8");
    assert_eq!(
        response.headers.get("x-dx-json-alias").map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["name"], "Ada");
    assert_eq!(response.body["count"], 2);
}

#[test]
fn react_route_handler_runtime_serializes_response_header_aliases_without_next_runtime() {
    let json_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/preferences/route.ts".to_string(),
            source: r#"export function GET() {
  const responseHeaders = new Headers();
  responseHeaders.set("x-dx-response", "alias");
  responseHeaders.append("set-cookie", "theme=dark; Path=/; HttpOnly");

  return Response.json(
    { ok: true },
    {
      status: 202,
      headers: responseHeaders,
    },
  );
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/preferences".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("Response.json header aliases");

    assert_eq!(json_response.status, 202);
    assert_eq!(
        json_response.content_type,
        "application/json; charset=utf-8"
    );
    assert_eq!(
        json_response
            .headers
            .get("x-dx-response")
            .map(String::as_str),
        Some("alias")
    );
    assert_eq!(
        json_response.headers.get("set-cookie").map(String::as_str),
        Some("theme=dark; Path=/; HttpOnly")
    );
    assert_eq!(json_response.body["ok"], true);
    assert_eq!(
        json_response.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!json_response.lifecycle_scripts_executed);

    let web_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/preferences/route.ts".to_string(),
            source: r#"export function POST() {
  const responseHeaders = new Headers([["x-dx-web-response", "alias"]]);
  responseHeaders.set("content-type", "text/plain; charset=utf-8");

  return new Response("accepted", {
    status: 202,
    headers: responseHeaders,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/preferences".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("new Response header aliases");

    assert_eq!(web_response.status, 202);
    assert_eq!(web_response.content_type, "text/plain; charset=utf-8");
    assert_eq!(
        web_response
            .headers
            .get("x-dx-web-response")
            .map(String::as_str),
        Some("alias")
    );
    assert_eq!(web_response.body, "accepted");
    assert_eq!(
        web_response.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!web_response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_typed_request_json_body_aliases() {
    let typed_annotation = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"type Payload = { email?: string; count?: number };

export async function POST(request: Request) {
  const body: Payload = await request.json();
  return Response.json({
    email: body.email,
    count: body.count,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
                "count": 2,
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("typed request json alias");
    let as_cast = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"type Payload = { email?: string };

export async function POST(request: Request) {
  const body = (await request.json()) as Payload;
  return Response.json({
    email: body.email,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "grace@example.com",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request json as-cast alias");

    assert_eq!(typed_annotation.body["email"], "ada@example.com");
    assert_eq!(typed_annotation.body["count"], 2);
    assert_eq!(as_cast.body["email"], "grace@example.com");
    assert_eq!(
        typed_annotation.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!typed_annotation.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_typed_request_json_body_alias_postfixes() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/typed-postfix/route.ts".to_string(),
            source: r#"type Payload = {
  email?: string;
  count?: number;
  profile?: { label?: string };
};

export async function POST(request: Request) {
  const body = await request.json() as Payload;
  return Response.json({
    email: body.email!,
    count: body.count as number,
    label: body.profile!.label satisfies string,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/typed-postfix".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "lin@example.com",
                "count": 7,
                "profile": { "label": "trusted" },
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("typed request json alias postfixes");

    assert_eq!(response.body["email"], "lin@example.com");
    assert_eq!(response.body["count"], 7);
    assert_eq!(response.body["label"], "trusted");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_typed_request_json_body_alias_root_postfixes() {
    let request = DxReactRouteHandlerRequest {
        method: "POST".to_string(),
        path: "/api/typed-root".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::json!({
            "email": "sam@example.com",
            "count": 11,
        }),
        route_params: BTreeMap::new(),
        search_params: BTreeMap::new(),
        runtime_env: BTreeMap::new(),
    };
    let as_cast = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/typed-root/route.ts".to_string(),
            source: r#"type Payload = { email?: string; count?: number };

export async function POST(request: Request) {
  const body = await request.json() as Payload;
  return Response.json(body as Payload);
}
"#
            .to_string(),
        },
        request.clone(),
    )
    .expect("typed request json alias root as-cast");
    let non_null = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/typed-root/route.ts".to_string(),
            source: r#"type Payload = { email?: string; count?: number };

export async function POST(request: Request) {
  const body: Payload = await request.json();
  return Response.json(body!);
}
"#
            .to_string(),
        },
        request,
    )
    .expect("typed request json alias root non-null assertion");

    assert_eq!(as_cast.body["email"], "sam@example.com");
    assert_eq!(as_cast.body["count"], 11);
    assert_eq!(non_null.body["email"], "sam@example.com");
    assert_eq!(non_null.body["count"], 11);
    assert_eq!(as_cast.execution_model, "source-owned-safe-interpreter");
    assert_eq!(non_null.execution_model, "source-owned-safe-interpreter");
    assert!(!as_cast.lifecycle_scripts_executed);
    assert!(!non_null.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_typed_request_json_cast_root_aliases() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/typed-root-cast/route.ts".to_string(),
            source: r#"type Payload = { email?: string; count?: number };

export async function POST(request: Request) {
  const body = await (request as Request).json() as Payload;
  return Response.json({
    email: body.email,
    count: body.count,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/typed-root-cast".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "maya@example.com",
                "count": 13,
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("typed request json cast root alias");

    assert_eq!(response.body["email"], "maya@example.com");
    assert_eq!(response.body["count"], 13);
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_typed_request_json_satisfies_aliases() {
    let request = DxReactRouteHandlerRequest {
        method: "POST".to_string(),
        path: "/api/typed-satisfies".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::json!({
            "email": "nora@example.com",
            "count": 17,
        }),
        route_params: BTreeMap::new(),
        search_params: BTreeMap::new(),
        runtime_env: BTreeMap::new(),
    };
    let root_alias = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/typed-satisfies/route.ts".to_string(),
            source: r#"type Payload = { email?: string; count?: number };

export async function POST(request: Request) {
  const body = await request.json() satisfies Payload;
  return Response.json({
    email: body.email,
    count: body.count,
  });
}
"#
            .to_string(),
        },
        request.clone(),
    )
    .expect("typed request json satisfies alias");
    let destructured_alias = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/typed-satisfies/route.ts".to_string(),
            source: r#"type Payload = { email?: string; count?: number };

export async function POST(request: Request) {
  const { email } = await request.json() satisfies Payload;
  return Response.json({ email });
}
"#
            .to_string(),
        },
        request,
    )
    .expect("destructured typed request json satisfies alias");

    assert_eq!(root_alias.body["email"], "nora@example.com");
    assert_eq!(root_alias.body["count"], 17);
    assert_eq!(destructured_alias.body["email"], "nora@example.com");
    assert_eq!(root_alias.execution_model, "source-owned-safe-interpreter");
    assert_eq!(
        destructured_alias.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!root_alias.lifecycle_scripts_executed);
    assert!(!destructured_alias.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_request_json_body_nullish_defaults() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/profile/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const body = await request.json();
  return Response.json({
    email: body.email ?? "missing@example.com",
    displayName: body?.displayName ?? "Anonymous",
    missing: body.missing ?? "fallback",
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/profile".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request json body nullish defaults");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["email"], "ada@example.com");
    assert_eq!(response.body["displayName"], "Anonymous");
    assert_eq!(response.body["missing"], "fallback");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_simple_conditional_json_returns() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/profile/route.ts".to_string(),
        source: r#"export async function POST(request: Request) {
  const body = await request.json();
  if (!body.email) {
    return Response.json(
      {
        ok: false,
        error: "Email required",
      },
      {
        status: 400,
      },
    );
  }

  return Response.json({
    ok: true,
    email: body.email,
  });
}
"#
        .to_string(),
    };

    let missing = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/profile".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({}),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("conditional missing body field response");
    let present = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/profile".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("conditional present body field response");

    assert_eq!(missing.status, 400);
    assert_eq!(missing.body["ok"], false);
    assert_eq!(missing.body["error"], "Email required");
    assert_eq!(present.status, 200);
    assert_eq!(present.body["ok"], true);
    assert_eq!(present.body["email"], "ada@example.com");
    assert_eq!(missing.execution_model, "source-owned-safe-interpreter");
    assert_eq!(present.execution_model, "source-owned-safe-interpreter");
    assert!(!missing.lifecycle_scripts_executed);
    assert!(!present.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_else_if_conditional_json_returns() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/session/route.ts".to_string(),
        source: r#"export async function POST(request: Request) {
  const body = await request.json();
  if (body.role === "owner") {
    return Response.json(
      {
        branch: "owner",
        id: body.id,
      },
      {
        status: 201,
      },
    );
  } else if (body.role === "admin") {
    return Response.json(
      {
        branch: "admin",
        id: body.id,
      },
      {
        status: 202,
      },
    );
  } else {
    return Response.json(
      {
        branch: "member",
        id: body.id ?? "guest",
      },
      {
        status: 203,
      },
    );
  }
}
"#
        .to_string(),
    };

    let owner = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/session".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "role": "owner",
                "id": "owner-1",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("conditional owner response");
    let admin = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/session".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "role": "admin",
                "id": "admin-1",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("conditional admin response");
    let member = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/session".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "role": "member",
                "id": "member-1",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("conditional member response");

    assert_eq!(owner.status, 201);
    assert_eq!(owner.body["branch"], "owner");
    assert_eq!(owner.body["id"], "owner-1");
    assert_eq!(admin.status, 202);
    assert_eq!(admin.body["branch"], "admin");
    assert_eq!(admin.body["id"], "admin-1");
    assert_eq!(member.status, 203);
    assert_eq!(member.body["branch"], "member");
    assert_eq!(member.body["id"], "member-1");
    assert_eq!(owner.execution_model, "source-owned-safe-interpreter");
    assert_eq!(admin.execution_model, "source-owned-safe-interpreter");
    assert_eq!(member.execution_model, "source-owned-safe-interpreter");
    assert!(!owner.lifecycle_scripts_executed);
    assert!(!admin.lifecycle_scripts_executed);
    assert!(!member.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_compound_conditional_json_returns() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/access/route.ts".to_string(),
        source: r#"export async function POST(request: Request) {
  const body = await request.json();
  if (body.email && body.role === "admin") {
    return Response.json(
      {
        access: "admin",
        email: body.email,
      },
      {
        status: 201,
      },
    );
  } else if (body.email || body.invite === "guest") {
    return Response.json(
      {
        access: "user",
        email: body.email ?? "guest@example.com",
      },
      {
        status: 202,
      },
    );
  } else {
    return Response.json(
      {
        access: "denied",
      },
      {
        status: 403,
      },
    );
  }
}
"#
        .to_string(),
    };

    let admin = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
                "role": "admin",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("compound admin access response");
    let invited = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "invite": "guest",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("compound invited access response");
    let denied = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({}),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("compound denied access response");

    assert_eq!(admin.status, 201);
    assert_eq!(admin.body["access"], "admin");
    assert_eq!(admin.body["email"], "ada@example.com");
    assert_eq!(invited.status, 202);
    assert_eq!(invited.body["access"], "user");
    assert_eq!(invited.body["email"], "guest@example.com");
    assert_eq!(denied.status, 403);
    assert_eq!(denied.body["access"], "denied");
    assert_eq!(admin.execution_model, "source-owned-safe-interpreter");
    assert_eq!(invited.execution_model, "source-owned-safe-interpreter");
    assert_eq!(denied.execution_model, "source-owned-safe-interpreter");
    assert!(!admin.lifecycle_scripts_executed);
    assert!(!invited.lifecycle_scripts_executed);
    assert!(!denied.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_parenthesized_conditional_json_returns() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/access/route.ts".to_string(),
        source: r#"export async function POST(request: Request) {
  const body = await request.json();
  if ((body.email && body.role === "admin")) {
    return Response.json(
      {
        access: "admin",
        email: body.email,
      },
      {
        status: 201,
      },
    );
  }

  if (!(body.email)) {
    return Response.json(
      {
        access: "denied",
      },
      {
        status: 403,
      },
    );
  }

  return Response.json(
    {
      access: "user",
      email: body.email,
    },
    {
      status: 202,
    },
  );
}
"#
        .to_string(),
    };

    let admin = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
                "role": "admin",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("parenthesized admin access response");
    let user = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "grace@example.com",
                "role": "member",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("parenthesized user access response");
    let denied = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({}),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("parenthesized denied access response");

    assert_eq!(admin.status, 201);
    assert_eq!(admin.body["access"], "admin");
    assert_eq!(admin.body["email"], "ada@example.com");
    assert_eq!(user.status, 202);
    assert_eq!(user.body["access"], "user");
    assert_eq!(user.body["email"], "grace@example.com");
    assert_eq!(denied.status, 403);
    assert_eq!(denied.body["access"], "denied");
    assert_eq!(admin.execution_model, "source-owned-safe-interpreter");
    assert_eq!(user.execution_model, "source-owned-safe-interpreter");
    assert_eq!(denied.execution_model, "source-owned-safe-interpreter");
    assert!(!admin.lifecycle_scripts_executed);
    assert!(!user.lifecycle_scripts_executed);
    assert!(!denied.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_literal_first_conditional_json_returns() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/access/route.ts".to_string(),
        source: r#"export async function POST(request: Request) {
  const body = await request.json();
  if ("preview" === request.headers.get("x-dx-mode")) {
    return Response.json(
      {
        access: "preview",
        mode: request.headers.get("x-dx-mode"),
      },
      {
        status: 200,
      },
    );
  }

  if ("admin" === body.role) {
    return Response.json(
      {
        access: "admin",
        role: body.role,
      },
      {
        status: 201,
      },
    );
  }

  if ("blocked" !== body.status) {
    return Response.json(
      {
        access: "user",
        status: body.status ?? "none",
      },
      {
        status: 202,
      },
    );
  }

  return Response.json(
    {
      access: "blocked",
    },
    {
      status: 403,
    },
  );
}
"#
        .to_string(),
    };

    let preview = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::from([("x-dx-mode".to_string(), "preview".to_string())]),
            body: serde_json::json!({
                "role": "member",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("literal-first preview access response");
    let admin = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "role": "admin",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("literal-first admin access response");
    let user = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "role": "member",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("literal-first user access response");
    let blocked = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "status": "blocked",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("literal-first blocked access response");

    assert_eq!(preview.status, 200);
    assert_eq!(preview.body["access"], "preview");
    assert_eq!(preview.body["mode"], "preview");
    assert_eq!(admin.status, 201);
    assert_eq!(admin.body["access"], "admin");
    assert_eq!(admin.body["role"], "admin");
    assert_eq!(user.status, 202);
    assert_eq!(user.body["access"], "user");
    assert_eq!(user.body["status"], "none");
    assert_eq!(blocked.status, 403);
    assert_eq!(blocked.body["access"], "blocked");
    assert_eq!(preview.execution_model, "source-owned-safe-interpreter");
    assert_eq!(admin.execution_model, "source-owned-safe-interpreter");
    assert_eq!(user.execution_model, "source-owned-safe-interpreter");
    assert_eq!(blocked.execution_model, "source-owned-safe-interpreter");
    assert!(!preview.lifecycle_scripts_executed);
    assert!(!admin.lifecycle_scripts_executed);
    assert!(!user.lifecycle_scripts_executed);
    assert!(!blocked.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_nested_conditional_json_returns() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/access/route.ts".to_string(),
        source: r#"export async function POST(request: Request) {
  const body = await request.json();
  if (body.email) {
    if (body.role === "admin") {
      return Response.json(
        {
          access: "admin",
          email: body.email,
        },
        {
          status: 201,
        },
      );
    }

    return Response.json(
      {
        access: "user",
        email: body.email,
      },
      {
        status: 202,
      },
    );
  }

  return Response.json(
    {
      access: "denied",
    },
    {
      status: 403,
    },
  );
}
"#
        .to_string(),
    };

    let admin = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
                "role": "admin",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("nested admin access response");
    let user = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "grace@example.com",
                "role": "member",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("nested user access response");
    let denied = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/access".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({}),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("nested denied access response");

    assert_eq!(admin.status, 201);
    assert_eq!(admin.body["access"], "admin");
    assert_eq!(admin.body["email"], "ada@example.com");
    assert_eq!(user.status, 202);
    assert_eq!(user.body["access"], "user");
    assert_eq!(user.body["email"], "grace@example.com");
    assert_eq!(denied.status, 403);
    assert_eq!(denied.body["access"], "denied");
    assert_eq!(admin.execution_model, "source-owned-safe-interpreter");
    assert_eq!(user.execution_model, "source-owned-safe-interpreter");
    assert_eq!(denied.execution_model, "source-owned-safe-interpreter");
    assert!(!admin.lifecycle_scripts_executed);
    assert!(!user.lifecycle_scripts_executed);
    assert!(!denied.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_destructured_request_json_fields() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const { email, count: total } = await request.json();
  return Response.json({
    email,
    total,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
                "count": 2,
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("destructured request json fields");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["email"], "ada@example.com");
    assert_eq!(response.body["total"], 2);
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_destructured_request_json_field_defaults() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const { email = "unknown", count: total = 0 } = await request.json();
  return Response.json({
    email,
    total,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({}),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("destructured request json field defaults");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["email"], "unknown");
    assert_eq!(response.body["total"], 0);
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_request_text_body_alias() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/webhook/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const raw = await request.text();
  return Response.json({
    ok: true,
    raw,
    method: request.method,
  }, {
    status: 202,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/webhook".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::String("stripe-signature-payload".to_string()),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request text body alias");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["raw"], "stripe-signature-payload");
    assert_eq!(response.body["method"], "POST");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_projects_structured_request_text_body_as_raw_text() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/webhook/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const raw = await request.text();
  const cloned = request.clone();
  const cloneRaw = await cloned.text();

  return Response.json({
    raw,
    cloneRaw,
    directCloneRaw: await request.clone().text(),
    wrappedRaw: (await request.text()),
    textProperty: (await request.text()).event ?? "missing",
    method: request.method,
  }, {
    status: 202,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/webhook".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "event": "checkout.session.completed",
                "livemode": true,
                "attempt": 2,
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("structured request text body projection");

    assert_eq!(response.status, 202);
    let raw = response.body["raw"]
        .as_str()
        .expect("request.text() should expose a raw text string");
    let raw_payload: serde_json::Value =
        serde_json::from_str(raw).expect("structured text body should be serialized JSON");
    assert_eq!(raw_payload["event"], "checkout.session.completed");
    assert_eq!(raw_payload["livemode"], true);
    assert_eq!(raw_payload["attempt"], 2);
    assert_eq!(response.body["cloneRaw"], raw);
    assert_eq!(response.body["directCloneRaw"], raw);
    assert_eq!(response.body["wrappedRaw"], raw);
    assert_eq!(response.body["textProperty"], "missing");
    assert_eq!(response.body["method"], "POST");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reports_unsupported_binary_body_readers_without_next_runtime() {
    let array_buffer_error = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/upload/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const bytes = await request.arrayBuffer();

  return Response.json({
    bytesLength: bytes.byteLength ?? 0,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/upload".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::String("binary-payload".to_string()),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect_err("arrayBuffer body reader should stay behind an explicit boundary");

    assert!(array_buffer_error.contains("request.arrayBuffer()"));
    assert!(array_buffer_error.contains("source-owned-safe-interpreter"));
    assert!(
        array_buffer_error
            .contains("request.json(), request.text(), request.formData(), and request.body")
    );
    assert!(array_buffer_error.contains("node_modules_required=false"));
    assert!(array_buffer_error.contains("external_runtime_executed=false"));

    let blob_error = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/upload/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const cloned = request.clone();

  return Response.json({
    blobSize: (await cloned.blob()).size ?? 0,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/upload".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::String("binary-payload".to_string()),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect_err("blob body reader should stay behind an explicit boundary");

    assert!(blob_error.contains("cloned.blob()"));
    assert!(blob_error.contains("source-owned-safe-interpreter"));
    assert!(blob_error.contains("binary body readers require an explicit adapter boundary"));
    assert!(blob_error.contains("node_modules_required=false"));
    assert!(blob_error.contains("external_runtime_executed=false"));
}

#[test]
fn react_route_handler_runtime_reads_request_clone_body_without_next_runtime() {
    let json_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const cloned = request.clone();
  const directBody = await request.clone().json();
  const clonedBody = await cloned.json();
  const { email: destructuredEmail, count: destructuredCount, missing: destructuredMissing = "fallback" } = await request.clone().json();
  const { email: clonedEmail } = await cloned.json();

  return Response.json({
    payload: await cloned.json(),
    directPayload: await request.clone().json(),
    directAliasEmail: directBody.email,
    directAliasMissing: directBody.missing ?? "fallback",
    clonedAliasCount: clonedBody.count,
    directAliasBracketEmail: directBody["email"],
    directAliasBracketMissing: directBody["missing"] ?? "fallback",
    clonedAliasBracketCount: clonedBody["count"],
    directAliasNestedEmail: directBody["profile"]["email"],
    directAliasNestedMissing: directBody["profile"]["missing"] ?? "fallback",
    clonedAliasNestedEmail: clonedBody["profile"]?.["email"],
    destructuredEmail,
    destructuredCount,
    destructuredMissing,
    clonedEmail,
    email: (await cloned.json()).email ?? "unknown",
    directEmail: (await request.clone().json()).email,
    missing: (await cloned.json()).missing ?? "fallback",
    bracketEmail: (await cloned.json())["email"] ?? "unknown",
    directBracketEmail: (await request.clone().json())["email"],
    bracketMissing: (await cloned.json())["missing"] ?? "fallback",
    optionalBracketCount: (await cloned.json())?.["count"],
    nestedBracketEmail: (await cloned.json())["profile"]["email"] ?? "unknown",
    directNestedBracketEmail: (await request.clone().json())["profile"]?.["email"],
    nestedBracketMissing: (await cloned.json())["profile"]["missing"] ?? "fallback",
  }, {
    status: 202,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
                "count": 2,
                "profile": {
                    "email": "profile@example.com",
                    "count": 7,
                },
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request clone json body");

    assert_eq!(json_response.status, 202);
    assert_eq!(json_response.body["payload"]["email"], "ada@example.com");
    assert_eq!(json_response.body["payload"]["count"], 2);
    assert_eq!(
        json_response.body["directPayload"]["email"],
        "ada@example.com"
    );
    assert_eq!(json_response.body["email"], "ada@example.com");
    assert_eq!(json_response.body["directEmail"], "ada@example.com");
    assert_eq!(json_response.body["missing"], "fallback");
    assert_eq!(json_response.body["directAliasEmail"], "ada@example.com");
    assert_eq!(json_response.body["directAliasMissing"], "fallback");
    assert_eq!(json_response.body["clonedAliasCount"], 2);
    assert_eq!(
        json_response.body["directAliasBracketEmail"],
        "ada@example.com"
    );
    assert_eq!(json_response.body["directAliasBracketMissing"], "fallback");
    assert_eq!(json_response.body["clonedAliasBracketCount"], 2);
    assert_eq!(
        json_response.body["directAliasNestedEmail"],
        "profile@example.com"
    );
    assert_eq!(json_response.body["directAliasNestedMissing"], "fallback");
    assert_eq!(
        json_response.body["clonedAliasNestedEmail"],
        "profile@example.com"
    );
    assert_eq!(json_response.body["destructuredEmail"], "ada@example.com");
    assert_eq!(json_response.body["destructuredCount"], 2);
    assert_eq!(json_response.body["destructuredMissing"], "fallback");
    assert_eq!(json_response.body["clonedEmail"], "ada@example.com");
    assert_eq!(json_response.body["bracketEmail"], "ada@example.com");
    assert_eq!(json_response.body["directBracketEmail"], "ada@example.com");
    assert_eq!(json_response.body["bracketMissing"], "fallback");
    assert_eq!(json_response.body["optionalBracketCount"], 2);
    assert_eq!(
        json_response.body["nestedBracketEmail"],
        "profile@example.com"
    );
    assert_eq!(
        json_response.body["directNestedBracketEmail"],
        "profile@example.com"
    );
    assert_eq!(json_response.body["nestedBracketMissing"], "fallback");
    assert_eq!(
        json_response.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!json_response.lifecycle_scripts_executed);

    let text_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/webhook/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  return Response.json({
    raw: await request.clone().text(),
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/webhook".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::String("stripe-event".to_string()),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request clone text body");

    assert_eq!(text_response.status, 200);
    assert_eq!(text_response.body["raw"], "stripe-event");
    assert_eq!(
        text_response.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!text_response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_request_method_alias() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/webhook/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const method = request.method;
  if (method !== "POST") {
    return Response.json({ ok: false, method }, { status: 405 });
  }

  return Response.json({ ok: true, method }, { status: 202 });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/webhook".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request method alias route response");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["method"], "POST");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_destructured_request_method_alias() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/webhook/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const { method } = request;
  if (method !== "POST") {
    return Response.json({ ok: false, method }, { status: 405 });
  }

  return Response.json({ ok: true, method }, { status: 202 });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/webhook".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("destructured request method alias route response");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["method"], "POST");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_evaluates_destructured_request_url_alias() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/webhook/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const { url: requestUrl } = request;
  if (requestUrl !== "/api/webhook?mode=preview") {
    return Response.json({ ok: false, requestUrl }, { status: 400 });
  }

  return Response.json({ ok: true, requestUrl }, { status: 202 });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/webhook?mode=preview".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([("mode".to_string(), "preview".to_string())]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("destructured request url alias route response");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["requestUrl"], "/api/webhook?mode=preview");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serializes_request_form_data_alias() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/contact/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const form = await request.formData();
  const cloned = request.clone();
  return Response.json({
    email: form.get("email"),
    plan: form.get("plan") ?? "free",
    directEmail: (await request.formData()).get("email") ?? "missing",
    cloneEmail: (await request.clone().formData()).get("email") ?? "missing",
    clonedEmail: (await cloned.formData()).get("email") ?? "missing",
    directMissing: (await request.formData()).get("missing") ?? "fallback",
  }, {
    status: 202,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/contact".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "email": "ada@example.com",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request formData body alias");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["email"], "ada@example.com");
    assert_eq!(response.body["plan"], "free");
    assert_eq!(response.body["directEmail"], "ada@example.com");
    assert_eq!(response.body["cloneEmail"], "ada@example.com");
    assert_eq!(response.body["clonedEmail"], "ada@example.com");
    assert_eq!(response.body["directMissing"], "fallback");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_executes_new_response_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  const body = await request.json();
  return new Response(JSON.stringify({
    ok: true,
    path: request.url,
    name: body.name,
  }), {
    status: 202,
    headers: {
      "content-type": "application/json; charset=utf-8",
      "x-dx-web-response": "source-owned-safe-interpreter",
    },
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo?preview=1".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "name": "Ada",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("new Response route handler");

    assert_eq!(response.status, 202);
    assert_eq!(response.content_type, "application/json; charset=utf-8");
    assert_eq!(
        response
            .headers
            .get("x-dx-web-response")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["path"], "/api/echo?preview=1");
    assert_eq!(response.body["name"], "Ada");
}

#[test]
fn react_route_handler_runtime_executes_exported_const_arrow_handlers() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/echo/route.ts".to_string(),
            source: r#"export const POST = async (request: Request) => {
  const body = await request.json();
  return {
    ok: true,
    payload: body,
    name: body.name,
    status: 202,
  };
};
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/echo".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "name": "Ada",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("const route handler");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["payload"]["name"], "Ada");
    assert_eq!(response.body["name"], "Ada");
}

#[test]
fn react_route_handler_runtime_executes_exported_ai_agent_factory_readiness() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/ai/agent/route.ts".to_string(),
            source: r#"import { createDxAgentRoute } from "@/lib/ai/agent";

export const POST = createDxAgentRoute({
  readStatus: () => ({ ok: true }),
});
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/ai/agent".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "message": "Check launch agent readiness",
                "requestId": "dx-agent-readiness-test",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("DX_ENABLE_EXTENDED_AI_ROUTES", "true"),
                ("AI_PROVIDER_API_KEY", "redacted-present"),
            ]),
        },
    )
    .expect("const AI agent factory route handler");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-ai-provider-boundary-interpreter"
    );
    assert_eq!(response.body["schema"], "dx.ai.provider_readiness");
    assert_eq!(
        response.body["status"],
        "provider-configured-readiness-only"
    );
    assert_eq!(response.body["packageId"], "ai/vercel-ai");
    assert_eq!(response.body["endpoint"], "/api/ai/agent");
    assert_eq!(response.body["capability"], "agent-loop");
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["providerBoundary"], true);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(response.body["providerRuntime"], false);
}

#[test]
fn react_route_handler_runtime_executes_exported_ai_chat_factory_readiness() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/ai/chat/route.ts".to_string(),
            source: r#"import { createDxAIChatRoute } from "@/lib/ai/route";

export const POST = createDxAIChatRoute({
  system: "You are a source-owned launch assistant.",
});
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/ai/chat".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "message": "Draft a launch handoff",
                "requestId": "dx-chat-factory-readiness-test",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[("AI_PROVIDER_API_KEY", "redacted-present")]),
        },
    )
    .expect("const AI chat factory route handler");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-ai-provider-boundary-interpreter"
    );
    assert_eq!(response.body["schema"], "dx.ai.provider_readiness");
    assert_eq!(
        response.body["status"],
        "provider-configured-readiness-only"
    );
    assert_eq!(response.body["packageId"], "ai/vercel-ai");
    assert_eq!(response.body["endpoint"], "/api/ai/chat");
    assert_eq!(response.body["capability"], "chat-stream");
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["providerBoundary"], true);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(response.body["providerRuntime"], false);
    assert_eq!(
        response.body["request"]["message"],
        "Draft a launch handoff"
    );
    assert_eq!(
        response.body["request"]["requestId"],
        "dx-chat-factory-readiness-test"
    );
}

#[test]
fn react_route_handler_runtime_uses_get_handler_for_head_requests_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/status/route.ts".to_string(),
            source: r#"export function GET() {
  return {
    ok: true,
    method: "GET",
  };
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "HEAD".to_string(),
            path: "/api/status".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("HEAD request should use safe GET route handler");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["method"], "GET");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_prefers_explicit_head_export_before_get_fallback() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/status/route.ts".to_string(),
            source: r#"export function GET() {
  return {
    ok: true,
    method: "GET",
    getOnly: true,
  };
}

export function HEAD() {
  return {
    ok: true,
    method: "HEAD",
    headOnly: true,
  };
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "HEAD".to_string(),
            path: "/api/status".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("HEAD request should prefer explicit HEAD route handler");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["method"], "HEAD");
    assert_eq!(response.body["headOnly"], true);
    assert_eq!(response.body.get("getOnly"), None);
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_answers_options_from_exported_methods_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/status/route.ts".to_string(),
            source: r#"export function GET() {
  return {
    ok: true,
  };
}

export async function POST(request: Request) {
  return Response.json({
    ok: true,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "OPTIONS".to_string(),
            path: "/api/status".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("OPTIONS request should use source-owned method discovery");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.headers.get("Allow").map(String::as_str),
        Some("GET, HEAD, POST, OPTIONS")
    );
    assert_eq!(
        response
            .headers
            .get("x-dx-route-handler-options")
            .map(String::as_str),
        Some("source-owned-method-discovery")
    );
    assert_eq!(response.body["allowedMethods"][0], "GET");
    assert_eq!(response.body["allowedMethods"][1], "HEAD");
    assert_eq!(response.body["allowedMethods"][2], "POST");
    assert_eq!(response.body["allowedMethods"][3], "OPTIONS");
    assert_eq!(response.body["nodeModulesRequired"], false);
    assert_eq!(response.body["runtimeBoundary"]["sourceOwned"], true);
    assert_eq!(
        response.body["runtimeBoundary"]["externalRuntimeRequired"],
        false
    );
    assert_eq!(
        response.body["runtimeBoundary"]["externalRuntimeExecuted"],
        false
    );
    assert!(response.body.get("fullNextRouteHandlerParity").is_none());
    assert_eq!(
        response.execution_model,
        "source-owned-route-handler-options"
    );
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_returns_405_for_unexported_methods_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/submit/route.ts".to_string(),
            source: r#"export async function POST(request: Request) {
  return Response.json({
    ok: true,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/submit".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("unexported method should return source-owned 405 response");

    assert_eq!(response.status, 405);
    assert_eq!(
        response.headers.get("Allow").map(String::as_str),
        Some("POST, OPTIONS")
    );
    assert_eq!(response.body["ok"], false);
    assert_eq!(response.body["method"], "GET");
    assert_eq!(response.body["allowedMethods"][0], "POST");
    assert_eq!(response.body["allowedMethods"][1], "OPTIONS");
    assert_eq!(response.body["methodNotAllowed"], true);
    assert_eq!(response.body["nodeModulesRequired"], false);
    assert_eq!(response.body["runtimeBoundary"]["sourceOwned"], true);
    assert_eq!(
        response.body["runtimeBoundary"]["externalRuntimeRequired"],
        false
    );
    assert_eq!(
        response.body["runtimeBoundary"]["externalRuntimeExecuted"],
        false
    );
    assert!(response.body.get("fullNextRouteHandlerParity").is_none());
    assert_eq!(
        response.execution_model,
        "source-owned-route-handler-method-guard"
    );
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_serves_source_owned_trpc_health_compat() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/trpc/[trpc]/route.ts".to_string(),
            source: r#"import { dxTrpcRouteHandler } from "@/lib/trpc/route-handler";

export const GET = dxTrpcRouteHandler;
export const POST = dxTrpcRouteHandler;
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/trpc/health".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::from([("trpc".to_string(), "health".to_string())]),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("source-owned trpc compat route handler");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-trpc-compat-interpreter"
    );
    assert_eq!(response.body["status"], "ok");
    assert_eq!(response.body["procedure"], "health.query");
    assert_eq!(
        response
            .headers
            .get("x-dx-route-handler-receipt")
            .map(String::as_str),
        Some("dx.next.appRouteHandlerReceipt")
    );
    assert_eq!(
        response.headers.get("x-dx-trpc-compat").map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
}

#[test]
fn react_route_handler_runtime_serves_source_owned_trpc_health_absolute_url() {
    let request_url = "https://example.test/api/trpc/health?batch=1#status";
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/trpc/[trpc]/route.ts".to_string(),
            source: r#"import { dxTrpcRouteHandler } from "@/lib/trpc/route-handler";

export const GET = dxTrpcRouteHandler;
export const POST = dxTrpcRouteHandler;
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: request_url.to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::from([("trpc".to_string(), "health".to_string())]),
            search_params: BTreeMap::from([("batch".to_string(), "1".to_string())]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("source-owned trpc compat route handler accepts absolute URL");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-trpc-compat-interpreter"
    );
    assert_eq!(response.body["procedure"], "health.query");
    assert_eq!(
        response.headers.get("x-dx-trpc-compat").map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
}

#[test]
fn react_route_handler_runtime_executes_instant_route_missing_config_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/instant/route.ts".to_string(),
            source: r#"import { createDxInstantRouteHandlers } from "@/lib/instant/route";

export const { GET, POST } = createDxInstantRouteHandlers();
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/instant".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "op": "ping",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("instant route handler missing-config response");

    assert_eq!(response.status, 501);
    assert_eq!(
        response.execution_model,
        "source-owned-instantdb-route-handler-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-instant-route-handler")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], false);
    assert_eq!(response.body["httpStatus"], 501);
    assert_eq!(response.body["providerBoundary"], true);
    assert_eq!(
        response.body["schema"],
        "dx.www.template.instantdb_route_handler"
    );
    assert_eq!(response.body["route"], "/api/instant");
    assert_eq!(response.body["packageId"], "instantdb/react");
    assert_eq!(response.body["upstreamPackage"], "@instantdb/react");
    assert_eq!(
        response.body["requiredEnv"][0],
        "NEXT_PUBLIC_INSTANT_APP_ID"
    );
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["nodeModulesRequired"], false);
    assert_eq!(response.body["runtimeProof"], false);
}

#[test]
fn react_route_handler_runtime_marks_instant_post_configured_without_provider_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/instant/route.ts".to_string(),
            source: r#"import { createDxInstantRouteHandlers } from "@/lib/instant/route";

export const { GET, POST } = createDxInstantRouteHandlers();
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/instant".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "op": "launchPing",
                "requestId": "dx-instant-post-test",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[("NEXT_PUBLIC_INSTANT_APP_ID", "dx-local-app")]),
        },
    )
    .expect("instant route handler configured dry-run response");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-instantdb-route-handler-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["status"], "configured-source-owned-dry-run");
    assert_eq!(response.body["httpStatus"], 202);
    assert_eq!(response.body["operation"], "launchPing");
    assert_eq!(response.body["appIdConfigured"], true);
    assert_eq!(
        response.body["providerBoundary"], true,
        "provider boundary stays visible even after local app id readiness"
    );
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["nodeModulesRequired"], false);
    assert_eq!(response.body["runtimeProof"], false);
}

#[test]
fn react_route_handler_runtime_executes_database_api_readiness_helper_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/database-api/readiness/route.ts".to_string(),
            source: r#"import { createDatabaseApiReadinessResponse } from "../../../../server/database-api/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createDatabaseApiReadinessResponse();
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/database-api/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("database api readiness route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-database-api-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response.body["schema"],
        "dx.www.template.database_api_readiness"
    );
    assert_eq!(
        response.body["sourceContract"]["schema"],
        "dx.www.template.database_api_source_contract"
    );
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["hostedCredentials"], false);
    assert_eq!(response.body["packages"].as_array().map(Vec::len), Some(4));
    assert_eq!(response.body["packages"][3]["packageId"], "api/trpc");
}

#[test]
fn react_route_handler_runtime_executes_database_orm_readiness_helper_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/database-orm/readiness/route.ts".to_string(),
            source: r#"import { createDatabaseOrmReadinessResponse } from "../../../../server/database-orm/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createDatabaseOrmReadinessResponse();
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/database-orm/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("database orm readiness route");

    assert_eq!(response.status, 501);
    assert_eq!(
        response.execution_model,
        "source-owned-database-orm-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response
            .headers
            .get("x-dx-database-orm-readiness")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(
        response.body["schema"],
        "dx.www.template.database_orm_readiness"
    );
    assert_eq!(response.body["packageId"], "db/drizzle-sqlite");
    assert_eq!(response.body["officialName"], "Database ORM");
    assert_eq!(response.body["route"], "/api/database-orm/readiness");
    assert_eq!(response.body["status"], "runtime-gated");
    assert_eq!(response.body["httpStatus"], 501);
    assert_eq!(response.body["runtimeProof"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["hostedCredentials"], false);
    assert_eq!(
        response.body["missingConfig"].as_array().map(Vec::len),
        Some(3)
    );
    assert_eq!(
        response.body["missingConfig"][0],
        "DX_DATABASE_URL or DX_SQLITE_DATABASE_PATH"
    );
    assert_eq!(
        response.body["sourceOwnedSurfaces"]
            .as_array()
            .map(Vec::len),
        Some(4)
    );
    assert_eq!(response.body["schemaTables"][0], "users");
    assert_eq!(response.body["schemaTables"][1], "posts");
}

#[test]
fn react_route_handler_runtime_executes_supabase_readiness_helper_without_provider_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/supabase/readiness/route.ts".to_string(),
            source: r#"import { createSupabaseReadinessResponse } from "../../../../server/supabase/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createSupabaseReadinessResponse();
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/supabase/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("supabase readiness route");

    assert_eq!(response.status, 501);
    assert_eq!(
        response.execution_model,
        "source-owned-supabase-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response
            .headers
            .get("x-dx-supabase-readiness")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(
        response.body["schema"],
        "dx.www.template.supabase_readiness"
    );
    assert_eq!(response.body["packageId"], "supabase/client");
    assert_eq!(response.body["officialName"], "Backend Platform Client");
    assert_eq!(response.body["route"], "/api/supabase/readiness");
    assert_eq!(response.body["status"], "provider-gated");
    assert_eq!(response.body["httpStatus"], 501);
    assert_eq!(response.body["runtimeProof"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["hostedCredentials"], false);
    assert_eq!(
        response.body["missingEnv"].as_array().map(Vec::len),
        Some(2)
    );
    assert_eq!(response.body["missingEnv"][0], "NEXT_PUBLIC_SUPABASE_URL");
    assert_eq!(
        response.body["missingEnv"][1],
        "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"
    );
    assert_eq!(response.body["validationError"], serde_json::Value::Null);
    assert_eq!(response.body["localProject"], serde_json::Value::Null);
    assert_eq!(
        response.body["sourceOwnedSurfaces"]
            .as_array()
            .map(Vec::len),
        Some(5)
    );
    assert_eq!(
        response.body["appOwnedBoundary"].as_array().map(Vec::len),
        Some(4)
    );
}

#[test]
fn react_route_handler_runtime_executes_instant_readiness_helper_without_provider_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/instant/readiness/route.ts".to_string(),
            source: r#"import { createInstantReadinessResponse } from "../../../../server/instant/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createInstantReadinessResponse();
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/instant/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("instant readiness route");

    assert_eq!(response.status, 501);
    assert_eq!(
        response.execution_model,
        "source-owned-instant-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response
            .headers
            .get("x-dx-instant-readiness")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["schema"], "dx.www.template.instant_readiness");
    assert_eq!(response.body["packageId"], "instantdb/react");
    assert_eq!(response.body["officialName"], "Realtime App Database");
    assert_eq!(response.body["route"], "/api/instant/readiness");
    assert_eq!(response.body["status"], "provider-gated");
    assert_eq!(response.body["httpStatus"], 501);
    assert_eq!(response.body["runtimeProof"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["hostedCredentials"], false);
    assert_eq!(
        response.body["missingEnv"].as_array().map(Vec::len),
        Some(1)
    );
    assert_eq!(response.body["missingEnv"][0], "NEXT_PUBLIC_INSTANT_APP_ID");
    assert_eq!(response.body["validationError"], serde_json::Value::Null);
    assert_eq!(
        response.body["sourceOwnedSurfaces"]
            .as_array()
            .map(Vec::len),
        Some(6)
    );
    assert_eq!(
        response.body["appOwnedBoundary"].as_array().map(Vec::len),
        Some(4)
    );
}

#[test]
fn react_route_handler_runtime_marks_launch_readiness_configured_without_runtime_proof() {
    let database_orm = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/database-orm/readiness/route.ts".to_string(),
            source: r#"import { createDatabaseOrmReadinessResponse } from "../../../../server/database-orm/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createDatabaseOrmReadinessResponse();
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/database-orm/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("DX_DATABASE_URL", "redacted-present"),
                ("DX_DATABASE_MIGRATIONS_REVIEWED", "true"),
                ("DX_DATABASE_AUTHORIZATION_REVIEWED", "true"),
            ]),
        },
    )
    .expect("configured database orm readiness route");

    assert_eq!(database_orm.status, 200);
    assert_eq!(
        database_orm.body["status"],
        "configured-source-owned-adapter-boundary"
    );
    assert_eq!(
        database_orm.body["missingConfig"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(database_orm.body["runtimeProof"], false);
    assert_eq!(database_orm.body["networkCalls"], false);

    let supabase = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/supabase/readiness/route.ts".to_string(),
            source: r#"import { createSupabaseReadinessResponse } from "../../../../server/supabase/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createSupabaseReadinessResponse();
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/supabase/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("NEXT_PUBLIC_SUPABASE_URL", "http://127.0.0.1:54321"),
                ("NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY", "anon-public-key"),
            ]),
        },
    )
    .expect("configured supabase readiness route");

    assert_eq!(supabase.status, 200);
    assert_eq!(
        supabase.body["status"],
        "configured-source-owned-adapter-boundary"
    );
    assert_eq!(
        supabase.body["missingEnv"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(supabase.body["validationError"], serde_json::Value::Null);
    assert_eq!(supabase.body["localProject"], true);
    assert_eq!(supabase.body["runtimeProof"], false);
    assert_eq!(supabase.body["networkCalls"], false);

    let instant = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/instant/readiness/route.ts".to_string(),
            source: r#"import { createInstantReadinessResponse } from "../../../../server/instant/readiness.ts";

export const dynamic = "force-dynamic";

export function GET() {
  return createInstantReadinessResponse();
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/instant/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("NEXT_PUBLIC_INSTANT_APP_ID", "launch-app"),
                ("NEXT_PUBLIC_INSTANT_DEVTOOL", "false"),
                ("NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT", "128"),
            ]),
        },
    )
    .expect("configured instant readiness route");

    assert_eq!(instant.status, 200);
    assert_eq!(
        instant.body["status"],
        "configured-source-owned-adapter-boundary"
    );
    assert_eq!(instant.body["missingEnv"].as_array().map(Vec::len), Some(0));
    assert_eq!(instant.body["validationError"], serde_json::Value::Null);
    assert_eq!(instant.body["runtimeProof"], false);
    assert_eq!(instant.body["networkCalls"], false);
}

#[test]
fn react_route_handler_runtime_executes_query_cache_readiness_helper_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/query-cache/readiness/route.ts".to_string(),
            source: r#"import { createDataFetchingCacheReadinessResponse } from "@/server/query-cache/readiness";

export const dynamic = "force-dynamic";

export function GET(request: Request) {
  return createDataFetchingCacheReadinessResponse(request);
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/query-cache/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([
                ("filter".to_string(), "review".to_string()),
                ("optimistic".to_string(), "queued".to_string()),
                ("visible".to_string(), "2".to_string()),
            ]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("query cache readiness route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-data-fetching-cache-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response.body["schema"],
        "dx.www.template.data_fetching_cache_readiness"
    );
    assert_eq!(response.body["route"], "/api/query-cache/readiness");
    assert_eq!(response.body["nodeModulesRequired"], false);
    assert_eq!(
        response.body["adapterBoundary"],
        "queryclient-adapter-required"
    );
    assert_eq!(
        response.body["cache"]["queryKey"],
        "dx:dashboard:projects:review"
    );
    assert_eq!(response.body["cache"]["readyEntryCount"], 2);
    assert_eq!(response.body["cache"]["optimisticState"], "queued");
}

#[test]
fn react_route_handler_runtime_executes_query_cache_action_helper_without_query_client_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/query-cache/readiness/route.ts".to_string(),
            source: r#"import {
  createDataFetchingCacheActionResponse,
  createDataFetchingCacheReadinessResponse,
} from "@/server/query-cache/readiness";

export const dynamic = "force-dynamic";

export function GET(request: Request) {
  return createDataFetchingCacheReadinessResponse(request);
}

export async function POST(request: Request) {
  return createDataFetchingCacheActionResponse(request);
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/query-cache/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "action": "refresh",
                "filter": "launch",
                "optimisticState": "queued",
                "queryKey": "dx:dashboard:projects:launch",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([
                ("filter".to_string(), "ignored-by-body".to_string()),
                ("optimistic".to_string(), "idle".to_string()),
            ]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("query cache dry-run action route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-data-fetching-cache-action-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response
            .headers
            .get("x-dx-query-cache-action")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(
        response.body["schema"],
        "dx.www.template.data_fetching_cache_action_receipt"
    );
    assert_eq!(response.body["route"], "/api/query-cache/readiness");
    assert_eq!(response.body["packageId"], "tanstack/query");
    assert_eq!(response.body["status"], "source-owned-cache-action-dry-run");
    assert_eq!(response.body["action"], "refresh");
    assert_eq!(response.body["queryKey"], "dx:dashboard:projects:launch");
    assert_eq!(response.body["runtimeProof"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["nodeModulesRequired"], false);
    assert_eq!(response.body["queryClientExecution"], false);
    assert_eq!(
        response.body["adapterBoundary"],
        "queryclient-adapter-required"
    );
    assert_eq!(response.body["cache"]["cacheEntryCount"], 2);
    assert_eq!(response.body["cache"]["readyEntryCount"], 2);
    assert_eq!(response.body["cache"]["staleEntryCount"], 0);
    assert_eq!(response.body["cache"]["invalidatedEntryCount"], 0);
    assert_eq!(response.body["cache"]["optimisticEntryCount"], 1);
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_executes_auth_readiness_helper_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/auth/readiness/route.ts".to_string(),
            source:
                r#"import { createTemplateBetterAuthReadiness } from "@/server/auth/better-auth";

export const runtime = "nodejs";

export function GET() {
  const readiness = createTemplateBetterAuthReadiness();

  return Response.json(
    {
      ok: true,
      ...readiness,
      packageReadinessStatus: readiness.status,
      status: readiness.canRunRouteHandlers
        ? "ready"
        : "adapter-boundary",
      liveRouteHandlersHttpStatus: readiness.canRunRouteHandlers ? 200 : 501,
      runtimeExecution: false,
      liveSessionExecution: false,
      adapter: "better-auth",
      officialPackageName: "Authentication",
      upstreamPackage: "better-auth",
      databaseAdapterConfigured: readiness.databaseAdapterConfigured,
      sessionStorage: readiness.sessionStorage,
      adapterBoundaries: readiness.adapterBoundaries,
      databaseBoundary: readiness.databaseBoundary,
      migrationsRequired: readiness.migrationsRequired,
    },
    { status: 200 },
  );
}
"#
                .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/auth/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("auth readiness route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-template-better-auth-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-auth-readiness")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["status"], "adapter-boundary");
    assert_eq!(response.body["packageReadinessStatus"], "missing-config");
    assert_eq!(response.body["liveRouteHandlersHttpStatus"], 501);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["liveSessionExecution"], false);
    assert_eq!(response.body["adapter"], "better-auth");
    assert_eq!(response.body["credentialsConfigured"], false);
    assert_eq!(response.body["databaseAdapterConfigured"], false);
    assert_eq!(response.body["sessionStorage"], "app-owned");
    assert_eq!(response.body["canRunRouteHandlers"], false);
    assert_eq!(response.body["migrationsRequired"], true);
    assert_eq!(
        response.body["missingConfig"].as_array().map(Vec::len),
        Some(4)
    );
    assert_eq!(
        response.body["databaseBoundary"]["schema"],
        "dx.template.authentication.database_boundary"
    );
    assert_eq!(response.body["databaseBoundary"]["appOwned"], true);
    assert_eq!(response.body["databaseBoundary"]["runtimeProof"], false);
}

#[test]
fn react_route_handler_runtime_marks_auth_credentials_configured_without_database_adapter() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/auth/readiness/route.ts".to_string(),
            source:
                r#"import { createTemplateBetterAuthReadiness } from "@/server/auth/better-auth";

export const runtime = "nodejs";

export function GET() {
  const readiness = createTemplateBetterAuthReadiness();

  return Response.json(
    {
      ok: true,
      ...readiness,
      packageReadinessStatus: readiness.status,
      status: readiness.canRunRouteHandlers
        ? "ready"
        : "adapter-boundary",
      liveRouteHandlersHttpStatus: readiness.canRunRouteHandlers ? 200 : 501,
      runtimeExecution: false,
      liveSessionExecution: false,
      adapter: "better-auth",
      officialPackageName: "Authentication",
      upstreamPackage: "better-auth",
      databaseAdapterConfigured: readiness.databaseAdapterConfigured,
      sessionStorage: readiness.sessionStorage,
      adapterBoundaries: readiness.adapterBoundaries,
      databaseBoundary: readiness.databaseBoundary,
      migrationsRequired: readiness.migrationsRequired,
    },
    { status: 200 },
  );
}
"#
                .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/auth/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("BETTER_AUTH_SECRET", "redacted-present"),
                ("BETTER_AUTH_URL", "https://auth.example.test"),
                ("GOOGLE_CLIENT_ID", "dx-google-client"),
                ("GOOGLE_CLIENT_SECRET", "redacted-present"),
            ]),
        },
    )
    .expect("configured auth readiness route");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["credentialsConfigured"], true);
    assert_eq!(response.body["packageReadinessStatus"], "configured");
    assert_eq!(response.body["databaseAdapterConfigured"], false);
    assert_eq!(response.body["canRunRouteHandlers"], false);
    assert_eq!(response.body["liveRouteHandlersHttpStatus"], 501);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["liveSessionExecution"], false);
    assert_eq!(response.body["baseURL"], "https://auth.example.test");
    assert_eq!(
        response.body["missingConfig"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(response.body["databaseBoundary"]["appOwned"], true);
}

#[test]
fn react_route_handler_runtime_executes_auth_catch_all_missing_config_without_next_runtime() {
    let route_source = r#"export const runtime = "nodejs";

export { GET, POST } from "@/server/auth/better-auth";
"#;

    for (method, path) in [
        ("GET", "/api/auth/session"),
        ("POST", "/api/auth/sign-in/email"),
    ] {
        let response = execute_react_route_handler(
            &DxReactServerSource {
                kind: DxReactServerSourceKind::RouteHandler,
                source_path: "app/api/auth/[...all]/route.ts".to_string(),
                source: route_source.to_string(),
            },
            DxReactRouteHandlerRequest {
                method: method.to_string(),
                path: path.to_string(),
                headers: BTreeMap::new(),
                body: serde_json::Value::Null,
                route_params: BTreeMap::from([("all".to_string(), path.to_string())]),
                search_params: BTreeMap::new(),
                runtime_env: BTreeMap::new(),
            },
        )
        .expect("auth catch-all missing-config route");

        assert_eq!(response.status, 501);
        assert_eq!(
            response.execution_model,
            "source-owned-template-better-auth-route-handler-interpreter"
        );
        assert!(!response.lifecycle_scripts_executed);
        assert_eq!(
            response
                .headers
                .get("x-dx-auth-route-handler")
                .map(String::as_str),
            Some("source-owned-safe-interpreter")
        );
        assert_eq!(response.body["ok"], false);
        assert_eq!(response.body["status"], "adapter-boundary");
        assert_eq!(response.body["httpStatus"], 501);
        assert_eq!(response.body["method"], method);
        assert_eq!(response.body["adapter"], "better-auth");
        assert_eq!(response.body["runtimeExecution"], false);
        assert_eq!(response.body["liveSessionExecution"], false);
        assert_eq!(response.body["credentialsConfigured"], false);
        assert_eq!(response.body["databaseAdapterConfigured"], false);
        assert_eq!(response.body["sessionStorage"], "app-owned");
        assert_eq!(response.body["migrationsRequired"], true);
        assert_eq!(
            response.body["missingConfig"].as_array().map(Vec::len),
            Some(4)
        );
        assert_eq!(
            response.body["databaseBoundary"]["schema"],
            "dx.template.authentication.database_boundary"
        );
        assert_eq!(response.body["databaseBoundary"]["appOwned"], true);
    }
}

#[test]
fn react_route_handler_runtime_executes_n8n_dry_run_without_provider_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/automations/n8n/dry-run/route.ts".to_string(),
            source:
                r#"import { buildDxN8nCredentialReadiness } from "@/lib/automations/n8n/readiness";
import { createDxN8nRunReceipt } from "@/lib/automations/n8n/receipt";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function POST(request: Request) {
  const body = await request.json();
  const receipt = createDxN8nRunReceipt({
    connector,
    intent,
    mode,
    workflowId,
  });

  return Response.json(
    {
      ok: receipt.status !== "blocked-missing-config",
      packageId: "automations/n8n",
      status:
        receipt.status === "blocked-missing-config"
          ? "missing-config"
          : "local-dry-run",
      httpStatus: receipt.status === "blocked-missing-config" ? 501 : 202,
      providerBoundary: receipt.status === "blocked-missing-config",
      readiness: buildDxN8nCredentialReadiness(connector),
      receipt,
      runtimeExecution: false,
      secretValues: [],
    },
    { status: receipt.status === "blocked-missing-config" ? 501 : 202 },
  );
}
"#
                .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/automations/n8n/dry-run".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "connectorId": "manual-trigger",
                "mode": "dry-run",
                "workflowId": "manual-trigger-launch-check",
                "intent": "Draft launch handoff",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("n8n dry-run route");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-automation-n8n-dry-run-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-automation-n8n-dry-run")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["packageId"], "automations/n8n");
    assert_eq!(response.body["status"], "local-dry-run");
    assert_eq!(response.body["httpStatus"], 202);
    assert_eq!(
        response.body["providerBoundary"], true,
        "provider boundary stays visible even after credentials are present"
    );
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(
        response.body["receipt"]["schema"],
        "dx.automation.n8n.run_receipt"
    );
    assert_eq!(response.body["receipt"]["connectorId"], "manual-trigger");
    assert_eq!(response.body["receipt"]["mode"], "dry-run");
    assert_eq!(response.body["receipt"]["runtimeExecution"], false);
    assert_eq!(
        response.body["readiness"]["requiredEnv"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_executes_ai_provider_boundary_without_model_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/ai/chat/route.ts".to_string(),
            source:
                r#"import { createDxAiMissingProviderResponse } from "@/lib/ai/provider-boundary";

export async function POST(request: Request) {
  const body = await readJsonBody(request);

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "chat-stream",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY in the app environment to stream model output.",
    });
  }

  return Response.json(
    {
      ok: true,
      status: "provider-configured-dry-run-only",
      httpStatus: 202,
      provider: "openai-compatible",
      message: typeof body.message === "string" ? body.message : undefined,
      requestId: typeof body.requestId === "string" ? body.requestId : undefined,
      credentialsConfigured: true,
      adapterBoundary: "provider-credential-boundary",
      runtimeExecution: false,
      modelStreaming: false,
      providerRuntime: false,
      secretValues: [],
    },
    { status: 202 },
  );
}
"#
                .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/ai/chat".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "message": "Draft a launch handoff",
                "requestId": "req_ai_boundary",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("ai provider boundary route");

    assert_eq!(response.status, 501);
    assert_eq!(
        response.execution_model,
        "source-owned-ai-provider-boundary-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-ai-provider-boundary")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], false);
    assert_eq!(response.body["schema"], "dx.ai.provider_boundary");
    assert_eq!(response.body["packageId"], "ai/vercel-ai");
    assert_eq!(response.body["status"], "missing-config");
    assert_eq!(response.body["httpStatus"], 501);
    assert_eq!(response.body["provider"], "openai-compatible");
    assert_eq!(response.body["capability"], "chat-stream");
    assert_eq!(response.body["credentialsConfigured"], false);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["modelStreaming"], false);
    assert_eq!(response.body["providerRuntime"], false);
    assert_eq!(response.body["providerBoundary"], true);
    assert_eq!(
        response.body["requiredEnv"].as_array().map(Vec::len),
        Some(1)
    );
    assert_eq!(response.body["requiredEnv"][0], "AI_PROVIDER_API_KEY");
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_marks_ai_chat_configured_without_model_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/ai/chat/route.ts".to_string(),
            source:
                r#"import { createDxAiMissingProviderResponse } from "@/lib/ai/provider-boundary";

export async function POST(request: Request) {
  const body = await readJsonBody(request);

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "chat-stream",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY in the app environment to stream model output.",
    });
  }

  return Response.json(
    {
      ok: true,
      status: "provider-configured-readiness-only",
      httpStatus: 202,
      provider: "openai-compatible",
      message: typeof body.message === "string" ? body.message : undefined,
      requestId: typeof body.requestId === "string" ? body.requestId : undefined,
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      adapterBoundary: "provider-credential-boundary",
      runtimeExecution: false,
      modelStreaming: false,
      providerRuntime: false,
      runtimeProof: false,
      liveProviderProof: false,
      secretValues: [],
    },
    { status: 202 },
  );
}
"#
                .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/ai/chat".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "message": "Draft a launch handoff",
                "requestId": "req_ai_configured",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::from([(
                "AI_PROVIDER_API_KEY".to_string(),
                "redacted-present".to_string(),
            )]),
        },
    )
    .expect("configured ai provider readiness route");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-ai-provider-boundary-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-ai-provider-boundary")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["schema"], "dx.ai.provider_readiness");
    assert_eq!(response.body["packageId"], "ai/vercel-ai");
    assert_eq!(
        response.body["status"],
        "provider-configured-readiness-only"
    );
    assert_eq!(response.body["httpStatus"], 202);
    assert_eq!(response.body["provider"], "openai-compatible");
    assert_eq!(response.body["capability"], "chat-stream");
    assert_eq!(response.body["credentialsConfigured"], true);
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(response.body["modelStreaming"], false);
    assert_eq!(response.body["providerRuntime"], false);
    assert_eq!(response.body["runtimeProof"], false);
    assert_eq!(response.body["liveProviderProof"], false);
    assert_eq!(
        response.body["providerBoundary"], true,
        "provider boundary stays visible even after credentials are present"
    );
    assert_eq!(response.body["message"], "Draft a launch handoff");
    assert_eq!(response.body["requestId"], "req_ai_configured");
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_executes_ai_text_stream_helper_as_local_receipt() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/ai/text-stream/route.ts".to_string(),
            source: r#"import {
  createDxLaunchTextStream,
  createDxLaunchTextStreamResponse,
} from "@/lib/ai/text-stream";

export async function GET(): Promise<Response> {
  const textStream = createDxLaunchTextStream({
    chunks: [
      "DX text stream bridge is wired.\n",
      "Connect it to live launch receipts before production.\n",
    ],
  });

  return createDxLaunchTextStreamResponse({ textStream });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/ai/text-stream".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("ai text stream route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-ai-local-stream-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-ai-local-stream")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["schema"], "dx.ai.local_stream_receipt");
    assert_eq!(response.body["packageId"], "ai/vercel-ai");
    assert_eq!(response.body["endpoint"], "/api/ai/text-stream");
    assert_eq!(response.body["streamKind"], "text-stream");
    assert_eq!(
        response.body["adapterBoundary"],
        "ai-sdk-stream-adapter-boundary"
    );
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["providerRuntime"], false);
    assert_eq!(response.body["modelStreaming"], false);
    assert_eq!(response.body["liveStreamingTransport"], false);
    assert_eq!(response.body["chunks"].as_array().map(Vec::len), Some(2));
    assert!(
        response.body["text"]
            .as_str()
            .is_some_and(|text| text.contains("DX text stream bridge is wired."))
    );
}

#[test]
fn react_route_handler_runtime_executes_ai_ui_stream_helper_as_local_receipt() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/ai/ui-stream/route.ts".to_string(),
            source: r#"import {
  createDxLaunchUIMessageStream,
  createDxLaunchUIMessageStreamResponse,
} from "@/lib/ai/ui-message-stream";

export async function POST(request: Request): Promise<Response> {
  const body = (await request.json()) as { text?: string };
  const text =
    body.text ??
    "DX UI message stream bridge is wired. Connect it to real route work before launch.";

  const stream = createDxLaunchUIMessageStream({
    execute: (writer) => {
      writer.write({ type: "text-start", id: "dx-launch-ui-stream" });
      writer.write({
        type: "text-delta",
        id: "dx-launch-ui-stream",
        delta: text,
      });
      writer.write({ type: "text-end", id: "dx-launch-ui-stream" });
    },
  });

  return createDxLaunchUIMessageStreamResponse({ stream });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/ai/ui-stream".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "text": "Launch briefing stream",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("ai ui stream route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-ai-local-stream-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["schema"], "dx.ai.local_stream_receipt");
    assert_eq!(response.body["endpoint"], "/api/ai/ui-stream");
    assert_eq!(response.body["streamKind"], "ui-message-stream");
    assert_eq!(response.body["request"]["text"], "Launch briefing stream");
    assert_eq!(response.body["events"].as_array().map(Vec::len), Some(3));
    assert_eq!(
        response.body["events"][1]["delta"],
        "Launch briefing stream"
    );
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["providerRuntime"], false);
    assert_eq!(response.body["modelStreaming"], false);
    assert_eq!(response.body["liveStreamingTransport"], false);
}

#[test]
fn react_route_handler_runtime_executes_fumadocs_search_factory_as_local_receipt() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search/route.ts".to_string(),
            source: r#"import { createDxFumadocsSearchApi } from "@/lib/fumadocs/search";

const searchApi = createDxFumadocsSearchApi();

export const GET = searchApi.GET;
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/search".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([
                ("query".to_string(), "launch".to_string()),
                ("limit".to_string(), "2".to_string()),
            ]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("fumadocs search route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-fumadocs-search-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-fumadocs-search")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["schema"], "dx.fumadocs.search_receipt");
    assert_eq!(response.body["packageId"], "content/fumadocs-next");
    assert_eq!(response.body["route"], "/api/search");
    assert_eq!(response.body["mode"], "dynamic");
    assert_eq!(response.body["query"], "launch");
    assert_eq!(
        response.body["adapterBoundary"],
        "createFromSource-adapter-boundary"
    );
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["fumadocsRuntime"], false);
    assert_eq!(response.body["nodeModulesRequired"], false);
    assert_eq!(response.body["results"].as_array().map(Vec::len), Some(1));
    assert_eq!(response.body["results"][0]["title"], "DX Launch Docs");
    assert_eq!(response.body["results"][0]["route"], "/docs");
}

#[test]
fn react_route_handler_runtime_executes_fumadocs_static_search_factory_as_local_index() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search-static/route.ts".to_string(),
            source: r#"import { createDxFumadocsSearchApi } from "@/lib/fumadocs/search";

export const revalidate = false;

const searchApi = createDxFumadocsSearchApi();

export const GET = searchApi.staticGET;
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/search-static".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("fumadocs static search route");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-fumadocs-search-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["schema"], "dx.fumadocs.search_receipt");
    assert_eq!(response.body["route"], "/api/search-static");
    assert_eq!(response.body["mode"], "static-index");
    assert_eq!(response.body["staticIndex"], true);
    assert_eq!(response.body["totalIndexedPages"], 1);
    assert_eq!(response.body["results"].as_array().map(Vec::len), Some(1));
    assert_eq!(response.body["results"][0]["route"], "/docs");
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["fumadocsRuntime"], false);
}

#[test]
fn react_route_handler_runtime_classifies_fumadocs_openapi_proxy_factory_as_boundary() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/openapi/proxy/route.ts".to_string(),
            source: r#"import {
  dxFumadocsOpenAPI,
  readDxFumadocsOpenAPIAllowedOrigins,
} from "@/lib/fumadocs/openapi";

const allowedOrigins = readDxFumadocsOpenAPIAllowedOrigins();

export const { GET, HEAD, PUT, POST, PATCH, DELETE } =
  dxFumadocsOpenAPI.createProxy({
    allowedOrigins,
    filterRequest(request) {
      return allowedOrigins.length > 0 && request.url.startsWith("https://");
    },
  });
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/openapi/proxy".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("fumadocs openapi proxy route");

    assert_eq!(response.status, 501);
    assert_eq!(
        response.execution_model,
        "source-owned-fumadocs-openapi-proxy-boundary-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-fumadocs-openapi-proxy")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], false);
    assert_eq!(
        response.body["schema"],
        "dx.fumadocs.openapi_proxy_boundary"
    );
    assert_eq!(response.body["packageId"], "content/fumadocs-next");
    assert_eq!(response.body["route"], "/api/openapi/proxy");
    assert_eq!(response.body["method"], "GET");
    assert_eq!(
        response.body["status"],
        "openapi-proxy-runtime-adapter-boundary"
    );
    assert_eq!(
        response.body["requiredEnv"][0],
        "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"
    );
    assert_eq!(
        response.body["adapterBoundary"],
        "fumadocs-openapi-createProxy-adapter-boundary"
    );
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["fumadocsRuntime"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["nodeModulesRequired"], false);
}

#[test]
fn react_route_handler_runtime_executes_stripe_readiness_without_stripe_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/payments/stripe-js/readiness/route.ts".to_string(),
            source: r#"import {
  createDxStripeDashboardCheckoutRequest,
  createDxStripeDashboardMissingConfigReceipt,
  dxStripeDashboardCheckoutReadiness,
  dxStripeDashboardPlans,
} from "@/lib/payments/stripe-js/dashboard-checkout";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function POST(request: Request) {
  const body = await readJsonBody(request);
  const checkoutRequest = createDxStripeDashboardCheckoutRequest({
    planId: readPlanId(body),
    checkoutMode: readCheckoutMode(body),
    contact: readCheckoutContact(body),
  });
  const readiness = buildDxStripeReadiness(checkoutRequest.plan.priceEnv);
  const missingConfigReceipt =
    createDxStripeDashboardMissingConfigReceipt(checkoutRequest);

  return Response.json({
    ok: readiness.status !== "missing-config",
    packageId: "payments/stripe-js",
    status: readiness.status,
    httpStatus: readiness.status === "missing-config" ? 501 : 202,
    providerBoundary: readiness.status === "missing-config",
    request: checkoutRequest,
    receipt: missingConfigReceipt,
    readiness,
    runtimeExecution: false,
    stripeLiveExecution: false,
    secretValues: [],
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/payments/stripe-js/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "planId": "team",
                "checkoutMode": "embedded",
                "contact": {
                    "email": "ada@example.com",
                    "name": "Ada Lovelace",
                    "organization": "Analytical Engine",
                },
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("payments stripe readiness route");

    assert_eq!(response.status, 501);
    assert_eq!(
        response.execution_model,
        "source-owned-payments-stripe-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-payments-stripe-readiness")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], false);
    assert_eq!(response.body["packageId"], "payments/stripe-js");
    assert_eq!(response.body["status"], "missing-config");
    assert_eq!(response.body["httpStatus"], 501);
    assert_eq!(response.body["providerBoundary"], true);
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["stripeLiveExecution"], false);
    assert_eq!(response.body["request"]["checkoutMode"], "embedded");
    assert_eq!(response.body["request"]["plan"]["id"], "team");
    assert_eq!(
        response.body["request"]["plan"]["priceEnv"],
        "STRIPE_PRICE_ID_TEAM"
    );
    assert_eq!(
        response.body["request"]["contact"]["email"],
        "ada@example.com"
    );
    assert_eq!(
        response.body["receipt"]["receiptId"],
        "stripe-dashboard-local-team-embedded-ada-example-com"
    );
    assert_eq!(response.body["receipt"]["status"], "missing-config");
    assert_eq!(
        response.body["readiness"]["schema"],
        "dx.payments.stripe_js.readiness"
    );
    assert_eq!(
        response.body["readiness"]["selectedPriceEnv"],
        "STRIPE_PRICE_ID_TEAM"
    );
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_marks_stripe_readiness_configured_without_live_checkout() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/payments/stripe-js/readiness/route.ts".to_string(),
            source: r#"import {
  createDxStripeDashboardCheckoutRequest,
  createDxStripeDashboardMissingConfigReceipt,
  dxStripeDashboardCheckoutReadiness,
  dxStripeDashboardPlans,
} from "@/lib/payments/stripe-js/dashboard-checkout";

export async function POST(request: Request) {
  const body = await readJsonBody(request);
  const checkoutRequest = createDxStripeDashboardCheckoutRequest({
    planId: readPlanId(body),
    checkoutMode: readCheckoutMode(body),
    contact: readCheckoutContact(body),
  });
  const readiness = buildDxStripeReadiness(checkoutRequest.plan.priceEnv);
  const missingConfigReceipt =
    createDxStripeDashboardMissingConfigReceipt(checkoutRequest);

  return Response.json(
    {
      ok: readiness.status !== "missing-config",
      packageId: "payments/stripe-js",
      status: readiness.status,
      httpStatus: readiness.status === "missing-config" ? 501 : 202,
      providerBoundary: readiness.status === "missing-config",
      request: checkoutRequest,
      receipt:
        readiness.status === "missing-config"
          ? missingConfigReceipt
          : {
              ...missingConfigReceipt,
              status: "dry-run-ready",
              message:
                "Checkout request is provider-configured; this readiness route still did not call Stripe.",
            },
      readiness,
      runtimeExecution: false,
      stripeLiveExecution: false,
      secretValues: [],
    },
    { status: readiness.status === "missing-config" ? 501 : 202 },
  );
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/payments/stripe-js/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "planId": "team",
                "checkoutMode": "embedded",
                "contact": {
                    "email": "ada@example.com",
                    "name": "Ada Lovelace",
                },
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY", "pk_test_dx"),
                ("STRIPE_SECRET_KEY", "redacted-present"),
                ("STRIPE_PRICE_ID_TEAM", "redacted-present"),
            ]),
        },
    )
    .expect("configured payments stripe readiness route");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-payments-stripe-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["status"], "provider-configured-dry-run-only");
    assert_eq!(response.body["httpStatus"], 202);
    assert_eq!(
        response.body["providerBoundary"], true,
        "provider boundary stays visible even after credentials are present"
    );
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(response.body["receipt"]["status"], "dry-run-ready");
    assert_eq!(
        response.body["readiness"]["missingRequiredEnv"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );
    assert_eq!(
        response.body["readiness"]["configuredEnv"][4]["name"],
        "STRIPE_PRICE_ID_TEAM"
    );
    assert_eq!(
        response.body["readiness"]["configuredEnv"][4]["configured"],
        true
    );
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_marks_stripe_checkout_configured_without_live_checkout() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/checkout/route.ts".to_string(),
            source: r#"import { createDxStripeCheckoutContactPayload } from "@/lib/payments/stripe-js/checkout";

export async function POST(request: Request) {
  const body = await request.json();
  const contact = createDxStripeCheckoutContactPayload(body.contact);
  const configured = hasStripeCheckoutConfig(body.plan.priceEnv);

  return Response.json({
    contact,
    configured,
    stripeLiveExecution: false,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/checkout".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "checkoutMode": "embedded",
                "contact": {
                    "email": "ada@example.com",
                    "name": "Ada Lovelace",
                },
                "plan": {
                    "id": "team",
                    "priceEnv": "STRIPE_PRICE_ID_TEAM",
                },
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY", "pk_test_dx"),
                ("STRIPE_SECRET_KEY", "redacted-present"),
                ("STRIPE_PRICE_ID_TEAM", "redacted-present"),
            ]),
        },
    )
    .expect("configured payments stripe checkout route");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-payments-stripe-checkout-boundary-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["status"], "provider-configured-dry-run-only");
    assert_eq!(response.body["httpStatus"], 202);
    assert_eq!(response.body["kind"], "contact");
    assert_eq!(response.body["credentialsConfigured"], true);
    assert_eq!(
        response.body["providerBoundary"], true,
        "provider boundary stays visible even after credentials are present"
    );
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(
        response.body["missingRequiredEnv"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(response.body["stripeLiveExecution"], false);
}

#[test]
fn react_route_handler_runtime_marks_stripe_webhook_configured_without_live_verification() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/stripe/webhook/route.ts".to_string(),
            source:
                r#"import { verifyDxStripeWebhookRequest } from "@/lib/payments/stripe-js/server";

export async function POST(request: Request) {
  const event = await verifyDxStripeWebhookRequest(request);
  const action = routeDxStripeWebhookEvent(event);

  return Response.json({
    received: true,
    eventId: event.id,
    eventType: event.type,
    ...action,
  });
}
"#
                .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/stripe/webhook".to_string(),
            headers: BTreeMap::from([(
                "stripe-signature".to_string(),
                "t=1710000000,v1=signature".to_string(),
            )]),
            body: serde_json::json!({
                "id": "evt_dx_launch",
                "type": "checkout.session.completed",
                "livemode": false,
                "created": 1710000000,
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: runtime_env(&[
                ("STRIPE_SECRET_KEY", "redacted-present"),
                ("STRIPE_WEBHOOK_SECRET", "redacted-present"),
            ]),
        },
    )
    .expect("configured payments stripe webhook route");

    assert_eq!(response.status, 202);
    assert_eq!(
        response.execution_model,
        "source-owned-payments-stripe-webhook-boundary-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(
        response
            .headers
            .get("x-dx-payments-stripe-webhook")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["received"], true);
    assert_eq!(
        response.body["status"],
        "configured-source-owned-webhook-receipt"
    );
    assert_eq!(response.body["httpStatus"], 202);
    assert_eq!(response.body["signatureHeaderPresent"], true);
    assert_eq!(response.body["payloadReceived"], true);
    assert_eq!(response.body["credentialsConfigured"], true);
    assert_eq!(
        response.body["providerBoundary"], true,
        "provider boundary stays visible even after credentials are present"
    );
    assert_eq!(response.body["providerConfigured"], true);
    assert_eq!(response.body["liveProviderExecution"], false);
    assert_eq!(response.body["webhookVerificationBoundary"], true);
    assert_eq!(response.body["webhookVerified"], false);
    assert_eq!(response.body["eventRouted"], false);
    assert_eq!(response.body["eventAction"], "checkout-session-completed");
    assert_eq!(response.body["event"]["id"], "evt_dx_launch");
    assert_eq!(
        response.body["missingRequiredEnv"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["stripeLiveExecution"], false);
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_rejects_stripe_readiness_missing_contact_without_stripe_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/payments/stripe-js/readiness/route.ts".to_string(),
            source: r#"import {
  createDxStripeDashboardCheckoutRequest,
  createDxStripeDashboardMissingConfigReceipt,
  dxStripeDashboardCheckoutReadiness,
  dxStripeDashboardPlans,
} from "@/lib/payments/stripe-js/dashboard-checkout";

export async function POST(request: Request) {
  try {
    const body = await readJsonBody(request);
    const checkoutRequest = createDxStripeDashboardCheckoutRequest({
      planId: readPlanId(body),
      checkoutMode: readCheckoutMode(body),
      contact: readCheckoutContact(body),
    });
    const readiness = buildDxStripeReadiness(checkoutRequest.plan.priceEnv);
    const missingConfigReceipt =
      createDxStripeDashboardMissingConfigReceipt(checkoutRequest);

    return Response.json({
      ok: readiness.status !== "missing-config",
      packageId: "payments/stripe-js",
      status: readiness.status,
      receipt: missingConfigReceipt,
    });
  } catch (error) {
    return Response.json(
      {
        ok: false,
        packageId: "payments/stripe-js",
        status: "bad-request",
        message:
          error instanceof Error
            ? error.message
            : "Payments readiness request failed.",
        runtimeExecution: false,
        stripeLiveExecution: false,
        secretValues: [],
      },
      { status: 400 },
    );
  }
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/payments/stripe-js/readiness".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::json!({
                "planId": "team",
                "checkoutMode": "embedded",
            }),
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("payments stripe readiness missing contact route");

    assert_eq!(response.status, 400);
    assert_eq!(
        response.execution_model,
        "source-owned-payments-stripe-readiness-interpreter"
    );
    assert!(!response.lifecycle_scripts_executed);
    assert_eq!(response.body["ok"], false);
    assert_eq!(response.body["packageId"], "payments/stripe-js");
    assert_eq!(response.body["status"], "bad-request");
    assert_eq!(
        response.body["message"],
        "Payments readiness dry-run requires contact details."
    );
    assert_eq!(response.body["runtimeExecution"], false);
    assert_eq!(response.body["stripeLiveExecution"], false);
    assert_eq!(
        response.body["secretValues"].as_array().map(Vec::len),
        Some(0)
    );
}

#[test]
fn react_route_handler_runtime_matches_dynamic_app_api_segments() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/trpc/[trpc]/route.ts".to_string(),
            source: r#"export function GET() {
  return {
    ok: true,
    runtime: "dynamic-api-route",
  };
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/trpc/health".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("dynamic route handler");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["runtime"], "dynamic-api-route");
}

#[test]
fn react_route_handler_runtime_reads_route_and_search_params() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/posts/[slug]/route.ts".to_string(),
            source: r#"export function GET(request: Request) {
  return {
    ok: true,
    slug: request.params.slug,
    preview: request.searchParams.preview,
    params: request.params,
    searchParams: request.searchParams,
  };
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/posts/acme".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::from([("slug".to_string(), "acme".to_string())]),
            search_params: BTreeMap::from([("preview".to_string(), "draft".to_string())]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("route request maps");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["slug"], "acme");
    assert_eq!(response.body["preview"], "draft");
    assert_eq!(response.body["params"]["slug"], "acme");
    assert_eq!(response.body["searchParams"]["preview"], "draft");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_next_request_search_params_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search/route.ts".to_string(),
            source: r#"import type { NextRequest } from "next/server";

export function GET(request: NextRequest) {
  const url = new URL(request.url);
  const aliasQuery = request.nextUrl.searchParams.get("q") ?? "none";
  const aliasMode = url.searchParams.get("mode") ?? "all";

  return Response.json({
    query: request.nextUrl.searchParams.get("q"),
    mode: url.searchParams.get("mode"),
    missing: request.nextUrl.searchParams.get("missing") ?? "fallback",
    aliasQuery,
    aliasMode,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/search?q=dx&mode=docs".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([
                ("q".to_string(), "dx".to_string()),
                ("mode".to_string(), "docs".to_string()),
            ]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("next request search params response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["query"], "dx");
    assert_eq!(response.body["mode"], "docs");
    assert_eq!(response.body["missing"], "fallback");
    assert_eq!(response.body["aliasQuery"], "dx");
    assert_eq!(response.body["aliasMode"], "docs");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_typed_request_aliases_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search/route.ts".to_string(),
            source: r#"import type { NextRequest } from "next/server";

export function GET(request: NextRequest) {
  const methodName: string = request.method;
  const requestUrl: string = request.url;
  const url: URL = new URL(request.url);
  const nextUrl: URL = request.nextUrl;
  const headers: Headers = request.headers;
  const agent: string = headers.get("user-agent") ?? "none";
  const mode: string = url.searchParams.get("mode") ?? "all";
  const query: string = nextUrl.searchParams.get("q") ?? "none";

  return Response.json({
    methodName,
    requestUrl,
    pathname: url.pathname,
    nextPathname: nextUrl.pathname,
    agent,
    mode,
    query,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "https://dx.local/api/search?q=dx&mode=docs#top".to_string(),
            headers: BTreeMap::from([("user-agent".to_string(), "dx-test".to_string())]),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("typed request aliases response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["methodName"], "GET");
    assert_eq!(
        response.body["requestUrl"],
        "https://dx.local/api/search?q=dx&mode=docs#top"
    );
    assert_eq!(response.body["pathname"], "/api/search");
    assert_eq!(response.body["nextPathname"], "/api/search");
    assert_eq!(response.body["agent"], "dx-test");
    assert_eq!(response.body["mode"], "docs");
    assert_eq!(response.body["query"], "dx");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_next_request_url_properties_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search/route.ts".to_string(),
            source: r#"import type { NextRequest } from "next/server";

export function GET(request: NextRequest) {
  const url = new URL(request.url);
  const alias = request.nextUrl;

  return Response.json({
    pathname: request.nextUrl.pathname,
    search: request.nextUrl.search,
    href: request.nextUrl.href,
    aliasPathname: alias.pathname,
    urlPathname: url.pathname,
    urlSearch: url.search,
    urlHref: url.href,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "https://dx.local/api/search?q=dx&mode=docs#top".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([
                ("q".to_string(), "dx".to_string()),
                ("mode".to_string(), "docs".to_string()),
            ]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("next request url property response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["pathname"], "/api/search");
    assert_eq!(response.body["search"], "?q=dx&mode=docs");
    assert_eq!(
        response.body["href"],
        "https://dx.local/api/search?q=dx&mode=docs#top"
    );
    assert_eq!(response.body["aliasPathname"], "/api/search");
    assert_eq!(response.body["urlPathname"], "/api/search");
    assert_eq!(response.body["urlSearch"], "?q=dx&mode=docs");
    assert_eq!(
        response.body["urlHref"],
        "https://dx.local/api/search?q=dx&mode=docs#top"
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_destructured_next_request_url_properties_without_next_runtime()
{
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search/route.ts".to_string(),
            source: r#"import type { NextRequest } from "next/server";

export function GET(request: NextRequest) {
  const { pathname, search: query, href } = request.nextUrl;
  const { pathname: urlPathname, search } = new URL(request.url);

  return Response.json({
    pathname,
    query,
    href,
    urlPathname,
    search,
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "https://dx.local/api/search?q=dx&mode=docs#top".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([
                ("q".to_string(), "dx".to_string()),
                ("mode".to_string(), "docs".to_string()),
            ]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("destructured next request url property response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["pathname"], "/api/search");
    assert_eq!(response.body["query"], "?q=dx&mode=docs");
    assert_eq!(
        response.body["href"],
        "https://dx.local/api/search?q=dx&mode=docs#top"
    );
    assert_eq!(response.body["urlPathname"], "/api/search");
    assert_eq!(response.body["search"], "?q=dx&mode=docs");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_destructured_next_request_search_params_without_next_runtime()
{
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search/route.ts".to_string(),
            source: r#"import type { NextRequest } from "next/server";

export function GET(request: NextRequest) {
  const { searchParams: query } = request.nextUrl;
  const { searchParams } = new URL(request.url);
  const mode = query.get("mode") ?? "all";

  return Response.json({
    query: searchParams.get("q") ?? "none",
    mode,
    directMode: query.get("mode"),
    missing: searchParams.get("missing") ?? "fallback",
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/search?q=dx&mode=docs".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([
                ("q".to_string(), "dx".to_string()),
                ("mode".to_string(), "docs".to_string()),
            ]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("destructured next request search params response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["query"], "dx");
    assert_eq!(response.body["mode"], "docs");
    assert_eq!(response.body["directMode"], "docs");
    assert_eq!(response.body["missing"], "fallback");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_url_search_params_from_request_path_without_matcher_params() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/search/route.ts".to_string(),
            source: r#"import type { NextRequest } from "next/server";

export function GET(request: NextRequest) {
  const { searchParams: query } = new URL(request.url);

  return Response.json({
    query: query.get("q") ?? "none",
    mode: request.nextUrl.searchParams.get("mode") ?? "all",
    directMode: request.searchParams.mode,
    encoded: query.get("encoded"),
    plus: request.nextUrl.searchParams.get("plus"),
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "https://dx.local/api/search?q=dx&mode=docs&encoded=a%2Fb&plus=hello+world#top"
                .to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request url search params fallback response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["query"], "dx");
    assert_eq!(response.body["mode"], "docs");
    assert_eq!(response.body["directMode"], "docs");
    assert_eq!(response.body["encoded"], "a/b");
    assert_eq!(response.body["plus"], "hello world");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_request_headers_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/preview/route.ts".to_string(),
            source: r#"export function GET(request: Request) {
  const headers = request.headers;
  const auth = request.headers.get("authorization") ?? "missing";
  const preview = headers.get("x-dx-preview") ?? "0";

  return Response.json({
    auth,
    preview,
    direct: request.headers.get("authorization"),
    missing: headers.get("x-missing") ?? "fallback",
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/preview".to_string(),
            headers: BTreeMap::from([
                ("authorization".to_string(), "Bearer dx".to_string()),
                ("x-dx-preview".to_string(), "1".to_string()),
            ]),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request headers route response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["auth"], "Bearer dx");
    assert_eq!(response.body["preview"], "1");
    assert_eq!(response.body["direct"], "Bearer dx");
    assert_eq!(response.body["missing"], "fallback");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_destructured_request_headers_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/preview/route.ts".to_string(),
            source: r#"export function GET(request: Request) {
  const { headers } = request;

  return Response.json({
    preview: headers.get("x-dx-preview") ?? "0",
    auth: headers.get("authorization"),
    missing: headers.get("x-missing") ?? "fallback",
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/preview".to_string(),
            headers: BTreeMap::from([
                ("authorization".to_string(), "Bearer dx".to_string()),
                ("x-dx-preview".to_string(), "1".to_string()),
            ]),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("destructured request headers route response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["preview"], "1");
    assert_eq!(response.body["auth"], "Bearer dx");
    assert_eq!(response.body["missing"], "fallback");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_next_headers_helper_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/preview/route.ts".to_string(),
            source: r#"import { headers } from "next/headers";

export function GET() {
  const requestHeaders = headers();
  const preview = headers().get("x-dx-preview") ?? "0";

  return Response.json({
    preview,
    auth: requestHeaders.get("authorization"),
    missing: headers().get("x-missing") ?? "fallback",
  });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/preview".to_string(),
            headers: BTreeMap::from([
                ("authorization".to_string(), "Bearer dx".to_string()),
                ("x-dx-preview".to_string(), "1".to_string()),
            ]),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("next headers helper route response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["preview"], "1");
    assert_eq!(response.body["auth"], "Bearer dx");
    assert_eq!(response.body["missing"], "fallback");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_next_context_params_without_next_runtime() {
    let request = DxReactRouteHandlerRequest {
        method: "GET".to_string(),
        path: "/api/posts/acme".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::Value::Null,
        route_params: BTreeMap::from([("slug".to_string(), "acme".to_string())]),
        search_params: BTreeMap::new(),
        runtime_env: BTreeMap::new(),
    };
    let context_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/posts/[slug]/route.ts".to_string(),
            source:
                r#"export function GET(request: Request, context: { params: { slug: string } }) {
  return Response.json({
    slug: context.params.slug,
    params: context.params,
  });
}
"#
                .to_string(),
        },
        request.clone(),
    )
    .expect("context params route response");
    let destructured_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/posts/[slug]/route.ts".to_string(),
            source: r#"export function GET(_request: Request, { params }: { params: { slug: string } }) {
  return {
    slug: params.slug,
    params,
  };
}
"#
            .to_string(),
        },
        request,
    )
    .expect("destructured context params route response");

    assert_eq!(context_response.body["slug"], "acme");
    assert_eq!(context_response.body["params"]["slug"], "acme");
    assert_eq!(destructured_response.body["slug"], "acme");
    assert_eq!(destructured_response.body["params"]["slug"], "acme");
    assert_eq!(
        context_response.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!context_response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_async_context_param_aliases_without_next_runtime() {
    let request = DxReactRouteHandlerRequest {
        method: "GET".to_string(),
        path: "/api/posts/acme".to_string(),
        headers: BTreeMap::new(),
        body: serde_json::Value::Null,
        route_params: BTreeMap::from([("slug".to_string(), "acme".to_string())]),
        search_params: BTreeMap::new(),
        runtime_env: BTreeMap::new(),
    };
    let destructured_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/posts/[slug]/route.ts".to_string(),
            source:
                r#"export async function GET(_request: Request, { params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  return Response.json({
    slug,
  });
}
"#
                .to_string(),
        },
        request.clone(),
    )
    .expect("async destructured context params response");
    let direct_alias_response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/posts/[slug]/route.ts".to_string(),
            source:
                r#"export function GET(_request: Request, context: { params: { slug: string } }) {
  const slugAlias = context.params.slug;
  return {
    slugAlias,
  };
}
"#
                .to_string(),
        },
        request,
    )
    .expect("direct context param alias response");

    assert_eq!(destructured_response.body["slug"], "acme");
    assert_eq!(direct_alias_response.body["slugAlias"], "acme");
    assert_eq!(
        destructured_response.execution_model,
        "source-owned-safe-interpreter"
    );
    assert!(!destructured_response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_supports_next_response_headers_cookies_and_redirects() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/session/route.ts".to_string(),
        source: r#"import { NextResponse } from "next/server";

export function GET(request: Request) {
  const theme = request.cookies.get("theme")?.value ?? "light";

  return NextResponse.json(
    {
      ok: true,
      theme,
    },
    {
      status: 201,
      headers: {
        "cache-control": "no-store",
        "x-dx-mode": "safe",
      },
    },
  );
}

export function POST() {
  return NextResponse.redirect("/login", 307);
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/session".to_string(),
            headers: BTreeMap::from([(
                "cookie".to_string(),
                "theme=dark; session=abc".to_string(),
            )]),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("next response json");

    assert_eq!(response.status, 201);
    assert_eq!(response.content_type, "application/json; charset=utf-8");
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["theme"], "dark");
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response.headers.get("x-dx-mode").map(String::as_str),
        Some("safe")
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);

    let redirect = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/session".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("next response redirect");

    assert_eq!(redirect.status, 307);
    assert_eq!(redirect.redirect_url.as_deref(), Some("/login"));
    assert_eq!(
        redirect.headers.get("location").map(String::as_str),
        Some("/login")
    );
    assert_eq!(redirect.body["redirect"], "/login");
}

#[test]
fn react_route_handler_runtime_supports_redirect_response_init_options() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/billing/route.ts".to_string(),
        source: r#"import { NextResponse } from "next/server";

export function GET() {
  return NextResponse.redirect("/billing", {
    status: 308,
    headers: {
      "cache-control": "no-store",
      "x-dx-mode": "redirect",
    },
  });
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/billing".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("redirect response init options");

    assert_eq!(response.status, 308);
    assert_eq!(response.redirect_url.as_deref(), Some("/billing"));
    assert_eq!(
        response.headers.get("location").map(String::as_str),
        Some("/billing")
    );
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response.headers.get("x-dx-mode").map(String::as_str),
        Some("redirect")
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_redirects_to_request_url_alias() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/login/route.ts".to_string(),
        source: r#"export function GET(request: Request) {
  const returnUrl = request.url;
  return Response.redirect(returnUrl, 308);
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "https://dx.local/api/login?next=/dashboard#form".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::from([("next".to_string(), "/dashboard".to_string())]),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("request url alias redirect response");

    assert_eq!(response.status, 308);
    assert_eq!(
        response.redirect_url.as_deref(),
        Some("https://dx.local/api/login?next=/dashboard#form")
    );
    assert_eq!(
        response.headers.get("location").map(String::as_str),
        Some("https://dx.local/api/login?next=/dashboard#form")
    );
    assert_eq!(
        response.body["redirect"],
        "https://dx.local/api/login?next=/dashboard#form"
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_direct_cookie_values_without_next_runtime() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/preferences/route.ts".to_string(),
        source: r#"import { cookies } from "next/headers";

export function GET(request: Request) {
  return Response.json({
    theme: request.cookies.get("theme")?.value ?? "light",
    locale: cookies().get("locale")?.value ?? "en",
    missing: request.cookies.get("missing")?.value ?? "fallback",
  });
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/preferences".to_string(),
            headers: BTreeMap::from([("cookie".to_string(), "theme=dark; locale=fr".to_string())]),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("direct cookie response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["theme"], "dark");
    assert_eq!(response.body["locale"], "fr");
    assert_eq!(response.body["missing"], "fallback");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_reads_cookie_root_aliases_and_metadata_without_next_runtime() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/preferences/route.ts".to_string(),
        source: r#"import { cookies } from "next/headers";

export function GET(request: Request) {
  const { cookies: requestCookies } = request;
  const directJar = request.cookies;
  const helperJar = cookies();

  return Response.json({
    directTheme: directJar.get("theme")?.value ?? "light",
    helperLocale: helperJar.get("locale")?.value ?? "en",
    themeName: requestCookies.get("theme")?.name ?? "missing",
    missingName: requestCookies.get("missing")?.name ?? "fallback",
    cookie: requestCookies.get("theme"),
  });
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/preferences".to_string(),
            headers: BTreeMap::from([("cookie".to_string(), "theme=dark; locale=fr".to_string())]),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("cookie root alias response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["directTheme"], "dark");
    assert_eq!(response.body["helperLocale"], "fr");
    assert_eq!(response.body["themeName"], "theme");
    assert_eq!(response.body["missingName"], "fallback");
    assert_eq!(response.body["cookie"]["name"], "theme");
    assert_eq!(response.body["cookie"]["value"], "dark");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_supports_safe_response_status_aliases() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/jobs/route.ts".to_string(),
        source: r#"export function POST() {
  const accepted = 202;

  return Response.json(
    {
      ok: true,
    },
    {
      status: accepted,
      headers: {
        "x-dx-mode": "safe",
      },
    },
  );
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "POST".to_string(),
            path: "/api/jobs".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("status alias response");

    assert_eq!(response.status, 202);
    assert_eq!(response.body["ok"], true);
    assert_eq!(
        response.headers.get("x-dx-mode").map(String::as_str),
        Some("safe")
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_supports_static_headers_tuple_arrays() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::RouteHandler,
        source_path: "app/api/cache/route.ts".to_string(),
        source: r#"export function GET() {
  return Response.json(
    {
      ok: true,
    },
    {
      headers: new Headers([
        ["cache-control", "no-store"],
        ["x-dx-mode", "tuple-array"],
      ]),
    },
  );
}
"#
        .to_string(),
    };

    let response = execute_react_route_handler(
        &source,
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/cache".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("static tuple headers response");

    assert_eq!(response.status, 200);
    assert_eq!(response.body["ok"], true);
    assert_eq!(
        response.headers.get("cache-control").map(String::as_str),
        Some("no-store")
    );
    assert_eq!(
        response.headers.get("x-dx-mode").map(String::as_str),
        Some("tuple-array")
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_route_handler_runtime_supports_next_not_found_helper_without_next_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/products/[id]/route.ts".to_string(),
            source: r#"import { notFound } from "next/navigation";

export function GET(request: Request, { params }: { params: { id: string } }) {
  if (params.id === "missing") {
    notFound();
  }

  return Response.json({ ok: true });
}
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/products/missing".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::from([("id".to_string(), "missing".to_string())]),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::new(),
        },
    )
    .expect("next notFound helper route response");

    assert_eq!(response.status, 404);
    assert_eq!(response.content_type, "application/json; charset=utf-8");
    assert_eq!(response.body["notFound"], true);
    assert_eq!(response.body["nextHelper"], "notFound");
    assert_eq!(
        response
            .headers
            .get("x-dx-route-handler-not-found")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
}

#[test]
fn react_server_action_protocol_requires_hooks_and_returns_receipt() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::Action,
        source_path: "server/actions.ts".to_string(),
        source: r#"export async function recordWelcomeView(payload: { count: number }) {
  return {
    ok: true,
    source: "dx-www-server-action",
  };
}
"#
        .to_string(),
    };

    let protocols = compile_react_server_action_protocols(&[source.clone()]);
    assert_eq!(protocols.len(), 1);
    assert_eq!(
        protocols[0].action_id,
        "server/actions.ts#recordWelcomeView"
    );
    assert!(protocols[0].endpoint.starts_with("/.dx/actions/"));
    assert_eq!(protocols[0].csrf_hook, "required");
    assert_eq!(protocols[0].session_hook, "required");
    assert_eq!(protocols[0].replay_protection, "idempotency-key");
    assert!(!protocols[0].lifecycle_scripts_executed);

    let blocked = execute_react_server_action(
        &source,
        DxReactServerActionRequest {
            action_id: "server/actions.ts#recordWelcomeView".to_string(),
            payload: serde_json::json!({ "count": 1 }),
            csrf_token: None,
            session_id: Some("session-a".to_string()),
            idempotency_key: Some("welcome-1".to_string()),
        },
    )
    .expect_err("csrf missing");
    assert!(blocked.contains("csrf"));

    let response = execute_react_server_action(
        &source,
        DxReactServerActionRequest {
            action_id: "server/actions.ts#recordWelcomeView".to_string(),
            payload: serde_json::json!({ "count": 1 }),
            csrf_token: Some("csrf-a".to_string()),
            session_id: Some("session-a".to_string()),
            idempotency_key: Some("welcome-1".to_string()),
        },
    )
    .expect("action response");

    assert_eq!(response.action_id, "server/actions.ts#recordWelcomeView");
    assert_eq!(response.body["ok"], true);
    assert_eq!(response.body["source"], "dx-www-server-action");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
    assert!(response.receipt.replay_safe);
    assert_eq!(response.receipt.export_name, "recordWelcomeView");
    assert_eq!(
        response.receipt.idempotency_key_hash.len(),
        "blake3:".len() + 16
    );
}

#[test]
fn react_server_action_runtime_validates_typed_payloads_and_receipts_without_leaking_values() {
    let source = DxReactServerSource {
        kind: DxReactServerSourceKind::Action,
        source_path: "server/actions.ts".to_string(),
        source: r#"export async function saveProfile(payload: { count: number; name: string; email?: string; active: boolean; mode: "draft" | "publish" }) {
  return {
    ok: true,
    saved: true,
    status: "accepted",
  };
}
"#
        .to_string(),
    };

    let protocols = compile_react_server_action_protocols(&[source.clone()]);
    let protocol = protocols
        .iter()
        .find(|protocol| protocol.action_id == "server/actions.ts#saveProfile")
        .expect("saveProfile protocol");
    assert_eq!(protocol.request_serialization, "typed-json-object");
    assert_eq!(protocol.response_serialization, "typed-json-object");
    assert_eq!(protocol.request_schema.mode, "typed-object");
    assert_eq!(protocol.response_schema.mode, "typed-object");
    assert!(protocol.request_schema.source_hash.starts_with("blake3:"));
    assert!(
        protocol.request_schema.fields.iter().any(|field| {
            field.name == "count" && field.value_type == "number" && field.required
        })
    );
    assert!(
        protocol.request_schema.fields.iter().any(|field| {
            field.name == "email" && field.value_type == "string" && !field.required
        })
    );
    assert!(protocol.request_schema.fields.iter().any(|field| {
        field.name == "mode"
            && field.value_type == "string-literal-union"
            && field.allowed_values == vec!["draft".to_string(), "publish".to_string()]
    }));
    assert!(
        protocol.response_schema.fields.iter().any(|field| {
            field.name == "saved" && field.value_type == "boolean" && field.required
        })
    );

    let invalid = execute_react_server_action(
        &source,
        DxReactServerActionRequest {
            action_id: "server/actions.ts#saveProfile".to_string(),
            payload: serde_json::json!({
                "count": "not-a-number",
                "name": "Ada",
                "active": true,
                "mode": "secret-invalid-mode",
            }),
            csrf_token: Some("csrf-a".to_string()),
            session_id: Some("session-a".to_string()),
            idempotency_key: Some("profile-1".to_string()),
        },
    )
    .expect_err("invalid typed payload");
    assert!(invalid.contains("payload.count expected number"));
    assert!(invalid.contains("payload.mode expected one of allowed string literals"));
    assert!(!invalid.contains("not-a-number"));
    assert!(!invalid.contains("secret-invalid-mode"));

    let response = execute_react_server_action(
        &source,
        DxReactServerActionRequest {
            action_id: "server/actions.ts#saveProfile".to_string(),
            payload: serde_json::json!({
                "count": 2,
                "name": "Ada",
                "active": true,
                "mode": "draft",
            }),
            csrf_token: Some("csrf-a".to_string()),
            session_id: Some("session-a".to_string()),
            idempotency_key: Some("profile-2".to_string()),
        },
    )
    .expect("valid typed action");

    assert_eq!(response.body["saved"], true);
    assert!(response.receipt.request_validated);
    assert!(response.receipt.response_validated);
    assert!(response.receipt.request_schema_hash.starts_with("blake3:"));
    assert!(response.receipt.response_schema_hash.starts_with("blake3:"));
    assert!(response.receipt.validation_errors.is_empty());
    assert!(response.receipt.replay_safe);
}

#[test]
fn react_server_data_manifest_evaluates_async_loader_for_server_component_route() {
    let loader = DxReactServerSource {
        kind: DxReactServerSourceKind::Loader,
        source_path: "server/loaders.ts".to_string(),
        source: r#"export async function loadHomeMetrics() {
  return {
    routes: 1,
    packages: 0,
    runtime: "js" as const,
  };
}
"#
        .to_string(),
    };

    let response =
        execute_react_server_loader(&loader, "loadHomeMetrics").expect("loader response");
    assert_eq!(response.body["routes"], 1);
    assert_eq!(response.body["runtime"], "js");
    assert_eq!(response.execution_model, "source-owned-safe-interpreter");
    assert!(!response.lifecycle_scripts_executed);
    assert!(response.response_hash.starts_with("blake3:"));

    let manifest = compile_react_server_data_manifest(
        "/",
        "app/page.tsx",
        r#"import { loadHomeMetrics } from "../server/loaders";

export default async function Page() {
  const metrics = await loadHomeMetrics();

  return <main>{metrics.runtime}</main>;
}
"#,
        &[loader],
    )
    .expect("server data manifest");

    assert_eq!(manifest.route, "/");
    assert_eq!(manifest.route_source_path, "app/page.tsx");
    assert!(!manifest.node_modules_required);
    assert!(!manifest.lifecycle_scripts_executed);
    assert_eq!(manifest.entries.len(), 1);
    assert_eq!(manifest.entries[0].binding, "metrics");
    assert_eq!(manifest.entries[0].local_name, "loadHomeMetrics");
    assert_eq!(manifest.entries[0].export_name, "loadHomeMetrics");
    assert_eq!(manifest.entries[0].source_path, "server/loaders.ts");
    assert_eq!(manifest.entries[0].value["packages"], 0);
    assert_eq!(manifest.entries[0].value["runtime"], "js");
    assert_eq!(
        manifest.entries[0].execution_model,
        "source-owned-safe-interpreter"
    );
}

#[test]
fn react_client_boundaries_select_static_micro_js_and_wasm_from_measured_complexity() {
    let static_boundaries = analyze_react_client_boundaries(&[DxReactClientSource {
        source_path: "app/page.tsx".to_string(),
        source: "export default function Page() { return <main><h1>Static</h1></main>; }"
            .to_string(),
    }]);
    assert!(static_boundaries.is_empty());
    assert_eq!(
        select_react_delivery_mode(&static_boundaries),
        DxDeliveryMode::Static
    );

    let micro_boundaries = analyze_react_client_boundaries(&[DxReactClientSource {
        source_path: "components/local/Counter.tsx".to_string(),
        source: r#""use client";
import { useState } from "react";

export function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>Count</button>;
}
"#
        .to_string(),
    }]);
    assert_eq!(micro_boundaries.len(), 1);
    assert_eq!(micro_boundaries[0].state_vars, 1);
    assert_eq!(micro_boundaries[0].event_handlers, 1);
    assert_eq!(micro_boundaries[0].delivery_mode, DxDeliveryMode::MicroJs);
    assert_eq!(
        select_react_delivery_mode(&micro_boundaries),
        DxDeliveryMode::MicroJs
    );

    let wasm_boundaries = analyze_react_client_boundaries(&[DxReactClientSource {
        source_path: "components/local/WorkspaceGrid.tsx".to_string(),
        source: r#""use client";
import { useEffect, useReducer, useState } from "react";

export function WorkspaceGrid() {
  const [a] = useState(0);
  const [b] = useState(0);
  const [c] = useState(0);
  const [d] = useState(0);
  const [e] = useState(0);
  const [f] = useState(0);
  useEffect(() => {}, []);
  return (
    <section>
      <button onClick={() => a}>A</button>
      <button onClick={() => b}>B</button>
      <button onClick={() => c}>C</button>
      <button onClick={() => d}>D</button>
      <button onClick={() => e}>E</button>
      <button onClick={() => f}>F</button>
      <button onClick={() => a + b}>G</button>
      <button onClick={() => c + d}>H</button>
      <button onClick={() => e + f}>I</button>
      <button onClick={() => a + f}>J</button>
    </section>
  );
}
"#
        .to_string(),
    }]);
    assert_eq!(wasm_boundaries[0].state_vars, 6);
    assert_eq!(wasm_boundaries[0].event_handlers, 10);
    assert_eq!(wasm_boundaries[0].delivery_mode, DxDeliveryMode::WasmCore);
    assert_eq!(
        select_react_delivery_mode(&wasm_boundaries),
        DxDeliveryMode::WasmCore
    );
}

#[test]
fn react_client_island_compiler_emits_deterministic_micro_js_events_and_keyed_updates() {
    let input = DxReactClientIslandInput {
        route: "/".to_string(),
        route_source_path: "app/page.tsx".to_string(),
        route_source: r#"import { CounterList } from "../components/local/CounterList";

export default function Page() {
  return <CounterList />;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "CounterList".to_string(),
            source_path: "components/local/CounterList.tsx".to_string(),
            source: r#""use client";

import { useState } from "react";

export function CounterList() {
  const [count, setCount] = useState(0);
  const items = [{ id: "first" }, { id: "second" }];

  function increment() {
    setCount((current) => current + 1);
  }

  return (
    <section>
      <button onClick={increment}>Add</button>
      <p>Count: {count}</p>
      <ul>{items.map((item) => <li key={item.id}>{item.id}</li>)}</ul>
    </section>
  );
}
"#
            .to_string(),
            package_id: None,
        }],
        route_delivery_mode: DxDeliveryMode::MicroJs,
    };

    let manifest = compile_react_client_islands(input.clone());
    let repeated = compile_react_client_islands(input);

    assert_eq!(manifest, repeated);
    assert_eq!(manifest.route, "/");
    assert_eq!(manifest.runtime, "js");
    assert!(!manifest.node_modules_required);
    assert!(manifest.deterministic);
    assert_eq!(manifest.islands.len(), 1);

    let island = &manifest.islands[0];
    assert_eq!(island.source_path, "components/local/CounterList.tsx");
    assert_eq!(island.source_kind, "component");
    assert!(island.use_client);
    assert_eq!(island.delivery_mode, DxDeliveryMode::MicroJs);
    assert_eq!(island.state[0].name, "count");
    assert_eq!(island.state[0].setter, "setCount");
    assert_eq!(island.state[0].initial_value, Some(0));
    assert!(island.events.iter().any(|event| {
        event.element == "button"
            && event.event == "click"
            && event.handler == "increment"
            && event.operation.as_deref() == Some("add")
    }));
    assert!(
        island
            .keyed_updates
            .iter()
            .any(|key| { key.element == "li" && key.expression.as_deref() == Some("item.id") })
    );

    let micro_js = island.micro_js.as_ref().expect("micro js");
    assert!(micro_js.deterministic);
    assert_eq!(micro_js.action_count, 1);
    assert!(micro_js.script.contains("addEventListener"));
    assert!(micro_js.script.contains("\"click\""));
    assert!(micro_js.script_hash.starts_with("blake3:"));
    assert!(
        react_client_island_micro_js_bundle(&manifest)
            .expect("bundle")
            .contains("www client islands")
    );
}

#[test]
fn react_client_island_micro_js_uses_native_dom_event_catalog() {
    let input = DxReactClientIslandInput {
        route: "/events".to_string(),
        route_source_path: "app/events/page.tsx".to_string(),
        route_source: r#"import { EventPanel } from "../../components/local/EventPanel";

export default function Page() {
  return <EventPanel />;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "EventPanel".to_string(),
            source_path: "components/local/EventPanel.tsx".to_string(),
            source: r#""use client";
import { useState } from "react";

export function EventPanel() {
  const [count, setCount] = useState(0);

  return (
    <section>
      <button onDoubleClick={() => setCount(count + 2)}>Double</button>
      <button onPointerMove={() => setCount(count + 3)}>Pointer</button>
      <button onMagicGesture={() => setCount(count + 4)}>Unsupported</button>
    </section>
  );
}
"#
            .to_string(),
            package_id: None,
        }],
        route_delivery_mode: DxDeliveryMode::MicroJs,
    };

    let manifest = compile_react_client_islands(input);
    let island = &manifest.islands[0];
    assert!(
        island
            .events
            .iter()
            .any(|event| { event.attribute == "onDoubleClick" && event.event == "dblclick" })
    );
    assert!(
        island
            .events
            .iter()
            .any(|event| { event.attribute == "onPointerMove" && event.event == "pointermove" })
    );
    assert!(
        !island
            .events
            .iter()
            .any(|event| event.attribute == "onMagicGesture")
    );

    let micro_js = island.micro_js.as_ref().expect("micro js");
    assert_eq!(micro_js.action_count, 2);
    assert!(micro_js.script.contains("\"dblclick\""));
    assert!(micro_js.script.contains("\"pointermove\""));
    assert!(!micro_js.script.contains("magicgesture"));
    assert!(!micro_js.script.contains("onclick"));
}

#[test]
fn react_client_island_compiler_wires_props_forms_and_dynamic_imports() {
    let input = DxReactClientIslandInput {
        route: "/settings".to_string(),
        route_source_path: "app/settings/page.tsx".to_string(),
        route_source: r#"import dynamic from "next/dynamic";
import { SettingsForm } from "../../components/local/SettingsForm";

const PreviewPanel = dynamic(() => import("../../components/local/PreviewPanel"), { ssr: false });

export default function Page() {
  return (
    <main>
      <SettingsForm initialName="Ada" initialCount={2} />
      <PreviewPanel />
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "SettingsForm".to_string(),
            source_path: "components/local/SettingsForm.tsx".to_string(),
            source: r#""use client";

import { useState } from "react";

type SettingsFormProps = {
  initialName: string;
  initialCount: number;
};

export function SettingsForm({ initialName, initialCount }: SettingsFormProps) {
  const [name, setName] = useState(initialName);
  const [email, setEmail] = useState("");
  const [count, setCount] = useState(initialCount);

  function submit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
  }

  return (
    <form onSubmit={submit}>
      <input name="name" value={name} onChange={(event) => setName(event.target.value)} />
      <input name="email" value={email} onInput={(event) => setEmail(event.target.value)} />
      <button type="button" onClick={() => setCount(count + 1)}>Add</button>
      <p>{count}</p>
    </form>
  );
}
"#
            .to_string(),
            package_id: None,
        }],
        route_delivery_mode: DxDeliveryMode::MicroJs,
    };

    let manifest = compile_react_client_islands(input);
    let island = manifest
        .islands
        .iter()
        .find(|island| island.source_path == "components/local/SettingsForm.tsx")
        .expect("settings island");

    assert_eq!(manifest.runtime, "js");
    assert!(!manifest.node_modules_required);
    assert!(island.hydration.deterministic);
    assert_eq!(island.hydration.strategy, "js-event-replay");
    assert!(island.hydration.props.iter().any(|prop| {
        prop.name == "initialName"
            && prop.source == "route-prop"
            && prop.value.as_deref() == Some("Ada")
    }));
    assert!(island.hydration.props.iter().any(|prop| {
        prop.name == "initialCount"
            && prop.source == "route-prop"
            && prop.expression.as_deref() == Some("2")
    }));
    assert!(island.hydration.events.iter().any(|event| {
        event.event == "submit"
            && event.form_id.as_deref() == Some("settingsform-form-0")
            && event.prevent_default
    }));
    assert!(island.hydration.events.iter().any(|event| {
        event.event == "change"
            && event.state.as_deref() == Some("name")
            && event.operation.as_deref() == Some("set-from-input")
    }));
    assert!(island.hydration.events.iter().any(|event| {
        event.event == "input"
            && event.state.as_deref() == Some("email")
            && event.operation.as_deref() == Some("set-from-input")
    }));
    assert!(island.hydration.forms.iter().any(|form| {
        form.form_id == "settingsform-form-0"
            && form
                .submit_event
                .as_deref()
                .is_some_and(|event| event.contains("submit"))
            && form.fields.iter().any(|field| {
                field.name == "name"
                    && field.value_state.as_deref() == Some("name")
                    && field
                        .change_event
                        .as_deref()
                        .is_some_and(|event| event.contains("change"))
            })
            && form.fields.iter().any(|field| {
                field.name == "email"
                    && field.value_state.as_deref() == Some("email")
                    && field
                        .change_event
                        .as_deref()
                        .is_some_and(|event| event.contains("input"))
            })
    }));
    assert!(manifest.dynamic_imports.iter().any(|import| {
        import.source == "../../components/local/PreviewPanel"
            && import.preload
            && import.ssr == Some(false)
    }));
    assert!(manifest.hydration_runtime.as_ref().is_some_and(|runtime| {
        runtime.source_owned
            && runtime.script_hash.starts_with("blake3:")
            && runtime.script.contains("addEventListener")
            && runtime.script.contains("data-dx-island")
    }));
}

#[test]
fn react_client_island_manifest_preserves_camel_case_directives() {
    let capabilities = react_client_island_abi_capabilities();
    let input = DxReactClientIslandInput {
        route: "/analytics".to_string(),
        route_source_path: "app/analytics/page.tsx".to_string(),
        route_source: r#"import { Chart } from "../../components/local/Chart";

export default function Page() {
  return (
    <main>
      <Chart
        title="Revenue"
        clientVisible={{ rootMargin: "200px" }}
        clientIdle={{ timeout: 1200 }}
        clientOnly="react"
      />
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "Chart".to_string(),
            source_path: "components/local/Chart.tsx".to_string(),
            source: r#""use client";

import { useState } from "react";

export function Chart({ title }: { title: string }) {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>{title}: {count}</button>;
}
"#
            .to_string(),
            package_id: None,
        }],
        route_delivery_mode: DxDeliveryMode::MicroJs,
    };

    let manifest = compile_react_client_islands(input);
    let island = manifest
        .islands
        .iter()
        .find(|island| island.source_path == "components/local/Chart.tsx")
        .expect("chart island");

    assert_eq!(manifest.abi.schema, "dx.react.clientIsland.abi");
    assert_eq!(
        capabilities.schema,
        "dx.react.clientIsland.abi.capabilities"
    );
    assert_eq!(
        capabilities.supported_directives,
        manifest.abi.supported_directives
    );
    assert!(!capabilities.readiness_release_ready);
    assert_eq!(
        capabilities.browser_proof_status,
        "foundation-not-release-proof"
    );
    assert!(
        capabilities
            .adapter_boundary_required
            .iter()
            .any(|requirement| requirement.contains("React hydration"))
    );
    assert_eq!(manifest.abi.directive_style, "camelCase-jsx-props");
    assert!(manifest.abi.source_owned_runtime);
    assert!(!manifest.abi.node_modules_required);
    assert!(!manifest.abi.full_react_hydration);
    assert!(manifest.abi.no_js_fallback_required);
    assert_eq!(manifest.abi.island_count, 1);
    assert_eq!(manifest.abi.framework_adapter_count, 1);
    assert_eq!(manifest.abi.client_visible_count, 1);
    assert_eq!(manifest.abi.client_idle_count, 1);
    assert_eq!(manifest.abi.client_only_count, 1);
    assert_eq!(manifest.abi.explicit_frameworks, vec!["react".to_string()]);
    assert!(
        manifest
            .abi
            .supported_directives
            .contains(&"clientVisible".to_string())
    );
    assert!(
        manifest
            .abi
            .unsupported_directive_syntax
            .contains(&"client:visible".to_string())
    );
    assert!(island.directives.iter().any(|directive| {
        directive.name == "clientVisible"
            && directive.expression.as_deref() == Some(r#"{ rootMargin: "200px" }"#)
    }));
    assert!(island.directives.iter().any(|directive| {
        directive.name == "clientIdle"
            && directive.expression.as_deref() == Some("{ timeout: 1200 }")
    }));
    assert!(island.directives.iter().any(|directive| {
        directive.name == "clientOnly" && directive.value.as_deref() == Some("react")
    }));
    assert_eq!(island.hydration.strategy, "framework-adapter-client-only");
    assert_eq!(island.hydration.directives, island.directives);
    assert!(island.hydration.props.iter().any(|prop| {
        prop.name == "title"
            && prop.source == "route-prop"
            && prop.value.as_deref() == Some("Revenue")
    }));
    assert!(
        !island
            .hydration
            .props
            .iter()
            .any(|prop| prop.name.starts_with("client"))
    );
}

#[test]
fn react_client_island_state_lowering_accepts_common_react_patterns() {
    let input = DxReactClientIslandInput {
        route: "/panel".to_string(),
        route_source_path: "app/panel/page.tsx".to_string(),
        route_source: r#"import { StatusPanel } from "../../components/local/StatusPanel";

export default function Page() {
  return <StatusPanel />;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "StatusPanel".to_string(),
            source_path: "components/local/StatusPanel.tsx".to_string(),
            source: r#""use client";

import React from "react";

export function StatusPanel() {
  const [count, setCount] = React.useState<number>(() => 2);
  const [open, setOpen] = React.useState<boolean>(() => false);
  const doubled = count * 2;

  function toggleOpen() {
    setOpen((current) => !current);
  }

  return (
    <section>
      <button onClick={() => setCount((value) => value + 3)}>Add three</button>
      <button onClick={toggleOpen}>Toggle</button>
      <p>{doubled}</p>
      <p>{open ? "Open" : "Closed"}</p>
    </section>
  );
}
"#
            .to_string(),
            package_id: None,
        }],
        route_delivery_mode: DxDeliveryMode::MicroJs,
    };

    let manifest = compile_react_client_islands(input);
    let island = manifest.islands.first().expect("client island");

    assert_eq!(manifest.runtime, "js");
    assert_eq!(island.state_vars, 2);
    assert_eq!(
        island
            .state
            .iter()
            .map(|state| (state.name.as_str(), state.initial_source.as_str()))
            .collect::<Vec<_>>(),
        vec![("count", "() => 2"), ("open", "() => false")]
    );
    assert!(island.events.iter().any(|event| {
        event.event == "click"
            && event.handler.contains("setCount")
            && event.operation.as_deref() == Some("add:3")
    }));
    assert!(island.events.iter().any(|event| {
        event.event == "click"
            && event.handler == "toggleOpen"
            && event.operation.as_deref() == Some("toggle")
    }));

    let micro_js = island.micro_js.as_ref().expect("js state runtime");
    assert_eq!(micro_js.action_count, 2);
    assert!(micro_js.script.contains("!"));
    assert!(micro_js.script.contains("+=3"));
}

#[test]
fn route_unit_state_graph_accepts_react_namespace_typed_and_lazy_state() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/panel".to_string(),
        route_source_path: "app/panel/page.tsx".to_string(),
        route_source: r#"import { StatusPanel } from "../../components/local/StatusPanel";

export default function Page() {
  return <StatusPanel />;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "StatusPanel".to_string(),
            source_path: "components/local/StatusPanel.tsx".to_string(),
            source: r#""use client";

import React from "react";

export function StatusPanel() {
  const [count, setCount] = React.useState<number>(() => 2);
  const [open, setOpen] = React.useState<boolean>(() => false);
  const label = open ? `Count ${count}` : "Closed";

  return (
    <section>
      <button onClick={() => setCount((value) => value + 3)}>Add three</button>
      <button onClick={() => setOpen((value) => !value)}>Toggle</button>
      <p>{label}</p>
    </section>
  );
}
"#
            .to_string(),
            package_id: Some("www/starter-ui".to_string()),
        }],
        styles: Vec::new(),
        source_manifest_hash: Some("forge-manifest-hash".to_string()),
    })
    .expect("compile route unit");

    let state = &proof.route_unit.state;
    assert_eq!(state.slots.len(), 2);
    assert!(state.slots.iter().any(|slot| {
        slot.name == "count"
            && slot.setter.as_deref() == Some("setCount")
            && slot.value_kind == "number"
            && slot.initial_source == "() => 2"
    }));
    assert!(state.slots.iter().any(|slot| {
        slot.name == "open"
            && slot.setter.as_deref() == Some("setOpen")
            && slot.value_kind == "boolean"
            && slot.initial_source == "() => false"
    }));
    assert!(
        state
            .derived_slots
            .iter()
            .any(|slot| slot.name == "label" && slot.dependencies == vec!["count", "open"])
    );
    assert_eq!(proof.delivery_mode, DxDeliveryMode::MicroJs);
}

#[test]
fn react_app_route_promotes_complex_client_boundary_to_wasm_core() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/workspace".to_string(),
        route_source_path: "app/workspace/page.tsx".to_string(),
        route_source: r#"import { WorkspaceGrid } from "../../components/local/WorkspaceGrid";

export default function Page() {
  return <WorkspaceGrid />;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "WorkspaceGrid".to_string(),
            source_path: "components/local/WorkspaceGrid.tsx".to_string(),
            source: r#""use client";
import { useEffect, useState } from "react";

export function WorkspaceGrid() {
  const [a] = useState(0);
  const [b] = useState(0);
  const [c] = useState(0);
  const [d] = useState(0);
  const [e] = useState(0);
  const [f] = useState(0);
  useEffect(() => {}, []);
  return (
    <section>
      <h1>Workspace grid</h1>
      <button onClick={() => a}>A</button>
      <button onClick={() => b}>B</button>
      <button onClick={() => c}>C</button>
      <button onClick={() => d}>D</button>
      <button onClick={() => e}>E</button>
      <button onClick={() => f}>F</button>
      <button onClick={() => a + b}>G</button>
      <button onClick={() => c + d}>H</button>
      <button onClick={() => e + f}>I</button>
      <button onClick={() => a + f}>J</button>
    </section>
  );
}
"#
            .to_string(),
            package_id: None,
        }],
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile complex client route");

    assert_eq!(proof.delivery_mode, DxDeliveryMode::WasmCore);
    assert!(
        proof
            .fallback
            .html
            .contains("data-dx-runtime=\"wasm-core\"")
    );
}

#[test]
fn react_import_resolution_handles_aliases_forge_files_adapters_and_strict_bare_imports() {
    let resolutions = resolve_react_imports(
        "app/page.tsx",
        r#"import { WelcomeCard } from "../components/local/WelcomeCard";
import { Button } from "@/components/ui/button";
import { cn } from "clsx";
import { SearchIcon } from "dx/icon/search";
import legacy from "legacy-unreviewed-package";
import { useState } from "react";
import Link from "next/link";
import Image from "next/image";
import { NextResponse } from "next/server";
import { redirect } from "next/navigation";
import { headers } from "next/headers";
import { cookies } from "next/cookies";
"#,
        DxReactImportResolverConfig {
            aliases: vec![DxReactImportAlias {
                prefix: "@/".to_string(),
                target_root: "".to_string(),
            }],
            forge_files: vec![DxReactForgeOwnedFile {
                import_specifier: "dx/icon/search".to_string(),
                source_path: "components/ui/search-icon.tsx".to_string(),
                package_id: "dx/icon/search".to_string(),
            }],
            reviewed_adapters: vec![DxReactReviewedAdapter {
                package_name: "clsx".to_string(),
                adapter_path: "lib/forge/clsx.ts".to_string(),
                package_id: "npm/clsx".to_string(),
                reviewed: true,
            }],
            strict_no_node_modules: true,
        },
    );

    assert!(resolutions.iter().any(|resolution| {
        resolution.specifier == "../components/local/WelcomeCard"
            && resolution.kind == DxReactImportResolutionKind::RelativeLocal
            && resolution.resolved_path.as_deref() == Some("components/local/WelcomeCard")
            && !resolution.requires_node_modules
    }));
    assert!(resolutions.iter().any(|resolution| {
        resolution.specifier == "@/components/ui/button"
            && resolution.kind == DxReactImportResolutionKind::AliasLocal
            && resolution.resolved_path.as_deref() == Some("components/ui/button")
    }));
    assert!(resolutions.iter().any(|resolution| {
        resolution.specifier == "dx/icon/search"
            && resolution.kind == DxReactImportResolutionKind::ForgeOwned
            && resolution.package_id.as_deref() == Some("dx/icon/search")
    }));
    assert!(resolutions.iter().any(|resolution| {
        resolution.specifier == "clsx"
            && resolution.kind == DxReactImportResolutionKind::ReviewedAdapter
            && resolution.resolved_path.as_deref() == Some("lib/forge/clsx.ts")
    }));
    assert!(resolutions.iter().any(|resolution| {
        resolution.specifier == "react"
            && resolution.kind == DxReactImportResolutionKind::CompilerIntrinsic
            && !resolution.requires_node_modules
    }));
    for specifier in [
        "next/link",
        "next/image",
        "next/server",
        "next/navigation",
        "next/headers",
        "next/cookies",
    ] {
        assert!(resolutions.iter().any(|resolution| {
            resolution.specifier == specifier
                && resolution.kind == DxReactImportResolutionKind::CompilerIntrinsic
                && !resolution.requires_node_modules
        }));
    }
    assert!(resolutions.iter().any(|resolution| {
        resolution.specifier == "legacy-unreviewed-package"
            && resolution.kind == DxReactImportResolutionKind::BlockedNodeModules
            && resolution.requires_node_modules
    }));
}

#[test]
fn react_app_route_compiles_metadata_canonical_links_and_style_sources() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/docs".to_string(),
        route_source_path: "app/docs/page.tsx".to_string(),
        route_source: r#"export const metadata = {
  title: "DX Docs",
  description: "Source-owned docs route.",
  alternates: {
    canonical: "/docs",
  },
};

export default function Page() {
  return <main><h1>Docs route</h1><img src="/logo.svg" alt="DX" /></main>;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: Vec::new(),
        styles: vec![
            DxReactStyleSource {
                source_path: "styles/tokens.css".to_string(),
                source: ":root { --dx-bg: #fff; }".to_string(),
            },
            DxReactStyleSource {
                source_path: "styles/global.css".to_string(),
                source: ".dx-shell { min-height: 100vh; }".to_string(),
            },
        ],
        source_manifest_hash: None,
    })
    .expect("compile metadata route");

    assert!(proof.fallback.html.contains("<title>DX Docs</title>"));
    assert!(
        proof
            .fallback
            .html
            .contains(r#"<meta name="description" content="Source-owned docs route.">"#)
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"<link rel="canonical" href="/docs">"#)
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"<link rel="stylesheet" href="/styles/tokens.css">"#)
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"<link rel="stylesheet" href="/styles/global.css">"#)
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"data-dx-assets="/logo.svg""#)
    );
}

#[test]
fn react_app_route_static_mode_emits_tiny_static_no_js_shell() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/tiny".to_string(),
        route_source_path: "app/tiny/page.tsx".to_string(),
        route_source: r#"export const metadata = {
  title: "Tiny Static",
  description: "A no-JS route.",
};

export default function Page() {
  return (
    <main className="tiny-shell">
      <h1>Tiny static page</h1>
      <p>No JavaScript needed.</p>
      <a href="/docs">Docs</a>
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: Vec::new(),
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile tiny static route");

    assert_eq!(proof.delivery_mode, DxDeliveryMode::Static);
    assert!(!proof.route_unit.shell.runtime_required);
    assert!(
        proof
            .route_unit
            .runtime_report
            .tiny_static_route_proof
            .selected
    );
    assert_eq!(
        proof
            .route_unit
            .runtime_report
            .tiny_static_route_proof
            .output_mode,
        "tiny-static"
    );
    assert_eq!(
        proof.route_unit.runtime_report.tiny_static_route_proof.js,
        "none"
    );
    assert_eq!(
        proof
            .route_unit
            .runtime_report
            .tiny_static_route_proof
            .script_tag_count,
        0
    );
    assert!(
        proof
            .route_unit
            .runtime_report
            .tiny_static_route_proof
            .no_js_capable
    );
    assert_eq!(
        proof
            .route_unit
            .runtime_report
            .tiny_static_route_proof
            .html_bytes,
        proof.fallback.bytes
    );
    assert_eq!(
        proof
            .route_unit
            .runtime_report
            .tiny_static_route_proof
            .astro_parity_status,
        "not_yet_claimed"
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"data-dx-output-mode="tiny-static""#)
    );
    assert!(proof.fallback.html.contains(r#"data-dx-js="none""#));
    assert!(proof.fallback.html.contains("<h1>Tiny static page</h1>"));
    assert!(proof.fallback.html.contains("<p>No JavaScript needed.</p>"));
    assert!(proof.fallback.html.contains(r#"<a href="/docs">Docs</a>"#));
    assert!(!proof.fallback.html.contains("<script"));
    assert!(!proof.fallback.html.contains("Copy command"));
    assert!(!proof.fallback.html.contains("Explore forge"));
    assert!(proof.fallback.bytes < 700);
}

#[test]
fn react_app_route_tiny_static_fallback_keeps_static_class_helper_calls() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/forge-proof".to_string(),
        route_source_path: "app/forge-proof/page.tsx".to_string(),
        route_source: r#"export const metadata = {
  title: "Forge package proof",
  description: "A source-owned Forge package route rendered through DX WWW.",
};

export default function Page() {
  return (
    <main
      className={clsx("starter-shell", "forge-backed-shell")}
      data-dx-route="/forge-proof"
      data-forge-package="npm/lodash"
    >
      <section
        className={cn("starter-card", "source-owned-forge-package")}
        aria-labelledby="forge-proof-title"
      >
        <h1 id="forge-proof-title">Forge package rendered from source</h1>
        <p>Rendered without node_modules.</p>
      </section>
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: Vec::new(),
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile forge class helper route");

    assert_eq!(proof.delivery_mode, DxDeliveryMode::Static);
    assert!(
        proof
            .route_unit
            .runtime_report
            .tiny_static_route_proof
            .no_js_capable
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"class="starter-shell forge-backed-shell""#),
        "{}",
        proof.fallback.html
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"class="starter-card source-owned-forge-package""#),
        "{}",
        proof.fallback.html
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"data-forge-package="npm/lodash""#)
    );
    assert!(!proof.fallback.html.contains("<script"));
}

#[test]
fn react_app_route_tiny_static_fallback_closes_self_closing_non_void_elements() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/mobile-companion".to_string(),
        route_source_path: "app/mobile-companion/page.tsx".to_string(),
        route_source: r#"export default function Page() {
  return (
    <main>
      <div data-dx-contract="dx-mobile-companion" hidden />
      <script src="/mobile-companion-runtime.js" defer />
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: Vec::new(),
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile mobile companion static fallback");

    assert_eq!(proof.delivery_mode, DxDeliveryMode::Static);
    assert!(
        proof
            .fallback
            .html
            .contains(r#"<div data-dx-contract="dx-mobile-companion" hidden></div>"#),
        "{}",
        proof.fallback.html
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"<script src="/mobile-companion-runtime.js" defer></script>"#),
        "{}",
        proof.fallback.html
    );
}

#[test]
fn react_app_route_tiny_static_fallback_preserves_mobile_form_attributes() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/mobile-companion".to_string(),
        route_source_path: "app/mobile-companion/page.tsx".to_string(),
        route_source: r#"export default function Page() {
  return (
    <main>
      <input
        id="dx-mobile-pairing-code"
        name="pairing_code"
        type="text"
        inputMode="numeric"
        autoComplete="one-time-code"
        autoCapitalize="none"
        enterKeyHint="done"
        required
      />
      <textarea
        id="dx-mobile-pairing-payload"
        name="pairing_payload"
        rows={4}
        spellCheck={false}
      ></textarea>
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: Vec::new(),
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile mobile companion static fallback");

    assert_eq!(proof.delivery_mode, DxDeliveryMode::Static);
    assert!(
        proof.fallback.html.contains(r#"<input id="dx-mobile-pairing-code" name="pairing_code" type="text" inputmode="numeric" autocomplete="one-time-code" autocapitalize="none" enterkeyhint="done" required>"#),
        "{}",
        proof.fallback.html
    );
    assert!(
        proof.fallback.html.contains(r#"<textarea id="dx-mobile-pairing-payload" name="pairing_payload" rows="4" spellcheck="false"></textarea>"#),
        "{}",
        proof.fallback.html
    );
}

#[test]
fn react_app_route_lowers_css_modules_and_tokens_into_generated_style_asset() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/products".to_string(),
        route_source_path: "app/products/page.tsx".to_string(),
        route_source: r#"import { ProductCard } from "../../components/ProductCard";

export default function Page() {
  return <main><ProductCard /></main>;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "ProductCard".to_string(),
            source_path: "components/ProductCard.tsx".to_string(),
            source: r#"import styles from "./ProductCard.module.css";

export function ProductCard() {
  return <section className={styles.card}><h1>Products</h1></section>;
}
"#
            .to_string(),
            package_id: None,
        }],
        styles: vec![
            DxReactStyleSource {
                source_path: "styles/tokens.css".to_string(),
                source: ":root { --dx-accent: #2563eb; }".to_string(),
            },
            DxReactStyleSource {
                source_path: "components/ProductCard.module.css".to_string(),
                source: ".card { color: var(--dx-accent); padding: 1rem; }".to_string(),
            },
        ],
        source_manifest_hash: None,
    })
    .expect("compile css module route");

    let generated = proof
        .generated_styles
        .first()
        .expect("generated style asset");
    assert!(generated.output_path.starts_with("_dx/styles/react-route-"));
    assert!(generated.href.starts_with("/_dx/styles/react-route-"));
    assert!(generated.css.contains("--dx-accent"));
    assert!(generated.css.contains("color:var(--dx-accent)"));
    assert_eq!(
        generated
            .module_exports
            .iter()
            .find(|export| export.local_name == "card")
            .map(|export| export.source_path.as_str()),
        Some("components/ProductCard.module.css")
    );
    let scoped_class = generated
        .module_exports
        .iter()
        .find(|export| export.local_name == "card")
        .map(|export| export.class_name.as_str())
        .expect("scoped css module class");
    assert!(scoped_class.starts_with("card__"));
    assert!(
        proof
            .page_graph
            .styles
            .classes
            .iter()
            .any(|class| class.name == scoped_class)
    );
    assert!(proof.fallback.html.contains(&format!(
        r#"<link rel="stylesheet" href="{}" data-dx-generated="true">"#,
        generated.href
    )));
}

#[test]
fn react_app_route_emits_streaming_deferred_and_resumable_island_proof() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/checkout".to_string(),
        route_source_path: "app/checkout/page.tsx".to_string(),
        route_source: r#"import { BuyButton } from "../../components/BuyButton";

export default async function Page() {
  return (
    <main>
      <h1>Checkout</h1>
      <BuyButton />
    </main>
  );
}
"#
        .to_string(),
        segments: vec![DxReactAppSegmentSource {
            kind: DxReactAppSegmentKind::Loading,
            source_path: "app/checkout/loading.tsx".to_string(),
            source: r#"export default function Loading() {
  return <p>Preparing checkout</p>;
}
"#
            .to_string(),
        }],
        components: vec![DxReactComponentSource {
            name: "BuyButton".to_string(),
            source_path: "components/BuyButton.tsx".to_string(),
            source: r#""use client";

import { useState } from "react";

export function BuyButton() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>Buy {count}</button>;
}
"#
            .to_string(),
            package_id: None,
        }],
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile streaming route");

    assert!(proof.streaming.enabled);
    assert_eq!(proof.streaming.strategy, "shell-first-deferred-boundaries");
    assert!(
        proof
            .streaming
            .first_flush_html
            .contains("data-dx-stream-shell")
    );
    assert!(proof.streaming.shell_bytes < proof.fallback.bytes);
    assert_eq!(proof.streaming.deferred_chunks.len(), 1);
    assert_eq!(
        proof.streaming.deferred_chunks[0].source_path,
        "app/checkout/loading.tsx"
    );
    assert_eq!(proof.streaming.deferred_chunks[0].boundary, "loading");
    assert_eq!(proof.streaming.resumable_islands.len(), 1);
    assert_eq!(
        proof.streaming.resumable_islands[0].source_path,
        "components/BuyButton.tsx"
    );
    assert_eq!(proof.streaming.resumable_islands[0].runtime, "js");
    assert!(
        proof
            .fallback
            .html
            .contains(r#"data-dx-streaming="shell-first""#)
    );
    assert!(
        proof
            .fallback
            .html
            .contains(r#"data-dx-resumable-islands="1""#)
    );
}

#[test]
fn react_app_route_exposes_template_control_flow_hints() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/feed".to_string(),
        route_source_path: "app/feed/page.tsx".to_string(),
        route_source: r#"export default function Page({ ready, items }) {
  return (
    <main>
      {ready && <h1>Feed Ready</h1>}
      <ul>
        {items.map((item) => (
          <li key={item.id}>{item.label}</li>
        ))}
      </ul>
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: Vec::new(),
        styles: Vec::new(),
        source_manifest_hash: None,
    })
    .expect("compile control-flow route");

    assert!(proof.fallback.html.contains(r#"data-dx-conditionals="1""#));
    assert!(proof.fallback.html.contains(r#"data-dx-lists="1""#));
    assert!(proof.fallback.html.contains(r#"data-dx-keys="1""#));
}

#[test]
fn canonical_packet_contract_uses_distinct_magic_and_round_trips() {
    let section = DxPacketSection::new(
        DxPacketSectionKind::TemplateSlots,
        DxPacketSectionEncoding::DeliveryLab,
        b"DXT1\x01\x00".to_vec(),
    );
    let packet = DxPacket::new(DxPacketKind::Route, vec![section.clone()]);

    assert_eq!(packet.header.magic, DX_PACKET_MAGIC);
    assert_eq!(packet.header.version, DX_PACKET_VERSION);
    assert!(packet.is_compatible());
    assert_eq!(packet.header.payload_len, section.bytes.len() as u32);
    assert_ne!(packet.header.magic, *b"DXOB");
    assert_ne!(packet.header.magic, *b"DXB1");

    let json = serde_json::to_string(&packet).expect("packet JSON");
    let restored: DxPacket = serde_json::from_str(&json).expect("packet round trip");

    assert_eq!(restored, packet);
    assert!(restored.is_compatible());

    let encoded = DxPacketCodec::encode(&packet).expect("packet encode");
    let decoded = DxPacketCodec::decode(&encoded).expect("packet decode");

    assert_eq!(decoded, packet);
    assert_eq!(
        encoded.len(),
        DxPacketCodec::HEADER_LEN as usize
            + DxPacketCodec::SECTION_HEADER_LEN as usize
            + section.bytes.len()
    );
}

#[test]
fn page_graph_contract_keeps_fallback_packet_and_forge_manifest_separate() {
    let fallback = DxFallbackHtml::crawlable("<main><h1>DX-WWW</h1></main>");
    let graph = DxPageGraph {
        route_id: "/".to_string(),
        source_path: Some("app/home.html".to_string()),
        root_component_id: "home".to_string(),
        components: DxComponentGraph {
            nodes: vec![DxComponentNode {
                id: "home".to_string(),
                name: "HomePage".to_string(),
                package_id: Some("shadcn/ui/button".to_string()),
                content_hash: "component-hash".to_string(),
            }],
            edges: Vec::new(),
        },
        styles: DxStyleGraph {
            tokens: vec![DxStyleToken {
                name: "color.accent".to_string(),
                value: "oklch(0.72 0.15 250)".to_string(),
            }],
            classes: vec![DxStyleClass {
                name: "btn".to_string(),
                rule: "display:inline-flex".to_string(),
                source_hash: Some("style-hash".to_string()),
            }],
            delivery: DxStyleDelivery::GeneratedCss,
        },
        source_manifest_hash: Some("forge-manifest-hash".to_string()),
    };
    let manifest = DxSourceManifest::default();

    assert!(fallback.crawlable);
    assert_eq!(fallback.bytes, fallback.html.len());
    assert_eq!(
        graph.components.nodes[0].package_id.as_deref(),
        Some("shadcn/ui/button")
    );
    assert_eq!(graph.styles.delivery, DxStyleDelivery::GeneratedCss);
    assert_eq!(manifest.version, 1);
}

#[test]
fn react_app_route_exposes_route_unit_state_graph_and_runtime_report() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/dashboard".to_string(),
        route_source_path: "app/dashboard/page.tsx".to_string(),
        route_source: r#"import { DashboardCounter } from "../../components/DashboardCounter";
import { saveDashboardView } from "../../server/actions";

export default function Page() {
  return (
    <main>
      <DashboardCounter onSave={saveDashboardView} />
    </main>
  );
}
"#
        .to_string(),
        segments: vec![DxReactAppSegmentSource {
            kind: DxReactAppSegmentKind::Layout,
            source_path: "app/layout.tsx".to_string(),
            source: "export default function Layout({ children }) { return <>{children}</>; }\n"
                .to_string(),
        }],
        components: vec![DxReactComponentSource {
            name: "DashboardCounter".to_string(),
            source_path: "components/DashboardCounter.tsx".to_string(),
            source: r#""use client";

import { useEffect, useState } from "react";

export function DashboardCounter({ onSave }) {
  const [count, setCount] = useState(0);
  const doubled = count * 2;

  useEffect(() => {
    document.title = `Views ${count}`;
  }, [count]);

  return (
    <section>
      <p>{doubled}</p>
      <button type="button" onClick={() => setCount(count + 1)}>Add</button>
      <form action={onSave}>
        <button type="submit">Save</button>
      </form>
    </section>
  );
}
"#
            .to_string(),
            package_id: Some("www/starter-ui".to_string()),
        }],
        styles: Vec::new(),
        source_manifest_hash: Some("forge-manifest-hash".to_string()),
    })
    .expect("compile route unit");

    let unit = &proof.route_unit;
    assert_eq!(unit.version, 1);
    assert_eq!(unit.route, "/dashboard");
    assert_eq!(unit.shell.fallback.bytes, proof.fallback.bytes);
    assert_eq!(unit.graph.route_id, proof.page_graph.route_id);
    assert_eq!(unit.packet.kind, DxPacketKind::Route);
    assert_eq!(
        unit.receipt.source_manifest_hash.as_deref(),
        Some("forge-manifest-hash")
    );
    assert_eq!(unit.state.default_scope, DxStateScope::Local);
    assert_eq!(unit.state.slots.len(), 1);
    assert_eq!(unit.state.slots[0].name, "count");
    assert_eq!(unit.state.slots[0].setter.as_deref(), Some("setCount"));
    assert_eq!(unit.state.derived_slots[0].name, "doubled");
    assert_eq!(unit.state.event_slots.len(), 2);
    assert!(
        unit.state
            .effects
            .iter()
            .any(|effect| effect.kind == "effect")
    );
    assert!(
        unit.state
            .server_actions
            .iter()
            .any(|edge| edge.action == "saveDashboardView")
    );
    assert_eq!(unit.runtime_report.selected_mode, DxDeliveryMode::MicroJs);
    assert!(
        unit.runtime_report
            .candidates
            .iter()
            .any(|candidate| candidate.mode == DxDeliveryMode::Static && !candidate.selected)
    );
    assert!(
        unit.runtime_report
            .rejected_modes
            .iter()
            .any(|mode| mode.mode == DxDeliveryMode::WasmCore)
    );
}

#[test]
fn route_unit_event_slots_use_native_dom_event_catalog() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/events".to_string(),
        route_source_path: "app/events/page.tsx".to_string(),
        route_source: r#"import { EventPanel } from "../../components/local/EventPanel";

export default function Page() {
  return <EventPanel />;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![DxReactComponentSource {
            name: "EventPanel".to_string(),
            source_path: "components/local/EventPanel.tsx".to_string(),
            source: r#""use client";
import { useState } from "react";

export function EventPanel() {
  const [count, setCount] = useState(0);
  const [name, setName] = useState("");

  return (
    <section>
      <button onClick={() => setCount(count + 1)}>Click</button>
      <button onDoubleClick={() => setCount(count + 2)}>Double</button>
      <button onPointerMove={() => setCount(count + 3)}>Pointer</button>
      <input name="name" value={name} onBeforeInput={(event) => setName(event.currentTarget.value)} />
      <button onClick="scale-up bg-accent">Class interaction</button>
      <button onMagicGesture={() => setCount(count + 4)}>Unsupported</button>
    </section>
  );
}
"#
            .to_string(),
            package_id: None,
        }],
        styles: Vec::new(),
        source_manifest_hash: Some("event-manifest-hash".to_string()),
    })
    .expect("compile event route");

    let events = proof
        .route_unit
        .state
        .event_slots
        .iter()
        .map(|event| event.event.as_str())
        .collect::<Vec<_>>();

    assert!(events.contains(&"click"));
    assert!(events.contains(&"dblclick"));
    assert!(events.contains(&"pointermove"));
    assert!(events.contains(&"beforeinput"));
    assert!(!events.contains(&"doubleClick"));
    assert!(!events.contains(&"magicgesture"));
    assert_eq!(events.len(), 4);
    assert_eq!(
        react_style_event_attribute_to_dom_event("onDoubleClick").as_deref(),
        Some("dblclick")
    );
    assert_eq!(
        react_style_event_attribute_to_dom_event("onPointerMove").as_deref(),
        Some("pointermove")
    );
    assert!(react_style_event_attribute_to_dom_event("onMagicGesture").is_none());
}

#[test]
fn route_unit_links_imported_global_store_actions_across_sources() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/stores".to_string(),
        route_source_path: "app/stores/page.tsx".to_string(),
        route_source: r#"import { CounterPanel } from "../../components/CounterPanel";

export default function Page() {
  return <CounterPanel />;
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![
            DxReactComponentSource {
                name: "CounterPanel".to_string(),
                source_path: "components/CounterPanel.tsx".to_string(),
                source: r#""use client";
import { counterStore } from "../lib/stores/counter";

export function CounterPanel() {
  return (
    <section>
      <p data-dx-state-read="counterStore.count">{counterStore.count}</p>
      <button type="button" onClick={() => counterStore.increment(counterStore)}>Add</button>
    </section>
  );
}
"#
                .to_string(),
                package_id: None,
            },
            DxReactComponentSource {
                name: "counterStore".to_string(),
                source_path: "lib/stores/counter.ts".to_string(),
                source: r#"export const counterStore = store({
  count: state(1),
  doubled: derived((store) => store.count * 2),
  increment: action((store) => { store.count += 1; }),
});
"#
                .to_string(),
                package_id: None,
            },
        ],
        styles: Vec::new(),
        source_manifest_hash: Some("store-manifest-hash".to_string()),
    })
    .expect("compile store route");

    let state = &proof.route_unit.state;
    assert_eq!(state.default_scope, DxStateScope::Global);
    assert_eq!(state.global_stores.len(), 1);
    assert_eq!(state.global_stores[0].name, "counterStore");
    assert!(
        state
            .slots
            .iter()
            .any(|slot| slot.name == "counterStore.count" && slot.scope == DxStateScope::Global)
    );
    assert!(
        state
            .derived_slots
            .iter()
            .any(|slot| slot.name == "counterStore.doubled")
    );
    assert!(
        state
            .event_slots
            .iter()
            .any(|event| event.action.as_deref() == Some("counterStore.increment"))
    );
}

#[test]
fn react_app_route_fallback_ignores_slot_children_headings() {
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: "/launch".to_string(),
        route_source_path: "app/page.tsx".to_string(),
        route_source: r#"import { SlotHeading } from "../../components/SlotHeading";
import { LaunchDashboard } from "../../components/LaunchDashboard";

export default function Page() {
  return (
    <main>
      <SlotHeading>
        <LaunchDashboard />
      </SlotHeading>
    </main>
  );
}
"#
        .to_string(),
        segments: Vec::new(),
        components: vec![
            DxReactComponentSource {
                name: "SlotHeading".to_string(),
                source_path: "components/SlotHeading.tsx".to_string(),
                source: r#"export function SlotHeading({ children }) {
  return <h1>{children}</h1>;
}
"#
                .to_string(),
                package_id: None,
            },
            DxReactComponentSource {
                name: "LaunchDashboard".to_string(),
                source_path: "components/LaunchDashboard.tsx".to_string(),
                source: r#"export function LaunchDashboard() {
  return (
    <section>
      <h1>Source-owned launch dashboard</h1>
      <p>React-familiar apps with source-owned packages.</p>
    </section>
  );
}
"#
                .to_string(),
                package_id: None,
            },
        ],
        styles: Vec::new(),
        source_manifest_hash: Some("tsx-slot-heading-hash".to_string()),
    })
    .expect("compile launch route");

    assert!(
        proof
            .fallback
            .html
            .contains("Source-owned launch dashboard")
    );
    assert!(!proof.fallback.html.contains("{children}"));
}

#[test]
fn next_app_router_conformance_fixtures_cover_strict_no_node_modules_routes() {
    let report = next_app_router_conformance_report();

    assert!(report.strict_no_node_modules);
    assert!(
        report
            .fixtures
            .iter()
            .any(|fixture| fixture.name == "static-page" && fixture.route == "/")
    );
    assert!(
        report
            .fixtures
            .iter()
            .any(|fixture| fixture.name == "client-island"
                && fixture.delivery_mode == DxDeliveryMode::MicroJs)
    );
    assert!(
        report
            .fixtures
            .iter()
            .any(|fixture| fixture.name == "form-server-action" && fixture.server_action_edges > 0)
    );
    assert!(
        report
            .fixtures
            .iter()
            .any(|fixture| fixture.name == "dynamic-route" && fixture.route.contains("[slug]"))
    );
    assert!(
        report
            .fixtures
            .iter()
            .any(|fixture| fixture.name == "metadata-image-font" && fixture.metadata_entries > 0)
    );
    assert!(
        report
            .fixtures
            .iter()
            .all(|fixture| !fixture.node_modules_required)
    );
}

#[test]
fn canonical_packet_codec_round_trips_every_packet_kind() {
    let kinds = [
        DxPacketKind::Route,
        DxPacketKind::TemplateDictionary,
        DxPacketKind::InstanceBatch,
        DxPacketKind::PatchStream,
        DxPacketKind::Style,
        DxPacketKind::Manifest,
    ];

    for kind in kinds {
        let packet = DxPacket::new(
            kind,
            vec![DxPacketSection::new(
                DxPacketSectionKind::SourceManifest,
                DxPacketSectionEncoding::Json,
                format!(r#"{{"kind":"{kind:?}"}}"#).into_bytes(),
            )],
        );

        let encoded = DxPacketCodec::encode(&packet).expect("packet encode");
        let decoded = DxPacketCodec::decode(&encoded).expect("packet decode");

        assert_eq!(decoded.header.kind, kind);
        assert_eq!(decoded, packet);
    }
}

#[test]
fn canonical_packet_codec_round_trips_section_kinds_and_encodings() {
    let sections = vec![
        DxPacketSection::new(
            DxPacketSectionKind::FallbackHtmlRef,
            DxPacketSectionEncoding::Json,
            br#"{"hash":"fallback"}"#.to_vec(),
        ),
        DxPacketSection::new(
            DxPacketSectionKind::TemplateSlots,
            DxPacketSectionEncoding::DeliveryLab,
            b"DXT1-template".to_vec(),
        ),
        DxPacketSection::new(
            DxPacketSectionKind::ColumnarSlots,
            DxPacketSectionEncoding::CanonicalBinary,
            b"column-binary".to_vec(),
        ),
        DxPacketSection::new(
            DxPacketSectionKind::SemanticCodec,
            DxPacketSectionEncoding::CanonicalBinary,
            b"semantic-binary".to_vec(),
        ),
        DxPacketSection::new(
            DxPacketSectionKind::PatchOps,
            DxPacketSectionEncoding::HtipV2,
            b"patch-htip".to_vec(),
        ),
        DxPacketSection::new(
            DxPacketSectionKind::StyleGraph,
            DxPacketSectionEncoding::Json,
            br#"{"delivery":"generated-css"}"#.to_vec(),
        ),
        DxPacketSection::new(
            DxPacketSectionKind::SourceManifest,
            DxPacketSectionEncoding::Json,
            br#"{"version":1}"#.to_vec(),
        ),
    ];
    let packet = DxPacket::new(DxPacketKind::Route, sections);

    let encoded = DxPacketCodec::encode(&packet).expect("packet encode");
    let decoded = DxPacketCodec::decode(&encoded).expect("packet decode");

    assert_eq!(decoded, packet);
    assert_eq!(decoded.sections.len(), 7);
}

#[test]
fn canonical_packet_codec_rejects_bad_magic_version_and_hashes() {
    let packet = DxPacket::new(
        DxPacketKind::Route,
        vec![DxPacketSection::new(
            DxPacketSectionKind::TemplateSlots,
            DxPacketSectionEncoding::DeliveryLab,
            b"DXT1".to_vec(),
        )],
    );
    let encoded = DxPacketCodec::encode(&packet).expect("packet encode");

    let mut bad_magic = encoded.clone();
    bad_magic[0] = b'X';
    assert!(matches!(
        DxPacketCodec::decode(&bad_magic),
        Err(DxPacketCodecError::InvalidMagic { .. })
    ));

    let mut bad_version = encoded.clone();
    bad_version[4] = 9;
    assert!(matches!(
        DxPacketCodec::decode(&bad_version),
        Err(DxPacketCodecError::UnsupportedVersion { .. })
    ));

    let mut bad_payload_hash = encoded.clone();
    bad_payload_hash[20] ^= 0xFF;
    assert!(matches!(
        DxPacketCodec::decode(&bad_payload_hash),
        Err(DxPacketCodecError::PayloadHashMismatch { .. })
    ));

    let mut bad_section_hash = encoded.clone();
    bad_section_hash[60] ^= 0xFF;
    assert!(matches!(
        DxPacketCodec::decode(&bad_section_hash),
        Err(DxPacketCodecError::SectionHashMismatch { .. })
    ));

    let mut truncated = encoded;
    truncated.pop();
    assert!(matches!(
        DxPacketCodec::decode(&truncated),
        Err(DxPacketCodecError::Truncated { .. })
    ));
}

#[test]
fn template_packet_is_smaller_than_plain_html_repetition() {
    let templates = vec![template()];
    let encoded = DxPacketEncoder::encode_template_slots(&templates);
    assert!(encoded.starts_with(b"DXT1"));
    assert!(encoded.len() < 128);
}

#[test]
fn column_batch_encodes_repeated_dashboard_rows() {
    let batch = sample_dashboard_column_batch(1200);
    let encoded = DxPacketEncoder::encode_column_batch(&batch);
    assert!(encoded.starts_with(b"DXC1"));
    assert!(encoded.len() < 24_000);
}

#[test]
fn patch_stream_encodes_small_updates() {
    let ops = sample_dashboard_patch_ops(1200, 12);
    let encoded = DxPacketEncoder::encode_patch_stream(&ops);
    assert!(encoded.starts_with(b"DXP1"));
    assert!(encoded.len() < 240);
}

#[test]
fn planner_selects_micro_for_tiny_interaction() {
    let templates = vec![template()];
    let plan = plan_delivery(&metrics(), RuntimeVariant::Micro, &templates, &[], &[], 128);
    assert_eq!(plan.primary_mode, DxDeliveryMode::MicroJs);
}

#[test]
fn planner_selects_static_when_no_runtime_is_needed() {
    let mut metrics = metrics();
    metrics.event_handler_count = 0;
    metrics.total_state_vars = 0;
    let templates = vec![template()];
    let plan = plan_delivery(&metrics, RuntimeVariant::Micro, &templates, &[], &[], 128);
    assert_eq!(plan.primary_mode, DxDeliveryMode::Static);
}

#[test]
fn semantic_sequence_is_tiny() {
    let sequence = DxSemanticSequence {
        prefix: "Customer ".to_string(),
        suffix: String::new(),
        start: 1,
        step: 1,
        count: 1200,
    };
    let encoded = DxPacketEncoder::encode_semantic_sequence(&sequence);
    assert!(encoded.starts_with(b"DXS1"));
    assert!(encoded.len() <= 32);
}

#[test]
fn micro_js_counter_ships_no_wasm_runtime() {
    let script = DxMicroJsEmitter::emit(&sample_counter_micro_program());
    assert!(script.len() < 260);
    assert!(script.contains("counter"));
    assert!(!script.contains("fetch"));
    assert!(!script.contains("WebAssembly"));
    assert!(!script.contains(".wasm"));
}

#[test]
fn generated_html_optimizer_removes_structural_noise() {
    let source = r#"
        <!doctype html>
        <html>
          <head>
            <style>
              .card {
                color: red;
                margin: 0  8px;
              }
            </style>
          </head>
          <body>
            <main>
              <p>Hello <strong>binary web</strong> route.</p>
            </main>
          </body>
        </html>
    "#;

    let optimized = optimize_generated_html(source);

    assert_eq!(optimized.profile().delivery_mode, DxDeliveryMode::Static);
    assert!(optimized.profile().saved_bytes > 100);
    assert!(
        optimized
            .html()
            .contains("Hello <strong>binary web</strong> route.")
    );
    assert!(
        optimized
            .html()
            .contains("<style>.card{color:red;margin:0 8px;}</style>")
    );
}

#[test]
fn generated_html_optimizer_profiles_repeated_routes() {
    let mut source = String::from("<!doctype html><html><body><table><tbody>");
    for index in 0..600 {
        source.push_str(&format!("<tr><td>Account {index}</td></tr>"));
    }
    source.push_str("</tbody></table><script>let rows=[];</script></body></html>");

    let optimized = optimize_generated_html(&source);

    assert_eq!(optimized.profile().script_count, 1);
    assert_eq!(optimized.profile().repeated_node_count, 600);
    assert_eq!(
        optimized.profile().delivery_mode,
        DxDeliveryMode::ColumnarSlots
    );
}

#[test]
fn generated_html_optimizer_adds_repeated_content_visibility_hints() {
    let mut source = String::from("<!doctype html><html><head></head><body>");
    source.push_str("<input type=\"search\">");
    for index in 0..180 {
        source.push_str(&format!(
            "<article data-search=\"component {index}\"><h2>Component {index}</h2><p>Repeated route block.</p></article>"
        ));
    }
    source.push_str("<script>let cards=[];</script></body></html>");

    let optimized = optimize_generated_html(&source);

    assert!(optimized.html().contains("content-visibility:auto"));
    assert_eq!(optimized.profile().repeated_node_count, 180);
    assert_eq!(
        optimized.profile().delivery_mode,
        DxDeliveryMode::TemplateSlots
    );
}

#[test]
fn generated_html_optimizer_keeps_static_article_docs_plain() {
    let mut source = String::from("<!doctype html><html><head></head><body>");
    for index in 0..180 {
        source.push_str(&format!(
            "<article><h2>Document {index}</h2><p>Static repeated content.</p></article>"
        ));
    }
    source.push_str("</body></html>");

    let optimized = optimize_generated_html(&source);

    assert!(!optimized.html().contains("content-visibility:auto"));
    assert_eq!(optimized.profile().repeated_node_count, 180);
    assert_eq!(
        optimized.profile().delivery_mode,
        DxDeliveryMode::TemplateSlots
    );
}
