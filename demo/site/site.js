(function () {
  "use strict";

  const maxPayloadKb = 200;
  const scenarios = {
    tiny: {
      title: "Tiny route compiler plan",
      runtime: "Micro JS",
      raw: 1.895,
      gzip: 0.926,
      brotli: 0.668,
      latency: 4.407,
      route: "/fair-counter",
      plan: [
        'route "/fair-counter"',
        "mode static_html + micro_js",
        "style default: generated_css",
        "binary sidecar: optional",
        "requests: 1",
        "ship before interaction",
      ],
    },
    content: {
      title: "Content route compiler plan",
      runtime: "Static shell",
      raw: 8.6,
      gzip: 3.4,
      brotli: 2.5,
      latency: 5.8,
      route: "/",
      plan: [
        'route "/"',
        "mode static_html + route_local_js",
        "style default: generated_css",
        "serializer packet: skipped",
        "wasm: not loaded",
        "cache policy: public shell",
      ],
    },
    dashboard: {
      title: "Dashboard route compiler plan",
      runtime: "Binary packet",
      raw: 24.5,
      gzip: 8.2,
      brotli: 6.3,
      latency: 7.4,
      route: "/demo/dashboard",
      plan: [
        'route "/demo/dashboard"',
        "mode streamed_html + binary_state",
        "serializer: dx-serializer packets",
        "wasm: after useful shell",
        "style default: generated_css",
        "patch path: route-owned updates",
      ],
    },
  };

  const metricNodes = {
    raw: document.querySelector('[data-metric="raw"]'),
    gzip: document.querySelector('[data-metric="gzip"]'),
    brotli: document.querySelector('[data-metric="brotli"]'),
    latency: document.querySelector('[data-metric="latency"]'),
  };

  const sizeNodes = {
    raw: document.querySelector('[data-size="raw"]'),
    gzip: document.querySelector('[data-size="gzip"]'),
    brotli: document.querySelector('[data-size="brotli"]'),
  };

  const bars = {
    raw: document.querySelector('[data-bar="raw"]'),
    gzip: document.querySelector('[data-bar="gzip"]'),
    brotli: document.querySelector('[data-bar="brotli"]'),
  };

  const routeTitle = document.querySelector("[data-route-title]");
  const runtimeBadge = document.querySelector("[data-runtime]");
  const planNode = document.querySelector("[data-plan]");
  const routeStatus = document.querySelector("#route-status");
  const scenarioButtons = Array.from(document.querySelectorAll("[data-scenario]"));

  function formatKb(value) {
    const precision = value < 10 ? 2 : 1;
    return `${value.toFixed(precision)} KB`;
  }

  function formatBytes(bytes) {
    if (bytes < 1024) {
      return `${bytes} B`;
    }

    return `${(bytes / 1024).toFixed(2)} KB`;
  }

  function setBar(name, value) {
    const bar = bars[name];
    const sizeNode = sizeNodes[name];

    if (!bar || !sizeNode) {
      return;
    }

    const percent = Math.max(1, Math.min(100, (value / maxPayloadKb) * 100));
    const fill = bar.querySelector("span");
    bar.setAttribute("aria-valuenow", value.toFixed(2));

    if (fill) {
      fill.style.width = `${percent.toFixed(1)}%`;
    }

    sizeNode.textContent = formatKb(value);
  }

  function setText(node, value) {
    if (node) {
      node.textContent = value;
    }
  }

  function selectScenario(key) {
    const scenario = scenarios[key] || scenarios.tiny;
    const activeClasses = ["bg-zinc-50", "text-zinc-950", "border-zinc-50"];
    const inactiveClasses = ["bg-zinc-950", "text-zinc-300", "border-zinc-800"];

    scenarioButtons.forEach((button) => {
      const isActive = button.dataset.scenario === key;
      button.classList.remove(...activeClasses, ...inactiveClasses);
      button.classList.add(...(isActive ? activeClasses : inactiveClasses));
      button.setAttribute("aria-selected", String(isActive));
    });

    setText(metricNodes.raw, formatKb(scenario.raw));
    setText(metricNodes.gzip, formatKb(scenario.gzip));
    setText(metricNodes.brotli, formatKb(scenario.brotli));
    setText(metricNodes.latency, `${scenario.latency.toFixed(2)} ms`);

    setBar("raw", scenario.raw);
    setBar("gzip", scenario.gzip);
    setBar("brotli", scenario.brotli);

    setText(routeTitle, scenario.title);
    setText(runtimeBadge, scenario.runtime);
    setText(planNode, scenario.plan.join("\n"));
  }

  scenarioButtons.forEach((button) => {
    button.addEventListener("click", () => selectScenario(button.dataset.scenario));
  });

  async function loadStyleManifest() {
    const statusNode = document.querySelector("#style-status");
    const tokenNode = document.querySelector("#theme-token-count");
    const recipeNode = document.querySelector("#recipe-count");
    const cssNode = document.querySelector("#style-css-size");
    const packetNode = document.querySelector("#style-packet-size");

    try {
      const manifestResponse = await fetch("/dx-style.json", { cache: "no-store" });

      if (!manifestResponse.ok) {
        throw new Error("style manifest failed");
      }

      const manifest = await manifestResponse.json();
      const defaultStylesheet = manifest.defaultStylesheet || manifest.fallbackCss || "/site.css";
      const sidecarPath =
        typeof manifest.binarySidecar === "string"
          ? manifest.binarySidecar
          : manifest.binarySidecar && manifest.binarySidecar.path;

      const [cssResponse, packetResponse] = await Promise.all([
        fetch(defaultStylesheet, { cache: "no-store" }),
        sidecarPath ? fetch(sidecarPath, { cache: "no-store" }) : Promise.resolve(null),
      ]);

      if (!cssResponse.ok || (packetResponse && !packetResponse.ok)) {
        throw new Error("style endpoint failed");
      }

      const css = await cssResponse.text();
      const packet = packetResponse ? await packetResponse.arrayBuffer() : null;

      setText(tokenNode, String(Object.keys(manifest.tokens || {}).length));
      setText(recipeNode, String((manifest.recipes || []).length));
      setText(cssNode, formatBytes(new TextEncoder().encode(css).byteLength));
      setText(packetNode, packet ? formatBytes(packet.byteLength) : "optional");
      setText(statusNode, manifest.defaultDelivery === "generated-css" ? "css default" : "ready");
    } catch (error) {
      setText(statusNode, "offline");
    }
  }

  async function loadBenchmarkReport() {
    const benchmarkNode = document.querySelector("#benchmark-updated");

    if (!benchmarkNode) {
      return;
    }

    try {
      const response = await fetch("/benchmarks/fair-counter.json", { cache: "no-store" });

      if (!response.ok) {
        throw new Error("benchmark report failed");
      }

      const report = await response.json();
      const generatedAt = report.generated_at
        ? new Date(report.generated_at).toLocaleString(undefined, {
            dateStyle: "medium",
            timeStyle: "short",
          })
        : "current local run";
      const ranking = Array.isArray(report.rankings && report.rankings.raw_bytes)
        ? report.rankings.raw_bytes.join(" -> ")
        : "DX-WWW -> Astro -> Svelte -> HTMX -> Next.js";

      benchmarkNode.textContent = `Measured ${generatedAt}. Payload ranking: ${ranking}.`;
    } catch (error) {
      benchmarkNode.textContent = "Benchmark report unavailable";
    }
  }

  async function checkRoute() {
    try {
      const response = await fetch("/fair-counter", {
        cache: "no-store",
        headers: { Accept: "text/html" },
      });
      setText(routeStatus, response.ok ? "tiny route online" : "tiny route failed");
    } catch (error) {
      setText(routeStatus, "tiny route offline");
    }
  }

  selectScenario("tiny");
  loadStyleManifest();
  loadBenchmarkReport();
  checkRoute();
})();
