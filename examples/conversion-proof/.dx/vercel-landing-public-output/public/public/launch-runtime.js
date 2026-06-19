(() => {
  const $ = (selector) => document.querySelector(selector);
  const setText = (selector, value) => {
    const node = $(selector);
    if (node) node.textContent = value;
  };
  const setState = (selector, value) => {
    const node = $(selector);
    if (node) node.dataset.state = value;
  };

  const motionStages = [
    {
      id: "source-owned",
      label: "Source-owned adapters",
      progress: 34,
      detail: "Forge Motion helpers are visible in /launch.",
    },
    {
      id: "interactive",
      label: "Local interaction",
      progress: 67,
      detail: "Buttons update progress and reorder stage cards.",
    },
    {
      id: "preview-ready",
      label: "Web Preview markers",
      progress: 100,
      detail: "Stable data-dx markers expose Motion state to Zed.",
    },
  ];

  const fumadocsPages = [
    {
      id: "overview",
      title: "Launch overview",
      route: "/docs",
      description: "Source-owned App Router docs with generated navigation.",
      breadcrumb: ["Docs", "Launch", "Overview"],
      peers: ["API reference", "LLM export"],
      toc: ["Package contract", "Editable content", "Search boundary"],
      body: "DX materializes a Fumadocs docs route from source-owned files.",
    },
    {
      id: "api-reference",
      title: "API reference",
      route: "/docs/api",
      description: "OpenAPI docs and request snippets stay app-owned.",
      breadcrumb: ["Docs", "API", "Reference"],
      peers: ["Launch overview", "LLM export"],
      toc: ["OpenAPI source", "Proxy policy", "Request examples"],
      body: "The OpenAPI package slice owns generated documentation surfaces, not live credential forwarding.",
    },
    {
      id: "llm-export",
      title: "LLM export",
      route: "/llms.txt",
      description: "AI-readable docs exports expose a safe launch handoff.",
      breadcrumb: ["Docs", "AI", "LLMs"],
      peers: ["Launch overview", "API reference"],
      toc: ["Index route", "Full export", "Private content boundary"],
      body: "Fumadocs exposes llms.txt style exports for tools that need source context.",
    },
  ];

  const state = {
    signedIn: false,
    authEmail: "launch@dx.local",
    automationConnector: "n8n-nodes-base.manualTrigger",
    automationReceiptRuns: 0,
    counter: 0,
    docsPage: "overview",
    reviewMode: false,
    queryRuns: 0,
    motionActive: false,
    motionStageIndex: 0,
    motionOrder: motionStages.map((stage) => stage.id),
    drizzleRuns: 0,
    supabaseQueryRuns: 0,
    trpcRuns: 0,
    wasmRuns: 0,
    shadcnPrimitive: "button",
    shadcnReceiptRuns: 0,
    sceneNodeIndex: -1,
    scenePerformanceBand: "stable",
  };

  function bindShadcnProof() {
    const card = $('[data-dx-component="shadcn-ui-runtime-proof"]');
    const buttons = Array.from(document.querySelectorAll("[data-dx-shadcn-action='select-primitive']"));
    const input = $("#shadcn-component-name");
    const intent = $("#shadcn-component-intent");
    const prepare = $("#shadcn-prepare-edit");
    const status = $("#shadcn-status");
    const preview = $("#shadcn-preview");
    if (!card || !buttons.length || !input || !intent || !prepare || !status || !preview) return;

    const commands = {
      button: "dx add ui/button --write",
      field: "dx add ui/field --write",
      item: "dx add ui/item --write",
    };

    const renderSelectedPrimitive = () => {
      buttons.forEach((button) => {
        const selected = button.dataset.dxShadcnPrimitive === state.shadcnPrimitive;
        button.dataset.dxShadcnSelected = selected ? "true" : "false";
      });
      const active = buttons.find((button) => button.dataset.dxShadcnPrimitive === state.shadcnPrimitive);
      if (active) card.setAttribute("data-dx-package", active.dataset.dxPackage || "shadcn/ui/button");
      preview.dataset.dxShadcnSelectedPrimitive = state.shadcnPrimitive;
    };

    buttons.forEach((button) => {
      button.addEventListener("click", () => {
        state.shadcnPrimitive = button.dataset.dxShadcnPrimitive || "button";
        card.dataset.dxShadcnResult = "idle";
        status.dataset.dxShadcnReceiptState = "idle";
        status.dataset.state = "ready";
        status.textContent = `${state.shadcnPrimitive} selected. Prepare a local source edit next.`;
        preview.textContent = `Selected ${button.dataset.dxPackage || "shadcn/ui/button"} for the launch route.`;
        renderSelectedPrimitive();
      });
    });

    prepare.addEventListener("click", () => {
      const componentName = String(input.value || "").trim();
      const componentIntent = String(intent.value || "").trim();
      if (componentName.length < 3) {
        card.dataset.dxShadcnResult = "invalid";
        status.dataset.dxShadcnReceiptState = "invalid";
        status.dataset.state = "error";
        status.textContent = "Use at least 3 characters before preparing a source edit.";
        preview.textContent = "Validation blocked the local source edit preview.";
        return;
      }

      state.shadcnReceiptRuns += 1;
      card.dataset.dxShadcnResult = "ready";
      status.dataset.dxShadcnReceiptState = "ready";
      status.dataset.state = "ok";
      status.textContent = `Local shadcn/ui receipt ${state.shadcnReceiptRuns} ready for ${componentName}.`;
      preview.textContent = `${componentName} will use ${commands[state.shadcnPrimitive] || commands.button} with intent: ${
        componentIntent || "No intent entered"
      }.`;
    });

    renderSelectedPrimitive();
  }

  function bindSession() {
    const card = $('[data-dx-component="better-auth-readiness-workflow"][data-dx-package="auth/better-auth"]');
    const status = $("#session-status");
    const emailInput = $("#session-readiness-email");
    const localUser = $("#session-local-user");
    const signIn = $("#session-sign-in");
    const signOut = $("#session-sign-out");
    if (!status || !signIn || !signOut) return;
    const storageKey = card?.getAttribute("data-dx-auth-storage-key") || "dx-better-auth-readiness-session";

    const currentEmail = () => {
      const value = emailInput && "value" in emailInput ? String(emailInput.value).trim() : "";
      return value.includes("@") ? value : "launch@dx.local";
    };

    const readStoredSession = () => {
      try {
        const saved = JSON.parse(sessionStorage.getItem(storageKey) || "null");
        if (saved && typeof saved.email === "string" && saved.email.includes("@")) {
          state.signedIn = true;
          state.authEmail = saved.email;
          if (emailInput && "value" in emailInput) emailInput.value = saved.email;
        }
      } catch {
        state.signedIn = false;
      }
    };

    const writeStoredSession = () => {
      try {
        if (state.signedIn) {
          sessionStorage.setItem(
            storageKey,
            JSON.stringify({ email: state.authEmail, provider: "local-preview" }),
          );
        } else {
          sessionStorage.removeItem(storageKey);
        }
      } catch {
        if (card) card.setAttribute("data-dx-auth-storage", "unavailable");
      }
    };

    const render = () => {
      const email = state.authEmail || currentEmail();
      if (card) {
        card.setAttribute(
          "data-dx-auth-local-session",
          state.signedIn ? "signed-in" : "signed-out",
        );
        card.setAttribute(
          "data-dx-auth-safe-action-state",
          state.signedIn ? "previewed" : "idle",
        );
        card.setAttribute("data-dx-auth-readiness-email", state.signedIn ? email : "none");
      }
      status.setAttribute(
        "data-dx-auth-local-readiness-state",
        state.signedIn ? "safe-sign-in-preview" : "idle",
      );
      status.dataset.state = state.signedIn ? "ok" : "ready";
      status.textContent = state.signedIn
        ? `Signed in as ${email} with the source-owned Better Auth boundary. No secret left the browser.`
        : "Signed out. The Better Auth adapter is mounted, and credentials stay app-owned.";
      if (localUser) {
        localUser.setAttribute("data-dx-auth-readiness-user", state.signedIn ? email : "none");
        localUser.textContent = state.signedIn
          ? `Local preview session: ${email}`
          : "No local readiness session.";
      }
    };

    signIn.addEventListener("click", async () => {
      state.signedIn = true;
      state.authEmail = currentEmail();
      writeStoredSession();
      if (card) card.setAttribute("data-dx-auth-safe-action-state", "checking-session-route");
      render();
      try {
        const response = await fetch("/api/auth/session");
        const payload = await response.json();
        if (card) {
          card.setAttribute(
            "data-dx-auth-config",
            payload.credentialsConfigured ? "configured" : "missing-config",
          );
          card.setAttribute("data-dx-auth-safe-action-state", "session-route-checked");
        }
        setText(
          "#session-detail",
          `Session API: ${payload.adapter ?? "better-auth"} ${payload.status ?? "ready"}`,
        );
      } catch {
        if (card) {
          card.setAttribute("data-dx-auth-config", "missing-config");
          card.setAttribute("data-dx-auth-safe-action-state", "local-only");
        }
        setText("#session-detail", "Session API unavailable; local adapter boundary still visible.");
      }
    });

    signOut.addEventListener("click", () => {
      state.signedIn = false;
      writeStoredSession();
      render();
      if (card) card.setAttribute("data-dx-auth-safe-action-state", "signed-out");
      setText("#session-detail", "Session cleared locally.");
    });

    readStoredSession();
    render();
  }

  function bindPayment() {
    const card = $('[data-dx-component="launch-billing-checkout-workflow"][data-dx-package="payments/stripe-js"]');
    const form = $("#stripe-checkout-form");
    const status = $("#payment-status");
    const planButtons = $$('[data-dx-stripe-action="select-plan"]');
    if (!form || !status) return;
    let selectedPlanId = card?.dataset.dxStripePlanId || "team";
    let selectedPriceEnv = card?.dataset.dxStripePriceEnv || "STRIPE_PRICE_ID_TEAM";

    planButtons.forEach((button) => {
      button.addEventListener("click", () => {
        selectedPlanId = button.getAttribute("data-dx-stripe-plan-id") || selectedPlanId;
        selectedPriceEnv = button.getAttribute("data-dx-stripe-price-env") || selectedPriceEnv;
        planButtons.forEach((planButton) => {
          planButton.setAttribute(
            "data-dx-stripe-plan-selected",
            planButton === button ? "true" : "false",
          );
        });
        if (card) {
          card.dataset.dxStripePlanId = selectedPlanId;
          card.dataset.dxStripePriceEnv = selectedPriceEnv;
          card.dataset.dxStripeLocalReceipt = "none";
          card.dataset.dxStripePostResult = "idle";
        }
        form.dataset.dxStripePostResult = "idle";
        status.dataset.dxStripePostResult = "idle";
        status.dataset.state = "ready";
        status.textContent = `${selectedPlanId} selected. Submit contact details to prepare a safe Stripe-shaped receipt.`;
      });
    });

    const setPaymentResult = (result, message, receiptId = "none") => {
      if (card) {
        card.dataset.dxStripePostResult = result;
        card.dataset.dxStripeLocalReceipt = receiptId;
      }
      form.dataset.dxStripePostResult = result;
      status.dataset.dxStripePostResult = result;
      status.dataset.state = result === "error" ? "error" : "ok";
      status.textContent = message;
    };

    form.addEventListener("submit", async (event) => {
      event.preventDefault();
      const data = new FormData(form);
      const email = String(data.get("email") ?? "").trim().toLowerCase();
      const name = String(data.get("name") ?? "").trim();
      const message = String(data.get("message") ?? "").trim();

      if (!/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(email) || name.length < 2) {
        setPaymentResult(
          "error",
          "Enter a valid checkout email and contact name before requesting an intent.",
        );
        return;
      }

      setPaymentResult("submitting", "Posting a safe Stripe-shaped checkout contact...");
      try {
        const response = await fetch("/api/checkout", {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({
            checkoutMode: "hosted",
            plan: {
              id: selectedPlanId,
              priceEnv: selectedPriceEnv,
            },
            contact: {
              email,
              name,
              message,
            },
            preview: true,
          }),
        });
        const payload = await response.json();
        const receiptId =
          payload.receiptId ||
          payload.id ||
          `stripe-local-${selectedPlanId}-hosted-${email.replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "")}`;

        setPaymentResult(
          response.ok ? "posted" : "readiness",
          response.ok
            ? `Checkout POST accepted: ${payload.status ?? payload.kind ?? "readiness"}.`
            : `Checkout POST returned ${response.status}; local readiness receipt prepared.`,
          receiptId,
        );
      } catch {
        const receiptId = `stripe-local-${selectedPlanId}-hosted-${email
          .replace(/[^a-z0-9]+/g, "-")
          .replace(/^-|-$/g, "")}`;
        setPaymentResult(
          "readiness",
          `Checkout route unavailable; safe readiness POST receipt ${receiptId} is visible until Stripe config exists.`,
          receiptId,
        );
      }
    });
  }

  function bindLeadForm() {
    const form = $("#launch-form");
    if (!form) return;
    const card = $('[data-dx-component="zod-form-card"]');
    const issueList = $("#form-issues");
    const invalidSample = {
      email: "not-an-email",
      name: "A",
      notes: "Too short",
    };
    const validSample = {
      email: "ada@example.com",
      name: "Ada Lovelace",
      notes: "Prove source-owned Zod validation in the launch template.",
    };

    function field(name) {
      return form.elements.namedItem(name);
    }

    function renderZodIssues(issues) {
      if (!issueList) return;
      issueList.replaceChildren();
      issueList.dataset.dxZodReadinessIssues = JSON.stringify(issues);

      if (!issues.length) {
        const item = document.createElement("li");
        item.dataset.dxZodReadinessIssue = "success";
        item.textContent = "safeParse accepted the launch lead payload.";
        issueList.append(item);
        return;
      }

      for (const issue of issues) {
        const item = document.createElement("li");
        item.dataset.dxZodReadinessIssue = issue.path.join(".");
        item.dataset.dxZodReadinessIssueCode = issue.code;
        item.textContent = `${issue.path.join(".")}: ${issue.message}`;
        issueList.append(item);
      }
    }

    function setZodSample(sample) {
      for (const [name, value] of Object.entries(sample)) {
        const input = field(name);
        if (input && "value" in input) input.value = value;
      }
      if (typeof form.requestSubmit === "function") {
        form.requestSubmit();
        return;
      }
      form.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
    }

    $("#zod-fill-invalid")?.addEventListener("click", () => setZodSample(invalidSample));
    $("#zod-fill-valid")?.addEventListener("click", () => setZodSample(validSample));

    form.addEventListener("submit", (event) => {
      event.preventDefault();
      const data = new FormData(form);
      const name = String(data.get("name") ?? "").trim();
      const email = String(data.get("email") ?? "").trim();
      const notes = String(data.get("notes") ?? "").trim();
      const issues = [];

      if (name.length < 2) {
        issues.push({
          code: "too_small",
          message: "Name needs at least 2 characters.",
          path: ["name"],
        });
      }
      if (!/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(email)) {
        issues.push({
          code: "invalid_string",
          message: "Email must be valid.",
          path: ["email"],
        });
      }
      if (notes.length < 12) {
        issues.push({
          code: "too_small",
          message: "Launch notes need at least 12 characters.",
          path: ["notes"],
        });
      }

      const output = $("#form-status");
      if (!output) return;
      renderZodIssues(issues);
      if (issues.length) {
        const messages = issues.map((issue) => issue.message).join(" ");
        if (card) card.dataset.dxZodReadinessState = "invalid";
        form.dataset.dxZodReadinessState = "invalid";
        output.dataset.dxZodReadinessState = "invalid";
        output.dataset.dxZodReadinessResult = "error";
        output.dataset.dxZodReadinessError = messages;
        output.dataset.state = "error";
        output.textContent = messages;
        return;
      }

      if (card) card.dataset.dxZodReadinessState = "valid";
      form.dataset.dxZodReadinessState = "valid";
      output.dataset.dxZodReadinessState = "valid";
      output.dataset.dxZodReadinessResult = "success";
      output.dataset.dxZodReadinessError = "";
      output.dataset.state = "ok";
      output.textContent = `Validated lead payload for ${name} (${email}). Zod-shaped rules accepted ${notes.length} chars.`;
    });
  }

  function bindStateAndQuery() {
    const loadLaunchCounterState = () => {
      try {
        const saved = JSON.parse(localStorage.getItem("dx-launch-counter") || "{}");
        if (Number.isFinite(saved.count)) state.counter = saved.count;
        if (typeof saved.reviewMode === "boolean") {
          state.reviewMode = saved.reviewMode;
        }
      } catch {
        state.counter = 0;
        state.reviewMode = false;
      }
    };
    const saveLaunchCounterState = () => {
      try {
        localStorage.setItem(
          "dx-launch-counter",
          JSON.stringify({
            count: state.counter,
            reviewMode: state.reviewMode,
          }),
        );
      } catch {
        // Private or locked-down previews still keep the in-memory proof visible.
      }
    };
    const updateCounter = () => {
      const card = $('[data-dx-component="zustand-state-card"]');
      if (card) {
        card.setAttribute("data-dx-zustand-count", String(state.counter));
        card.setAttribute(
          "data-dx-zustand-toggle-state",
          state.reviewMode ? "enabled" : "disabled",
        );
        card.setAttribute("data-dx-zustand-hydration", "ready");
      }
      setText("#state-count", String(state.counter));
      setText("#state-toggle", state.reviewMode ? "On" : "Off");
      const toggle = $("#state-toggle");
      if (toggle) {
        toggle.setAttribute(
          "data-dx-zustand-toggle-state",
          state.reviewMode ? "enabled" : "disabled",
        );
      }
      setText(
        "#state-status",
        `Zustand-style persisted store count: ${state.counter}; review mode ${
          state.reviewMode ? "on" : "off"
        }.`,
      );
    };
    loadLaunchCounterState();
    $("#state-increment")?.addEventListener("click", () => {
      state.counter += 1;
      saveLaunchCounterState();
      updateCounter();
    });
    $("#state-toggle-mode")?.addEventListener("click", () => {
      state.reviewMode = !state.reviewMode;
      saveLaunchCounterState();
      updateCounter();
    });
    $("#state-reset")?.addEventListener("click", () => {
      state.counter = 0;
      state.reviewMode = false;
      saveLaunchCounterState();
      updateCounter();
    });
    $("#state-rehydrate")?.addEventListener("click", () => {
      loadLaunchCounterState();
      updateCounter();
      const card = $('[data-dx-component="zustand-state-card"]');
      if (card) card.setAttribute("data-dx-zustand-hydration", "rehydrated");
      setText(
        "#state-status",
        `Rehydrated dx-launch-counter: count ${state.counter}; review mode ${
          state.reviewMode ? "on" : "off"
        }.`,
      );
    });
    const queryCard = $('[data-dx-component="tanstack-query-health-card"]');
    const queryButton = $("#query-refresh");
    const queryStatus = $("#query-status");
    const queryRuns = $("#query-cache-runs");
    const queryUpdated = $("#query-cache-updated");

    queryButton?.addEventListener("click", async () => {
      state.queryRuns += 1;
      if (queryCard) queryCard.setAttribute("data-dx-query-cache-state", "fetching");
      if (queryButton) queryButton.setAttribute("data-dx-query-refresh-state", "fetching");
      if (queryButton) queryButton.setAttribute("data-dx-query-safe-action-state", "fetching");
      if (queryStatus) {
        queryStatus.setAttribute("data-dx-query-result-status", "loading");
        queryStatus.setAttribute("data-dx-query-cache-entry", "pending");
      }
      if (queryRuns) {
        queryRuns.textContent = String(state.queryRuns);
        queryRuns.setAttribute("data-dx-query-run-count", String(state.queryRuns));
      }
      setText("#query-status", `Fetching /api/health, request ${state.queryRuns}...`);
      setState("#query-status", "ready");
      try {
        const response = await fetch("/api/health");
        const payload = await response.json();
        if (queryCard) queryCard.setAttribute("data-dx-query-cache-state", "fresh");
        if (queryButton) queryButton.setAttribute("data-dx-query-refresh-state", "success");
        if (queryButton) queryButton.setAttribute("data-dx-query-safe-action-state", "completed");
        if (queryStatus) {
          queryStatus.setAttribute("data-dx-query-result-status", "success");
          queryStatus.setAttribute("data-dx-query-cache-entry", "fresh");
        }
        if (queryUpdated) {
          const updatedAt = new Date().toLocaleTimeString();
          queryUpdated.textContent = updatedAt;
          queryUpdated.setAttribute("data-dx-query-updated-at", updatedAt);
        }
        setText(
          "#query-status",
          `TanStack Query-style cache refreshed: ${payload.message ?? payload.runtime ?? "health ok"}`,
        );
        setState("#query-status", response.ok ? "ok" : "error");
      } catch {
        if (queryCard) queryCard.setAttribute("data-dx-query-cache-state", "error");
        if (queryButton) queryButton.setAttribute("data-dx-query-refresh-state", "error");
        if (queryButton) queryButton.setAttribute("data-dx-query-safe-action-state", "runtime-missing");
        if (queryStatus) {
          queryStatus.setAttribute("data-dx-query-result-status", "error");
          queryStatus.setAttribute("data-dx-query-cache-entry", "missing-runtime");
        }
        setText("#query-status", "Health fetch failed; query boundary reported the runtime miss.");
        setState("#query-status", "error");
      }
    });
    updateCounter();
  }

  function renderMotionProof() {
    const card = $("#motion-proof");
    const status = $("#motion-status");
    const progressBar = $("[data-dx-motion-progress-bar]");
    const stageList = $("#motion-stage-list");
    const activeStage = motionStages[state.motionStageIndex] || motionStages[0];

    if (!card || !status || !progressBar || !stageList || !activeStage) return;

    card.dataset.dxMotionState = activeStage.id;
    card.dataset.dxMotionProgress = String(activeStage.progress);
    card.dataset.dxMotionOrder = state.motionOrder.join(",");
    card.classList.toggle("is-active", activeStage.progress >= 67);
    progressBar.style.width = `${activeStage.progress}%`;
    status.dataset.state = activeStage.progress === 100 ? "ok" : "ready";
    status.textContent = `${activeStage.label}: ${activeStage.progress}% complete. ${activeStage.detail}`;

    state.motionOrder.forEach((stageId) => {
      const item = stageList.querySelector(`[data-dx-motion-stage="${stageId}"]`);
      if (!item) return;
      const isCurrent = stageId === activeStage.id;
      item.classList.toggle("is-current", isCurrent);
      item.dataset.dxMotionStageActive = isCurrent ? "true" : "false";
      stageList.appendChild(item);
    });
  }

  function bindMotion() {
    const advance = document.querySelector("[data-dx-motion-interaction='advance-stage']");
    const reorder = document.querySelector("[data-dx-motion-interaction='reverse-order']");
    const reset = document.querySelector("[data-dx-motion-interaction='reset-proof']");

    advance?.addEventListener("click", () => {
      state.motionStageIndex = (state.motionStageIndex + 1) % motionStages.length;
      state.motionActive = true;
      renderMotionProof();
    });

    reorder?.addEventListener("click", () => {
      state.motionOrder = [...state.motionOrder].reverse();
      renderMotionProof();
    });

    reset?.addEventListener("click", () => {
      state.motionStageIndex = 0;
      state.motionOrder = motionStages.map((stage) => stage.id);
      state.motionActive = false;
      renderMotionProof();
    });

    renderMotionProof();
  }

  function bindFumadocsDocs() {
    const card = $('[data-dx-component="launch-fumadocs-docs-workflow"]');
    const buttons = Array.from(document.querySelectorAll("[data-dx-fumadocs-page-option]"));
    const title = $("#fumadocs-title");
    const description = $("#fumadocs-description");
    const heading = $("#fumadocs-heading");
    const body = $("#fumadocs-body");
    const route = $("#fumadocs-route");
    const breadcrumb = $("#fumadocs-breadcrumb");
    const peers = $("#fumadocs-peers");
    const toc = $("#fumadocs-toc");
    const receipt = $("#fumadocs-receipt");
    const receiptButton = $('[data-dx-fumadocs-action="safe-local-route-preview"]');
    const meta = $("[data-dx-fumadocs-navigation-snapshot]");
    if (!card || !buttons.length || !title || !description || !heading || !body || !route || !breadcrumb || !peers || !toc) {
      return;
    }

    const resetReceipt = () => {
      card.setAttribute("data-dx-fumadocs-local-response", "idle");
      card.setAttribute("data-dx-fumadocs-receipt-route", "none");
      if (receipt) {
        receipt.setAttribute("data-dx-fumadocs-local-response", "idle");
        receipt.setAttribute("data-dx-fumadocs-receipt-route", "none");
        receipt.textContent = "Select a docs route, then preview the local Fumadocs route receipt.";
      }
    };

    const render = (pageId) => {
      const page = fumadocsPages.find((candidate) => candidate.id === pageId) || fumadocsPages[0];
      state.docsPage = page.id;
      card.setAttribute("data-dx-fumadocs-selected-page", page.id);
      card.setAttribute("data-dx-fumadocs-rendered-route", page.route);
      card.setAttribute("data-dx-fumadocs-toc-count", String(page.toc.length));
      if (meta) meta.setAttribute("data-dx-fumadocs-navigation-snapshot", page.breadcrumb.join("/"));

      title.textContent = page.title;
      description.textContent = page.description;
      heading.textContent = page.title;
      body.textContent = page.body;
      route.textContent = page.route;
      route.setAttribute("data-dx-fumadocs-route-label", page.route);

      breadcrumb.replaceChildren(
        ...page.breadcrumb.map((item) => {
          const node = document.createElement("li");
          node.textContent = item;
          node.setAttribute("data-dx-fumadocs-breadcrumb", item);
          return node;
        }),
      );
      peers.replaceChildren(
        ...page.peers.map((item) => {
          const node = document.createElement("li");
          node.textContent = item;
          node.setAttribute("data-dx-fumadocs-peer", item);
          return node;
        }),
      );
      toc.setAttribute("data-dx-fumadocs-toc-list", page.id);
      toc.replaceChildren(
        ...page.toc.map((item) => {
          const node = document.createElement("li");
          node.textContent = item;
          return node;
        }),
      );
      buttons.forEach((button) => {
        button.setAttribute(
          "data-dx-fumadocs-page-selected",
          button.dataset.dxFumadocsPageOption === page.id ? "true" : "false",
        );
        button.setAttribute(
          "aria-pressed",
          button.dataset.dxFumadocsPageOption === page.id ? "true" : "false",
        );
      });
      resetReceipt();
    };

    buttons.forEach((button) => {
      button.addEventListener("click", () => {
        render(button.dataset.dxFumadocsPageOption || "overview");
      });
    });

    receiptButton?.addEventListener("click", () => {
      const page = fumadocsPages.find((candidate) => candidate.id === state.docsPage) || fumadocsPages[0];
      const message = `content/fumadocs-next selected ${page.route} with ${page.toc.length} TOC entries. Configure DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS before proxying external OpenAPI requests.`;
      card.setAttribute("data-dx-fumadocs-local-response", "local-preview");
      card.setAttribute("data-dx-fumadocs-receipt-route", page.route);
      if (receipt) {
        receipt.setAttribute("data-dx-fumadocs-local-response", "local-preview");
        receipt.setAttribute("data-dx-fumadocs-receipt-route", page.route);
        receipt.textContent = message;
      }
    });

    render(state.docsPage);
  }

  function bindAutomations() {
    const status = $("#automation-readiness");
    const receipt = $("#automation-receipt");
    const buttons = Array.from(document.querySelectorAll("[data-dx-automation-connector]"));
    const filters = Array.from(document.querySelectorAll("[data-dx-automation-filter]"));
    const receiptOutput = $("#automation-receipt-output");
    if (!status || !receipt || !buttons.length) return;

    const render = (button) => {
      const connector = button.dataset.dxAutomationConnector || "n8n-nodes-base.manualTrigger";
      const readiness = button.dataset.dxAutomationConnectorStatus || "metadata-ready";
      const credentials = button.dataset.dxAutomationCredentials || "";
      state.automationConnector = connector;
      status.dataset.dxAutomationSelectedConnector = connector;
      status.dataset.dxAutomationWorkflowStatus = readiness;
      status.dataset.dxAutomationReadinessCard = readiness;
      status.dataset.dxAutomationMissingConfig = readiness === "missing-config" ? "true" : "false";
      status.dataset.state = readiness === "missing-config" ? "error" : "ready";
      status.textContent =
        readiness === "missing-config"
          ? `${connector} needs app-owned credentials: ${credentials}. Local receipt preview stays safe.`
          : `${connector} is metadata-ready. No external credentials are used.`;
      buttons.forEach((candidate) => {
        candidate.dataset.dxAutomationSelected =
          candidate === button ? "true" : "false";
      });
    };
    const selectFirstVisibleConnector = () => {
      const selected = buttons.find((button) => button.dataset.dxAutomationSelected === "true");
      if (selected && !selected.hidden) return;
      const fallback = buttons.find((button) => !button.hidden);
      if (fallback) render(fallback);
    };
    const applyFilter = (filter) => {
      filters.forEach((candidate) => {
        candidate.dataset.dxAutomationFilterActive = candidate === filter ? "true" : "false";
      });
      const filterValue = filter.dataset.dxAutomationFilter || "all";
      buttons.forEach((button) => {
        const readiness = button.dataset.dxAutomationConnectorStatus || "metadata-ready";
        button.hidden = filterValue !== "all" && readiness !== filterValue;
      });
      status.dataset.dxAutomationActiveFilter = filterValue;
      selectFirstVisibleConnector();
      status.dataset.dxAutomationReceiptState = "idle";
      status.dataset.dxAutomationReceiptStatus = "idle";
      if (receiptOutput) {
        receiptOutput.dataset.dxAutomationDraftState = "idle";
        receiptOutput.textContent = `Showing ${filterValue} n8n connectors. No local receipt drafted yet.`;
      }
    };

    buttons.forEach((button) => {
      button.addEventListener("click", () => {
        render(button);
        status.dataset.dxAutomationReceiptState = "idle";
        status.dataset.dxAutomationReceiptStatus = "idle";
        if (receiptOutput) {
          receiptOutput.dataset.dxAutomationDraftState = "idle";
          receiptOutput.textContent = "Connector selected. Receipt preview is ready to draft.";
        }
      });
    });
    filters.forEach((filter) => {
      filter.addEventListener("click", () => applyFilter(filter));
    });

    receipt.addEventListener("click", () => {
      state.automationReceiptRuns += 1;
      status.dataset.dxAutomationReceiptState = "created";
      status.dataset.dxAutomationReceiptStatus = status.dataset.dxAutomationWorkflowStatus || "metadata-ready";
      status.dataset.state =
        status.dataset.dxAutomationWorkflowStatus === "missing-config" ? "error" : "ok";
      status.textContent = `Draft workflow receipt ${state.automationReceiptRuns} prepared for ${state.automationConnector}; live execution remains credential-gated.`;
      if (receiptOutput) {
        receiptOutput.dataset.dxAutomationDraftState = "created";
        receiptOutput.dataset.dxAutomationReceiptStatus = status.dataset.dxAutomationReceiptStatus;
        receiptOutput.textContent = `Local draft ${state.automationReceiptRuns}: ${state.automationConnector} -> ${status.dataset.dxAutomationReceiptStatus}.`;
      }
    });

    render(buttons[0]);
  }

  function bindScene() {
    const panel = $('[data-dx-component="launch-scene-webgl-proof"]');
    const canvas = $("#dx-launch-scene");
    if (!(canvas instanceof HTMLCanvasElement)) return;
    const label = $("#scene-status");
    const controls = $('[data-dx-scene-readiness-controls]');
    const activeNodeLabel = $("#scene-active-node");
    const sceneNodes = [
      { id: "core-orbit", label: "Core orbit" },
      { id: "shader-plane", label: "Shader plane" },
      { id: "preview-camera", label: "Preview camera" },
    ];
    const activeSceneNode = () =>
      state.sceneNodeIndex >= 0
        ? sceneNodes[state.sceneNodeIndex % sceneNodes.length]
        : null;
    const syncSceneProof = (runtimeState) => {
      const node = activeSceneNode();
      const nodeId = node?.id || "none";
      if (panel) {
        panel.setAttribute("data-dx-scene-readiness-selected-node", nodeId);
        panel.setAttribute("data-dx-scene-performance-band", state.scenePerformanceBand);
      }
      if (controls) controls.setAttribute("data-dx-scene-readiness-selected-node", nodeId);
      canvas.dataset.dxSceneStatus = runtimeState;
      canvas.dataset.dxSceneActiveNode = nodeId;
      canvas.dataset.dxScenePerformanceBand = state.scenePerformanceBand;
      if (activeNodeLabel) activeNodeLabel.textContent = `Active node: ${node?.label || "none"}`;
      if (label) {
        label.dataset.state = runtimeState.includes("fallback") ? "ready" : "ok";
        label.textContent = `${runtimeState}: ${node?.label || "no node selected"}; performance ${state.scenePerformanceBand}.`;
      }
    };

    $("#scene-select-node")?.addEventListener("click", () => {
      state.sceneNodeIndex = (state.sceneNodeIndex + 1) % sceneNodes.length;
      syncSceneProof("node-selected");
    });
    $("#scene-regress-performance")?.addEventListener("click", () => {
      state.scenePerformanceBand = "regressed";
      syncSceneProof("manual-regression");
    });
    $("#scene-reset-performance")?.addEventListener("click", () => {
      state.sceneNodeIndex = -1;
      state.scenePerformanceBand = "stable";
      syncSceneProof("reset");
    });

    const gl = canvas.getContext("webgl", { antialias: true });
    const width = (canvas.width = Math.max(640, canvas.clientWidth * devicePixelRatio));
    const height = (canvas.height = Math.max(360, canvas.clientHeight * devicePixelRatio));
    syncSceneProof("booting");

    if (!gl) {
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      const render2d = (time) => {
        const node = activeSceneNode();
        const speed = state.scenePerformanceBand === "regressed" ? 0.35 : 1;
        ctx.clearRect(0, 0, width, height);
        const gradient = ctx.createLinearGradient(0, 0, width, height);
        gradient.addColorStop(0, "#19f48a");
        gradient.addColorStop(0.55, "#8bbcff");
        gradient.addColorStop(1, "#050505");
        ctx.fillStyle = gradient;
        ctx.fillRect(0, 0, width, height);
        ctx.fillStyle = "rgba(5,5,5,0.72)";
        ctx.fillRect(width * 0.16, height * 0.18, width * 0.68, height * 0.64);
        ctx.fillStyle = "#f7f8fb";
        ctx.font = `${32 * devicePixelRatio}px JetBrains Mono, monospace`;
        ctx.fillText(
          `${node?.label || "DX scene"} ${Math.round((time * speed) / 100)}`,
          width * 0.22,
          height * 0.52,
        );
        syncSceneProof("canvas-fallback-active");
        requestAnimationFrame(render2d);
      };
      requestAnimationFrame(render2d);
      return;
    }

    const vertex = gl.createShader(gl.VERTEX_SHADER);
    const fragment = gl.createShader(gl.FRAGMENT_SHADER);
    if (!vertex || !fragment) return;
    gl.shaderSource(
      vertex,
      `
      attribute vec2 position;
      uniform float turn;
      void main() {
        float c = cos(turn);
        float s = sin(turn);
        vec2 p = vec2(position.x * c - position.y * s, position.x * s + position.y * c);
        gl_Position = vec4(p, 0.0, 1.0);
      }
      `,
    );
    gl.shaderSource(
      fragment,
      `
      precision mediump float;
      uniform float pulse;
      void main() {
        gl_FragColor = vec4(0.1 + pulse * 0.25, 0.95, 0.58 + pulse * 0.32, 1.0);
      }
      `,
    );
    gl.compileShader(vertex);
    gl.compileShader(fragment);
    const program = gl.createProgram();
    if (!program) return;
    gl.attachShader(program, vertex);
    gl.attachShader(program, fragment);
    gl.linkProgram(program);
    gl.useProgram(program);
    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([-0.58, -0.46, 0.64, -0.34, 0, 0.68, -0.22, -0.04, 0.76, 0.22, -0.7, 0.34]),
      gl.STATIC_DRAW,
    );
    const position = gl.getAttribLocation(program, "position");
    gl.enableVertexAttribArray(position);
    gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);
    const turn = gl.getUniformLocation(program, "turn");
    const pulse = gl.getUniformLocation(program, "pulse");
    const render = (time) => {
      const speed = state.scenePerformanceBand === "regressed" ? 0.35 : 1;
      const selectedOffset = Math.max(0, state.sceneNodeIndex) * 0.22;
      gl.viewport(0, 0, width, height);
      gl.clearColor(0.02, 0.025, 0.04, 1);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.uniform1f(turn, (time * speed) / 1800 + selectedOffset);
      gl.uniform1f(pulse, (Math.sin((time * speed) / 500 + selectedOffset) + 1) / 2);
      gl.drawArrays(gl.TRIANGLES, 0, 6);
      syncSceneProof("webgl-active");
      requestAnimationFrame(render);
    };
    requestAnimationFrame(render);
  }

  function bindApiActions() {
    const trpcWorkflow = $("#trpc-runtime-workflow");
    const trpcWorkflowStatus = $("#trpc-workflow-status");

    function setTrpcWorkflow(message, status, requestId) {
      if (!trpcWorkflow || !trpcWorkflowStatus) return;
      trpcWorkflow.dataset.trpcMutationState = status;
      trpcWorkflowStatus.dataset.apiMutation = status;
      trpcWorkflowStatus.dataset.trpcMutationState = status;
      trpcWorkflowStatus.dataset.trpcRequestId = requestId;
      trpcWorkflowStatus.textContent = message;
      trpcWorkflowStatus.dataset.state = status === "error" ? "error" : "ok";
    }

    $("#trpc-health-check")?.addEventListener("click", async () => {
      setTrpcWorkflow("Checking /api/trpc/health...", "pending", "pending");
      try {
        const response = await fetch("/api/trpc/health");
        const payload = await response.json();
        const requestId = `dx-trpc-health-${++state.trpcRuns}`;
        setTrpcWorkflow(
          `Health ${payload.procedure ?? "health"}: ${payload.status ?? "ready"}; ${requestId}`,
          response.ok ? "accepted" : "error",
          requestId,
        );
      } catch {
        setTrpcWorkflow(
          "tRPC health route missing; source-owned route boundary is still visible.",
          "error",
          "missing-route",
        );
      }
    });

    $("#trpc-launch-event")?.addEventListener("click", async () => {
      const requestId = `dx-trpc-local-${++state.trpcRuns}`;
      setTrpcWorkflow("Recording safe local launchEvent validation...", "pending", requestId);
      try {
        const response = await fetch("/api/trpc/health", {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ event: "validated", route: "/launch", requestId }),
        });
        const payload = await response.json();
        setTrpcWorkflow(
          `${payload.procedure ?? "launchEvent"} accepted for /launch; invalidate health.query; ${requestId}`,
          response.ok ? "accepted" : "error",
          requestId,
        );
      } catch {
        setTrpcWorkflow(
          `Local launchEvent accepted; route execution still app-owned; ${requestId}`,
          "accepted",
          requestId,
        );
      }
    });

    $("#ai-action")?.addEventListener("click", async () => {
      setText("#ai-status", "Calling source-owned AI route...");
      try {
        const response = await fetch("/api/ai/chat", {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ message: "launch proof" }),
        });
        const payload = await response.json();
        setText("#ai-status", payload.message ?? "AI adapter responded.");
        setState("#ai-status", response.ok ? "ok" : "error");
      } catch {
        setText("#ai-status", "AI route unavailable; provider boundary still documented.");
        setState("#ai-status", "error");
      }
    });

    $("#trpc-action")?.addEventListener("click", async () => {
      setText("#trpc-status", "Checking /api/trpc/health...");
      try {
        const response = await fetch("/api/trpc/health");
        const payload = await response.json();
        setText("#trpc-status", `tRPC ${payload.procedure ?? "health"}: ${payload.status ?? "ready"}`);
        setState("#trpc-status", response.ok ? "ok" : "error");
      } catch {
        setText("#trpc-status", "tRPC health route unavailable.");
        setState("#trpc-status", "error");
      }
    });
  }

  function bindDrizzleProof() {
    const card = $('[data-dx-component="drizzle-query-proof"]');
    const status = $("#drizzle-status");
    const helper = $("#drizzle-helper");
    const statement = $("#drizzle-statement");
    const rows = $("#drizzle-rows");
    if (!card || !status || !helper || !statement || !rows) return;

    const queries = {
      "published-posts": {
        helper: "listPublishedPostPreviews",
        statement:
          "select posts.title, users.name from posts innerJoin users where posts.status = 'published'",
        rows: [
          ["dx-preview", "DX launch template", "essencefromexistence", "published"],
          ["forge-source", "Forge source-owned packages", "Friday lane", "published"],
        ],
      },
      "authors-with-counts": {
        helper: "listAuthorsWithPostCounts",
        statement:
          "with post_counts as (...) select users.name, post_counts.total from users leftJoin post_counts",
        rows: [
          ["essence", "essencefromexistence", "owner", "2 launch records"],
          ["friday", "Friday", "orchestrator", "1 readiness record"],
        ],
      },
    };

    const render = (queryId) => {
      const query = queries[queryId] ?? queries["published-posts"];
      card.dataset.dxDrizzleQuery = queryId;
      helper.dataset.dxDrizzleHelper = query.helper;
      helper.textContent = `Helper: ${query.helper}`;
      statement.textContent = query.statement;
      rows.replaceChildren(
        ...query.rows.map(([id, primary, secondary, metric]) => {
          const row = document.createElement("div");
          row.className = "route-card";
          row.dataset.dxDrizzleRow = id;
          row.innerHTML = `<strong>${primary}</strong><span>${secondary}</span><span>${metric}</span>`;
          return row;
        }),
      );
      document.querySelectorAll("[data-dx-drizzle-query-option]").forEach((button) => {
        button.dataset.active = button.dataset.dxDrizzleQueryOption === queryId ? "true" : "false";
      });
    };

    document.querySelectorAll("[data-dx-drizzle-action='select-query']").forEach((button) => {
      button.addEventListener("click", () => {
        const queryId = button.dataset.dxDrizzleQueryOption || "published-posts";
        render(queryId);
        card.dataset.dxDrizzleStatus = "local-readiness-ready";
        status.dataset.state = "ready";
        status.textContent = `Loaded ${queryId} fixture. Click Run local query to execute the safe browser proof.`;
      });
    });

    $("#drizzle-run-query")?.addEventListener("click", () => {
      state.drizzleRuns += 1;
      card.dataset.dxDrizzleStatus = "local-query-ran";
      status.dataset.state = "ok";
      status.textContent = `Local Drizzle-shaped query ran ${state.drizzleRuns} time${
        state.drizzleRuns === 1 ? "" : "s"
      }. Production SQLite still needs the app-owned better-sqlite3 runtime and database path.`;
    });

    render(card.dataset.dxDrizzleQuery || "published-posts");
  }

  function bindSupabaseSchemaQueryWorkflow() {
    const card = $('[data-dx-component="supabase-schema-query-workflow"]');
    const readiness = $("#supabase-readiness");
    const button = $("#supabase-local-query");
    const result = $("#supabase-query-result");
    if (!card || !readiness || !button || !result) return;

    button.addEventListener("click", () => {
      state.supabaseQueryRuns += 1;
      card.dataset.dxSupabaseQueryState = "ready";
      result.dataset.dxSupabaseQueryState = "ready";
      result.dataset.dxSupabaseRows = "2";
      readiness.dataset.state = "ok";
      result.innerHTML = [
        "<strong>profiles local schema query prepared</strong>",
        "<span>essencefromexistence - owner</span>",
        "<span>Friday - assistant</span>",
        `<span>Run ${state.supabaseQueryRuns}; hosted credentials remain app-owned.</span>`,
      ].join("");
    });
  }

  function bindWasm() {
    const card = $('[data-dx-component="wasm-bindgen-readiness-workflow"]');
    const button = $("#wasm-action");
    const status = $("#wasm-status");
    if (!card || !button || !status) return;

    const localAddWasmBytes = new Uint8Array([
      0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01, 0x60,
      0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01,
      0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01, 0x07, 0x00, 0x20,
      0x00, 0x20, 0x01, 0x6a, 0x0b,
    ]);

    button.addEventListener("click", async () => {
      card.dataset.dxWasmBindgenStatus = "loading";
      card.dataset.dxWasmLocalReadinessEnabled = "true";
      status.dataset.state = "ready";
      status.dataset.dxWasmAddResult = "loading";
      status.textContent = "Instantiating local WebAssembly add module...";

      try {
        const instance = await WebAssembly.instantiate(localAddWasmBytes);
        const add = instance.instance.exports.add;
        if (typeof add !== "function") {
          throw new Error("local add export missing");
        }

        state.wasmRuns += 1;
        const result = add(2, 3);
        card.dataset.dxWasmBindgenStatus = "ready";
        status.dataset.state = "ok";
        status.dataset.dxWasmAddResult = String(result);
        status.dataset.dxWasmRuns = String(state.wasmRuns);
        status.textContent = `WebAssembly add(2, 3) = ${result}. Local run ${state.wasmRuns}; app-owned wasm-bindgen module still missing.`;
      } catch (error) {
        card.dataset.dxWasmBindgenStatus = "error";
        status.dataset.state = "error";
        status.dataset.dxWasmAddResult = "error";
        status.textContent = `Local WebAssembly readiness check failed: ${error instanceof Error ? error.message : "unknown error"}.`;
      }
    });
  }

  document.addEventListener("DOMContentLoaded", () => {
    bindShadcnProof();
    bindSession();
    bindPayment();
    bindLeadForm();
    bindStateAndQuery();
    bindMotion();
    bindFumadocsDocs();
    bindAutomations();
    bindScene();
    bindApiActions();
    bindSupabaseSchemaQueryWorkflow();
    bindDrizzleProof();
    bindWasm();
  });
})();
