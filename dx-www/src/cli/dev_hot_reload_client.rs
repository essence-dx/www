use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_EVENT_NAME, DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT, DX_HOT_RELOAD_RESOURCE_MARKER,
    DX_HOT_RELOAD_VERSION_ENDPOINT,
};

pub(super) fn dev_hot_reload_client_script(token: &str, event_stream_supported: bool) -> String {
    let token = serde_json::to_string(token).unwrap_or_else(|_| "\"disabled\"".to_string());
    let event_stream_supported = if event_stream_supported {
        "true"
    } else {
        "false"
    };
    let version_endpoint = DX_HOT_RELOAD_VERSION_ENDPOINT;
    let event_stream_endpoint = DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT;
    let event_stream_event = DX_HOT_RELOAD_EVENT_NAME;
    let resource_marker = DX_HOT_RELOAD_RESOURCE_MARKER;
    let resource_marker_selector = format!("[{resource_marker}]");

    format!(
        r#"<script type="module" data-dx-hot-reload>
(() => {{
  if (window.__DX_HOT_RELOAD_CLIENT_ACTIVE__) return;
  window.__DX_HOT_RELOAD_CLIENT_ACTIVE__ = true;
  let dxReloadToken = {token};
  let dxEventSource = null;
  let dxSubscribedResource = "";
  let dxEventStreamHealthy = false;
  let dxTransportIssueVisible = false;
  const versionEndpoint = "{version_endpoint}";
  const eventStreamEndpoint = "{event_stream_endpoint}";
  const eventStreamEvent = "{event_stream_event}";
  const eventStreamSupported = {event_stream_supported};
  const resourceMarker = "{resource_marker}";
  const resourceMarkerSelector = "{resource_marker_selector}";
  const dxBrowserIsOffline = () => {{
    return ("onLine" in navigator) && navigator.onLine === false;
  }};
  const dxReloadResource = () => {{
    const target = document.querySelector(resourceMarkerSelector)?.getAttribute(resourceMarker);
    return target || `route:${{location.pathname || "/"}}`;
  }};
  const dxPayloadResource = (payload) => {{
    return payload?.resource || payload?.instruction?.resource || {{}};
  }};
  const dxResourceId = (payload) => {{
    return dxPayloadResource(payload)?.id || "";
  }};
  const dxRoutePathFromResource = (payload) => {{
    const id = dxResourceId(payload);
    if (!id.startsWith("route:")) return "";
    const route = id.slice("route:".length) || "/";
    return route.startsWith("/") ? route : `/${{route}}`;
  }};
  const dxNormalizeRoutePath = (value) => {{
    const normalized = (value || "/").replace(/\/+$/, "");
    return normalized || "/";
  }};
  const dxRouteSegmentIsOptionalCatchAll = (segment) => {{
    return segment.startsWith("[[...") && segment.endsWith("]]");
  }};
  const dxRouteSegmentIsCatchAll = (segment) => {{
    return segment.startsWith("[...") && segment.endsWith("]");
  }};
  const dxRouteSegmentIsDynamic = (segment) => {{
    return segment.startsWith("[") && segment.endsWith("]");
  }};
  const dxRoutePatternMatchesPath = (routePath, currentPath) => {{
    const routeSegments = dxNormalizeRoutePath(routePath).split("/").filter(Boolean);
    const currentSegments = dxNormalizeRoutePath(currentPath).split("/").filter(Boolean);
    if (!routeSegments.length) return true;
    let currentIndex = 0;
    for (const routeSegment of routeSegments) {{
      if (dxRouteSegmentIsOptionalCatchAll(routeSegment)) return true;
      if (dxRouteSegmentIsCatchAll(routeSegment)) return currentIndex < currentSegments.length;
      if (currentIndex >= currentSegments.length) return false;
      if (!dxRouteSegmentIsDynamic(routeSegment) && routeSegment !== currentSegments[currentIndex]) return false;
      currentIndex += 1;
    }}
    return currentIndex === currentSegments.length;
  }};
  const dxRouteResourceMatchesCurrentPage = (payload) => {{
    const routePath = dxRoutePathFromResource(payload);
    if (!routePath || routePath === "/") return true;
    return dxRoutePatternMatchesPath(routePath, location.pathname || "/");
  }};
  const dxCleanStaticResourcePath = (value, {{ stripPublic = false }} = {{}}) => {{
    const clean = (value || "").replaceAll("\\\\", "/").replace(/^\/+/, "").split(/[?#]/, 1)[0];
    return stripPublic ? clean.replace(/^public\//, "") : clean;
  }};
  const dxStylePathFromResource = (resource) => {{
    const id = resource?.id || "";
    return id.startsWith("style:") ? dxCleanStaticResourcePath(id.slice("style:".length)) : "";
  }};
  const dxAssetPathFromResource = (resource) => {{
    const id = resource?.id || "";
    return id.startsWith("asset:") ? dxCleanStaticResourcePath(id.slice("asset:".length), {{ stripPublic: true }}) : "";
  }};
  const dxCacheBustUrl = (value, reloadToken) => {{
    const url = new URL(value, location.href);
    url.searchParams.set("_dx_reload", reloadToken);
    return url.href;
  }};
  const dxPathMatchesAsset = (value, assetPath) => {{
    if (!value || !assetPath) return false;
    try {{
      const url = new URL(value, location.href);
      const pathname = url.pathname.replace(/^\/+/, "");
      const normalizedAssetPath = assetPath.replace(/^\/+/, "");
      const publicAssetPath = `public/${{normalizedAssetPath}}`;
      return pathname === normalizedAssetPath || pathname === publicAssetPath;
    }} catch (_) {{
      return false;
    }}
  }};
  const dxPathMatchesStyle = (value, stylePath) => {{
    if (!value || !stylePath) return false;
    try {{
      const url = new URL(value, location.href);
      const pathname = url.pathname.replace(/^\/+/, "");
      const normalizedStylePath = stylePath.replace(/^\/+/, "");
      return pathname === normalizedStylePath;
    }} catch (_) {{
      return false;
    }}
  }};
  const dxRefreshStylesheets = (payload) => {{
    const links = Array.from(document.querySelectorAll('link[rel~="stylesheet"][href]'));
    if (!links.length) return false;
    const stylePath = dxStylePathFromResource(dxPayloadResource(payload));
    const matchingLinks = stylePath ? links.filter((link) => {{
      return dxPathMatchesStyle(link.getAttribute("href"), stylePath);
    }}) : [];
    const refreshLinks = stylePath ? matchingLinks : links;
    if (!refreshLinks.length) return false;
    const reloadToken = payload?.token || String(payload?.version || Date.now());
    for (const link of refreshLinks) {{
      link.setAttribute("href", dxCacheBustUrl(link.getAttribute("href"), reloadToken));
    }}
    return refreshLinks.length > 0;
  }};
  const dxRefreshAssetReferences = (payload) => {{
    const assetPath = dxAssetPathFromResource(dxPayloadResource(payload));
    if (!assetPath) return false;
    const reloadToken = payload?.token || String(payload?.version || Date.now());
    const elements = Array.from(document.querySelectorAll('img[src], source[srcset], video[poster], link[rel~="icon"][href], link[rel~="manifest"][href]'));
    let refreshed = 0;
    for (const element of elements) {{
      for (const attribute of ["src", "href", "poster"]) {{
        const value = element.getAttribute(attribute);
        if (dxPathMatchesAsset(value, assetPath)) {{
          element.setAttribute(attribute, dxCacheBustUrl(value, reloadToken));
          refreshed += 1;
        }}
      }}
      const srcset = element.getAttribute("srcset");
      if (srcset && srcset.split(",").some((entry) => dxPathMatchesAsset(entry.trim().split(/\s+/)[0], assetPath))) {{
        const nextSrcset = srcset.split(",").map((entry) => {{
          const parts = entry.trim().split(/\s+/);
          if (!dxPathMatchesAsset(parts[0], assetPath)) return entry.trim();
          return [dxCacheBustUrl(parts[0], reloadToken), ...parts.slice(1)].join(" ");
        }}).join(", ");
        element.setAttribute("srcset", nextSrcset);
        refreshed += 1;
      }}
    }}
    return refreshed > 0;
  }};
  const dxStylesheetMatchesPayload = (payload) => {{
    const stylePath = dxStylePathFromResource(dxPayloadResource(payload));
    if (!stylePath) return false;
    return Array.from(document.querySelectorAll('link[rel~="stylesheet"][href]')).some((link) => {{
      return dxPathMatchesStyle(link.getAttribute("href"), stylePath);
    }});
  }};
  const dxAssetReferenceMatchesPayload = (payload) => {{
    const assetPath = dxAssetPathFromResource(dxPayloadResource(payload));
    if (!assetPath) return false;
    const elements = Array.from(document.querySelectorAll('img[src], source[srcset], video[poster], link[rel~="icon"][href], link[rel~="manifest"][href]'));
    for (const element of elements) {{
      for (const attribute of ["src", "href", "poster"]) {{
        if (dxPathMatchesAsset(element.getAttribute(attribute), assetPath)) return true;
      }}
      const srcset = element.getAttribute("srcset");
      if (srcset && srcset.split(",").some((entry) => dxPathMatchesAsset(entry.trim().split(/\s+/)[0], assetPath))) {{
        return true;
      }}
    }}
    return false;
  }};
  const dxIssueResourceMatchesCurrentPage = (payload) => {{
    const resourceId = dxResourceId(payload);
    if (resourceId.startsWith("style:")) return dxStylesheetMatchesPayload(payload);
    if (resourceId.startsWith("asset:")) return dxAssetReferenceMatchesPayload(payload);
    return dxRouteResourceMatchesCurrentPage(payload);
  }};
  const ensureDxOverlay = () => {{
    let overlay = document.querySelector("[data-dx-www-error-overlay]");
    if (overlay) return overlay;
    overlay = document.createElement("section");
    overlay.setAttribute("data-dx-www-error-overlay", "true");
    overlay.style.cssText = "position:fixed;inset:0;z-index:2147483647;display:none;align-items:flex-start;justify-content:center;background:color-mix(in srgb, Canvas 72%, CanvasText 28%);padding:32px;font-family:ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,Segoe UI,sans-serif;color:CanvasText;";
    overlay.innerHTML = '<div style="width:min(880px,100%);border:1px solid Mark;background:Canvas;color:CanvasText;border-radius:8px;box-shadow:0 24px 80px color-mix(in srgb, CanvasText 28%, transparent);overflow:hidden"><div style="display:flex;align-items:center;justify-content:space-between;background:Mark;color:MarkText;padding:12px 16px"><strong>DX-WWW error overlay</strong><button type="button" data-dx-www-error-close style="background:transparent;border:0;color:MarkText;font-size:20px;cursor:pointer">x</button></div><div style="padding:16px"><p data-dx-www-error-message style="margin:0 0 12px;color:CanvasText;line-height:1.5"></p><pre data-dx-www-error-frame style="display:none;margin:0 0 12px;padding:12px;background:Field;border-radius:6px;overflow:auto;font:13px/1.5 ui-monospace,SFMono-Regular,Consolas,monospace;color:FieldText"></pre><p style="margin:0;color:GrayText">Fix the file and save. DX-WWW will reload after the next successful change.</p></div></div>';
    document.body.appendChild(overlay);
    overlay.querySelector("[data-dx-www-error-close]")?.addEventListener("click", () => {{
      overlay.style.display = "none";
    }});
    return overlay;
  }};
  window.__DX_SHOW_ERROR__ = (payload) => {{
    const overlay = ensureDxOverlay();
    const message = typeof payload === "string" ? payload : (payload?.message || "Unknown DX-WWW error");
    const frame = typeof payload === "object" ? payload?.codeFrame : "";
    const source = typeof payload === "object" && payload?.transportIssue === true ? "hot-reload-transport" : "runtime-or-diagnostic";
    overlay.setAttribute("data-dx-www-error-source", source);
    overlay.querySelector("[data-dx-www-error-message]").textContent = message;
    const frameNode = overlay.querySelector("[data-dx-www-error-frame]");
    if (frame) {{
      frameNode.textContent = frame;
      frameNode.style.display = "block";
    }} else {{
      frameNode.textContent = "";
      frameNode.style.display = "none";
    }}
    overlay.style.display = "flex";
  }};
  window.__DX_HIDE_ERROR__ = () => {{
    const overlay = document.querySelector("[data-dx-www-error-overlay]");
    if (overlay) overlay.style.display = "none";
  }};
  window.addEventListener("error", (event) => {{
    window.__DX_SHOW_ERROR__({{
      message: event.message || "Runtime error",
      codeFrame: event.filename ? `${{event.filename}}:${{event.lineno || 0}}:${{event.colno || 0}}` : ""
    }});
  }});
  window.addEventListener("unhandledrejection", (event) => {{
    const reason = event.reason;
    window.__DX_SHOW_ERROR__({{
      message: reason?.message || String(reason || "Unhandled promise rejection"),
      codeFrame: reason?.stack || ""
    }});
  }});
  const dxIssueOverlayPayload = (data) => {{
    const issues = Array.isArray(data?.issues) ? data.issues : (Array.isArray(data?.issue_receipt?.issues) ? data.issue_receipt.issues : []);
    const issue = issues[0] || {{}};
    const file = issue.file || "";
    const line = issue.line ? String(issue.line) : "";
    const column = issue.column ? String(issue.column) : "";
    const location = [file, line, column].filter(Boolean).join(":");
    return {{
      message: issue.message || data?.message || "DX-WWW issue",
      codeFrame: issue.code_frame || issue.codeFrame || data?.codeFrame || location
    }};
  }};
  const dxReportHotReloadTransportIssue = (message, error) => {{
    dxEventStreamHealthy = false;
    dxTransportIssueVisible = true;
    window.__DX_SHOW_ERROR__({{
      message,
      transportIssue: true,
      codeFrame: `The SSE payload could not be parsed. ${{error?.message || "Unknown parse error"}}\nDX-WWW will use polling fallback until the stream recovers.`
    }});
  }};
  const dxClearHotReloadTransportIssue = () => {{
    if (!dxTransportIssueVisible) return;
    const overlay = document.querySelector("[data-dx-www-error-overlay]");
    dxTransportIssueVisible = false;
    if (overlay?.getAttribute("data-dx-www-error-source") === "hot-reload-transport") {{
      window.__DX_HIDE_ERROR__();
    }}
  }};
  const dxHandleHotReloadPayload = (data) => {{
    dxClearHotReloadTransportIssue();
    const instructionType = data?.instruction?.type || "restart";
    const initialDiagnosticReplay = data?.event_stream?.initial === true && (instructionType === "report-issue" || instructionType === "clear-issue");
    if (data?.event_stream?.initial === true && !initialDiagnosticReplay) {{
      dxReloadToken = data.token || dxReloadToken;
      return;
    }}
    if (instructionType === "clear-issue" && !dxIssueResourceMatchesCurrentPage(data)) {{
      dxReloadToken = data.token || dxReloadToken;
      return;
    }}
    if (instructionType === "clear-issue") {{
      window.__DX_HIDE_ERROR__();
      dxReloadToken = data.token || dxReloadToken;
      return;
    }}
    if (instructionType === "report-issue" && !dxIssueResourceMatchesCurrentPage(data)) {{
      dxReloadToken = data.token || dxReloadToken;
      return;
    }}
    if (instructionType === "report-issue") {{
      window.__DX_SHOW_ERROR__(dxIssueOverlayPayload(data));
      dxReloadToken = data.token || dxReloadToken;
      return;
    }}
    if (data.token && data.token !== dxReloadToken && instructionType !== "disabled") {{
      if (instructionType === "restart" && !dxRouteResourceMatchesCurrentPage(data)) {{
        dxReloadToken = data.token;
        return;
      }}
      if (instructionType === "refresh-style") {{
        dxRefreshStylesheets(data);
        dxReloadToken = data.token;
        return;
      }}
      if (instructionType === "refresh-asset") {{
        dxRefreshAssetReferences(data);
        dxReloadToken = data.token;
        return;
      }}
      window.__DX_HIDE_ERROR__();
      location.reload();
    }}
  }};
  const dxCloseEventStream = () => {{
    if (dxEventSource) {{
      dxEventSource.close();
      dxEventSource = null;
    }}
    dxSubscribedResource = "";
    dxEventStreamHealthy = false;
  }};
  const dxStartEventStream = () => {{
    if (dxBrowserIsOffline()) {{
      dxCloseEventStream();
      return false;
    }}
    if (!eventStreamSupported || !("EventSource" in window)) return false;
    const nextResource = dxReloadResource();
    if (dxEventSource && dxEventStreamHealthy && dxSubscribedResource === nextResource) return true;
    dxCloseEventStream();
    try {{
      const eventSource = new EventSource(eventStreamEndpoint + "?resource=" + encodeURIComponent(nextResource));
      dxEventSource = eventSource;
      dxSubscribedResource = nextResource;
      eventSource.onopen = () => {{
        dxEventStreamHealthy = true;
      }};
      eventSource.onerror = () => {{
        dxEventStreamHealthy = false;
      }};
      eventSource.addEventListener(eventStreamEvent, (event) => {{
        try {{
          dxHandleHotReloadPayload(JSON.parse(event.data));
        }} catch (error) {{
          dxReportHotReloadTransportIssue("DX-WWW hot reload received an invalid event stream payload.", error);
        }}
      }});
      return true;
    }} catch (_) {{
      dxSubscribedResource = "";
      return false;
    }}
  }};
  if (eventStreamSupported) dxStartEventStream();
  const dxRestartEventStreamForCurrentResource = () => {{
    const nextResource = dxReloadResource();
    if (eventStreamSupported && (!dxEventSource || !dxEventStreamHealthy || dxSubscribedResource !== nextResource)) dxStartEventStream();
  }};
  const dxPatchHistory = (method) => {{
    const original = history[method];
    if (typeof original !== "function") return;
    history[method] = function (...args) {{
      const result = original.apply(this, args);
      queueMicrotask(dxRestartEventStreamForCurrentResource);
      return result;
    }};
  }};
  dxPatchHistory("pushState");
  dxPatchHistory("replaceState");
  window.addEventListener("popstate", dxRestartEventStreamForCurrentResource);
  window.addEventListener("pagehide", dxCloseEventStream);
  window.addEventListener("pageshow", dxRestartEventStreamForCurrentResource);
  window.addEventListener("offline", dxCloseEventStream);
  window.addEventListener("online", dxStartEventStream);
  document.addEventListener("visibilitychange", () => {{
    if (!document.hidden) dxRestartEventStreamForCurrentResource();
  }});
  const poll = async () => {{
    if (dxBrowserIsOffline()) return;
    try {{
      const res = await fetch(versionEndpoint + "?token=" + encodeURIComponent(dxReloadToken) + "&resource=" + encodeURIComponent(dxReloadResource()), {{
        cache: "no-store",
        headers: {{ "Accept": "application/json" }}
      }});
      if (!res.ok) return;
      const data = await res.json();
      dxHandleHotReloadPayload(data);
      if (eventStreamSupported) dxRestartEventStreamForCurrentResource();
    }} catch (_) {{}}
  }};
  setInterval(poll, 500);
}})();
</script>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_script_scopes_polling_and_ignores_disabled_instructions() {
        let script = dev_hot_reload_client_script("1-token", true);

        assert!(script.contains(DX_HOT_RELOAD_VERSION_ENDPOINT));
        assert!(script.contains(DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT));
        assert!(script.contains(DX_HOT_RELOAD_EVENT_NAME));
        assert!(script.contains("const eventStreamSupported = true"));
        assert!(script.contains(DX_HOT_RELOAD_RESOURCE_MARKER));
        assert!(script.contains("if (window.__DX_HOT_RELOAD_CLIENT_ACTIVE__) return;"));
        assert!(script.contains("window.__DX_HOT_RELOAD_CLIENT_ACTIVE__ = true;"));
        assert!(script.contains("encodeURIComponent(dxReloadResource())"));
        assert!(script.contains("new EventSource"));
        assert!(script.contains("eventSource.addEventListener(eventStreamEvent"));
        assert!(script.contains("let dxEventSource = null"));
        assert!(script.contains("let dxSubscribedResource = \"\""));
        assert!(script.contains("let dxEventStreamHealthy = false"));
        assert!(script.contains(
            "dxEventSource && dxEventStreamHealthy && dxSubscribedResource === nextResource"
        ));
        assert!(script.contains("const dxBrowserIsOffline = () =>"));
        assert!(script.contains("navigator.onLine === false"));
        assert!(script.contains("dxCloseEventStream"));
        assert!(script.contains("dxSubscribedResource = \"\""));
        assert!(script.contains("eventSource.onopen = () => {"));
        assert!(script.contains("eventSource.onerror = () => {"));
        assert!(script.contains("dxRestartEventStreamForCurrentResource"));
        assert!(script.contains("const nextResource = dxReloadResource()"));
        assert!(script.contains(
            "!dxEventSource || !dxEventStreamHealthy || dxSubscribedResource !== nextResource"
        ));
        assert!(!script.contains("dxSubscribedResource !== dxReloadResource()"));
        assert!(script.contains("dxPatchHistory(\"pushState\")"));
        assert!(script.contains("dxPatchHistory(\"replaceState\")"));
        assert!(script.contains(
            "window.addEventListener(\"popstate\", dxRestartEventStreamForCurrentResource)"
        ));
        assert!(script.contains("window.addEventListener(\"pagehide\", dxCloseEventStream)"));
        assert!(script.contains(
            "window.addEventListener(\"pageshow\", dxRestartEventStreamForCurrentResource)"
        ));
        assert!(script.contains("window.addEventListener(\"offline\", dxCloseEventStream)"));
        assert!(script.contains("window.addEventListener(\"online\", dxStartEventStream)"));
        assert!(script.contains("document.addEventListener(\"visibilitychange\""));
        assert!(script.contains("if (!document.hidden) dxRestartEventStreamForCurrentResource()"));
        assert!(
            script.contains("const initialDiagnosticReplay = data?.event_stream?.initial === true")
        );
        assert!(script.contains(
            "instructionType === \"report-issue\" || instructionType === \"clear-issue\""
        ));
        assert!(
            script.contains("data?.event_stream?.initial === true && !initialDiagnosticReplay")
        );
        assert!(script.contains("dxReloadToken = data.token || dxReloadToken"));
        assert!(script.contains("dxHandleHotReloadPayload"));
        assert!(script.contains("dxIssueOverlayPayload"));
        assert!(script.contains("dxReportHotReloadTransportIssue"));
        assert!(script.contains("let dxTransportIssueVisible = false"));
        assert!(script.contains("dxTransportIssueVisible = true"));
        assert!(script.contains("transportIssue: true"));
        assert!(script.contains("data-dx-www-error-source"));
        assert!(script.contains("\"hot-reload-transport\""));
        assert!(script.contains("\"runtime-or-diagnostic\""));
        assert!(script.contains("const dxClearHotReloadTransportIssue = () =>"));
        assert!(script.contains("dxClearHotReloadTransportIssue();"));
        assert!(script.contains("DX-WWW hot reload received an invalid event stream payload."));
        assert!(script.contains("DX-WWW will use polling fallback until the stream recovers."));
        assert!(script.contains("dxPayloadResource"));
        assert!(script.contains("dxIssueResourceMatchesCurrentPage"));
        assert!(script.contains("dxStylesheetMatchesPayload"));
        assert!(script.contains("dxAssetReferenceMatchesPayload"));
        assert!(script.contains("instructionType === \"clear-issue\""));
        assert!(script.contains(
            "instructionType === \"clear-issue\" && !dxIssueResourceMatchesCurrentPage(data)"
        ));
        assert!(script.contains("instructionType === \"report-issue\""));
        assert!(script.contains(
            "instructionType === \"report-issue\" && !dxIssueResourceMatchesCurrentPage(data)"
        ));
        assert!(script.contains("window.__DX_SHOW_ERROR__(dxIssueOverlayPayload(data))"));
        assert!(script.contains("if (dxBrowserIsOffline()) return"));
        assert!(script.contains("dxRestartEventStreamForCurrentResource();"));
        assert!(script.contains("dxRefreshStylesheets"));
        assert!(script.contains("dxCleanStaticResourcePath"));
        assert!(script.contains(".split(/[?#]/, 1)[0]"));
        assert!(script.contains("dxPathMatchesStyle"));
        assert!(script.contains("const normalizedStylePath = stylePath.replace(/^\\/+/, \"\")"));
        assert!(script.contains("return pathname === normalizedStylePath"));
        assert!(script.contains("const refreshLinks = stylePath ? matchingLinks : links"));
        assert!(script.contains("if (!refreshLinks.length) return false"));
        assert!(script.contains("dxRefreshAssetReferences"));
        assert!(script.contains("const normalizedAssetPath = assetPath.replace(/^\\/+/, \"\")"));
        assert!(script.contains("const publicAssetPath = `public/${normalizedAssetPath}`"));
        assert!(
            script.contains("pathname === normalizedAssetPath || pathname === publicAssetPath")
        );
        assert!(script.contains("dxRouteResourceMatchesCurrentPage"));
        assert!(script.contains("const dxRoutePatternMatchesPath = (routePath, currentPath) =>"));
        assert!(script.contains("const dxRouteSegmentIsDynamic = (segment) =>"));
        assert!(script.contains("const dxRouteSegmentIsCatchAll = (segment) =>"));
        assert!(script.contains("const dxRouteSegmentIsOptionalCatchAll = (segment) =>"));
        assert!(script.contains("return currentIndex === currentSegments.length"));
        assert!(
            script.contains("dxRoutePatternMatchesPath(routePath, location.pathname || \"/\")")
        );
        assert!(script.contains("_dx_reload"));
        assert!(script.contains("instructionType === \"refresh-style\""));
        assert!(script.contains("instructionType === \"refresh-asset\""));
        assert!(
            script
                .contains("instructionType === \"restart\" && !dxRouteResourceMatchesCurrentPage")
        );
        assert!(script.contains("instructionType !== \"disabled\""));
        assert!(script.contains("data.token !== dxReloadToken"));
    }

    #[test]
    fn client_script_can_disable_event_stream_for_tcp_fallback() {
        let script = dev_hot_reload_client_script("1-token", false);

        assert!(script.contains("const eventStreamSupported = false"));
        assert!(script.contains("if (eventStreamSupported) dxStartEventStream()"));
        assert!(
            script.contains(
                "if (!eventStreamSupported || !(\"EventSource\" in window)) return false"
            )
        );
    }
}
