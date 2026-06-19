use serde_json::Value;

const CORE_DIRECTIVES: &[&str] = &["clientLoad", "clientVisible", "clientIdle", "clientOnly"];

pub(super) fn client_island_dev_runtime(source_render: &Value) -> String {
    let Some(manifest) = source_render.get("client_islands") else {
        return String::new();
    };
    let Some(islands) = manifest.get("islands").and_then(Value::as_array) else {
        return String::new();
    };
    if islands.is_empty() {
        return String::new();
    }

    let directives = collect_client_directives(source_render);
    let directive_attr = if directives.is_empty() {
        "none".to_string()
    } else {
        directives.join(",")
    };
    let hydration_strategy = hydration_strategy(&directives);
    let client_only_adapter = client_only_adapter(source_render);

    let mut markers = String::from(
        r#"<div hidden data-dx-client-island-bridge="source-owned" data-dx-client-island-abi="camelCase" data-dx-island-abi-schema="dx.react.clientIsland.abi" data-dx-island-directive-style="camelCase-jsx-props" data-dx-no-js-fallback="preserved" data-dx-client-only-adapters="preview-only" data-dx-client-media-support="recognized-not-executed" data-dx-client-interaction-support="recognized-not-executed" data-dx-browser-proof="browser-event-observed" data-dx-browser-runtime-proof="source-owned-dev-runtime" data-dx-provider-runtime-proof="not-claimed" data-dx-provider-adapters="not-executed" data-dx-client-island-event-count="0" data-dx-client-island-event-node-count="0" data-dx-client-island-event-log="[]">"#,
    );
    for island in islands {
        let id = string_field(island, "id", "client-island");
        let source = string_field(island, "source_path", "unknown");
        markers.push_str(&format!(
            r#"<div data-dx-island="{}" data-dx-island-source="{}" data-dx-delivery-mode="dev-runtime" data-dx-island-abi-schema="dx.react.clientIsland.abi" data-dx-island-directive-style="camelCase-jsx-props" data-dx-island-hydration-strategy="{}" data-dx-island-directives="{}" data-dx-client-only-adapter="{}" data-dx-client-load="{}" data-dx-client-visible="{}" data-dx-client-idle="{}" data-dx-client-media="recognized-not-executed" data-dx-client-interaction="recognized-not-executed" data-dx-no-js-fallback="preserved" data-dx-browser-proof="browser-event-observed" data-dx-browser-runtime-proof="source-owned-dev-runtime" data-dx-provider-runtime-proof="not-claimed" data-dx-provider-adapter="not-executed">"#,
            escape_attr(&id),
            escape_attr(&source),
            escape_attr(&hydration_strategy),
            escape_attr(&directive_attr),
            escape_attr(&client_only_adapter),
            directive_status(&directives, "clientLoad"),
            directive_status(&directives, "clientVisible"),
            directive_status(&directives, "clientIdle"),
        ));
        for (index, event) in event_names(island).into_iter().enumerate() {
            let event_id =
                event_slot_id(island, index).unwrap_or_else(|| format!("{id}-{event}-{index}"));
            markers.push_str(&format!(
                r#"<span data-dx-event-id="{}" data-dx-event="{}" data-dx-event-element="{}" data-dx-event-handler="source-owned-dev-island-runtime"></span>"#,
                escape_attr(&event_id),
                escape_attr(&event),
                escape_attr(&target_tag(island)),
            ));
        }
        markers.push_str("</div>");
    }
    markers.push_str("</div>");

    let manifest_json = serde_json::to_string(manifest)
        .unwrap_or_else(|_| "{\"islands\":[]}".to_string())
        .replace("</script", "<\\/script");
    let runtime = format!(
        r#"<script type="module" id="__DX_CLIENT_ISLAND_DEV_RUNTIME__" data-dx-client-island-runtime="source-owned-dev">
(() => {{
  const manifest = {manifest_json};
  const bridge = document.querySelector("[data-dx-client-island-bridge]");
  if (!bridge) return;
  const eventLog = [];
  function eventsFor(island) {{
    return Array.isArray(island && island.events) ? island.events.filter(Boolean) : [];
  }}
  function textMatches(element, expected) {{
    return !expected || (element.textContent || "").trim() === String(expected).trim();
  }}
  function targetMatches(element, target) {{
    if (!element || !target) return false;
    if (target.tag && element.localName !== target.tag) return false;
    if (target.type && element.localName === "input" && (element.getAttribute("type") || "text") !== target.type) return false;
    if (target.name && element.getAttribute("name") !== target.name) return false;
    return textMatches(element, target.text);
  }}
  function isEvidencePreview(element) {{
    return !!(element && element.closest("[hidden], [aria-hidden='true'], [data-dx-static-dom-preview-hidden='true'], [data-dx-no-js-fallback-preview='true']"));
  }}
  function eventSlotId(island, index, eventName) {{
    const ids = Array.isArray(island && island.event_slot_ids) ? island.event_slot_ids : [];
    return ids[index] || `${{island.id || "client-island"}}-${{eventName}}-${{index}}`;
  }}
  function publishEvent(element, island, eventName, eventId, event) {{
    const detail = {{
      schema: "dx.tsx.clientIslandBrowserEvent",
      schema_revision: 1,
      islandId: island.id || null,
      sourcePath: island.source_path || null,
      eventId,
      event: eventName,
      element: element.localName,
      handler: "source-owned-dev-island-runtime",
      state: "browser-event-observed",
      operation: event && event.type ? event.type : eventName,
      formId: element.form ? element.form.id || null : null,
      full_react_hydration: false,
      react_synthetic_events: false,
      node_modules_required: false
    }};
    eventLog.push(detail);
    element.setAttribute("data-dx-event-id", eventId);
    element.setAttribute("data-dx-event", eventName);
    element.setAttribute("data-dx-client-island-event-count", String(Number(element.getAttribute("data-dx-client-island-event-count") || "0") + 1));
    bridge.setAttribute("data-dx-client-island-event-count", String(eventLog.length));
    bridge.setAttribute("data-dx-client-island-event-log", JSON.stringify(eventLog.slice(-64)));
    element.dispatchEvent(new CustomEvent("dx:client-island-event", {{ bubbles: true, detail }}));
    document.dispatchEvent(new CustomEvent("dx:client-island-event", {{ detail }}));
  }}
  function attachIsland(island) {{
    const target = island && island.target ? island.target : null;
    if (!target || !target.tag) return 0;
    let attached = 0;
    document.querySelectorAll(target.tag).forEach((element) => {{
      if (isEvidencePreview(element)) return;
      if (!targetMatches(element, target)) return;
      eventsFor(island).forEach((eventName, index) => {{
        const eventId = eventSlotId(island, index, eventName);
        const key = `${{island.id}}:${{eventName}}:${{eventId}}`;
        const existing = (element.getAttribute("data-dx-client-island-bound") || "").split(" ").filter(Boolean);
        if (existing.includes(key)) return;
        element.setAttribute("data-dx-client-island-bound", existing.concat(key).join(" "));
        element.setAttribute("data-dx-island-runtime", "source-owned-dev");
        element.addEventListener(eventName, (event) => publishEvent(element, island, eventName, eventId, event));
        attached += 1;
      }});
    }});
    return attached;
  }}
  const islands = Array.isArray(manifest.islands) ? manifest.islands : [];
  let attached = 0;
  islands.forEach((island) => {{ attached += attachIsland(island); }});
  bridge.setAttribute("data-dx-client-island-event-node-count", String(attached));
  window.__DX_CLIENT_ISLAND_RUNTIME__ = {{
    schema: "dx.tsx.clientIslandDevRuntime",
    schema_revision: 1,
    source: "source-owned-dev-runtime",
    manifest,
    attached_event_node_count: attached,
    node_modules_required: false,
    full_react_hydration: false,
    react_synthetic_events: false,
    provider_adapter_executed: false,
    getSnapshot() {{
      return {{
        schema: "dx.tsx.clientIslandDevRuntimeSnapshot",
        schema_revision: 1,
        island_count: islands.length,
        attached_event_node_count: attached,
        client_island_event_count: eventLog.length,
        event_log: eventLog.slice(),
        source_owned_bridge: true,
        bridge_abi_style: "camelCase",
        directive_style: "camelCase-jsx-props",
        no_js_fallback_preserved: true,
        node_modules_required: false,
        full_react_hydration: false,
        react_synthetic_events: false,
        provider_adapter_executed: false
      }};
    }}
  }};
  window.dispatchEvent(new CustomEvent("dx:client-island-runtime-ready", {{ detail: window.__DX_CLIENT_ISLAND_RUNTIME__.getSnapshot() }}));
}})();
</script>"#
    );

    format!("{markers}{runtime}")
}

fn collect_client_directives(source_render: &Value) -> Vec<String> {
    let mut directives = Vec::new();
    collect_directive_props(source_render, &mut directives);
    collect_directive_attributes(source_render, &mut directives);
    collect_composition_directives(source_render, &mut directives);
    CORE_DIRECTIVES
        .iter()
        .filter(|directive| directives.iter().any(|value| value == *directive))
        .map(|directive| (*directive).to_string())
        .collect()
}

fn collect_directive_props(source_render: &Value, directives: &mut Vec<String>) {
    if let Some(props) = source_render.get("prop_bindings").and_then(Value::as_array) {
        for prop in props {
            if let Some(name) = prop.get("name").and_then(Value::as_str) {
                push_directive(directives, name);
            }
        }
    }
}

fn collect_directive_attributes(source_render: &Value, directives: &mut Vec<String>) {
    if let Some(elements) = source_render
        .get("renderable_elements")
        .and_then(Value::as_array)
    {
        for element in elements {
            let Some(attributes) = element.get("attributes").and_then(Value::as_array) else {
                continue;
            };
            for attribute in attributes {
                if attribute.get("name").and_then(Value::as_str)
                    == Some("data-dx-client-directives")
                {
                    if let Some(value) = attribute.get("value").and_then(Value::as_str) {
                        for token in value
                            .split([',', ';', ' '])
                            .filter(|token| !token.is_empty())
                        {
                            push_directive(directives, token);
                        }
                    }
                }
            }
        }
    }
}

fn collect_composition_directives(source_render: &Value, directives: &mut Vec<String>) {
    if let Some(compositions) = source_render
        .get("component_compositions")
        .and_then(Value::as_array)
    {
        for composition in compositions {
            let Some(props) = composition
                .pointer("/invocation_inputs/props")
                .and_then(Value::as_array)
            else {
                continue;
            };
            for prop in props {
                if let Some(name) = prop.get("name").and_then(Value::as_str) {
                    push_directive(directives, name);
                }
            }
        }
    }
}

fn push_directive(directives: &mut Vec<String>, name: &str) {
    if CORE_DIRECTIVES.contains(&name) && !directives.iter().any(|value| value == name) {
        directives.push(name.to_string());
    }
}

fn hydration_strategy(directives: &[String]) -> String {
    if directives.iter().any(|value| value == "clientOnly") {
        "only"
    } else if directives.iter().any(|value| value == "clientIdle") {
        "idle"
    } else if directives.iter().any(|value| value == "clientVisible") {
        "visible"
    } else {
        "load"
    }
    .to_string()
}

fn client_only_adapter(source_render: &Value) -> String {
    source_render
        .get("prop_bindings")
        .and_then(Value::as_array)
        .and_then(|props| {
            props.iter().find_map(|prop| {
                (prop.get("name").and_then(Value::as_str) == Some("clientOnly"))
                    .then(|| prop.get("value").and_then(Value::as_str))
                    .flatten()
            })
        })
        .unwrap_or("not-requested")
        .to_string()
}

fn directive_status(directives: &[String], name: &str) -> &'static str {
    if directives.iter().any(|value| value == name) {
        "observed"
    } else {
        "not-requested"
    }
}

fn event_names(island: &Value) -> Vec<String> {
    island
        .get("events")
        .and_then(Value::as_array)
        .map(|events| {
            events
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn event_slot_id(island: &Value, index: usize) -> Option<String> {
    island
        .get("event_slot_ids")
        .and_then(Value::as_array)
        .and_then(|ids| ids.get(index))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn target_tag(island: &Value) -> String {
    island
        .pointer("/target/tag")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string()
}

fn string_field(value: &Value, field: &str, fallback: &str) -> String {
    value
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
