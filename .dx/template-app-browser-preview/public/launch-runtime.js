(() => {
  const $ = (selector) => document.querySelector(selector);
  const $$ = (selector) => Array.from(document.querySelectorAll(selector));
  const setText = (selector, value) => {
    const node = $(selector);
    if (node) node.textContent = value;
  };
  const setState = (selector, value) => {
    const node = $(selector);
    if (node) node.dataset.state = value;
  };
  function sceneTokenColor(tokenName, alpha = 1) {
    const fallback =
      tokenName === "foreground" ||
      tokenName === "primary" ||
      tokenName === "success"
        ? "CanvasText"
        : "Canvas";
    if (typeof getComputedStyle !== "function" || !document.documentElement) {
      return fallback;
    }

    const tokenValue = getComputedStyle(document.documentElement)
      .getPropertyValue(`--${tokenName}`)
      .trim();
    if (!tokenValue) {
      return fallback;
    }

    return alpha === 1 ? `hsl(${tokenValue})` : `hsl(${tokenValue} / ${alpha})`;
  }
  const escapeHtml = (value) =>
    String(value).replace(/[&<>"']/g, (character) => {
      const entities = {
        "&": "&amp;",
        "<": "&lt;",
        ">": "&gt;",
        '"': "&quot;",
        "'": "&#39;",
      };
      return entities[character];
    });

  const motionStages = [
    {
      id: "source-owned",
      label: "Source-owned adapters",
      progress: 34,
      detail: "Forge Motion helpers are visible on the root dashboard.",
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
  const motionPreferenceStorageKey = "dx.launch.motion.dashboard";
  const initialMotionOrder = motionStages.map((stage) => stage.id);

  function normalizeMotionPreferenceOrder(order) {
    if (!Array.isArray(order)) return null;

    const stageIds = order.filter(
      (stageId) =>
        typeof stageId === "string" &&
        motionStages.some((stage) => stage.id === stageId),
    );
    const uniqueStageIds = Array.from(new Set(stageIds));

    if (uniqueStageIds.length !== motionStages.length) return null;
    return uniqueStageIds;
  }

  function readMotionPreference() {
    if (typeof localStorage === "undefined") return null;

    try {
      const rawPreference = localStorage.getItem(motionPreferenceStorageKey);
      if (!rawPreference) return null;

      const preference = JSON.parse(rawPreference);
      const motionOrder = normalizeMotionPreferenceOrder(preference.motionOrder);
      if (!motionOrder) return null;

      return {
        motionOrder,
        motionReduced: Boolean(preference.motionReduced),
      };
    } catch {
      return null;
    }
  }

  function writeMotionPreference() {
    if (typeof localStorage === "undefined") return;

    try {
      localStorage.setItem(
        motionPreferenceStorageKey,
        JSON.stringify({
          motionOrder: state.motionOrder,
          motionReduced: state.motionReduced,
        }),
      );
    } catch {
      // Storage can be unavailable in embedded previews; visible state still updates.
    }
  }

  const initialMotionPreference = readMotionPreference();

  const fumadocsPages = [
    {
      id: "overview",
      title: "Launch overview",
      route: "/docs",
      description: "Source-owned App Router docs with generated navigation.",
      breadcrumb: ["Docs", "Launch", "Overview"],
      peers: ["API reference", "LLM export"],
      toc: ["Package contract", "Editable content", "Search boundary"],
      body: "DX materializes a Documentation System route from source-owned files.",
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
      body: "The Documentation System exposes llms.txt style exports for tools that need source context.",
    },
  ];

  const dashboardIntlMessages = {
    en: {
      label: "English",
      status: "English",
      detail: "next-intl dashboard copy boundary ready.",
      copy: "Dashboard copy uses source-owned next-intl messages.",
      readiness: "English copy is ready for product review.",
      planLabel: "Launch Pro",
      routePreview: "/",
      hrefLang: "en",
      localePrefix: "as-needed",
      formatPreview: "May 23, 2026, 10:00 AM",
      formatSourceApi: "useFormatter.dateTime",
      formatTimeZone: "UTC",
      numberPreview: "$49",
      numberSourceApi: "useFormatter.number",
      numberCurrency: "USD",
      alternateLinks: [
        { href: "/", hrefLang: "en", locale: "en", rel: "alternate" },
        { href: "/?locale=bn", hrefLang: "bn-BD", locale: "bn", rel: "alternate" },
      ],
      supportSla: "Same business day",
      receipt: "createDxDashboardIntlReceipt prepared for English dashboard copy.",
    },
    bn: {
      label: "বাংলা",
      status: "বাংলা",
      detail: "বাংলা কপি অ্যাপ-মালিকানাধীন অনুবাদ পর্যালোচনার জন্য প্রস্তুত।",
      copy: "ড্যাশবোর্ড কপি বাংলায় বদলেছে; কোনো credential লোড করা হয়নি।",
      readiness: "বাংলা কপি পণ্য পর্যালোচনার জন্য প্রস্তুত।",
      planLabel: "লঞ্চ প্রো",
      routePreview: "/?locale=bn",
      hrefLang: "bn-BD",
      localePrefix: "as-needed",
      formatPreview: "May 23, 2026, 10:00 AM",
      formatSourceApi: "useFormatter.dateTime",
      formatTimeZone: "UTC",
      numberPreview: "$49",
      numberSourceApi: "useFormatter.number",
      numberCurrency: "USD",
      alternateLinks: [
        { href: "/", hrefLang: "en", locale: "en", rel: "alternate" },
        { href: "/?locale=bn", hrefLang: "bn-BD", locale: "bn", rel: "alternate" },
      ],
      supportSla: "পরবর্তী কর্মদিবস",
      receipt: "createDxDashboardIntlReceipt বাংলা ড্যাশবোর্ড কপির জন্য প্রস্তুত।",
    },
  };

  const state = {
    authBoundaryReviewed: false,
    authEmail: "launch@dx.local",
    authSessionSource: "missing-config",
    locale: "en",
    intlReceiptRuns: 0,
    automationConnector: "n8n-nodes-base.manualTrigger",
    automationReceiptRuns: 0,
    counter: 0,
    docsPage: "overview",
    reviewMode: false,
    queryRuns: 0,
    queryDashboardData: null,
    motionActive: false,
    motionStageIndex: 0,
    motionOrder: initialMotionPreference?.motionOrder || initialMotionOrder,
    motionReduced: Boolean(initialMotionPreference?.motionReduced),
    drizzleRuns: 0,
    instantdbReceiptRuns: 0,
    supabaseQueryRuns: 0,
    trpcRuns: 0,
    wasmRuns: 0,
    aiRuns: 0,
    aiProvider: "openai-compatible",
    shadcnDashboardDensity: "comfortable",
    shadcnDashboardQueue: "payment",
    shadcnDashboardReceiptRuns: 0,
    zustandDashboardCommandHints: true,
    zustandDashboardSavedAt: "not-saved",
    sceneNodeIndex: -1,
    scenePerformanceBand: "stable",
    sceneQualityProfile: "preview",
    sceneMaterialPalette: "aurora",
    sceneCameraRig: "orbit",
    sceneFrameSample: "pending",
    sceneCapabilityReport: "pending",
    sceneCapabilityStatus: "canvas-missing",
    sceneViewportReport: "pending",
    sceneViewportStatus: "canvas-missing",
    sceneBoundsReport: "pending",
    sceneBoundsStatus: "pending",
    sceneRaycastReport: "pending",
    sceneRaycastStatus: "pending",
    sceneWorkflowId: "visual-runtime-health",
    sceneReceiptId: "none",
    sceneReceiptState: "idle",
    sceneReceiptRuns: 0,
    zodSettingsValidation: "idle",
    zodSettingsEmail: "",
    zodSettingsWorkspaceName: "",
    zodSettingsLaunchScoreTarget: "",
    zodSettingsTheme: "system",
    zodSettingsLocale: "en",
    zodSettingsPreviewMode: "preview",
    zodSettingsIssueCount: 0,
    zodSettingsFieldErrorCount: 0,
  };

  const zodSettingsValidationContract = {
    packageId: "validation/zod",
    formPackageId: "forms/react-hook-form",
    schema: "dxDashboardSettingsSchema",
    publicApi: "safeParseDxDashboardSettingsForm",
    receiptApi: "createDxDashboardSettingsReceipt",
    fieldErrorsApi: "z.flattenError",
    runtimeBoundary: "runtime-safe-form",
    sourceOwnedApi: "lib/validation/zod/dashboard-settings.ts",
  };

  const templateFormsValidationContract = {
    formPackageId: "forms/react-hook-form",
    validationPackageId: "validation/zod",
    dryRunReceiptApi: "createDxFormDryRunReceipt",
    dryRunReceiptSchema: "dx.forms.dry_run_receipt",
    runtimeBoundary: "runtime-safe-form",
    schemas: {
      login: "templateLoginSchema",
      workspace: "templateWorkspaceSettingsSchema",
      profile: "templateProfileSchema",
      billingContact: "templateBillingContactSchema",
    },
    enumFields: {
      billingPlan: {
        inputId: "dashboard-billing-plan",
        options: ["starter", "team", "scale"],
      },
    },
    runtimeProof: false,
    sourceProof: "source-owned-static-preview",
  };

  const launchQueryDashboardPackages = [
    {
      appOwnedBoundaryCount: 2,
      command: "dx add data-fetching-cache --write",
      displayName: "Data Fetching & Cache",
      packageId: "tanstack/query",
      receiptCount: 9,
      requiredEnvCount: 0,
      role: "server-state",
      status: "ready",
    },
    {
      appOwnedBoundaryCount: 3,
      command: "dx add authentication --write",
      displayName: "Authentication",
      packageId: "auth/better-auth",
      receiptCount: 4,
      requiredEnvCount: 2,
      role: "auth",
      status: "needs-env",
    },
    {
      appOwnedBoundaryCount: 3,
      command: "dx add stripe-js --write",
      packageId: "payments/stripe-js",
      receiptCount: 4,
      requiredEnvCount: 2,
      role: "payments",
      status: "needs-env",
    },
  ];

  const launchQueryDashboardSeed = {
    appOwnedBoundaryCount: 43,
    dashboardPackages: launchQueryDashboardPackages,
    packageCount: 30,
    requiredEnvCount: 13,
    roleCount: 22,
    source: "launch-runtime-catalog",
    status: "ready",
  };

  function readLaunchQueryDashboardData() {
    return {
      ...launchQueryDashboardSeed,
      readAt: new Date().toLocaleTimeString(),
    };
  }

  function updateMissionControl(message = "Dashboard synced from runtime interactions.") {
    const dashboard = $('[data-dx-component="launch-operating-dashboard"]');
    if (!dashboard) return;

    const paymentCard = $('[data-dx-component="launch-billing-checkout-workflow"]');
    const databaseSurface = $('[data-dx-component="database-backend-proof"], [data-dx-component="database-backend-card"]');
    const docsWorkflow = $('[data-dx-component="launch-fumadocs-docs-workflow"]');
    const docsDashboardCard = $('[data-dx-dashboard-card="docs-help"]');
    const paymentState = paymentCard?.dataset.dxStripePostResult || "idle";
    const databaseStatus = databaseSurface?.dataset.dxBackendStatus;
    const databaseDetail = databaseSurface?.dataset.dxBackendDetail;
    const docsRoute = docsWorkflow?.dataset.dxFumadocsRenderedRoute || "/docs";
    const docsPage = docsWorkflow?.dataset.dxFumadocsSelectedPage || state.docsPage;
    const docsTocCount = docsWorkflow?.dataset.dxFumadocsTocCount || "3";
    const docsReceiptState = docsWorkflow?.dataset.dxFumadocsLocalResponse || "idle";
    const automationMission = $('[data-dx-component="launch-automation-mission-summary"]');
    const automationWorkflow = $('[data-dx-component="launch-automation-dashboard-workflow"]');
    const automationSchema = $("#automation-schema-summary");
    const automationWorkflowStatus =
      automationWorkflow?.dataset.dxDashboardAutomationStatus ||
      automationSchema?.dataset.dxAutomationWorkflowStatus ||
      "metadata-ready";
    const automationNodeReadiness =
      automationWorkflow?.dataset.dxDashboardAutomationNodeReadiness ||
      automationSchema?.dataset.dxAutomationWorkflowNodeReadiness ||
      "metadata-ready";
    const automationAuthKinds =
      automationSchema?.dataset.dxAutomationAuthKinds || "none";
    const automationCredentialTypes =
      automationSchema?.dataset.dxAutomationCredentialTypes || "none";
    const automationToolReady =
      automationSchema?.dataset.dxAutomationUsableAsTool || "false";
    const automationTriggerNode =
      automationSchema?.dataset.dxAutomationTriggerNode || "false";
    const automationDashboardState =
      automationWorkflow?.dataset.dxDashboardAutomationRunReceipt === "created"
        ? "zed-handoff-created"
        : automationWorkflow?.dataset.dxDashboardAutomationReceipt === "created"
          ? "draft-created"
          : "idle";
    const automationName = state.automationConnector
      .replace(/^n8n-nodes-base\./, "")
      .replace(/([a-z])([A-Z])/g, "$1 $2");
    const dataRuns =
      state.supabaseQueryRuns + state.drizzleRuns + state.instantdbReceiptRuns + state.trpcRuns;
    const motionStage = motionStages[state.motionStageIndex] || motionStages[0];
    const motionReduced = state.motionReduced ? "preview" : "system";
    const motionSummary = $('[data-dx-component="launch-motion-dashboard-summary"]');
    const sceneWorkflowLabels = {
      "visual-runtime-health": "Runtime health",
      "render-budget": "Render budget",
      "web-preview-readiness": "Web Preview readiness",
    };
    const sceneWorkflowLabel = sceneWorkflowLabels[state.sceneWorkflowId] || "Runtime health";
    const sceneQualityLabel =
      state.sceneQualityProfile === "cinematic" ? "Cinematic" : "Web Preview";
    const sceneMaterialLabel =
      state.sceneMaterialPalette === "graphite" ? "Graphite review" : "Aurora launch";
    const sceneCameraLabel = {
      orbit: "Orbit",
      isometric: "Isometric",
      inspection: "Inspection",
    }[state.sceneCameraRig] || "Orbit";

    dashboard.dataset.dxDashboardState = "synced";
    dashboard.dataset.dxDashboardSession = state.authBoundaryReviewed
      ? "boundary-reviewed"
      : "adapter-boundary";
    dashboard.dataset.dxDashboardPayment = paymentState;
    dashboard.dataset.dxDashboardAutomation = state.automationConnector;
    dashboard.dataset.dxDashboardAutomationWorkflow = state.automationConnector;
    dashboard.dataset.dxDashboardAutomationState = automationDashboardState;
    dashboard.dataset.dxDashboardAutomationStatus = automationWorkflowStatus;
    dashboard.dataset.dxDashboardAutomationNodeReadiness = automationNodeReadiness;
    dashboard.dataset.dxDashboardDataRuns = String(dataRuns);
    dashboard.dataset.dxDashboardDatabase = databaseStatus || (dataRuns ? "runtime-updated" : "local-fixtures");
    dashboard.dataset.dxDashboardQueryRuns = String(state.queryRuns);
    dashboard.dataset.dxDashboardDocsRoute = docsRoute;
    dashboard.dataset.dxDashboardDocsPage = docsPage;
    dashboard.dataset.dxDashboardDocsReceipt = docsReceiptState;
    dashboard.dataset.dxDashboardMotion = motionStage.id;
    dashboard.dataset.dxDashboardMotionOrder = state.motionOrder.join(",");
    dashboard.dataset.dxDashboardMotionReduced = motionReduced;
    dashboard.dataset.dxDashboardWasmRuns = String(state.wasmRuns);
    dashboard.dataset.dxDashboardSettingsValidation = state.zodSettingsValidation;
    dashboard.dataset.dxDashboardSettingsIssueCount = String(state.zodSettingsIssueCount);
    dashboard.dataset.dxDashboardSettingsFieldErrorCount = String(state.zodSettingsFieldErrorCount);
    dashboard.dataset.dxDashboardSettingsWorkspace = state.zodSettingsWorkspaceName || "none";
    dashboard.dataset.dxDashboardSettingsScoreTarget = String(
      state.zodSettingsLaunchScoreTarget || "none",
    );
    dashboard.dataset.dxDashboardSettingsTheme = state.zodSettingsTheme;
    dashboard.dataset.dxDashboardSettingsLocale = state.zodSettingsLocale;
    dashboard.dataset.dxDashboardControlsDensity = state.shadcnDashboardDensity;
    dashboard.dataset.dxDashboardControlsQueue = state.shadcnDashboardQueue;
    dashboard.dataset.dxDashboardSceneWorkflow = state.sceneWorkflowId;
    dashboard.dataset.dxDashboardSceneReceipt = state.sceneReceiptState;
    dashboard.dataset.dxDashboardSceneQualityProfile = state.sceneQualityProfile;
    dashboard.dataset.dxDashboardSceneMaterialPalette = state.sceneMaterialPalette;
    dashboard.dataset.dxDashboardSceneCameraRig = state.sceneCameraRig;
    dashboard.dataset.dxDashboardSceneFrameSample = state.sceneFrameSample;
    dashboard.dataset.dxDashboardSceneCapabilityReport = state.sceneCapabilityReport;
    dashboard.dataset.dxDashboardSceneCapabilityStatus = state.sceneCapabilityStatus;
    dashboard.dataset.dxDashboardSceneViewportReport = state.sceneViewportReport;
    dashboard.dataset.dxDashboardSceneViewportStatus = state.sceneViewportStatus;
    dashboard.dataset.dxDashboardSceneBoundsReport = state.sceneBoundsReport;
    dashboard.dataset.dxDashboardSceneBoundsStatus = state.sceneBoundsStatus;
    dashboard.dataset.dxDashboardSceneRaycastReport = state.sceneRaycastReport;
    dashboard.dataset.dxDashboardSceneRaycastStatus = state.sceneRaycastStatus;
    dashboard.dataset.dxDashboardLocale = state.locale;
    document.documentElement.lang = state.locale;

    setText("#mission-session-status", state.authBoundaryReviewed ? "Reviewed" : "Not reviewed");
    setText(
      "#mission-session-detail",
      state.authBoundaryReviewed
        ? `${state.authEmail} has reviewed sign-in readiness.`
        : "Sign-in readiness mounted; credentials stay app-owned.",
    );
    setText("#mission-payment-status", paymentState === "idle" ? "Idle" : paymentState);
    setText(
      "#mission-payment-detail",
      paymentState === "posted" || paymentState === "readiness"
        ? "Stripe-shaped checkout receipt is visible."
        : "Stripe-shaped checkout contract waiting.",
    );
    setText(
      "#mission-settings-status",
      state.zodSettingsValidation === "idle" ? "Waiting" : state.zodSettingsValidation,
    );
    setText(
      "#mission-settings-detail",
      state.zodSettingsValidation === "valid"
        ? `${state.zodSettingsWorkspaceName || "The dashboard"} uses ${state.zodSettingsTheme} theme, ${state.zodSettingsLocale} locale, and score target ${
            state.zodSettingsLaunchScoreTarget || "unset"
          }; ${zodSettingsValidationContract.fieldErrorsApi} has ${state.zodSettingsFieldErrorCount} field groups.`
        : state.zodSettingsValidation === "invalid"
          ? `${zodSettingsValidationContract.publicApi} blocked ${state.zodSettingsIssueCount} settings issue${
              state.zodSettingsIssueCount === 1 ? "" : "s"
            }; ${zodSettingsValidationContract.fieldErrorsApi} grouped ${state.zodSettingsFieldErrorCount} field error${
              state.zodSettingsFieldErrorCount === 1 ? "" : "s"
            } before dashboard state changed.`
          : "Zod validation has not run yet.",
    );
    setText("#mission-automation-status", automationName || "Manual Trigger");
    setText(
      "#mission-automation-detail",
      automationDashboardState === "zed-handoff-created"
        ? "Rust/Zed run handoff receipt is ready; live execution remains operator gated."
        : state.automationReceiptRuns
          ? `${state.automationReceiptRuns} local n8n receipt preview${
              state.automationReceiptRuns === 1 ? "" : "s"
            } prepared.`
          : "n8n connector metadata ready.",
    );
    setText(
      "#mission-automation-schema",
      `Auth schema: ${automationAuthKinds}; credentials: ${automationCredentialTypes}; workflow node: ${automationNodeReadiness}; tool ready: ${automationToolReady}.`,
    );
    if (automationMission) {
      automationMission.dataset.dxAutomationAuthKinds = automationAuthKinds;
      automationMission.dataset.dxAutomationCredentialTypes = automationCredentialTypes;
      automationMission.dataset.dxAutomationDashboardState = automationDashboardState;
      automationMission.dataset.dxAutomationSelectedConnector = state.automationConnector;
      automationMission.dataset.dxAutomationTriggerNode = automationTriggerNode;
      automationMission.dataset.dxAutomationUsableAsTool = automationToolReady;
      automationMission.dataset.dxAutomationWorkflowNodeReadiness = automationNodeReadiness;
      automationMission.dataset.dxAutomationWorkflowStatus = automationWorkflowStatus;
    }
    setText(
      "#mission-database-status",
      databaseStatus ||
        (dataRuns ? `${dataRuns} local check${dataRuns === 1 ? "" : "s"}` : "Local fixtures"),
    );
    setText(
      "#mission-database-detail",
      databaseDetail ||
        (dataRuns
          ? "Supabase, Drizzle, InstantDB, or tRPC runtime actions have updated the dashboard."
          : "Supabase, Drizzle, InstantDB, and tRPC adapters visible."),
    );
    const queryData = state.queryDashboardData || launchQueryDashboardSeed;
    setText("#mission-query-status", `${queryData.packageCount} packages`);
    setText(
      "#mission-query-detail",
      state.queryRuns
        ? `${state.queryRuns} Data Fetching & Cache dashboard refresh${
            state.queryRuns === 1 ? "" : "es"
          }; source ${queryData.source}.`
        : "Data Fetching & Cache read model waiting for a dashboard refresh.",
    );
    if (docsDashboardCard) {
      docsDashboardCard.dataset.dxFumadocsDashboardRoute = docsRoute;
      docsDashboardCard.dataset.dxFumadocsDashboardPage = docsPage;
      docsDashboardCard.dataset.dxFumadocsDashboardReceipt = docsReceiptState;
    }
    setText("#mission-docs-status", docsRoute);
    setText(
      "#mission-docs-detail",
      docsReceiptState === "local-preview"
        ? `Fumadocs local receipt prepared for ${docsRoute} with ${docsTocCount} TOC entries.`
        : `Fumadocs selected ${docsPage} with ${docsTocCount} TOC entries; OpenAPI proxy config stays app-owned.`,
    );
    setText("#mission-scene-status", sceneWorkflowLabel);
    setText(
      "#mission-scene-detail",
      state.sceneReceiptState === "prepared"
        ? `${state.sceneReceiptId} prepared from the ${sceneQualityLabel} 3D render budget workflow with ${sceneMaterialLabel} and ${sceneCameraLabel}.`
        : `${sceneQualityLabel} renderer; ${sceneMaterialLabel} material; ${sceneCameraLabel} camera; ${state.scenePerformanceBand} budget; frame sample ${state.sceneFrameSample}; capability ${state.sceneCapabilityReport}; viewport ${state.sceneViewportReport}; bounds ${state.sceneBoundsReport}; raycast ${state.sceneRaycastReport}. Focus a scene node or prepare a render budget receipt.`,
    );
    setText(
      "#mission-instantdb-status",
      state.instantdbReceiptRuns
        ? `${state.instantdbReceiptRuns} receipt${state.instantdbReceiptRuns === 1 ? "" : "s"}`
        : "Missing config",
    );
    setText(
      "#mission-instantdb-detail",
      state.instantdbReceiptRuns
        ? "InstantDB local realtime receipt is visible; hosted app id stays app-owned."
        : "InstantDB receipt waiting for app-owned app id.",
    );
    setText("#mission-motion-status", motionStage.label);
    setText(
      "#mission-motion-detail",
      `${motionStage.progress}% ready. ${motionStage.detail}`,
    );
    setText(
      "#mission-motion-policy",
      state.motionReduced
        ? "Reduced motion preview is active for dashboard choreography review."
        : "Motion policy follows the system preference.",
    );
    if (motionSummary) {
      motionSummary.dataset.dxMotionPolicy = "app-owned-reduced-motion-preview";
      motionSummary.dataset.dxMotionReduced = motionReduced;
    }
    setText(
      "#mission-wasm-status",
      state.wasmRuns ? `${state.wasmRuns} local run${state.wasmRuns === 1 ? "" : "s"}` : "Idle",
    );
    setText(
      "#mission-wasm-detail",
      state.wasmRuns
        ? "wasm-bindgen local compute updated the dashboard."
        : "WebAssembly local add check waiting.",
    );
    setText(
      "#mission-controls-status",
      state.shadcnDashboardDensity === "compact" ? "Compact" : "Comfortable",
    );
    setText(
      "#mission-controls-detail",
      state.shadcnDashboardReceiptRuns
        ? `${state.shadcnDashboardQueue} queue has ${state.shadcnDashboardReceiptRuns} shadcn receipt preview${
            state.shadcnDashboardReceiptRuns === 1 ? "" : "s"
          }.`
        : `shadcn/ui controls focused on ${state.shadcnDashboardQueue}.`,
    );
    const intl = dashboardIntlMessages[state.locale] ?? dashboardIntlMessages.en;
    const localeCard = $('[data-dx-component="next-intl-dashboard-locale-workflow"]');
    if (localeCard) {
      localeCard.dataset.dxDashboardCopyLocale = state.locale;
      localeCard.dataset.dxIntlReadinessCopy = intl.readiness;
      localeCard.dataset.dxIntlHreflang = intl.hrefLang;
      localeCard.dataset.dxIntlLocalePrefix = intl.localePrefix;
      localeCard.dataset.dxIntlRoutePreview = intl.routePreview;
      localeCard.dataset.dxIntlFormatPreview = intl.formatPreview;
      localeCard.dataset.dxIntlFormatSourceApi = intl.formatSourceApi;
      localeCard.dataset.dxIntlFormatTimeZone = intl.formatTimeZone;
      localeCard.dataset.dxIntlNumberPreview = intl.numberPreview;
      localeCard.dataset.dxIntlNumberSourceApi = intl.numberSourceApi;
      localeCard.dataset.dxIntlNumberCurrency = intl.numberCurrency;
    }
    setText("#mission-locale-status", intl.status);
    setText("#mission-locale-detail", intl.detail);
    setText("#mission-locale-copy", intl.copy);
    setText("#mission-locale-readiness", intl.readiness);
    setText("#mission-locale-format", `Formatter preview: ${intl.formatPreview}.`);
    setText("#mission-locale-price", `Localized plan price: ${intl.numberPreview}/mo.`);
    setText("#mission-locale-plan", `Plan copy: ${intl.planLabel}.`);
    setText("#mission-locale-support", `Support SLA: ${intl.supportSla}.`);
    const localePlan = $("#mission-locale-plan");
    const localeSupport = $("#mission-locale-support");
    const localeFormat = $("#mission-locale-format");
    const localePrice = $("#mission-locale-price");
    const localeRoute = $("#mission-locale-route");
    const routeLink = $("#mission-locale-route-link");
    if (localePlan) localePlan.dataset.dxIntlPlanLabel = intl.planLabel;
    if (localeSupport) localeSupport.dataset.dxIntlSupportSla = intl.supportSla;
    if (localeFormat) {
      localeFormat.dataset.dxIntlFormatPreview = intl.formatPreview;
      localeFormat.dataset.dxIntlFormatSourceApi = intl.formatSourceApi;
      localeFormat.dataset.dxIntlFormatTimeZone = intl.formatTimeZone;
    }
    if (localePrice) {
      localePrice.dataset.dxIntlNumberPreview = intl.numberPreview;
      localePrice.dataset.dxIntlNumberSourceApi = intl.numberSourceApi;
      localePrice.dataset.dxIntlNumberCurrency = intl.numberCurrency;
    }
    if (localeRoute) {
      localeRoute.dataset.dxIntlRoutePreview = intl.routePreview;
      localeRoute.dataset.dxIntlHreflang = intl.hrefLang;
      localeRoute.dataset.dxIntlLocalePrefix = intl.localePrefix;
    }
    if (routeLink) {
      routeLink.href = intl.routePreview;
      routeLink.hreflang = intl.hrefLang;
      routeLink.dataset.dxIntlHreflang = intl.hrefLang;
      routeLink.dataset.dxIntlLocalePrefix = intl.localePrefix;
      routeLink.textContent = intl.routePreview;
    }
    $$('[data-dx-intl-action="review-alternate-link"]').forEach((link) => {
      const alternate = intl.alternateLinks.find(
        (item) => item.locale === link.dataset.dxIntlAlternateLocale,
      );
      if (!alternate) return;
      link.href = alternate.href;
      link.hreflang = alternate.hrefLang;
      link.rel = alternate.rel;
      link.dataset.dxIntlAlternateHref = alternate.href;
      link.textContent = alternate.hrefLang;
    });
    const status = $("#mission-dashboard-status");
    if (status) {
      status.dataset.dxDashboardStatus = "synced";
      status.dataset.state = "ok";
      status.textContent = message;
    }
  }

  function bindMissionControl() {
    const sync = document.querySelector("[data-dx-dashboard-action='sync-from-runtime']");
    sync?.addEventListener("click", () => updateMissionControl("Dashboard synced on demand."));
    updateMissionControl("Dashboard connected to the source-owned runtime proof surfaces.");
  }

  function bindIntlDashboard() {
    const card = $('[data-dx-component="next-intl-dashboard-locale-workflow"]');
    const buttons = Array.from(
      document.querySelectorAll("[data-dx-intl-action='switch-dashboard-locale']"),
    );
    const receiptButton = $("#mission-locale-receipt-action");
    const receipt = $("#mission-locale-receipt");
    if (!card || !buttons.length) return;

    const renderLocale = (message) => {
      const intl = dashboardIntlMessages[state.locale] ?? dashboardIntlMessages.en;
      card.dataset.dxDashboardCopyLocale = state.locale;
      card.dataset.dxIntlReadinessCopy = intl.readiness;
      card.dataset.dxIntlHreflang = intl.hrefLang;
      card.dataset.dxIntlLocalePrefix = intl.localePrefix;
      card.dataset.dxIntlRoutePreview = intl.routePreview;
      card.dataset.dxIntlFormatPreview = intl.formatPreview;
      card.dataset.dxIntlFormatSourceApi = intl.formatSourceApi;
      card.dataset.dxIntlFormatTimeZone = intl.formatTimeZone;
      card.dataset.dxIntlNumberPreview = intl.numberPreview;
      card.dataset.dxIntlNumberSourceApi = intl.numberSourceApi;
      card.dataset.dxIntlNumberCurrency = intl.numberCurrency;
      buttons.forEach((button) => {
        const selected = button.dataset.dxIntlLocaleOption === state.locale;
        button.dataset.dxIntlLocaleSelected = selected ? "true" : "false";
        button.setAttribute("aria-pressed", selected ? "true" : "false");
      });
      setText("#mission-locale-status", intl.status);
      setText("#mission-locale-detail", intl.detail);
      setText("#mission-locale-copy", intl.copy);
      setText("#mission-locale-readiness", intl.readiness);
      setText("#mission-locale-format", `Formatter preview: ${intl.formatPreview}.`);
      setText("#mission-locale-price", `Localized plan price: ${intl.numberPreview}/mo.`);
      document.documentElement.lang = state.locale;
      updateMissionControl(message);
    };

    buttons.forEach((button) => {
      button.addEventListener("click", () => {
        state.locale = button.dataset.dxIntlLocaleOption === "bn" ? "bn" : "en";
        renderLocale(`Dashboard copy switched to ${dashboardIntlMessages[state.locale].label}.`);
      });
    });

    receiptButton?.addEventListener("click", () => {
      state.intlReceiptRuns += 1;
      if (receipt) {
        const intl = dashboardIntlMessages[state.locale] ?? dashboardIntlMessages.en;
        receipt.dataset.dxIntlReceiptState = "ready";
        receipt.dataset.dxIntlReceiptLocale = state.locale;
        receipt.dataset.dxIntlReceiptRuns = String(state.intlReceiptRuns);
        receipt.dataset.dxIntlReceiptFactory = "createDxDashboardIntlReceipt";
        receipt.dataset.dxIntlReceiptRoute = intl.routePreview;
        receipt.dataset.dxIntlReceiptHreflang = intl.hrefLang;
        receipt.dataset.dxIntlReceiptFormatSource = intl.formatSourceApi;
        receipt.dataset.dxIntlReceiptNumberSource = intl.numberSourceApi;
        receipt.textContent = `${intl.receipt} Run ${state.intlReceiptRuns}.`;
      }
      renderLocale(`Dashboard locale receipt prepared for ${dashboardIntlMessages[state.locale].label}.`);
    });

    renderLocale("Dashboard locale copy connected to next-intl workflow.");
  }

  function normalizeLaunchDashboardSettings(settings = {}) {
    const density =
      settings.density === "compact" || settings.density === "comfortable"
        ? settings.density
        : "comfortable";
    const focus = ["session", "payment", "data", "studio"].includes(settings.focus)
      ? settings.focus
      : "payment";

    return {
      density,
      focus,
      commandHints:
        typeof settings.commandHints === "boolean" ? settings.commandHints : true,
      savedAt:
        typeof settings.savedAt === "string" && settings.savedAt
          ? settings.savedAt
          : "not-saved",
    };
  }

  function loadLaunchDashboardSettings() {
    try {
      const saved = JSON.parse(
        localStorage.getItem("dx-launch-dashboard-settings") || "{}",
      );
      const settings = normalizeLaunchDashboardSettings(saved);
      state.shadcnDashboardDensity = settings.density;
      state.shadcnDashboardQueue = settings.focus;
      state.zustandDashboardCommandHints = settings.commandHints;
      state.zustandDashboardSavedAt = settings.savedAt;
    } catch {
      state.shadcnDashboardDensity = "comfortable";
      state.shadcnDashboardQueue = "payment";
      state.zustandDashboardCommandHints = true;
      state.zustandDashboardSavedAt = "not-saved";
    }
  }

  function saveLaunchDashboardSettings(reason = "dashboard-state") {
    const savedAt = new Date().toISOString();
    state.zustandDashboardSavedAt = savedAt;
    try {
      localStorage.setItem(
        "dx-launch-dashboard-settings",
        JSON.stringify({
          density: state.shadcnDashboardDensity,
          focus: state.shadcnDashboardQueue,
          commandHints: state.zustandDashboardCommandHints,
          savedAt,
          reason,
        }),
      );
      return true;
    } catch {
      return false;
    }
  }

  function markLaunchDashboardHydration(event) {
    [
      '[data-dx-component="launch-operating-dashboard"]',
      '[data-dx-component="launch-dashboard-state-shell"]',
      '[data-dx-component="launch-dashboard-state-summary"]',
      '[data-dx-component="launch-dashboard-state-workflow"]',
      '[data-dx-component="shadcn-dashboard-controls-runtime"]',
    ].forEach((selector) => {
      document
        .querySelectorAll(selector)
        .forEach((node) =>
          node.setAttribute("data-dx-zustand-hydration-event", event),
        );
    });
  }

  function markLaunchDashboardRehydrateState(nextState) {
    [
      '[data-dx-component="launch-operating-dashboard"]',
      '[data-dx-component="launch-dashboard-state-shell"]',
      '[data-dx-component="launch-dashboard-state-summary"]',
      '[data-dx-component="launch-dashboard-state-workflow"]',
      '[data-dx-component="shadcn-dashboard-controls-runtime"]',
      '[data-dx-zustand-action="rehydrate-dashboard-settings"]',
    ].forEach((selector) => {
      document
        .querySelectorAll(selector)
        .forEach((node) =>
          node.setAttribute("data-dx-zustand-rehydrate-state", nextState),
        );
    });

    const rehydrate = $("#dashboard-settings-rehydrate");
    if (!rehydrate) return;

    const isLoading = nextState === "loading";
    rehydrate.disabled = isLoading;
    rehydrate.setAttribute("aria-disabled", isLoading ? "true" : "false");
    rehydrate.textContent = isLoading
      ? "Rehydrating saved view"
      : "Rehydrate saved view";
  }

  function applyLaunchDashboardSettings(
    message = "Zustand dashboard settings applied.",
  ) {
    const dashboard = $('[data-dx-component="launch-operating-dashboard"]');
    const shell = $('[data-dx-component="launch-dashboard-state-shell"]');
    const panel = $('[data-dx-component="shadcn-dashboard-controls-runtime"]');
    const workflow = $('[data-dx-component="launch-dashboard-state-workflow"]');
    const commandHints = state.zustandDashboardCommandHints ? "enabled" : "disabled";

    [dashboard, shell, panel, workflow].forEach((node) => {
      if (!node) return;
      node.setAttribute(
        "data-dx-zustand-dashboard-density",
        state.shadcnDashboardDensity,
      );
      node.setAttribute(
        "data-dx-zustand-dashboard-focus",
        state.shadcnDashboardQueue,
      );
      node.setAttribute("data-dx-zustand-command-hints", commandHints);
      node.setAttribute("data-dx-zustand-dashboard-applied", "true");
    });

    if (panel) {
      panel.dataset.dxShadcnDashboardDensity = state.shadcnDashboardDensity;
      panel.dataset.dxShadcnDashboardQueue = state.shadcnDashboardQueue;
    }

    const summary = $("#zustand-dashboard-state-summary");
    if (summary) {
      summary.dataset.dxZustandSummaryDensity = state.shadcnDashboardDensity;
      summary.dataset.dxZustandSummaryFocus = state.shadcnDashboardQueue;
      summary.dataset.dxZustandSummaryHints = commandHints;
      summary.dataset.dxZustandSummarySavedAt = state.zustandDashboardSavedAt;
    }
    setText(
      "#zustand-summary-density",
      `Dashboard density: ${state.shadcnDashboardDensity}`,
    );
    setText(
      "#zustand-summary-focus",
      `Focused workflow: ${state.shadcnDashboardQueue}`,
    );
    setText(
      "#zustand-summary-hints",
      `Command hints: ${state.zustandDashboardCommandHints ? "on" : "off"}`,
    );
    setText("#zustand-summary-saved", `Saved: ${state.zustandDashboardSavedAt}`);

    const activeQueueControl = document.querySelector(
      `[data-dx-shadcn-dashboard-queue-option="${state.shadcnDashboardQueue}"]`,
    );
    const controlsTarget =
      activeQueueControl?.getAttribute("aria-controls") || "mission-controls-status";
    [dashboard, shell, panel, workflow].forEach((node) => {
      if (node) {
        node.setAttribute("data-dx-shadcn-dashboard-controls-target", controlsTarget);
      }
    });

    document
      .querySelectorAll("[data-dx-shadcn-dashboard-density-option]")
      .forEach((button) => {
        const selected =
          button.dataset.dxShadcnDashboardDensityOption ===
          state.shadcnDashboardDensity;
        button.dataset.dxShadcnDashboardSelected = selected ? "true" : "false";
        button.dataset.variant = selected ? "secondary" : "outline";
        button.setAttribute("aria-pressed", selected ? "true" : "false");
      });
    document
      .querySelectorAll("[data-dx-shadcn-dashboard-queue-option]")
      .forEach((button) => {
        const selected =
          button.dataset.dxShadcnDashboardQueueOption ===
          state.shadcnDashboardQueue;
        button.dataset.dxShadcnDashboardSelected = selected ? "true" : "false";
        button.setAttribute("aria-pressed", selected ? "true" : "false");
      });
    document.querySelectorAll("[data-dx-dashboard-card]").forEach((card) => {
      const cardId = card.dataset.dxDashboardCard || "";
      const isFocused =
        cardId === state.shadcnDashboardQueue ||
        (state.shadcnDashboardQueue === "data" &&
          ["database", "local-compute"].includes(cardId));
      card.setAttribute(
        "data-dx-zustand-dashboard-focused",
        isFocused ? "true" : "false",
      );
    });

    setText(
      "#zustand-dashboard-state-status",
      `${message} Density ${state.shadcnDashboardDensity}; focus ${state.shadcnDashboardQueue}; hints ${commandHints}.`,
    );
    updateMissionControl("Zustand persisted dashboard settings updated.");
  }

  function bindLaunchDashboardSettings() {
    const reset = $("#dashboard-settings-reset");
    const rehydrate = $("#dashboard-settings-rehydrate");
    const hints = $("#dashboard-command-hints");

    loadLaunchDashboardSettings();
    markLaunchDashboardHydration("onFinishHydration");
    markLaunchDashboardRehydrateState("idle");
    hints?.addEventListener("click", () => {
      state.zustandDashboardCommandHints = !state.zustandDashboardCommandHints;
      const saved = saveLaunchDashboardSettings("command-hints");
      applyLaunchDashboardSettings(
        saved
          ? "Saved command-hint preference to dx-launch-dashboard-settings."
          : "Command-hint preference changed in memory; local storage unavailable.",
      );
    });
    reset?.addEventListener("click", () => {
      try {
        localStorage.removeItem("dx-launch-dashboard-settings");
      } catch {
        // Keep the reset visible even in locked-down browser storage.
      }
      state.shadcnDashboardDensity = "comfortable";
      state.shadcnDashboardQueue = "payment";
      state.zustandDashboardCommandHints = true;
      state.zustandDashboardSavedAt = "not-saved";
      const status = $("#shadcn-dashboard-status");
      if (status) {
        status.dataset.dxShadcnDashboardReceipt = "reset";
        status.dataset.state = "ready";
        status.textContent = "Zustand dashboard settings reset to the launch defaults.";
      }
      applyLaunchDashboardSettings("Reset persisted dashboard settings.");
    });
    rehydrate?.addEventListener("click", () => {
      markLaunchDashboardHydration("onHydrate");
      markLaunchDashboardRehydrateState("loading");
      loadLaunchDashboardSettings();
      markLaunchDashboardHydration("onFinishHydration");
      markLaunchDashboardRehydrateState("idle");
      const status = $("#shadcn-dashboard-status");
      if (status) {
        status.dataset.dxShadcnDashboardReceipt = "rehydrated";
        status.dataset.state = "ready";
        status.textContent =
          "Rehydrated dx-launch-dashboard-settings from local storage.";
      }
      applyLaunchDashboardSettings("Rehydrated dx-launch-dashboard-settings.");
    });
    applyLaunchDashboardSettings("Restored dx-launch-dashboard-settings.");
  }

  function bindShadcnDashboardControls() {
    const panel = $('[data-dx-component="shadcn-dashboard-controls-runtime"]');
    const densityButtons = Array.from(
      document.querySelectorAll("[data-dx-shadcn-dashboard-action='set-density']"),
    );
    const queueButtons = Array.from(
      document.querySelectorAll("[data-dx-shadcn-dashboard-action='select-queue']"),
    );
    const filterInput = $("#shadcn-dashboard-filter");
    const noteInput = $("#shadcn-dashboard-note");
    const focusTargetButton = document.querySelector(
      "[data-dx-shadcn-dashboard-action='focus-target-card']",
    );
    const save =
      document.querySelector("[data-dx-shadcn-dashboard-action='preview-dashboard-receipt']") ||
      $("#shadcn-dashboard-save");
    const status = $("#shadcn-dashboard-status");
    if (!panel || !densityButtons.length || !queueButtons.length || !save || !status) return;

    const render = () => {
      panel.dataset.dxShadcnDashboardDensity = state.shadcnDashboardDensity;
      panel.dataset.dxShadcnDashboardQueue = state.shadcnDashboardQueue;
      const activeQueueButton = queueButtons.find(
        (button) =>
          button.dataset.dxShadcnDashboardQueueOption === state.shadcnDashboardQueue,
      );
      panel.dataset.dxShadcnDashboardControlsTarget =
        activeQueueButton?.getAttribute("aria-controls") || "mission-controls-status";
      panel.dataset.dxShadcnDashboardFocusTarget =
        panel.dataset.dxShadcnDashboardControlsTarget;
      if (focusTargetButton) {
        focusTargetButton.setAttribute(
          "aria-controls",
          panel.dataset.dxShadcnDashboardControlsTarget,
        );
        focusTargetButton.setAttribute(
          "data-dx-shadcn-dashboard-focus-target",
          panel.dataset.dxShadcnDashboardControlsTarget,
        );
      }
      densityButtons.forEach((button) => {
        const selected =
          button.dataset.dxShadcnDashboardDensityOption === state.shadcnDashboardDensity;
        button.dataset.dxShadcnDashboardSelected = selected ? "true" : "false";
        button.dataset.variant = selected ? "secondary" : "outline";
        button.setAttribute("aria-pressed", selected ? "true" : "false");
      });
      queueButtons.forEach((button) => {
        const selected =
          button.dataset.dxShadcnDashboardQueueOption === state.shadcnDashboardQueue;
        button.dataset.dxShadcnDashboardSelected = selected ? "true" : "false";
        button.setAttribute("aria-pressed", selected ? "true" : "false");
        button.setAttribute("aria-current", selected ? "true" : "false");
        button.setAttribute("tabindex", selected ? "0" : "-1");
      });
      applyLaunchDashboardSettings("Dashboard controls synced from Zustand store.");
    };

    function focusDashboardQueueButton(button, reason) {
      if (!button) return;
      state.shadcnDashboardQueue =
        button.dataset.dxShadcnDashboardQueueOption || "payment";
      const saved = saveLaunchDashboardSettings(reason);
      panel.dataset.dxShadcnDashboardReceipt = "idle";
      status.dataset.dxShadcnDashboardReceipt = "idle";
      status.dataset.state = "ready";
      status.textContent = `${state.shadcnDashboardQueue} queue selected for dashboard review${
        saved ? " and saved." : "; storage unavailable."
      }`;
      render();
      if (reason === "keyboard" && typeof button.focus === "function") {
        button.focus();
      }
    }

    function focusShadcnDashboardTarget() {
      const targetId =
        panel.dataset.dxShadcnDashboardControlsTarget || "mission-payment-status";
      const target = document.getElementById(targetId);
      const card = target?.closest("[data-dx-dashboard-card]");
      const focusTarget = card || target;
      document
        .querySelectorAll("[data-dx-shadcn-dashboard-target-focused]")
        .forEach((node) => {
          node.setAttribute("data-dx-shadcn-dashboard-target-focused", "false");
        });
      if (focusTarget) {
        focusTarget.setAttribute("tabindex", "-1");
        focusTarget.setAttribute("data-dx-shadcn-dashboard-target-focused", "true");
        focusTarget.setAttribute(
          "data-dx-shadcn-dashboard-focus-source",
          "shadcn-dashboard-controls",
        );
        focusTarget.focus({ preventScroll: true });
        focusTarget.scrollIntoView({ block: "nearest", behavior: "smooth" });
      }
      panel.dataset.dxShadcnDashboardFocusTarget = targetId;
      status.dataset.dxShadcnDashboardTargetFocused = targetId;
      status.dataset.state = "ready";
      status.textContent = `${targetId} focused from shadcn/ui dashboard controls.`;
      updateMissionControl("shadcn/ui focused the selected dashboard card.");
    }

    densityButtons.forEach((button) => {
      button.addEventListener("click", () => {
        state.shadcnDashboardDensity =
          button.dataset.dxShadcnDashboardDensityOption || "comfortable";
        const saved = saveLaunchDashboardSettings("density");
        panel.dataset.dxShadcnDashboardReceipt = "idle";
        status.dataset.dxShadcnDashboardReceipt = "idle";
        status.dataset.state = "ready";
        status.textContent = `${state.shadcnDashboardDensity} dashboard density selected${
          saved ? " and saved." : "; storage unavailable."
        }`;
        render();
      });
    });

    queueButtons.forEach((button) => {
      button.addEventListener("click", () => {
        focusDashboardQueueButton(button, "focus");
      });
      button.addEventListener("keydown", (event) => {
        const currentIndex = queueButtons.indexOf(button);
        let nextIndex = currentIndex;

        switch (event.key) {
          case "ArrowRight":
          case "ArrowDown":
            nextIndex = (currentIndex + 1) % queueButtons.length;
            break;
          case "ArrowLeft":
          case "ArrowUp":
            nextIndex =
              (currentIndex - 1 + queueButtons.length) % queueButtons.length;
            break;
          case "Home":
            nextIndex = 0;
            break;
          case "End":
            nextIndex = queueButtons.length - 1;
            break;
          default:
            return;
        }

        event.preventDefault();
        focusDashboardQueueButton(queueButtons[nextIndex], "keyboard");
      });
    });

    focusTargetButton?.addEventListener("click", focusShadcnDashboardTarget);

    save.addEventListener("click", () => {
      const filter =
        filterInput && "value" in filterInput
          ? String(filterInput.value).trim()
          : "all controls";
      const note =
        noteInput && "value" in noteInput ? String(noteInput.value).trim() : "";
      const normalizedFilter =
        filter.toLowerCase().replace(/[^a-z0-9]+/g, "-") || "all-controls";
      state.shadcnDashboardReceiptRuns += 1;
      const receiptId = `dx-shadcn-dashboard-${state.shadcnDashboardDensity}-${state.shadcnDashboardQueue}-${normalizedFilter}`;
      const saved = saveLaunchDashboardSettings(receiptId);
      panel.dataset.dxShadcnDashboardReceipt = receiptId;
      status.dataset.dxShadcnDashboardReceipt = receiptId;
      status.dataset.state = "ok";
      status.textContent = `${receiptId}: ${filter || "all controls"} ready for app-owned dashboard persistence.${
        saved ? " Stored by dx-launch-dashboard-settings." : " Storage unavailable; receipt is in memory."
      }${note ? ` Note: ${note}` : ""}`;
      applyLaunchDashboardSettings("Saved dashboard view through Zustand persist.");
      updateMissionControl("shadcn/ui dashboard control receipt prepared.");
    });

    render();
  }

  function bindSession() {
    const card = $('[data-dx-component="better-auth-boundary-review"][data-dx-package="auth/better-auth"]');
    const dashboardCard = $(
      '[data-dx-component="better-auth-account-dashboard-workflow"][data-dx-dashboard-card="session"]',
    );
    const status = $("#session-status");
    const emailInput = $("#session-boundary-email");
    const missionEmailInput = $("#mission-auth-email");
    const localUser = $("#session-boundary-review-user");
    const localAccount = $("#session-boundary-review-account");
    const signIn = $("#session-sign-in");
    const signUp = $("#session-sign-up");
    const localPreviewButton = $("#session-boundary-review");
    const signOut = $("#session-sign-out");
    const missionPreview = $("#mission-auth-preview");
    const missionSignUp = $("#mission-auth-sign-up");
    const missionLocalSession = $("#mission-auth-boundary-review");
    const missionLocalPreview = $("#mission-auth-boundary-review-state");
    const missionReceipt = $("#mission-auth-receipt");
    const missionProviderButtons = Array.from(
      document.querySelectorAll("[data-dx-auth-provider-option]"),
    );
    const emailSignUpButtons = $$("[data-dx-auth-interaction='email-sign-up']");
    const localPreviewButtons = $$("[data-dx-auth-interaction='mark-boundary-reviewed']");
    if (!status || !signIn || !signUp || !signOut) return;
    const storageKey = card?.getAttribute("data-dx-auth-storage-key") || "dx-better-auth-boundary-review";
    const setAuthNetworkState = (networkState) => {
      const isConfigured = networkState === "configured";
      [card, dashboardCard].forEach((target) => {
        if (!target) return;
        target.setAttribute("data-dx-auth-network-state", networkState);
        target.setAttribute(
          "data-dx-auth-network-disabled",
          isConfigured ? "false" : "true",
        );
      });
      [signIn, signUp, missionPreview, missionSignUp].forEach((target) => {
        if (!target) return;
        target.setAttribute(
          "data-dx-auth-network-disabled",
          isConfigured ? "false" : "true",
        );
      });
    };

    const currentEmail = () => {
      const missionValue =
        missionEmailInput && "value" in missionEmailInput
          ? String(missionEmailInput.value).trim()
          : "";
      const value =
        missionValue ||
        (emailInput && "value" in emailInput ? String(emailInput.value).trim() : "");
      return value.includes("@") ? value : "launch@dx.local";
    };

    function setBetterAuthRuntimeState(actionState, options = {}) {
      const email = options.email || state.authEmail || currentEmail();
      const sessionSource =
        options.sessionSource ||
        (state.authBoundaryReviewed ? state.authSessionSource || "boundary-review" : "missing-config");
      const localSession =
        options.localSession || (state.authBoundaryReviewed ? "reviewed" : "not-created");
      const boundaryReview =
        options.boundaryReview ||
        (sessionSource === "boundary-review" ? "active" : "idle");
      const boundaryEmail = options.boundaryEmail || (state.authBoundaryReviewed ? email : "none");

      [card, dashboardCard].forEach((target) => {
        if (!target) return;
        target.setAttribute("data-dx-auth-safe-action-state", actionState);
        target.setAttribute("data-dx-auth-local-session", localSession);
        target.setAttribute("data-dx-auth-session-source", sessionSource);
        target.setAttribute("data-dx-auth-boundary-review", boundaryReview);
        target.setAttribute("data-dx-auth-boundary-email", boundaryEmail);
      });
    }

    const setAuthReceipt = (receiptId, message, stateValue = "ready") => {
      setBetterAuthRuntimeState(stateValue);
      if (missionReceipt) {
        missionReceipt.setAttribute("data-dx-auth-dashboard-receipt", receiptId);
        missionReceipt.setAttribute(
          "data-dx-auth-missing-provider-state",
          stateValue === "provider-missing-config" ? "missing-config" : "local-ready",
        );
        missionReceipt.textContent = message;
      }
    };

    const readStoredSession = () => {
      try {
        const saved = JSON.parse(sessionStorage.getItem(storageKey) || "null");
        if (saved && typeof saved.email === "string" && saved.email.includes("@")) {
          state.authBoundaryReviewed = true;
          state.authEmail = saved.email;
          state.authSessionSource = "boundary-review";
          if (emailInput && "value" in emailInput) emailInput.value = saved.email;
          if (missionEmailInput && "value" in missionEmailInput) {
            missionEmailInput.value = saved.email;
          }
        }
      } catch {
        state.authBoundaryReviewed = false;
      }
    };

    const writeStoredSession = () => {
      try {
        if (state.authBoundaryReviewed) {
          sessionStorage.setItem(
            storageKey,
            JSON.stringify({
              email: state.authEmail,
              provider: "boundary-review",
              sessionSource: state.authSessionSource,
            }),
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
      const sessionSource = state.authBoundaryReviewed
        ? state.authSessionSource || "boundary-review"
        : "missing-config";
      setBetterAuthRuntimeState(state.authBoundaryReviewed ? "previewed" : "idle", {
        email,
        sessionSource,
      });
      if (emailInput && "value" in emailInput) emailInput.value = email;
      if (missionEmailInput && "value" in missionEmailInput) missionEmailInput.value = email;
      status.setAttribute(
        "data-dx-auth-boundary-review-state",
        state.authBoundaryReviewed ? "safe-action-preview" : "idle",
      );
      status.dataset.state = state.authBoundaryReviewed ? "ok" : "ready";
      status.textContent = state.authBoundaryReviewed
        ? `Sign-in readiness reviewed for ${email}. No secret left the browser.`
        : "Sign-in readiness idle. The Authentication adapter is mounted, and credentials stay app-owned.";
      if (localUser) {
        localUser.setAttribute(
          "data-dx-auth-boundary-review-user",
          state.authBoundaryReviewed ? email : "none",
        );
        localUser.textContent = state.authBoundaryReviewed
          ? `Local sign-in review: ${email}`
          : "No local sign-in review marker.";
      }
      if (localAccount) {
        localAccount.setAttribute("data-dx-auth-boundary-review-account", "none");
        localAccount.textContent =
          "No account has been created. Email sign-up remains a missing-config Authentication contract in this runtime.";
      }
      if (missionLocalPreview) {
        missionLocalPreview.setAttribute("data-dx-auth-session-source", "boundary-review");
        missionLocalPreview.setAttribute(
          "data-dx-auth-boundary-review",
          state.authBoundaryReviewed && sessionSource === "boundary-review" ? "active" : "idle",
        );
        missionLocalPreview.textContent =
          state.authBoundaryReviewed && sessionSource === "boundary-review"
            ? `Sign-in readiness reviewed for ${email}. No cookie, server session, or account was created.`
            : "Sign-in readiness idle. No cookie, server session, or account is created until the app owns Authentication config.";
      }
      updateMissionControl("Session state updated in the launch dashboard.");
    };

    signUp.addEventListener("click", () => {
      const email = currentEmail().toLowerCase();
      state.authEmail = email;
      state.authSessionSource = "missing-config";
      render();
      setBetterAuthRuntimeState("email-sign-up-missing-config", {
        email,
        sessionSource: "missing-config",
        localSession: "not-created",
        boundaryReview: "idle",
        boundaryEmail: email,
      });
      emailSignUpButtons.forEach((button) => {
        button.setAttribute("data-dx-auth-email-sign-up-state", "missing-config");
      });
      status.dataset.state = "ok";
      status.textContent = `Prepared Authentication email sign-up for ${email}; no account was created because the runtime bridge has no app-owned Authentication server config.`;
      setAuthReceipt(
        `better-auth-dashboard-email-sign-up-${email.replace(/[^a-z0-9]+/g, "-")}`,
        `Email sign-up is correctly blocked until BETTER_AUTH_SECRET, BETTER_AUTH_URL, database adapter, and email policy are app-owned.`,
        "email-sign-up-missing-config",
      );
      setText(
        "#session-detail",
        "signUp.email preview only: app-owned Authentication route, database adapter, and email delivery are required before creating an account.",
      );
    });

    function markBoundaryReviewed() {
      const email = currentEmail().toLowerCase();
      state.authBoundaryReviewed = true;
      state.authEmail = email;
      state.authSessionSource = "boundary-review";
      writeStoredSession();
      render();
      setBetterAuthRuntimeState("boundary-review-marked", {
        email,
        sessionSource: "boundary-review",
        localSession: "reviewed",
        boundaryReview: "active",
        boundaryEmail: email,
      });
      localPreviewButtons.forEach((button) => {
        button.setAttribute("data-dx-auth-boundary-review", "active");
      });
      setAuthReceipt(
        `better-auth-dashboard-boundary-review-${email.replace(/[^a-z0-9]+/g, "-")}`,
        `Sign-in readiness reviewed for ${email}. No cookie, server session, or account was created.`,
        "boundary-review-marked",
      );
      setText(
        "#session-detail",
        "Sign-in review is browser-only; production sessions still require Authentication server config.",
      );
      updateMissionControl("Sign-in readiness marked from the account workflow.");
    }

    signIn.addEventListener("click", async () => {
      const email = currentEmail().toLowerCase();
      state.authBoundaryReviewed = true;
      state.authEmail = email;
      state.authSessionSource = "safe-action-preview";
      writeStoredSession();
      render();
      setBetterAuthRuntimeState("preview-session-without-account", {
        email,
        sessionSource: "safe-action-preview",
      });
      setText(
        "#session-boundary-review-account",
        "Preview sign-in works without creating an account. Use Mark reviewed for a browser-only sign-in marker.",
      );
      setAuthReceipt(
        `better-auth-dashboard-read-session-${email.replace(/[^a-z0-9]+/g, "-")}`,
        `Prepared Authentication read-session receipt for ${email}; server session route remains credential-gated.`,
        "safe-action-preview",
      );
      try {
        const response = await fetch("/api/auth/session");
        const payload = await response.json();
        if (card) {
          card.setAttribute(
            "data-dx-auth-config",
            payload.credentialsConfigured ? "configured" : "missing-config",
          );
          setAuthNetworkState(
            payload.credentialsConfigured ? "configured" : "missing-config",
          );
        }
        setBetterAuthRuntimeState("session-route-checked", {
          email,
          sessionSource: "safe-action-preview",
        });
        setText(
          "#session-detail",
          `Session API: ${payload.adapter ?? "better-auth"} ${payload.status ?? "ready"}`,
        );
      } catch {
        if (card) {
          card.setAttribute("data-dx-auth-config", "missing-config");
        }
        setBetterAuthRuntimeState("local-only", {
          email,
          sessionSource: "safe-action-preview",
        });
        setAuthNetworkState("missing-config");
        setText("#session-detail", "Session API unavailable; local adapter boundary still visible.");
      }
    });

    signOut.addEventListener("click", () => {
      state.authBoundaryReviewed = false;
      state.authSessionSource = "missing-config";
      writeStoredSession();
      render();
      setBetterAuthRuntimeState("boundary-review-cleared", {
        sessionSource: "missing-config",
        localSession: "not-created",
        boundaryReview: "idle",
      });
      localPreviewButtons.forEach((button) => {
        button.setAttribute("data-dx-auth-boundary-review", "idle");
      });
      setAuthReceipt(
        "better-auth-dashboard-boundary-review-cleared",
        "Local sign-in readiness review cleared.",
        "boundary-review-cleared",
      );
      setText("#session-detail", "Sign-in review cleared locally.");
    });

    missionPreview?.addEventListener("click", () => {
      signIn.click();
      updateMissionControl("Sign-in account workflow prepared a session receipt.");
    });

    missionSignUp?.addEventListener("click", () => {
      signUp.click();
      updateMissionControl("Sign-in account workflow prepared an email sign-up missing-config receipt.");
    });

    localPreviewButton?.addEventListener("click", markBoundaryReviewed);

    missionLocalSession?.addEventListener("click", () => {
      markBoundaryReviewed();
    });

    missionProviderButtons.forEach((button) => {
      button.addEventListener("click", () => {
        const provider = button.getAttribute("data-dx-auth-provider-option") || "provider";
        if (dashboardCard) {
          dashboardCard.setAttribute("data-dx-auth-provider-readiness", "missing-config");
        }
        setBetterAuthRuntimeState("provider-missing-config");
        button.setAttribute("data-dx-auth-provider-state", "missing-config");
        setAuthReceipt(
          `better-auth-dashboard-link-provider-${provider}-missing-config`,
          `${provider} linking is correctly blocked until app-owned OAuth credentials and redirect policy are configured.`,
          "provider-missing-config",
        );
        updateMissionControl("Authentication provider readiness checked without fake OAuth success.");
      });
    });

    setAuthNetworkState(card?.getAttribute("data-dx-auth-config") || "missing-config");
    readStoredSession();
    render();
  }

  function bindPayment() {
    const card = $('[data-dx-component="launch-billing-checkout-workflow"][data-dx-package="payments/stripe-js"]');
    const form = $("#stripe-checkout-form");
    const status = $("#payment-status");
    const fillSample = $("#payment-fill-sample");
    const planButtons = $$('[data-dx-stripe-action="select-plan"]');
    if (!form || !status) return;
    const paymentSample = {
      email: "billing@dx.local",
      name: "DX Founder",
      message: "Launch readiness checkout contract for Forge package proof.",
    };

    const setPaymentResult = (result, message, receiptId = "none") => {
      if (card) {
        card.dataset.dxStripePostResult = result;
        card.dataset.dxStripeLocalReceipt = receiptId;
      }
      form.dataset.dxStripePostResult = result;
      status.dataset.dxStripePostResult = result;
      status.dataset.state = result === "error" ? "error" : "ok";
      status.textContent = message;
      updateMissionControl("Payment boundary updated in the launch dashboard.");
    };

    planButtons.forEach((button) => {
      button.addEventListener("click", () => {
        const planId = button.getAttribute("data-dx-stripe-plan-id") || "team";
        const priceEnv = button.getAttribute("data-dx-stripe-price-env") || "STRIPE_PRICE_ID_TEAM";
        planButtons.forEach((item) => {
          item.setAttribute(
            "data-dx-stripe-plan-selected",
            item === button ? "true" : "false",
          );
        });
        if (card) {
          card.dataset.dxStripePlanId = planId;
          card.dataset.dxStripePriceEnv = priceEnv;
          card.dataset.dxStripeLocalReceipt = "none";
        }
        setPaymentResult(
          "plan-selected",
          `${planId} plan selected. ${priceEnv} remains app-owned until Stripe credentials are configured.`,
          "none",
        );
      });
    });

    fillSample?.addEventListener("click", () => {
      for (const [name, value] of Object.entries(paymentSample)) {
        const field = form.elements.namedItem(name);
        if (field && "value" in field) field.value = value;
      }
      setPaymentResult(
        "sample-loaded",
        "Loaded a safe checkout contact. Submit it to verify the Stripe-shaped boundary.",
        "sample-contact-ready",
      );
    });

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
              id: card?.dataset.dxStripePlanId || "team",
              priceEnv: card?.dataset.dxStripePriceEnv || "STRIPE_PRICE_ID_TEAM",
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
          `stripe-local-hosted-${email.replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "")}`;

        setPaymentResult(
          response.ok ? "posted" : "readiness",
          response.ok
            ? `Checkout POST accepted: ${payload.status ?? payload.kind ?? "readiness"}.`
            : `Checkout POST returned ${response.status}; local readiness receipt prepared.`,
          receiptId,
        );
      } catch {
        const receiptId = `stripe-local-hosted-${email
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

  function bindSettingsForm() {
    const form = $("#launch-form");
    if (!form) return;
    const card = $('[data-dx-component="zod-form-card"]');
    const issueList = $("#form-issues");
    const fieldErrorsPreview = $("#form-field-errors");
    const settingsSummary = $("#form-settings-summary");
    const payloadPreview = $("#form-payload");
    const missionSummaryCard = $('[data-dx-component="launch-settings-validation-summary"]');
    const missionReceipt = $("#mission-settings-receipt");
    const missionReceiptJson = $("#mission-settings-receipt-json");
    form.dataset.dxRhfBoundary = zodSettingsValidationContract.runtimeBoundary;
    form.dataset.dxZodSchema = zodSettingsValidationContract.schema;
    form.dataset.dxZodPublicApi = zodSettingsValidationContract.publicApi;
    form.dataset.dxZodFieldErrorsApi = zodSettingsValidationContract.fieldErrorsApi;
    form.dataset.dxSourceOwnedApi = zodSettingsValidationContract.sourceOwnedApi;
    if (card) {
      card.dataset.dxRhfBoundary = zodSettingsValidationContract.runtimeBoundary;
      card.dataset.dxZodSchema = zodSettingsValidationContract.schema;
      card.dataset.dxZodPublicApi = zodSettingsValidationContract.publicApi;
      card.dataset.dxZodFieldErrorsApi = zodSettingsValidationContract.fieldErrorsApi;
      card.dataset.dxSourceOwnedApi = zodSettingsValidationContract.sourceOwnedApi;
    }
    const invalidSample = {
      workspaceName: "DX",
      contactEmail: "not-an-email",
      defaultLocale: "en",
      theme: "system",
      previewMode: "preview",
      packageReceiptsRequired: true,
      launchScoreTarget: "42",
    };
    const validSample = {
      workspaceName: "DX Launch Dashboard",
      contactEmail: "operator@example.com",
      defaultLocale: "en",
      theme: "system",
      previewMode: "preview",
      packageReceiptsRequired: true,
      launchScoreTarget: "94",
    };

    function field(name) {
      return form.elements.namedItem(name);
    }

    function renderZodIssues(issues) {
      if (!issueList) return;
      issueList.replaceChildren();
      issueList.dataset.dxZodValidationIssues = JSON.stringify(issues);

      if (!issues.length) {
        const item = document.createElement("li");
        item.dataset.dxZodValidationIssue = "success";
        item.textContent = "safeParse accepted the dashboard settings payload.";
        issueList.append(item);
        return;
      }

      for (const issue of issues) {
        const item = document.createElement("li");
        item.dataset.dxZodValidationIssue = issue.path.join(".");
        item.dataset.dxZodValidationIssueCode = issue.code;
        item.textContent = `${issue.path.join(".")}: ${issue.message}`;
        issueList.append(item);
      }
    }

    function createZodFieldErrors(issues) {
      const flattened = { formErrors: [], fieldErrors: {} };
      for (const issue of issues) {
        const key = issue.path.join(".") || "_form";
        if (key === "_form") {
          flattened.formErrors.push(issue.message);
          continue;
        }
        if (!flattened.fieldErrors[key]) flattened.fieldErrors[key] = [];
        flattened.fieldErrors[key].push(issue.message);
      }
      return flattened;
    }

    function renderZodFieldErrors(issues) {
      const fieldErrors = createZodFieldErrors(issues);
      const fieldErrorsJson = JSON.stringify(fieldErrors.fieldErrors);
      const fieldGroupCount =
        Object.keys(fieldErrors.fieldErrors).length + fieldErrors.formErrors.length;
      form.dataset.dxZodFieldErrors = fieldErrorsJson;
      form.dataset.dxZodFieldErrorsApi = zodSettingsValidationContract.fieldErrorsApi;
      form.dataset.dxZodFieldErrorCount = String(fieldGroupCount);
      if (card) {
        card.dataset.dxZodFieldErrors = fieldErrorsJson;
        card.dataset.dxZodFieldErrorsApi = zodSettingsValidationContract.fieldErrorsApi;
        card.dataset.dxZodFieldErrorCount = String(fieldGroupCount);
      }
      if (fieldErrorsPreview) {
        fieldErrorsPreview.dataset.dxZodFieldErrors = fieldErrorsJson;
        fieldErrorsPreview.dataset.dxZodFieldErrorsApi =
          zodSettingsValidationContract.fieldErrorsApi;
        fieldErrorsPreview.dataset.dxZodValidationFieldErrors = issues.length ? "rejected" : "accepted";
        fieldErrorsPreview.textContent = JSON.stringify(fieldErrors, null, 2);
      }
      state.zodSettingsFieldErrorCount = fieldGroupCount;
      return fieldErrors;
    }

    function renderZodSettingsSummary(payload, result, fieldErrors) {
      if (!settingsSummary) return;
      if (result.success) {
        settingsSummary.dataset.state = "ok";
        settingsSummary.dataset.dxZodSettingsSummary = "valid";
        settingsSummary.dataset.dxZodSettingsWorkspace = payload.workspaceName;
        settingsSummary.dataset.dxZodSettingsScoreTarget = String(payload.launchScoreTarget);
        settingsSummary.textContent = `${payload.workspaceName} uses ${payload.theme} theme, ${payload.defaultLocale} locale, ${payload.previewMode} preview, score target ${payload.launchScoreTarget}, and ${
          payload.packageReceiptsRequired ? "requires" : "does not require"
        } source receipts.`;
        return;
      }
      const fieldNames = Object.keys(fieldErrors.fieldErrors);
      settingsSummary.dataset.state = "error";
      settingsSummary.dataset.dxZodSettingsSummary = "invalid";
      settingsSummary.dataset.dxZodSettingsWorkspace = payload.workspaceName || "missing";
      settingsSummary.dataset.dxZodSettingsScoreTarget = String(payload.launchScoreTarget ?? "invalid");
      settingsSummary.textContent = `${zodSettingsValidationContract.fieldErrorsApi} grouped ${
        fieldNames.length || fieldErrors.formErrors.length
      } settings field error group${fieldNames.length === 1 ? "" : "s"}: ${
        fieldNames.join(", ") || "form"
      }.`;
    }

    function createMissionSettingsReceipt(payload, issues, fieldErrors) {
      const formattedIssues = issues.map((issue) => ({
        path: issue.path.join(".") || "settings",
        code: issue.code,
        message: issue.message,
      }));
      const baseReceipt = {
        api: zodSettingsValidationContract.receiptApi,
        schema: zodSettingsValidationContract.schema,
        packageId: zodSettingsValidationContract.packageId,
        sourceOwnedApi: zodSettingsValidationContract.sourceOwnedApi,
      };

      if (!issues.length) {
        return {
          ...baseReceipt,
          status: "accepted",
          data: payload,
          issues: [],
          fieldErrors: null,
        };
      }

      return {
        ...baseReceipt,
        status: "blocked",
        data: null,
        issues: formattedIssues,
        fieldErrors,
        input: payload,
      };
    }

    function renderMissionSettingsReceipt(payload, issues, fieldErrors) {
      const receipt = createMissionSettingsReceipt(payload, issues, fieldErrors);
      const receiptState = receipt.status === "accepted" ? "accepted" : "blocked";
      if (missionReceiptJson) {
        missionReceiptJson.dataset.dxZodDashboardReceiptJson = receiptState;
        missionReceiptJson.dataset.dxZodDashboardReceiptState = receiptState;
        missionReceiptJson.dataset.dxZodDashboardReceiptApi =
          zodSettingsValidationContract.receiptApi;
        missionReceiptJson.textContent = JSON.stringify(receipt, null, 2);
      }
      if (missionReceipt) {
        missionReceipt.dataset.dxZodDashboardReceipt = receiptState;
        missionReceipt.dataset.dxZodDashboardReceiptApi =
          zodSettingsValidationContract.receiptApi;
        missionReceipt.textContent =
          receiptState === "accepted"
            ? `${zodSettingsValidationContract.receiptApi} accepted ${payload.workspaceName}.`
            : `${zodSettingsValidationContract.receiptApi} blocked ${issues.length} settings issue${
                issues.length === 1 ? "" : "s"
              }.`;
      }
      if (missionSummaryCard) {
        missionSummaryCard.dataset.dxZodDashboardReceipt = receiptState;
        missionSummaryCard.dataset.dxZodDashboardReceiptApi =
          zodSettingsValidationContract.receiptApi;
      }
      return receipt;
    }

    function renderZodPayload(result) {
      if (!payloadPreview) return;
      payloadPreview.dataset.dxZodValidationPayload = "safe-parse-result";
      if (result.success) {
        payloadPreview.dataset.dxZodValidationOutput = "accepted";
      } else {
        payloadPreview.dataset.dxZodValidationOutput = "rejected";
      }
      payloadPreview.textContent = JSON.stringify(result, null, 2);
    }

    function setZodSample(sample) {
      for (const [name, value] of Object.entries(sample)) {
        const input = field(name);
        if (!input) continue;
        if ("checked" in input && input.type === "checkbox") {
          input.checked = Boolean(value);
          continue;
        }
        if ("value" in input) input.value = value;
      }
      if (typeof form.requestSubmit === "function") {
        form.requestSubmit();
        return;
      }
      form.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
    }

    function bindMissionSettingsShortcuts() {
      const editableFields = {
        workspaceName: $("#mission-settings-workspace"),
        contactEmail: $("#mission-settings-email"),
        launchScoreTarget: $("#mission-settings-score"),
        defaultLocale: $("#mission-settings-locale"),
        theme: $("#mission-settings-theme"),
        previewMode: $("#mission-settings-preview-mode"),
        packageReceiptsRequired: $("#mission-settings-receipts-required"),
      };

      function writeMissionSettingsControls(sample) {
        for (const [name, input] of Object.entries(editableFields)) {
          if (!input || !("value" in input)) continue;
          if ("checked" in input && input.type === "checkbox") {
            input.checked = Boolean(sample[name]);
            input.dataset.dxZodDashboardFieldState = "edited";
            continue;
          }
          input.value = String(sample[name] ?? "");
          input.dataset.dxZodDashboardFieldState = "edited";
        }
        if (missionSummaryCard) {
          missionSummaryCard.dataset.dxZodDashboardEditableSettings = "changed";
        }
      }

      function readMissionSettingsPayload() {
        const workspaceName = editableFields.workspaceName?.value || "";
        const contactEmail = editableFields.contactEmail?.value || "";
        const launchScoreTarget = editableFields.launchScoreTarget?.value || "";
        const defaultLocale = editableFields.defaultLocale?.value || "en";
        const theme = editableFields.theme?.value || "system";
        const previewMode = editableFields.previewMode?.value || "preview";
        const packageReceiptsRequired =
          editableFields.packageReceiptsRequired?.checked ?? true;
        return {
          workspaceName,
          contactEmail,
          defaultLocale,
          theme,
          previewMode,
          packageReceiptsRequired,
          launchScoreTarget,
        };
      }

      function runMissionSettingsShortcut(button, sample, receiptState, label) {
        if (!button) return;
        button.addEventListener("click", () => {
          if (sample) writeMissionSettingsControls(sample);
          const payload = readMissionSettingsPayload();
          button.dataset.dxZodDashboardActionState = "submitted";
          if (missionSummaryCard) {
            missionSummaryCard.dataset.dxZodDashboardActionState = "submitted";
            missionSummaryCard.dataset.dxZodDashboardReceipt = receiptState;
            missionSummaryCard.dataset.dxZodDashboardWorkspace = payload.workspaceName || "missing";
            missionSummaryCard.dataset.dxZodDashboardEmail = payload.contactEmail || "missing";
            missionSummaryCard.dataset.dxZodDashboardScoreTarget = payload.launchScoreTarget || "missing";
            missionSummaryCard.dataset.dxZodDashboardLocale = payload.defaultLocale;
            missionSummaryCard.dataset.dxZodDashboardTheme = payload.theme;
            missionSummaryCard.dataset.dxZodDashboardPreviewMode = payload.previewMode;
            missionSummaryCard.dataset.dxZodDashboardReceiptsRequired = String(
              payload.packageReceiptsRequired,
            );
          }
          if (missionReceipt) {
            missionReceipt.dataset.dxZodDashboardActionState = "submitted";
            missionReceipt.dataset.dxZodDashboardReceipt = receiptState;
            missionReceipt.textContent = label;
          }
          setZodSample(payload);
        });
      }

      runMissionSettingsShortcut(
        $("#mission-settings-show-errors"),
        invalidSample,
        "invalid-sample-submitted",
        `${zodSettingsValidationContract.publicApi} prepared grouped field errors for mission control.`,
      );
      runMissionSettingsShortcut(
        $("#mission-settings-validate"),
        null,
        "editable-settings-submitted",
        `${zodSettingsValidationContract.publicApi} received editable mission-control settings.`,
      );
    }

    bindMissionSettingsShortcuts();
    $("#zod-fill-invalid")?.addEventListener("click", () => setZodSample(invalidSample));
    $("#zod-fill-valid")?.addEventListener("click", () => setZodSample(validSample));

    form.addEventListener("submit", (event) => {
      event.preventDefault();
      const data = new FormData(form);
      const workspaceName = String(data.get("workspaceName") ?? "").trim();
      const contactEmail = String(data.get("contactEmail") ?? "").trim().toLowerCase();
      const defaultLocale = String(data.get("defaultLocale") ?? "en");
      const theme = String(data.get("theme") ?? "system");
      const previewMode = String(data.get("previewMode") ?? "preview");
      const packageReceiptsRequired = data.get("packageReceiptsRequired") === "on";
      const scoreInput = String(data.get("launchScoreTarget") ?? "").trim();
      const launchScoreTarget = Number(scoreInput);
      const payload = {
        workspaceName,
        contactEmail,
        defaultLocale,
        theme,
        previewMode,
        packageReceiptsRequired,
        launchScoreTarget: scoreInput ? launchScoreTarget : null,
      };
      const issues = [];

      if (workspaceName.length < 3) {
        issues.push({
          code: "too_small",
          message: "Workspace name needs at least 3 characters.",
          path: ["workspaceName"],
        });
      }
      if (!/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(contactEmail)) {
        issues.push({
          code: "invalid_string",
          message: "Contact email must be valid.",
          path: ["contactEmail"],
        });
      }
      if (!["en", "bn", "hi"].includes(defaultLocale)) {
        issues.push({
          code: "invalid_enum_value",
          message: "Locale must be en, bn, or hi.",
          path: ["defaultLocale"],
        });
      }
      if (!["system", "light", "dark"].includes(theme)) {
        issues.push({
          code: "invalid_enum_value",
          message: "Theme must be system, light, or dark.",
          path: ["theme"],
        });
      }
      if (!["stable", "preview"].includes(previewMode)) {
        issues.push({
          code: "invalid_enum_value",
          message: "Preview mode must be stable or preview.",
          path: ["previewMode"],
        });
      }
      if (!Number.isInteger(launchScoreTarget) || launchScoreTarget < 70 || launchScoreTarget > 100) {
        issues.push({
          code: Number.isFinite(launchScoreTarget) ? "too_small" : "invalid_type",
          message: "Launch score target must be an integer from 70 to 100.",
          path: ["launchScoreTarget"],
        });
      }

      const output = $("#form-status");
      if (!output) return;
      renderZodIssues(issues);
      const fieldErrors = renderZodFieldErrors(issues);
      const receipt = renderMissionSettingsReceipt(payload, issues, fieldErrors);
      state.zodSettingsIssueCount = issues.length;
      form.dataset.dxZodSettingsIssueCount = String(issues.length);
      if (card) card.dataset.dxZodSettingsIssueCount = String(issues.length);
      output.dataset.dxZodSettingsIssueCount = String(issues.length);
      if (issues.length) {
        const messages = issues.map((issue) => issue.message).join(" ");
        renderZodSettingsSummary(payload, { success: false }, fieldErrors);
        renderZodPayload({ error: { issues, fieldErrors }, input: payload, receipt, success: false });
        if (card) card.dataset.dxZodValidationState = "invalid";
        form.dataset.dxZodValidationState = "invalid";
        output.dataset.dxZodValidationState = "invalid";
        output.dataset.dxZodValidationResult = "error";
        output.dataset.dxZodValidationError = messages;
        output.dataset.state = "error";
        output.textContent = messages;
        state.zodSettingsValidation = "invalid";
        state.zodSettingsEmail = contactEmail;
        state.zodSettingsWorkspaceName = workspaceName;
        state.zodSettingsLaunchScoreTarget = scoreInput || "invalid";
        state.zodSettingsTheme = theme;
        state.zodSettingsLocale = defaultLocale;
        state.zodSettingsPreviewMode = previewMode;
        updateMissionControl("Settings validation updated the launch dashboard.");
        return;
      }

      if (card) card.dataset.dxZodValidationState = "valid";
      renderZodSettingsSummary(payload, { success: true }, fieldErrors);
      renderZodPayload({ data: payload, fieldErrors, receipt, success: true });
      form.dataset.dxZodValidationState = "valid";
      output.dataset.dxZodValidationState = "valid";
      output.dataset.dxZodValidationResult = "success";
      output.dataset.dxZodValidationError = "";
      output.dataset.state = "ok";
      output.textContent = `Validated settings for ${workspaceName} (${contactEmail}). ${zodSettingsValidationContract.publicApi} accepted score target ${launchScoreTarget} with ${state.zodSettingsIssueCount} field errors.`;
      state.zodSettingsValidation = "valid";
      state.zodSettingsEmail = contactEmail;
      state.zodSettingsWorkspaceName = workspaceName;
      state.zodSettingsLaunchScoreTarget = String(launchScoreTarget);
      state.zodSettingsTheme = theme;
      state.zodSettingsLocale = defaultLocale;
      state.zodSettingsPreviewMode = previewMode;
      updateMissionControl("Settings validation updated the launch dashboard.");
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
    const queryCard = $('[data-dx-component="tanstack-query-dashboard-data-workflow"]');
    const queryButton = $("#query-refresh");
    const queryStatus = $("#query-status");
    const queryRuns = $("#query-cache-runs");
    const queryUpdated = $("#query-cache-updated");
    const queryPackageCount = $("#query-package-count");
    const queryRoleCount = $("#query-role-count");
    const queryRequiredEnvCount = $("#query-required-env-count");
    const queryPackageQueue = $("#query-package-queue");
    const laneThreePanel = $('[data-dx-component="lane-three-state-data-panel"]');
    const laneThreeCacheMeter = $("#lane-three-cache-meter");

    const updateLaneThreeQueryCacheAction = (
      actionState,
      lastAction,
      invalidatedCount,
    ) => {
      if (!laneThreePanel) return;
      laneThreePanel.dataset.dxQueryCacheActionState = actionState;
      laneThreePanel.dataset.dxQueryCacheLastAction = lastAction;
      laneThreePanel.dataset.dxQueryCacheInvalidatedCount = String(invalidatedCount);
      if (laneThreeCacheMeter) {
        laneThreeCacheMeter.textContent = `${Math.max(0, 2 - invalidatedCount)}/2`;
        laneThreeCacheMeter.dataset.dxQueryCacheInvalidatedCount =
          String(invalidatedCount);
      }
    };

    const renderQueryPackageQueue = (packages) => {
      if (!queryPackageQueue) return;

      queryPackageQueue.innerHTML = packages
        .map((item) => {
          const statusLabel =
            item.status === "ready"
              ? "ready"
              : `${item.requiredEnvCount} env gates`;

          return [
            `<div data-dx-query-dashboard-row="${escapeHtml(item.packageId)}"`,
            ` data-dx-query-package-boundary-count="${escapeHtml(item.appOwnedBoundaryCount)}"`,
            ` data-dx-query-package-command="${escapeHtml(item.command)}"`,
            ` data-dx-query-package-id="${escapeHtml(item.packageId)}"`,
            ` data-dx-query-package-name="${escapeHtml(item.displayName || item.packageId)}"`,
            ` data-dx-query-package-receipt-count="${escapeHtml(item.receiptCount)}"`,
            ` data-dx-query-package-role="${escapeHtml(item.role)}"`,
            ` data-dx-query-package-status="${escapeHtml(item.status)}">`,
            `<strong>${escapeHtml(item.displayName || item.packageId)}</strong>`,
            `<span>${escapeHtml(item.packageId)} - ${escapeHtml(item.role)}</span>`,
            `<span>${escapeHtml(statusLabel)}</span>`,
            "</div>",
          ].join("");
        })
        .join("");
    };

    const renderQueryDashboardData = (data) => {
      if (queryCard) {
        queryCard.setAttribute("data-dx-query-dashboard-source", data.source);
        queryCard.setAttribute("data-dx-query-dashboard-queue", "package-readiness");
        queryCard.setAttribute(
          "data-dx-query-dashboard-package-count",
          String(data.packageCount),
        );
        queryCard.setAttribute(
          "data-dx-query-dashboard-role-count",
          String(data.roleCount),
        );
        queryCard.setAttribute(
          "data-dx-query-dashboard-required-env-count",
          String(data.requiredEnvCount),
        );
        queryCard.setAttribute(
          "data-dx-query-dashboard-boundary-count",
          String(data.appOwnedBoundaryCount),
        );
        queryCard.setAttribute("data-dx-query-dashboard-updated-at", data.readAt);
      }
      if (queryPackageCount) queryPackageCount.textContent = String(data.packageCount);
      if (queryRoleCount) queryRoleCount.textContent = String(data.roleCount);
      if (queryRequiredEnvCount) {
        queryRequiredEnvCount.textContent = String(data.requiredEnvCount);
      }
      if (queryUpdated) {
        queryUpdated.textContent = data.readAt;
        queryUpdated.setAttribute("data-dx-query-updated-at", data.readAt);
      }
      setText("#mission-query-status", `${data.packageCount} packages`);
      setText(
        "#mission-query-detail",
        `${data.roleCount} roles, ${data.requiredEnvCount} env gates, ${data.appOwnedBoundaryCount} app-owned boundaries from ${data.source}.`,
      );
      renderQueryPackageQueue(data.dashboardPackages || launchQueryDashboardPackages);
    };

    state.queryDashboardData = readLaunchQueryDashboardData();
    renderQueryDashboardData(state.queryDashboardData);
    updateLaneThreeQueryCacheAction("no-cache-action-requested", "none", 0);

    queryButton?.addEventListener("click", async () => {
      state.queryRuns += 1;
      updateLaneThreeQueryCacheAction(
        "source-owned-cache-action-dry-run",
        "refresh-dashboard-data",
        1,
      );
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
      setText("#query-status", `Reading launch dashboard data, request ${state.queryRuns}...`);
      setState("#query-status", "ready");
      try {
        state.queryDashboardData = readLaunchQueryDashboardData();
        if (queryCard) queryCard.setAttribute("data-dx-query-cache-state", "fresh");
        if (queryButton) queryButton.setAttribute("data-dx-query-refresh-state", "success");
        if (queryButton) queryButton.setAttribute("data-dx-query-safe-action-state", "completed");
        if (queryStatus) {
          queryStatus.setAttribute("data-dx-query-result-status", "success");
          queryStatus.setAttribute("data-dx-query-cache-entry", "fresh");
        }
        updateLaneThreeQueryCacheAction(
          "source-owned-cache-action-dry-run",
          "refresh-dashboard-data",
          1,
        );
        renderQueryDashboardData(state.queryDashboardData);
        setText(
          "#query-status",
          `Dashboard data refreshed from ${state.queryDashboardData.source}: ${state.queryDashboardData.packageCount} packages, ${state.queryDashboardData.roleCount} roles, ${state.queryDashboardData.requiredEnvCount} env gates.`,
        );
        setState("#query-status", "ok");
        updateMissionControl("Data Fetching & Cache read model refreshed launch dashboard data.");
      } catch {
        if (queryCard) queryCard.setAttribute("data-dx-query-cache-state", "error");
        if (queryButton) queryButton.setAttribute("data-dx-query-refresh-state", "error");
        if (queryButton) queryButton.setAttribute("data-dx-query-safe-action-state", "runtime-missing");
        if (queryStatus) {
          queryStatus.setAttribute("data-dx-query-result-status", "error");
          queryStatus.setAttribute("data-dx-query-cache-entry", "missing-runtime");
        }
        updateLaneThreeQueryCacheAction("cache-action-error", "refresh-dashboard-data", 0);
        setText("#query-status", "Dashboard data read failed; query boundary reported the runtime miss.");
        setState("#query-status", "error");
      }
    });
    updateCounter();
  }

  function bindTemplateApp() {
    const authBoundaryKey = "dx-template-auth-boundary-review";
    const dashboardStateKey = "dx-template-workspace-state";

    function readJson(key, fallback) {
      try {
        const raw = localStorage.getItem(key);
        return raw ? JSON.parse(raw) : fallback;
      } catch {
        return fallback;
      }
    }

    function writeJson(key, value) {
      try {
        localStorage.setItem(key, JSON.stringify(value));
        return true;
      } catch {
        return false;
      }
    }

    function clearSession() {
      try {
        localStorage.removeItem(authBoundaryKey);
      } catch {
        // Storage can be unavailable in embedded previews.
      }
    }

    function getSession() {
      const session = readJson(authBoundaryKey, null);
      if (!session || typeof session.email !== "string") return null;
      return session;
    }

    function setSession(email) {
      const session = {
        email,
        reviewedAt: new Date().toISOString(),
      };
      writeJson(authBoundaryKey, session);
      return session;
    }

    function renderSession() {
      const session = getSession();
      const reviewed = Boolean(session);
      $$("[data-dx-auth-state]").forEach((node) => {
        node.dataset.dxAuthState = reviewed ? "boundary-reviewed" : "adapter-boundary";
      });
      $$("[data-app-session-email]").forEach((node) => {
        node.textContent = "No hosted session";
      });
      $$("[data-app-auth-label]").forEach((node) => {
        node.textContent = reviewed ? "Sign-in reviewed" : "Sign-in readiness";
      });
      return session;
    }

    function setTemplateFieldError(form, fieldName, message) {
      const error = form?.querySelector(`[data-template-field-error="${fieldName}"]`);
      const input = form?.querySelector(`[name="${fieldName}"]`);
      if (!error) return;
      error.textContent = message || "";
      error.hidden = !message;
      if (message) {
        error.setAttribute("role", "alert");
      } else {
        error.removeAttribute("role");
      }
      if (input) {
        input.setAttribute("aria-invalid", message ? "true" : "false");
      }
    }

    function clearTemplateFieldErrors(form) {
      form?.querySelectorAll("[data-template-field-error]").forEach((error) => {
        error.textContent = "";
        error.hidden = true;
        error.removeAttribute("role");
      });
      form?.querySelectorAll("[aria-invalid]").forEach((input) => {
        input.setAttribute("aria-invalid", "false");
      });
    }

    function validateTemplateForm(form, validators) {
      clearTemplateFieldErrors(form);
      const data = new FormData(form);
      const values = {};
      const issues = [];

      Object.entries(validators).forEach(([fieldName, validator]) => {
        const rawValue = data.get(fieldName);
        const value = typeof rawValue === "string" ? rawValue.trim() : rawValue;
        values[fieldName] = value;
        const message = validator(value);
        if (message) {
          issues.push({ fieldName, message });
          setTemplateFieldError(form, fieldName, message);
        }
      });

      form.dataset.dxFormValidation = issues.length ? "blocked" : "accepted";
      form.dataset.dxRhfBoundary = templateFormsValidationContract.runtimeBoundary;
      return { values, issues };
    }

    function createTemplateFormDryRunReceipt(formId, schemaId, values, redactedFields = []) {
      const fieldNames = Object.keys(values).sort();
      const redacted = Array.from(new Set(redactedFields)).sort();
      const packageId = templateFormsValidationContract.formPackageId;
      const validationPackageId = templateFormsValidationContract.validationPackageId;
      return {
        schema: templateFormsValidationContract.dryRunReceiptSchema,
        packageId,
        validationPackageId,
        formId,
        schemaId,
        receiptId: `${packageId}:${formId}:${schemaId}:${fieldNames.join(",")}`,
        state: "accepted",
        mode: "local-dry-run",
        persistence: "none",
        secretAccess: false,
        fieldNames,
        fieldTypes: Object.fromEntries(
          fieldNames.map((fieldName) => {
            const value = values[fieldName];
            return [fieldName, Array.isArray(value) ? "array" : value === null ? "null" : typeof value];
          }),
        ),
        redactedFields: redacted,
        createdAt: new Date().toISOString(),
      };
    }

    function markTemplateFormDryRunBoundary(form, status, receiptState) {
      [form, status].filter(Boolean).forEach((node) => {
        node.setAttribute("data-dx-form-dry-run-receipt", receiptState);
        node.setAttribute("data-dx-form-submit-mode", "local-dry-run");
        node.setAttribute("data-dx-form-persistence", "none");
        node.setAttribute("data-dx-form-secret-access", "false");
        node.setAttribute("data-dx-form-receipt-schema", templateFormsValidationContract.dryRunReceiptSchema);
      });
    }

    function applyTemplateFormDryRunReceipt(form, status, receipt) {
      markTemplateFormDryRunBoundary(form, status, receipt.state);
      [form, status].filter(Boolean).forEach((node) => {
        node.setAttribute("data-dx-form-receipt-id", receipt.receiptId);
        node.setAttribute("data-dx-form-receipt-fields", receipt.fieldNames.join(" "));
        node.setAttribute("data-dx-form-receipt-redacted-fields", receipt.redactedFields.join(" "));
      });
    }

    function bindLogin() {
      const form = $("#app-login-form");
      if (!form) return;
      const emailInput = $("#app-login-email");
      const passwordInput = $("#app-login-password");
      const status = $("#app-login-status");

      form.addEventListener("submit", (event) => {
        event.preventDefault();
        const { values, issues } = validateTemplateForm(form, {
          email: (value) =>
            /^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(String(value || "").toLowerCase())
              ? ""
              : "Enter a valid email.",
          password: (value) =>
            String(value || "").length >= 6
              ? ""
              : "Password needs at least 6 characters.",
        });
        const email = String(values.email || emailInput?.value || "").trim().toLowerCase();

        if (issues.length) {
          markTemplateFormDryRunBoundary(form, status, "blocked");
          if (status) {
            status.dataset.state = "error";
            status.textContent = issues.map((issue) => issue.message).join(" ");
          }
          return;
        }

        setSession(email);
        form.dataset.dxFormValidation = "accepted";
        if (status) {
          status.dataset.state = "ok";
          status.textContent = "Sign-in readiness reviewed. Opening dashboard.";
        }
        renderSession();
        window.setTimeout(() => {
          window.location.href = "/dashboard";
        }, 120);
      });
    }

    function bindLogout() {
      const button = $("#app-logout-action");
      const status = $("#app-logout-status");
      if (!button) return;
      button.addEventListener("click", () => {
        clearSession();
        renderSession();
        if (status) {
          status.dataset.state = "ok";
          status.textContent = "Sign-in review cleared.";
        }
      });
    }

    function bindDashboardSidebar() {
      const shell = $('[data-dx-component="template-dashboard-page"]');
      const button = $("#dashboard-menu-toggle");
      if (!shell || !button) return;
      button.addEventListener("click", () => {
        const nextOpen = shell.dataset.dxSidebarOpen !== "true";
        shell.dataset.dxSidebarOpen = String(nextOpen);
        button.setAttribute("aria-expanded", String(nextOpen));
      });
      $$(".dashboard-nav a").forEach((link) => {
        link.addEventListener("click", () => {
          shell.dataset.dxSidebarOpen = "false";
          button.setAttribute("aria-expanded", "false");
        });
      });
    }

    function currentDashboardTheme() {
      return document.documentElement.dataset.theme === "light" ? "light" : "dark";
    }

    function applyReactiveStoreMarkers(target, filter, optimisticState, visibleRows) {
      if (!target) return;
      const safeFilter = filter === "ready" || filter === "review" ? filter : "all";
      const safeOptimisticState =
        optimisticState === "queued" || optimisticState === "applied"
          ? optimisticState
          : "idle";
      const safeTheme = currentDashboardTheme();
      target.dataset.dxReactiveStoreRuntime = "source-owned-template-store";
      target.dataset.dxReactiveStoreProviderBoundary = "react-context-template-provider";
      target.dataset.dxReactiveStoreSubscription = "source-owned-subscription-ready";
      target.dataset.dxReactiveStoreSnapshotKey =
        `dashboard:${safeFilter}:${safeOptimisticState}:${safeTheme}`;
      target.dataset.dxReactiveStoreVisibleCount = String(visibleRows);
    }

    function updateReactiveStoreMarkers(filter, optimisticState, visibleRows) {
      const shell = $('[data-dx-component="template-dashboard-page"]');
      const laneThreePanel = $('[data-dx-component="lane-three-state-data-panel"]');
      applyReactiveStoreMarkers(shell, filter, optimisticState, visibleRows);
      applyReactiveStoreMarkers(laneThreePanel, filter, optimisticState, visibleRows);
    }

    function bindDashboardFilters() {
      const buttons = $$("[data-dashboard-filter]");
      const rows = $$("[data-project-state]");
      const shell = $('[data-dx-component="template-dashboard-page"]');
      const laneThreePanel = $('[data-dx-component="lane-three-state-data-panel"]');
      const filterStatus = $("#lane-three-filter");
      const visibleRowsStatus = $("#lane-three-visible-rows");
      if (!buttons.length || !rows.length) return;
      buttons.forEach((button) => {
        button.addEventListener("click", () => {
          const filter = button.dataset.dashboardFilter || "all";
          buttons.forEach((item) => item.setAttribute("aria-pressed", "false"));
          button.setAttribute("aria-pressed", "true");
          let visibleRows = 0;
          rows.forEach((row) => {
            const visible = filter === "all" || row.dataset.projectState === filter;
            row.hidden = !visible;
            if (visible) visibleRows += 1;
          });
          if (shell) shell.dataset.dxDashboardFilterState = filter;
          if (shell) shell.dataset.dxQueryCacheKey = `dx:dashboard:projects:${filter}`;
          if (laneThreePanel) {
            laneThreePanel.dataset.dxDashboardFilterState = filter;
            laneThreePanel.dataset.dxQueryCacheKey = `dx:dashboard:projects:${filter}`;
          }
          updateReactiveStoreMarkers(
            filter,
            shell?.dataset.dxOptimisticUiState || "idle",
            visibleRows,
          );
          if (filterStatus) filterStatus.textContent = filter;
          if (visibleRowsStatus) visibleRowsStatus.textContent = `${visibleRows} visible rows`;
        });
      });
    }

    function hydrateDashboardSettings() {
      const saved = readJson(dashboardStateKey, null);
      if (!saved) return;
      const workspace = $("#dashboard-workspace-name");
      const email = $("#dashboard-contact-email");
      const teamSize = $("#dashboard-team-size");
      if (workspace && typeof saved.workspaceName === "string") workspace.value = saved.workspaceName;
      if (email && typeof saved.contactEmail === "string") email.value = saved.contactEmail;
      if (teamSize && Number.isFinite(saved.teamSize)) teamSize.value = String(saved.teamSize);
      setText("#dashboard-profile-state", "Saved");
      setText(
        "#dashboard-profile-detail",
        `${saved.workspaceName || "Workspace"} keeps ${saved.teamSize || 1} teammates in view.`,
      );
    }

    function bindDashboardSettings() {
      const form = $("#dashboard-settings-form");
      const shell = $('[data-dx-component="template-dashboard-page"]');
      const status = $("#dashboard-settings-status");
      if (!form) return;
      hydrateDashboardSettings();
      form.addEventListener("submit", (event) => {
        event.preventDefault();
        const { values, issues } = validateTemplateForm(form, {
          workspaceName: (value) =>
            String(value || "").length >= 3
              ? ""
              : "Workspace name needs at least 3 characters.",
          contactEmail: (value) =>
            /^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(String(value || "").toLowerCase())
              ? ""
              : "Enter a valid contact email.",
          teamSize: (value) => {
            const teamSize = Number(String(value || "").trim());
            return Number.isInteger(teamSize) && teamSize >= 1 && teamSize <= 99
              ? ""
              : "Team size must be from 1 to 99.";
          },
        });
        const workspaceName = String(values.workspaceName || "").trim();
        const contactEmail = String(values.contactEmail || "").trim().toLowerCase();
        const teamSize = Number(String(values.teamSize || "").trim());

        if (issues.length) {
          if (shell) shell.dataset.dxFormValidation = "blocked";
          if (status) {
            status.dataset.state = "error";
            status.textContent = issues.map((issue) => issue.message).join(" ");
          }
          return;
        }

        const saved = {
          workspaceName,
          contactEmail,
          teamSize,
          savedAt: new Date().toISOString(),
        };
        const receipt = createTemplateFormDryRunReceipt(
          "dashboard-settings-form",
          "templateWorkspaceSettingsSchema",
          { workspaceName, contactEmail, teamSize },
        );
        writeJson(dashboardStateKey, saved);
        form.dataset.dxFormValidation = "accepted";
        applyTemplateFormDryRunReceipt(form, status, receipt);
        if (shell) {
          shell.dataset.dxFormValidation = "accepted";
          shell.dataset.dxStateSaved = saved.savedAt;
        }
        setText("#dashboard-profile-state", "Saved");
        setText("#dashboard-profile-detail", `${workspaceName} keeps ${teamSize} teammates in view.`);
        if (status) {
          status.dataset.state = "ok";
          status.textContent = `Saved ${workspaceName}.`;
        }
      });
    }

    function bindDashboardProfile() {
      const form = $('[data-dx-component="template-profile-form"]');
      const status = form?.querySelector('[data-template-form-status="profile"]');
      if (!form) return;
      form.addEventListener("submit", (event) => {
        event.preventDefault();
        const { values, issues } = validateTemplateForm(form, {
          displayName: (value) =>
            String(value || "").length >= 2
              ? ""
              : "Display name needs at least 2 characters.",
          role: (value) =>
            String(value || "").length >= 2 ? "" : "Role needs at least 2 characters.",
        });
        if (issues.length) {
          markTemplateFormDryRunBoundary(form, status, "blocked");
          if (status) {
            status.dataset.state = "error";
            status.textContent = issues.map((issue) => issue.message).join(" ");
          }
          return;
        }
        setText(
          "#dashboard-profile-detail",
          `${values.displayName} is ready as ${values.role}.`,
        );
        const receipt = createTemplateFormDryRunReceipt(
          "template-profile-form",
          "templateProfileSchema",
          {
            displayName: String(values.displayName || "").trim(),
            role: String(values.role || "").trim(),
          },
        );
        applyTemplateFormDryRunReceipt(form, status, receipt);
        if (status) {
          status.dataset.state = "ok";
          status.textContent = "Profile draft validated locally.";
        }
      });
    }

    function bindBillingContact() {
      const form = $('[data-dx-component="template-billing-contact-form"]');
      const status = form?.querySelector('[data-template-form-status="billing-contact"]');
      if (!form) return;
      form.addEventListener("submit", (event) => {
        event.preventDefault();
        const { values, issues } = validateTemplateForm(form, {
          billingEmail: (value) =>
            /^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(String(value || "").toLowerCase())
              ? ""
              : "Enter a valid billing email.",
          organization: (value) =>
            String(value || "").length >= 2
              ? ""
              : "Organization needs at least 2 characters.",
          plan: (value) =>
            ["starter", "team", "scale"].includes(String(value || ""))
              ? ""
              : "Plan must be starter, team, or scale.",
        });
        if (issues.length) {
          markTemplateFormDryRunBoundary(form, status, "blocked");
          if (status) {
            status.dataset.state = "error";
            status.textContent = issues.map((issue) => issue.message).join(" ");
          }
          return;
        }
        setText(
          "#dashboard-profile-detail",
          `${values.billingEmail} prepared for the ${values.plan} billing flow.`,
        );
        const receipt = createTemplateFormDryRunReceipt(
          "template-billing-contact-form",
          "templateBillingContactSchema",
          {
            billingEmail: String(values.billingEmail || "").trim().toLowerCase(),
            organization: String(values.organization || "").trim(),
            plan: String(values.plan || ""),
          },
          ["billingEmail"],
        );
        applyTemplateFormDryRunReceipt(form, status, receipt);
        if (status) {
          status.dataset.state = "ok";
          status.textContent = "Billing contact validated locally; no Stripe session created.";
        }
      });
    }

    function applyTemplateTheme(theme) {
      const nextTheme = theme === "light" ? "light" : "dark";
      document.documentElement.dataset.theme = nextTheme;
      $$("[data-dx-theme]").forEach((node) => {
        node.dataset.dxTheme = nextTheme;
      });
      $$("[data-theme-toggle]").forEach((button) => {
        const label = button.querySelector("[data-theme-toggle-label]");
        if (label) {
          label.textContent = nextTheme === "dark" ? "Light" : "Dark";
        } else {
          button.textContent = nextTheme === "dark" ? "Light" : "Dark";
        }
        button.setAttribute("aria-pressed", String(nextTheme === "light"));
      });
      try {
        localStorage.setItem("www-template-theme", nextTheme);
      } catch {
        // Theme still applies for the current preview session.
      }
      const shell = $('[data-dx-component="template-dashboard-page"]');
      updateReactiveStoreMarkers(
        shell?.dataset.dxDashboardFilterState || "all",
        shell?.dataset.dxOptimisticUiState || "idle",
        $$("[data-project-state]").filter((row) => !row.hidden).length ||
          $$("[data-project-state]").length,
      );
    }

    function bindTemplateTheme() {
      let initialTheme = "dark";
      try {
        initialTheme = localStorage.getItem("www-template-theme") === "light" ? "light" : "dark";
      } catch {
        initialTheme = "dark";
      }
      applyTemplateTheme(initialTheme);
      $$("[data-theme-toggle]").forEach((button) => {
        button.addEventListener("click", () => {
          applyTemplateTheme(document.documentElement.dataset.theme === "light" ? "dark" : "light");
        });
      });
    }

    function nextTemplateModuleStatus(moduleId, current) {
      switch (moduleId) {
        case "ui-density":
          return current === "Comfortable" ? "Compact" : "Comfortable";
        case "state-counter":
          return `${Number(current.match(/\d+/)?.[0] || "0") + 1} updates`;
        case "query-refresh":
          return "Cache dry-run receipt updated";
        case "lane7-motion-stage":
          return current === "Source-owned motion stage ready"
            ? "Local motion stage advanced"
            : "Source-owned motion stage ready";
        case "validation-settings":
          return current === "19 files locked" ? "Schema receipt ready" : "19 files locked";
        case "form-shell":
          return current === "6 files locked" ? "Form receipt ready" : "6 files locked";
        default:
          return "Updated";
      }
    }

    function bindTemplateModules() {
      const shell = $('[data-dx-component="template-dashboard-page"]');
      const moduleCards = $$("[data-template-module]");
      const buttons = $$("[data-template-module-action]");
      if (!buttons.length) return;

      buttons.forEach((button) => {
        button.addEventListener("click", () => {
          const moduleId = button.dataset.templateModuleAction;
          const card = moduleCards.find((item) => item.dataset.templateModule === moduleId);
          const output = card?.querySelector("[data-template-module-status]");
          const current = output?.textContent || "Idle";
          const next = nextTemplateModuleStatus(moduleId, current);
          if (output) output.textContent = next;
          moduleCards.forEach((item) => {
            item.dataset.moduleActive = String(item === card);
          });
          if (moduleId === "ui-density" && shell) {
            shell.dataset.dxUiDensity = next === "Compact" ? "compact" : "comfortable";
          }
          if (moduleId === "query-refresh") {
            const laneThreePanel = $('[data-dx-component="lane-three-state-data-panel"]');
            const optimisticStatus = $("#lane-three-optimistic");
            const queryStatus = $("#lane-three-query-label");
            const cacheMeter = $("#lane-three-cache-meter");
            const currentOptimisticState = shell?.dataset.dxOptimisticUiState || "idle";
            const nextOptimisticState = currentOptimisticState === "queued" ? "applied" : "queued";
            const currentFilter = shell?.dataset.dxDashboardFilterState || "all";
            const visibleRows = $$("[data-project-state]").filter((row) => !row.hidden).length;
            const action = nextOptimisticState === "queued" ? "invalidate" : "refresh";
            const cacheEntryCount = 2;
            const readyEntryCount = action === "invalidate" ? 1 : 2;
            const invalidatedEntryCount = action === "invalidate" ? 1 : 0;
            const optimisticEntryCount = action === "invalidate" ? 1 : 0;
            const queryKey = `dx:dashboard:projects:${currentFilter}`;
            const applyQueryCacheActionMarkers = (target) => {
              if (!target) return;
              target.dataset.dxOptimisticUiState = nextOptimisticState;
              target.dataset.dxQueryCacheStatus = "source-owned-cache-readiness";
              target.dataset.dxQueryCacheRuntime = "source-owned-template-cache";
              target.dataset.dxQueryCacheKey = queryKey;
              target.dataset.dxQueryCacheActionState = "source-owned-cache-action-dry-run";
              target.dataset.dxQueryCacheLastAction = action;
              target.dataset.dxQueryCacheReadyCount = String(readyEntryCount);
              target.dataset.dxQueryCacheInvalidatedCount = String(invalidatedEntryCount);
              target.dataset.dxQueryCacheOptimisticCount = String(optimisticEntryCount);
              target.dataset.dxTanstackQueryRuntimeBoundary = "queryclient-adapter-required";
            };
            applyQueryCacheActionMarkers(shell);
            applyQueryCacheActionMarkers(laneThreePanel);
            updateReactiveStoreMarkers(
              currentFilter,
              nextOptimisticState,
              visibleRows || $$("[data-project-state]").length,
            );
            if (optimisticStatus) {
              optimisticStatus.textContent =
                action === "refresh"
                  ? "Dry-run cache refresh applied"
                  : "Dry-run cache invalidation queued";
            }
            if (queryStatus) {
              queryStatus.textContent =
                action === "refresh"
                  ? "Local cache receipt refreshed"
                  : "Local cache invalidation dry-run";
            }
            if (cacheMeter) {
              cacheMeter.textContent = `${readyEntryCount}/${cacheEntryCount}`;
            }
          }
          if (moduleId === "lane7-motion-stage" && card) {
            card.dataset.dxMotionState =
              next === "Local motion stage advanced" ? "stage-advanced" : "source-owned";
          }
        });
      });
    }

    renderSession();
    bindLogin();
    bindLogout();
    bindDashboardSidebar();
    bindDashboardFilters();
    bindDashboardSettings();
    bindDashboardProfile();
    bindBillingContact();
    bindTemplateTheme();
    bindTemplateModules();
  }

  function setMotionMoveAvailability(button, available) {
    if (!button) return;

    const availability = available ? "true" : "false";
    button.dataset.dxMotionOrderAvailable = availability;
    button.setAttribute("data-dx-motion-order-available", availability);
    button.disabled = !available;
    button.setAttribute("aria-disabled", available ? "false" : "true");
  }

  function setMotionKeyboardReorderState(stageList, activeStage) {
    if (!stageList || !activeStage) return;

    stageList.dataset.dxMotionKeyboardReorder = "arrow-home-end";
    stageList.dataset.dxMotionKeyboardState = activeStage.id;
    stageList.setAttribute("data-dx-motion-keyboard-reorder", "arrow-home-end");
    stageList.setAttribute("data-dx-motion-keyboard-state", activeStage.id);
    stageList.setAttribute("aria-activedescendant", `motion-stage-${activeStage.id}`);
  }

  function renderMotionProof() {
    const card = $("#motion-proof");
    const status = $("#motion-status");
    const policyStatus = $("#motion-policy-status");
    const progressBar = $("[data-dx-motion-progress-bar]");
    const stageList = $("#motion-stage-list");
    const reducedButton = $("[data-dx-motion-interaction='toggle-reduced-motion']");
    const movePreviousButton = $("[data-dx-motion-interaction='move-stage-previous']");
    const moveNextButton = $("[data-dx-motion-interaction='move-stage-next']");
    const activeStage = motionStages[state.motionStageIndex] || motionStages[0];

    if (!card || !status || !progressBar || !stageList || !activeStage) return;

    const motionReduced = state.motionReduced ? "preview" : "system";
    card.dataset.dxMotionState = activeStage.id;
    card.dataset.dxMotionProgress = String(activeStage.progress);
    card.dataset.dxMotionOrder = state.motionOrder.join(",");
    card.dataset.dxMotionPreferenceStorage = "local-storage";
    card.dataset.dxMotionStorageKey = motionPreferenceStorageKey;
    card.dataset.dxMotionPolicy = "app-owned-reduced-motion-preview";
    card.dataset.dxMotionReduced = motionReduced;
    card.classList.toggle("is-active", activeStage.progress >= 67);
    card.classList.toggle("is-reduced", state.motionReduced);
    progressBar.style.width = `${activeStage.progress}%`;
    status.dataset.state = activeStage.progress === 100 ? "ok" : "ready";
    status.textContent = `${activeStage.label}: ${activeStage.progress}% complete. ${activeStage.detail}`;
    if (policyStatus) {
      policyStatus.dataset.dxMotionPolicyStatus = motionReduced;
      policyStatus.dataset.state = state.motionReduced ? "ok" : "ready";
      policyStatus.textContent = state.motionReduced
        ? "Reduced motion preview is active; transforms are suppressed while state markers still update."
        : "Motion policy follows the system preference until the app overrides it.";
    }
    if (reducedButton) {
      reducedButton.dataset.dxMotionReduced = motionReduced;
      reducedButton.setAttribute("aria-pressed", state.motionReduced ? "true" : "false");
      reducedButton.textContent = state.motionReduced
        ? "Use system motion"
        : "Preview reduced motion";
    }

    const activeOrderIndex = state.motionOrder.indexOf(activeStage.id);
    setMotionMoveAvailability(movePreviousButton, activeOrderIndex > 0);
    setMotionMoveAvailability(
      moveNextButton,
      activeOrderIndex >= 0 && activeOrderIndex < state.motionOrder.length - 1,
    );
    setMotionKeyboardReorderState(stageList, activeStage);

    state.motionOrder.forEach((stageId) => {
      const item = stageList.querySelector(`[data-dx-motion-stage="${stageId}"]`);
      if (!item) return;
      const isCurrent = stageId === activeStage.id;
      item.classList.toggle("is-current", isCurrent);
      item.dataset.dxMotionStageActive = isCurrent ? "true" : "false";
      item.dataset.dxMotionReduced = motionReduced;
      item.setAttribute("aria-selected", isCurrent ? "true" : "false");
      stageList.appendChild(item);
    });
    updateMissionControl("Motion & Animation dashboard updated.");
  }

  function placeMotionStage(target) {
    const activeStage = motionStages[state.motionStageIndex] || motionStages[0];
    if (!activeStage) return;

    const currentIndex = state.motionOrder.indexOf(activeStage.id);
    if (currentIndex < 0) return;

    const nextIndex = {
      first: 0,
      last: state.motionOrder.length - 1,
      previous: Math.max(0, currentIndex - 1),
      next: Math.min(state.motionOrder.length - 1, currentIndex + 1),
    }[target];

    if (nextIndex === currentIndex) return;

    const nextOrder = [...state.motionOrder];
    const [stageId] = nextOrder.splice(currentIndex, 1);
    nextOrder.splice(nextIndex, 0, stageId);
    state.motionOrder = nextOrder;
    writeMotionPreference();
    renderMotionProof();
  }

  function moveMotionStage(direction) {
    placeMotionStage(direction);
  }

  function bindMotion() {
    const advance = document.querySelector("[data-dx-motion-interaction='advance-stage']");
    const reorder = document.querySelector("[data-dx-motion-interaction='reverse-order']");
    const movePrevious = document.querySelector("[data-dx-motion-interaction='move-stage-previous']");
    const moveNext = document.querySelector("[data-dx-motion-interaction='move-stage-next']");
    const reset = document.querySelector("[data-dx-motion-interaction='reset-proof']");
    const reduced = document.querySelector("[data-dx-motion-interaction='toggle-reduced-motion']");
    const stageList =
      document.querySelector("#motion-stage-list") ||
      document.querySelector("[data-dx-motion-keyboard-reorder='arrow-home-end']");

    advance?.addEventListener("click", () => {
      state.motionStageIndex = (state.motionStageIndex + 1) % motionStages.length;
      state.motionActive = true;
      renderMotionProof();
    });

    reorder?.addEventListener("click", () => {
      state.motionOrder = [...state.motionOrder].reverse();
      writeMotionPreference();
      renderMotionProof();
    });

    movePrevious?.addEventListener("click", () => moveMotionStage("previous"));
    moveNext?.addEventListener("click", () => moveMotionStage("next"));
    stageList?.addEventListener("keydown", (event) => {
      const target = {
        ArrowLeft: "previous",
        ArrowUp: "previous",
        ArrowRight: "next",
        ArrowDown: "next",
        Home: "first",
        End: "last",
      }[event.key];

      if (!target) return;
      event.preventDefault();
      placeMotionStage(target);
    });

    reset?.addEventListener("click", () => {
      state.motionStageIndex = 0;
      state.motionOrder = motionStages.map((stage) => stage.id);
      state.motionActive = false;
      state.motionReduced = false;
      writeMotionPreference();
      renderMotionProof();
    });

    reduced?.addEventListener("click", () => {
      state.motionReduced = !state.motionReduced;
      writeMotionPreference();
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
        receipt.textContent = "Select a docs route, then preview the local Documentation System route receipt.";
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
      updateMissionControl("Documentation System route updated the launch dashboard.");
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
      updateMissionControl("Documentation System route receipt previewed in mission control.");
    });

    render(state.docsPage);
  }

  function bindAutomations() {
    const status = $("#automation-readiness");
    const receipt = $("#automation-receipt");
    const buttons = Array.from(document.querySelectorAll("[data-dx-automation-connector]"));
    const filters = Array.from(document.querySelectorAll("[data-dx-automation-filter]"));
    const intentInput = $("#automation-intent");
    const workflow = $('[data-dx-component="launch-automation-dashboard-workflow"]');
    const panel = $('[data-dx-component="launch-automation-connector-workflow"]');
    const schemaSummary = $("#automation-schema-summary");
    const receiptOutput = $("#automation-receipt-output");
    const zedRun = $("#automation-zed-run-handoff");
    const zedOutput = $("#automation-zed-output");
    const zedRunReceiptPath = "G:/Dx/.dx/receipts/automations/run-latest.json";
    if (!status || !receipt || !buttons.length) return;

    const workflowIntent = () =>
      intentInput && "value" in intentInput && intentInput.value.trim()
        ? intentInput.value.trim()
        : "Notify the launch channel when a release receipt is ready for Friday review.";
    const resetZedHandoff = (message) => {
      if (zedRun) zedRun.dataset.dxAutomationSafeActionState = "idle";
      if (zedOutput) {
        zedOutput.dataset.dxAutomationZedRunState = "idle";
        zedOutput.dataset.dxAutomationRunReceiptPath = zedRunReceiptPath;
        zedOutput.dataset.dxAutomationRunReceiptIntent = workflowIntent();
        zedOutput.textContent = message;
      }
      if (workflow) workflow.dataset.dxDashboardAutomationRunReceipt = "idle";
    };
    const render = (button) => {
      const connector = button.dataset.dxAutomationConnector || "n8n-nodes-base.manualTrigger";
      const readiness = button.dataset.dxAutomationConnectorStatus || "metadata-ready";
      const credentials = button.dataset.dxAutomationCredentials || "";
      const authKinds = button.dataset.dxAutomationAuthKinds || "none";
      const credentialTypes = button.dataset.dxAutomationCredentialTypes || credentials;
      const requiredEnv =
        button.dataset.dxAutomationRequiredEnv || "DX_AUTOMATIONS_OPERATOR_APPROVAL";
      const workflowMode = button.dataset.dxAutomationWorkflowNodeReadiness || readiness;
      const usableAsTool = button.dataset.dxAutomationUsableAsTool || "false";
      const triggerNode = button.dataset.dxAutomationTriggerNode || "false";
      state.automationConnector = connector;
      if (workflow) {
        workflow.dataset.dxDashboardAutomationWorkflow = connector;
        workflow.dataset.dxDashboardAutomationStatus = readiness;
        workflow.dataset.dxDashboardAutomationNodeReadiness = workflowMode;
        workflow.dataset.dxDashboardAutomationRequiredEnv = requiredEnv;
      }
      if (panel) {
        panel.dataset.dxAutomationReceiptPath =
          panel.dataset.dxAutomationReceiptPath ||
          "G:/Dx/.dx/receipts/automations/launch-release-notification.json";
        panel.dataset.dxAutomationRequiredEnv = requiredEnv;
      }
      status.dataset.dxAutomationSelectedConnector = connector;
      status.dataset.dxAutomationWorkflowStatus = readiness;
      status.dataset.dxAutomationReadinessCard = readiness;
      status.dataset.dxAutomationMissingConfig = readiness === "missing-config" ? "true" : "false";
      status.dataset.dxAutomationRequiredEnv = requiredEnv;
      status.dataset.state = readiness === "missing-config" ? "error" : "ready";
      status.textContent =
        readiness === "missing-config"
          ? `${connector} needs app-owned credentials: ${credentials}. Local receipt preview stays safe.`
          : `${connector} is metadata-ready. No external credentials are used.`;
      if (schemaSummary) {
        schemaSummary.dataset.dxAutomationCredentialSchema = connector;
        schemaSummary.dataset.dxAutomationAuthKinds = authKinds || "none";
        schemaSummary.dataset.dxAutomationCredentialTypes = credentialTypes;
        schemaSummary.dataset.dxAutomationRequiredEnv = requiredEnv;
        schemaSummary.dataset.dxAutomationWorkflowNodeReadiness = workflowMode;
        schemaSummary.dataset.dxAutomationUsableAsTool = usableAsTool;
        schemaSummary.dataset.dxAutomationTriggerNode = triggerNode;
        schemaSummary.textContent = `Credential schema: ${authKinds || "none"}; credentials: ${credentialTypes || "none"}; required env: ${requiredEnv}; workflow node: ${workflowMode}; usable as tool: ${usableAsTool}.`;
      }
      buttons.forEach((candidate) => {
        candidate.dataset.dxAutomationSelected =
        candidate === button ? "true" : "false";
      });
      updateMissionControl("Automation connector updated in the launch dashboard.");
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
      receipt.dataset.dxAutomationSafeActionState = "idle";
      if (receiptOutput) {
        receiptOutput.dataset.dxAutomationDraftState = "idle";
        receiptOutput.dataset.dxAutomationReceiptIntent = workflowIntent();
        receiptOutput.textContent = `Showing ${filterValue} n8n connectors. No local receipt drafted yet.`;
      }
      resetZedHandoff("Connector filter changed. Zed run handoff is idle.");
    };

    buttons.forEach((button) => {
      button.addEventListener("click", () => {
            render(button);
            status.dataset.dxAutomationReceiptState = "idle";
            status.dataset.dxAutomationReceiptStatus = "idle";
            receipt.dataset.dxAutomationSafeActionState = "idle";
            if (receiptOutput) {
              receiptOutput.dataset.dxAutomationDraftState = "idle";
              receiptOutput.dataset.dxAutomationReceiptIntent = workflowIntent();
              receiptOutput.textContent = "Connector selected. Receipt preview is ready to draft.";
        }
            resetZedHandoff("Connector selected. Prepare a Zed run handoff when ready.");
      });
    });
    filters.forEach((filter) => {
      filter.addEventListener("click", () => applyFilter(filter));
    });
    if (intentInput) {
      intentInput.addEventListener("input", () => {
        status.dataset.dxAutomationReceiptState = "idle";
        status.dataset.dxAutomationReceiptStatus = "idle";
        receipt.dataset.dxAutomationSafeActionState = "idle";
        if (workflow) workflow.dataset.dxDashboardAutomationIntent = workflowIntent();
        if (receiptOutput) {
          receiptOutput.dataset.dxAutomationDraftState = "idle";
          receiptOutput.dataset.dxAutomationReceiptIntent = workflowIntent();
          receiptOutput.textContent = "Release intent changed. Prepare a new redacted receipt.";
        }
        resetZedHandoff("Release intent changed. Prepare a fresh Zed run handoff.");
      });
    }

    receipt.addEventListener("click", () => {
      state.automationReceiptRuns += 1;
      status.dataset.dxAutomationReceiptState = "created";
      status.dataset.dxAutomationReceiptStatus = status.dataset.dxAutomationWorkflowStatus || "metadata-ready";
      receipt.dataset.dxAutomationSafeActionState = "created";
      status.dataset.state =
        status.dataset.dxAutomationWorkflowStatus === "missing-config" ? "error" : "ok";
      status.textContent = `Draft workflow receipt ${state.automationReceiptRuns} prepared for ${state.automationConnector}; live execution remains credential-gated.`;
      if (receiptOutput) {
        receiptOutput.dataset.dxAutomationDraftState = "created";
        receiptOutput.dataset.dxAutomationReceiptIntent = workflowIntent();
        receiptOutput.dataset.dxAutomationReceiptStatus = status.dataset.dxAutomationReceiptStatus;
        receiptOutput.textContent = `Local draft ${state.automationReceiptRuns}: ${state.automationConnector} -> ${status.dataset.dxAutomationReceiptStatus}. Intent: ${workflowIntent()}`;
      }
      if (workflow) {
        workflow.dataset.dxDashboardAutomationWorkflow = state.automationConnector;
        workflow.dataset.dxDashboardAutomationReceipt = "created";
        workflow.dataset.dxDashboardAutomationIntent = workflowIntent();
      }
      updateMissionControl("Automation receipt updated in the launch dashboard.");
    });

    if (zedRun) {
      zedRun.addEventListener("click", () => {
        zedRun.dataset.dxAutomationSafeActionState = "created";
        if (zedOutput) {
          zedOutput.dataset.dxAutomationZedRunState = "created";
          zedOutput.dataset.dxAutomationRunReceiptPath = zedRunReceiptPath;
          zedOutput.dataset.dxAutomationRunReceiptIntent = workflowIntent();
          zedOutput.textContent = `Zed handoff ready: dx automations run --json writes ${zedRunReceiptPath} for ${state.automationConnector}. Intent: ${workflowIntent()}. Live execution remains credential-gated until operator approval.`;
        }
        if (workflow) {
          workflow.dataset.dxDashboardAutomationWorkflow = state.automationConnector;
          workflow.dataset.dxDashboardAutomationRunReceipt = "created";
          workflow.dataset.dxDashboardAutomationIntent = workflowIntent();
        }
        updateMissionControl("Automation Zed handoff updated in the launch dashboard.");
      });
    }

    render(buttons[0]);
  }

  function bindScene() {
    const panel = $('[data-dx-component="launch-scene-webgl-proof"]');
    const canvas = $("#dx-launch-scene");
    if (!(canvas instanceof HTMLCanvasElement)) return;
    const sceneSurface = canvas.closest('[data-dx-component="template-landing-scene"]');
    const label = $("#scene-status");
    const controls = $('[data-dx-scene-workflow-controls]');
    const qualityButton = $('[data-dx-scene-action="toggle-quality-profile"]');
    const materialPaletteButton = $('[data-dx-scene-action="cycle-material-palette"]');
    const materialPaletteLabel = $("#scene-material-palette");
    const cameraRigButton = $('[data-dx-scene-action="cycle-camera-rig"]');
    const cameraRigLabel = $("#scene-camera-rig");
    const frameSampleButton = $('[data-dx-scene-action="capture-frame-sample"]');
    const frameSampleLabel = $("#scene-frame-sample");
    const capabilityButton = $('[data-dx-scene-action="inspect-renderer-capabilities"]');
    const capabilityLabel = $("#scene-capability-report");
    const viewportButton = $('[data-dx-scene-action="measure-viewport-dpr"]');
    const viewportLabel = $("#scene-viewport-report");
    const boundsButton = $('[data-dx-scene-action="fit-scene-bounds"]');
    const boundsLabel = $("#scene-bounds-report");
    const raycastButton = $('[data-dx-scene-action="inspect-raycast-hit"]');
    const raycastLabel = $("#scene-raycast-report");
    const activeNodeLabel = $("#scene-active-node");
    const sceneNodes = [
      {
        id: "core-orbit",
        label: "Core orbit",
        workflowId: "visual-runtime-health",
        radius: 0.42,
        orbit: 0.18,
        opacity: 0.86,
      },
      {
        id: "shader-plane",
        label: "Shader plane",
        workflowId: "render-budget",
        radius: 0.72,
        orbit: 0.34,
        opacity: 0.62,
      },
      {
        id: "preview-camera",
        label: "Preview camera",
        workflowId: "web-preview-readiness",
        radius: 1,
        orbit: 0.52,
        opacity: 0.42,
      },
    ];
    const sceneWorkflowProfiles = [
      {
        id: "visual-runtime-health",
        label: "Runtime health",
        detail: "Canvas renderer is connected to dashboard source markers.",
      },
      {
        id: "render-budget",
        label: "Render budget",
        detail: "Performance band and render receipt are ready for Web Preview inspection.",
      },
      {
        id: "web-preview-readiness",
        label: "Web Preview readiness",
        detail: "Camera and editable DOM markers are stable for Zed source mapping.",
      },
    ];
    const sceneQualityProfiles = {
      preview: {
        id: "preview",
        label: "Web Preview",
        motionMultiplier: 1,
        pulseBoost: 1,
      },
      cinematic: {
        id: "cinematic",
        label: "Cinematic",
        motionMultiplier: 1.22,
        pulseBoost: 1.35,
      },
    };
    const sceneMaterialPalettes = {
      aurora: {
        id: "aurora",
        label: "Aurora launch",
        gradientTokens: ["success", "accent", "background"],
        overlayToken: "background",
        overlayAlpha: 0.72,
        textToken: "foreground",
        clear: [0.02, 0.025, 0.04],
        accent: [0.1, 0.95, 0.58],
      },
      graphite: {
        id: "graphite",
        label: "Graphite review",
        gradientTokens: ["accent", "primary", "background"],
        overlayToken: "background",
        overlayAlpha: 0.76,
        textToken: "foreground",
        clear: [0.018, 0.019, 0.024],
        accent: [0.58, 0.84, 1],
      },
    };
    const sceneCameraRigOrder = ["orbit", "isometric", "inspection"];
    const sceneCameraRigs = {
      orbit: {
        id: "orbit",
        label: "Orbit",
        turnOffset: 0,
        pulseMultiplier: 1,
        offsetX: 0,
        offsetY: 0,
        scale: 1,
      },
      isometric: {
        id: "isometric",
        label: "Isometric",
        turnOffset: 0.58,
        pulseMultiplier: 1.14,
        offsetX: 0.05,
        offsetY: -0.06,
        scale: 0.88,
      },
      inspection: {
        id: "inspection",
        label: "Inspection",
        turnOffset: -0.44,
        pulseMultiplier: 0.92,
        offsetX: -0.08,
        offsetY: 0.04,
        scale: 1.18,
      },
    };
    const activeQualityProfile = () =>
      sceneQualityProfiles[state.sceneQualityProfile] || sceneQualityProfiles.preview;
    const activeMaterialPalette = () =>
      sceneMaterialPalettes[state.sceneMaterialPalette] || sceneMaterialPalettes.aurora;
    const activeCameraRig = () =>
      sceneCameraRigs[state.sceneCameraRig] || sceneCameraRigs.orbit;
    let readSceneFrameSample = () =>
      `${state.sceneQualityProfile}-${state.sceneMaterialPalette}-${state.scenePerformanceBand}-waiting`;
    let scenePixelProofFrames = 0;
    const markScenePixelProof = (proof, sample) => {
      canvas.dataset.dxScenePixelProof = proof;
      canvas.dataset.dxScenePixelSample = sample;
      if (panel) {
        panel.setAttribute("data-dx-scene-pixel-proof", proof);
        panel.setAttribute("data-dx-scene-pixel-sample", sample);
      }
    };
    let inspectSceneCapabilities = () => {
      state.sceneCapabilityStatus = "canvas-missing";
      state.sceneCapabilityReport = "canvas-missing";
      return state.sceneCapabilityReport;
    };
    const measureSceneViewport = () => {
      const cssWidth = Math.round(canvas.clientWidth || canvas.width || 0);
      const cssHeight = Math.round(canvas.clientHeight || canvas.height || 0);
      const quality = activeQualityProfile();
      const maxDpr = quality.id === "cinematic" ? 2 : 1.5;
      const budgetDpr =
        state.scenePerformanceBand === "regressed"
          ? 1
          : state.scenePerformanceBand === "recovering"
            ? Math.min(1.5, maxDpr)
            : maxDpr;
      const effectivePixelRatio = Math.round(
        Math.min(Math.max(window.devicePixelRatio || 1, 1), budgetDpr) * 100,
      ) / 100;
      state.sceneViewportStatus = cssWidth > 0 && cssHeight > 0 ? "ready" : "zero-size";
      state.sceneViewportReport = [
        "source-owned-webgl",
        state.sceneQualityProfile,
        state.scenePerformanceBand,
        `${cssWidth}x${cssHeight}`,
        `dpr${Math.round((window.devicePixelRatio || 1) * 100) / 100}`,
        `effectivePixelRatio${effectivePixelRatio}`,
        `buffer${canvas.width}x${canvas.height}`,
      ].join(":");
      return state.sceneViewportReport;
    };
    const sceneNodeHitMap = () =>
      sceneNodes.map((node, index) => {
        const nodeCount = Math.max(sceneNodes.length, 1);
        const phase = (index / nodeCount) * Math.PI * 2;
        const radial = Math.min(0.92, Math.max(0.12, node.radius * 0.34 + node.orbit * 0.64));
        const anchor =
          index === 0
            ? [0, node.radius * 0.32, 0]
            : [Math.cos(phase) * radial, Math.sin(phase) * radial * 0.72, (index - 1) * 0.18];
        const radius = Math.min(0.3, Math.max(0.08, node.radius * 0.18 + node.opacity * 0.06));
        return { anchor, index, node, radius };
      });
    const sceneBoundsBox = () => {
      if (!sceneNodes.length) {
        return {
          center: [0, 0, 0],
          max: [0, 0, 0],
          min: [0, 0, 0],
          nodeCount: 0,
          radius: 0,
          size: [0, 0, 0],
        };
      }
      const anchors = sceneNodeHitMap();
      const min = [
        Math.min(...anchors.map((entry) => entry.anchor[0] - entry.radius)),
        Math.min(...anchors.map((entry) => entry.anchor[1] - entry.radius)),
        Math.min(...anchors.map((entry) => entry.anchor[2] - entry.radius * 0.48)),
      ];
      const max = [
        Math.max(...anchors.map((entry) => entry.anchor[0] + entry.radius)),
        Math.max(...anchors.map((entry) => entry.anchor[1] + entry.radius)),
        Math.max(...anchors.map((entry) => entry.anchor[2] + entry.radius * 0.48)),
      ];
      const size = max.map((value, index) => Math.round((value - min[index]) * 1000) / 1000);
      const center = min.map((value, index) => Math.round((value + (max[index] - value) / 2) * 1000) / 1000);
      return {
        center,
        max: max.map((value) => Math.round(value * 1000) / 1000),
        min: min.map((value) => Math.round(value * 1000) / 1000),
        nodeCount: sceneNodes.length,
        radius: Math.round((Math.hypot(size[0], size[1], size[2]) / 2) * 1000) / 1000,
        size,
      };
    };
    const fitSceneBounds = () => {
      const box = sceneBoundsBox();
      const fitTarget = box.center;
      const cameraDistance = Math.round(Math.max(box.radius * 2.65, 2.8) * 1000) / 1000;
      state.sceneBoundsStatus = box.nodeCount > 0 ? "ready" : "empty-scene";
      state.sceneBoundsReport = [
        "Box3",
        state.sceneBoundsStatus,
        state.sceneCameraRig,
        `nodes${box.nodeCount}`,
        `size${box.size.join("x")}`,
        `fitTarget${fitTarget.join(",")}`,
        `cameraDistance${cameraDistance}`,
      ].join(":");
      return state.sceneBoundsReport;
    };
    const sceneRaycastHit = () => {
      const node =
        state.sceneNodeIndex >= 0
          ? sceneNodes[state.sceneNodeIndex % sceneNodes.length]
          : sceneNodes[0];
      if (!node) {
        state.sceneRaycastStatus = "empty-scene";
        state.sceneRaycastReport = "Raycaster:empty-scene:none:pointer0,0:meshBounds";
        return state.sceneRaycastReport;
      }
      const map = sceneNodeHitMap();
      const selectedIndex = Math.max(0, state.sceneNodeIndex);
      const hit = map[selectedIndex % map.length] || map[0];
      const pointer = hit?.anchor || [0, 0, 0];
      const distance = 0;
      const threshold = Math.round((node.radius * 0.18 + node.opacity * 0.06 + 0.18) * 1000) / 1000;
      state.sceneRaycastStatus = "hit";
      state.sceneRaycastReport = [
        "Raycaster",
        "hit",
        node.id,
        `pointer${pointer.slice(0, 2).join(",")}`,
        `distance${distance}`,
        `threshold${threshold}`,
        "intersectObject",
        "meshBounds",
      ].join(":");
      return state.sceneRaycastReport;
    };
    const activeSceneNode = () =>
      state.sceneNodeIndex >= 0
        ? sceneNodes[state.sceneNodeIndex % sceneNodes.length]
        : null;
    const workflowById = (workflowId) =>
      sceneWorkflowProfiles.find((workflow) => workflow.id === workflowId) ||
      sceneWorkflowProfiles[0];
    const activeWorkflow = (runtimeState) => {
      if (runtimeState === "receipt-prepared" || state.sceneReceiptState === "prepared") {
        return workflowById("render-budget");
      }
      if (runtimeState === "manual-regression" || state.scenePerformanceBand === "regressed") {
        return workflowById("render-budget");
      }
      const node = activeSceneNode();
      return workflowById(node?.workflowId || state.sceneWorkflowId);
    };
    const syncSceneDashboardWorkflow = (runtimeState) => {
      const node = activeSceneNode();
      const nodeId = node?.id || "none";
      const workflow = activeWorkflow(runtimeState);
      const quality = activeQualityProfile();
      const materialPalette = activeMaterialPalette();
      const cameraRig = activeCameraRig();
      const receiptState = state.sceneReceiptState || "idle";
      state.sceneWorkflowId = workflow.id;
      if (panel) {
        panel.setAttribute("data-dx-scene-workflow-active", workflow.id);
        panel.setAttribute("data-dx-scene-workflow-receipt-state", receiptState);
        panel.setAttribute("data-dx-scene-quality-profile", quality.id);
        panel.setAttribute("data-dx-scene-material-palette", materialPalette.id);
        panel.setAttribute("data-dx-scene-camera-rig", cameraRig.id);
        panel.setAttribute("data-dx-scene-frame-sample", state.sceneFrameSample);
        panel.setAttribute("data-dx-scene-capability-report", state.sceneCapabilityReport);
        panel.setAttribute("data-dx-scene-capability-status", state.sceneCapabilityStatus);
        panel.setAttribute("data-dx-scene-viewport-report", state.sceneViewportReport);
        panel.setAttribute("data-dx-scene-viewport-status", state.sceneViewportStatus);
        panel.setAttribute("data-dx-scene-bounds-report", state.sceneBoundsReport);
        panel.setAttribute("data-dx-scene-bounds-status", state.sceneBoundsStatus);
        panel.setAttribute("data-dx-scene-raycast-report", state.sceneRaycastReport);
        panel.setAttribute("data-dx-scene-raycast-status", state.sceneRaycastStatus);
      }
      if (controls) {
        controls.setAttribute("data-dx-scene-workflow-active", workflow.id);
        controls.setAttribute("data-dx-scene-workflow-receipt-state", receiptState);
        controls.setAttribute("data-dx-scene-quality-profile", quality.id);
        controls.setAttribute("data-dx-scene-material-palette", materialPalette.id);
        controls.setAttribute("data-dx-scene-camera-rig", cameraRig.id);
        controls.setAttribute("data-dx-scene-frame-sample", state.sceneFrameSample);
        controls.setAttribute("data-dx-scene-capability-report", state.sceneCapabilityReport);
        controls.setAttribute("data-dx-scene-capability-status", state.sceneCapabilityStatus);
        controls.setAttribute("data-dx-scene-viewport-report", state.sceneViewportReport);
        controls.setAttribute("data-dx-scene-viewport-status", state.sceneViewportStatus);
        controls.setAttribute("data-dx-scene-bounds-report", state.sceneBoundsReport);
        controls.setAttribute("data-dx-scene-bounds-status", state.sceneBoundsStatus);
        controls.setAttribute("data-dx-scene-raycast-report", state.sceneRaycastReport);
        controls.setAttribute("data-dx-scene-raycast-status", state.sceneRaycastStatus);
      }
      const workflowCard = $('[data-dx-scene-dashboard-card="visual-operations"]');
      if (workflowCard) {
        workflowCard.setAttribute("data-dx-scene-quality-profile", quality.id);
        workflowCard.setAttribute("data-dx-scene-material-palette", materialPalette.id);
        workflowCard.setAttribute("data-dx-scene-camera-rig", cameraRig.id);
        workflowCard.setAttribute("data-dx-scene-frame-sample", state.sceneFrameSample);
        workflowCard.setAttribute("data-dx-scene-capability-report", state.sceneCapabilityReport);
        workflowCard.setAttribute("data-dx-scene-capability-status", state.sceneCapabilityStatus);
        workflowCard.setAttribute("data-dx-scene-viewport-report", state.sceneViewportReport);
        workflowCard.setAttribute("data-dx-scene-viewport-status", state.sceneViewportStatus);
        workflowCard.setAttribute("data-dx-scene-bounds-report", state.sceneBoundsReport);
        workflowCard.setAttribute("data-dx-scene-bounds-status", state.sceneBoundsStatus);
        workflowCard.setAttribute("data-dx-scene-raycast-report", state.sceneRaycastReport);
        workflowCard.setAttribute("data-dx-scene-raycast-status", state.sceneRaycastStatus);
      }
      const dashboardCard = $('[data-dx-component="launch-scene-dashboard-workflow"]');
      if (dashboardCard) {
        dashboardCard.setAttribute("data-dx-scene-workflow-active", workflow.id);
        dashboardCard.setAttribute("data-dx-scene-workflow-receipt-state", receiptState);
        dashboardCard.setAttribute("data-dx-scene-dashboard-node", nodeId);
        dashboardCard.setAttribute("data-dx-scene-quality-profile", quality.id);
        dashboardCard.setAttribute("data-dx-scene-material-palette", materialPalette.id);
        dashboardCard.setAttribute("data-dx-scene-camera-rig", cameraRig.id);
        dashboardCard.setAttribute("data-dx-scene-frame-sample", state.sceneFrameSample);
        dashboardCard.setAttribute("data-dx-scene-capability-report", state.sceneCapabilityReport);
        dashboardCard.setAttribute("data-dx-scene-capability-status", state.sceneCapabilityStatus);
        dashboardCard.setAttribute("data-dx-scene-viewport-report", state.sceneViewportReport);
        dashboardCard.setAttribute("data-dx-scene-viewport-status", state.sceneViewportStatus);
        dashboardCard.setAttribute("data-dx-scene-bounds-report", state.sceneBoundsReport);
        dashboardCard.setAttribute("data-dx-scene-bounds-status", state.sceneBoundsStatus);
        dashboardCard.setAttribute("data-dx-scene-raycast-report", state.sceneRaycastReport);
        dashboardCard.setAttribute("data-dx-scene-raycast-status", state.sceneRaycastStatus);
      }
      const workflowStatus = $("#scene-workflow-status");
      if (workflowStatus) {
        workflowStatus.dataset.dxSceneWorkflowActive = workflow.id;
        workflowStatus.textContent = `${workflow.label}: ${workflow.detail} Quality ${quality.label}.`;
      }
      const receiptOutput = $("#scene-receipt-output");
      if (receiptOutput) {
        receiptOutput.dataset.dxSceneDashboardReceipt = receiptState;
        receiptOutput.dataset.dxSceneWorkflowActive = workflow.id;
        receiptOutput.dataset.dxSceneRenderBudgetReceipt = state.sceneReceiptId;
        receiptOutput.dataset.dxSceneQualityProfile = quality.id;
        receiptOutput.textContent =
          receiptState === "prepared"
            ? `${state.sceneReceiptId}: ${workflow.label} receipt prepared for ${node?.label || "runtime scene"} in ${quality.label}.`
            : "Render budget receipt not prepared yet.";
      }
      if (qualityButton) {
        qualityButton.setAttribute("aria-pressed", quality.id === "cinematic" ? "true" : "false");
        qualityButton.textContent = `Quality: ${quality.id}`;
      }
      if (materialPaletteButton) {
        materialPaletteButton.textContent = `Material: ${materialPalette.id}`;
      }
      if (materialPaletteLabel) {
        materialPaletteLabel.dataset.dxSceneMaterialPalette = materialPalette.id;
        materialPaletteLabel.textContent = `Material: ${materialPalette.label}`;
      }
      if (cameraRigButton) {
        cameraRigButton.textContent = `Camera: ${cameraRig.id}`;
      }
      if (cameraRigLabel) {
        cameraRigLabel.dataset.dxSceneCameraRig = cameraRig.id;
        cameraRigLabel.textContent = `Camera rig: ${cameraRig.label}`;
      }
      if (frameSampleLabel) {
        frameSampleLabel.dataset.dxSceneFrameSample = state.sceneFrameSample;
        frameSampleLabel.textContent = `Frame sample: ${state.sceneFrameSample}`;
      }
      if (capabilityLabel) {
        capabilityLabel.dataset.dxSceneCapabilityReport = state.sceneCapabilityReport;
        capabilityLabel.dataset.dxSceneCapabilityStatus = state.sceneCapabilityStatus;
        capabilityLabel.textContent = `Renderer: ${state.sceneCapabilityReport}`;
      }
      if (viewportLabel) {
        viewportLabel.dataset.dxSceneViewportReport = state.sceneViewportReport;
        viewportLabel.dataset.dxSceneViewportStatus = state.sceneViewportStatus;
        viewportLabel.textContent = `Viewport: ${state.sceneViewportReport}`;
      }
      if (boundsLabel) {
        boundsLabel.dataset.dxSceneBoundsReport = state.sceneBoundsReport;
        boundsLabel.dataset.dxSceneBoundsStatus = state.sceneBoundsStatus;
        boundsLabel.textContent = `Bounds: ${state.sceneBoundsReport}`;
      }
      if (raycastLabel) {
        raycastLabel.dataset.dxSceneRaycastReport = state.sceneRaycastReport;
        raycastLabel.dataset.dxSceneRaycastStatus = state.sceneRaycastStatus;
        raycastLabel.textContent = `Raycast: ${state.sceneRaycastReport}`;
      }
      setText("#mission-scene-status", workflow.label);
      setText(
        "#mission-scene-detail",
        receiptState === "prepared"
          ? `${state.sceneReceiptId} prepared for ${node?.label || "runtime scene"} in ${quality.label}, ${materialPalette.label}, ${cameraRig.label}.`
          : `${workflow.detail} ${quality.label} profile; ${materialPalette.label} material; ${cameraRig.label} camera; sample ${state.sceneFrameSample}; capability ${state.sceneCapabilityReport}; viewport ${state.sceneViewportReport}; bounds ${state.sceneBoundsReport}; raycast ${state.sceneRaycastReport}; current renderer state: ${runtimeState}.`,
      );
    };
    const syncSceneProof = (runtimeState) => {
      const node = activeSceneNode();
      const nodeId = node?.id || "none";
      const quality = activeQualityProfile();
      const materialPalette = activeMaterialPalette();
      const cameraRig = activeCameraRig();
      if (panel) {
        panel.setAttribute("data-dx-scene-workflow-selected-node", nodeId);
        panel.setAttribute("data-dx-scene-performance-band", state.scenePerformanceBand);
        panel.setAttribute("data-dx-scene-quality-profile", quality.id);
        panel.setAttribute("data-dx-scene-material-palette", materialPalette.id);
        panel.setAttribute("data-dx-scene-camera-rig", cameraRig.id);
        panel.setAttribute("data-dx-scene-frame-sample", state.sceneFrameSample);
        panel.setAttribute("data-dx-scene-capability-report", state.sceneCapabilityReport);
        panel.setAttribute("data-dx-scene-capability-status", state.sceneCapabilityStatus);
        panel.setAttribute("data-dx-scene-viewport-report", state.sceneViewportReport);
        panel.setAttribute("data-dx-scene-viewport-status", state.sceneViewportStatus);
        panel.setAttribute("data-dx-scene-bounds-report", state.sceneBoundsReport);
        panel.setAttribute("data-dx-scene-bounds-status", state.sceneBoundsStatus);
        panel.setAttribute("data-dx-scene-raycast-report", state.sceneRaycastReport);
        panel.setAttribute("data-dx-scene-raycast-status", state.sceneRaycastStatus);
      }
      if (sceneSurface) {
        sceneSurface.setAttribute("data-dx-scene-active-node", nodeId);
        sceneSurface.setAttribute("data-dx-scene-status", runtimeState);
        sceneSurface.setAttribute("data-dx-scene-performance-band", state.scenePerformanceBand);
        sceneSurface.setAttribute("data-dx-scene-quality-profile", quality.id);
        sceneSurface.setAttribute("data-dx-scene-material-palette", materialPalette.id);
        sceneSurface.setAttribute("data-dx-scene-camera-rig", cameraRig.id);
        sceneSurface.setAttribute("data-dx-scene-frame-sample", state.sceneFrameSample);
      }
      if (controls) {
        controls.setAttribute("data-dx-scene-workflow-selected-node", nodeId);
        controls.setAttribute("data-dx-scene-quality-profile", quality.id);
        controls.setAttribute("data-dx-scene-material-palette", materialPalette.id);
        controls.setAttribute("data-dx-scene-camera-rig", cameraRig.id);
        controls.setAttribute("data-dx-scene-frame-sample", state.sceneFrameSample);
        controls.setAttribute("data-dx-scene-capability-report", state.sceneCapabilityReport);
        controls.setAttribute("data-dx-scene-capability-status", state.sceneCapabilityStatus);
        controls.setAttribute("data-dx-scene-viewport-report", state.sceneViewportReport);
        controls.setAttribute("data-dx-scene-viewport-status", state.sceneViewportStatus);
        controls.setAttribute("data-dx-scene-bounds-report", state.sceneBoundsReport);
        controls.setAttribute("data-dx-scene-bounds-status", state.sceneBoundsStatus);
        controls.setAttribute("data-dx-scene-raycast-report", state.sceneRaycastReport);
        controls.setAttribute("data-dx-scene-raycast-status", state.sceneRaycastStatus);
      }
      canvas.dataset.dxSceneStatus = runtimeState;
      canvas.dataset.dxSceneActiveNode = nodeId;
      canvas.dataset.dxScenePerformanceBand = state.scenePerformanceBand;
      canvas.dataset.dxSceneQualityProfile = quality.id;
      canvas.dataset.dxSceneMaterialPalette = materialPalette.id;
      canvas.dataset.dxSceneCameraRig = cameraRig.id;
      canvas.dataset.dxSceneFrameSample = state.sceneFrameSample;
      canvas.dataset.dxSceneCapabilityReport = state.sceneCapabilityReport;
      canvas.dataset.dxSceneCapabilityStatus = state.sceneCapabilityStatus;
      canvas.dataset.dxSceneViewportReport = state.sceneViewportReport;
      canvas.dataset.dxSceneViewportStatus = state.sceneViewportStatus;
      canvas.dataset.dxSceneBoundsReport = state.sceneBoundsReport;
      canvas.dataset.dxSceneBoundsStatus = state.sceneBoundsStatus;
      canvas.dataset.dxSceneRaycastReport = state.sceneRaycastReport;
      canvas.dataset.dxSceneRaycastStatus = state.sceneRaycastStatus;
      if (activeNodeLabel) activeNodeLabel.textContent = `Active node: ${node?.label || "none"}`;
      if (label) {
        label.dataset.state = runtimeState.includes("fallback") ? "ready" : "ok";
        label.textContent = `${runtimeState}: ${node?.label || "no node selected"}; quality ${quality.label}; material ${materialPalette.label}; camera ${cameraRig.label}; performance ${state.scenePerformanceBand}.`;
      }
      syncSceneDashboardWorkflow(runtimeState);
    };
    const captureSceneFrameSample = () => {
      state.sceneFrameSample = readSceneFrameSample();
      syncSceneProof("frame-sample-captured");
      updateMissionControl("3D scene frame sample updated the launch dashboard.");
    };
    const inspectRendererCapabilities = () => {
      inspectSceneCapabilities();
      syncSceneProof("renderer-capabilities-inspected");
      updateMissionControl("3D renderer capabilities updated the launch dashboard.");
    };
    const measureViewportDpr = () => {
      measureSceneViewport();
      syncSceneProof("viewport-dpr-measured");
      updateMissionControl("3D viewport DPR updated the launch dashboard.");
    };
    const fitSceneBoundsForDashboard = () => {
      fitSceneBounds();
      syncSceneProof("scene-bounds-fit");
      updateMissionControl("3D scene bounds updated the launch dashboard.");
    };
    const inspectSceneRaycastHit = () => {
      sceneRaycastHit();
      syncSceneProof("scene-raycast-inspected");
      updateMissionControl("3D scene raycast hit map updated the launch dashboard.");
    };

    $("#scene-select-node")?.addEventListener("click", () => {
      state.sceneNodeIndex = (state.sceneNodeIndex + 1) % sceneNodes.length;
      state.sceneReceiptId = "none";
      state.sceneReceiptState = "idle";
      syncSceneProof("node-selected");
      updateMissionControl("3D scene focus changed the launch dashboard.");
    });
    $("#scene-regress-performance")?.addEventListener("click", () => {
      state.scenePerformanceBand = "regressed";
      state.sceneReceiptId = "none";
      state.sceneReceiptState = "idle";
      syncSceneProof("manual-regression");
      updateMissionControl("3D render budget moved to the regressed lane.");
    });
    $("#scene-reset-performance")?.addEventListener("click", () => {
      state.sceneNodeIndex = -1;
      state.scenePerformanceBand = "stable";
      state.sceneReceiptId = "none";
      state.sceneReceiptState = "idle";
      syncSceneProof("reset");
      updateMissionControl("3D scene workflow reset to runtime health.");
    });
    $("#scene-apply-render-budget")?.addEventListener("click", () => {
      if (state.sceneNodeIndex < 0) state.sceneNodeIndex = 0;
      const node = activeSceneNode() || sceneNodes[0];
      state.sceneWorkflowId = "render-budget";
      state.sceneReceiptRuns += 1;
      state.sceneReceiptState = "prepared";
      state.sceneReceiptId = `scene-render-budget-${node.id}-${state.scenePerformanceBand}-${state.sceneQualityProfile}-${state.sceneMaterialPalette}-${state.sceneCameraRig}-${state.sceneReceiptRuns}`;
      syncSceneProof("receipt-prepared");
      updateMissionControl("3D render budget receipt updated the launch dashboard.");
    });
    qualityButton?.addEventListener("click", () => {
      state.sceneQualityProfile =
        state.sceneQualityProfile === "preview" ? "cinematic" : "preview";
      state.sceneFrameSample = "pending";
      state.sceneReceiptId = "none";
      state.sceneReceiptState = "idle";
      syncSceneProof("quality-profile-changed");
      updateMissionControl("3D scene quality profile changed the launch dashboard.");
    });
    materialPaletteButton?.addEventListener("click", () => {
      state.sceneMaterialPalette =
        state.sceneMaterialPalette === "aurora" ? "graphite" : "aurora";
      state.sceneFrameSample = "pending";
      state.sceneReceiptId = "none";
      state.sceneReceiptState = "idle";
      syncSceneProof("material-palette-changed");
      updateMissionControl("3D scene material palette changed the launch dashboard.");
    });
    cameraRigButton?.addEventListener("click", () => {
      const currentIndex = sceneCameraRigOrder.indexOf(state.sceneCameraRig);
      state.sceneCameraRig =
        sceneCameraRigOrder[(currentIndex + 1) % sceneCameraRigOrder.length] ||
        "orbit";
      state.sceneFrameSample = "pending";
      state.sceneReceiptId = "none";
      state.sceneReceiptState = "idle";
      syncSceneProof("camera-rig-changed");
      updateMissionControl("3D scene camera rig changed the launch dashboard.");
    });
    frameSampleButton?.addEventListener("click", captureSceneFrameSample);
    capabilityButton?.addEventListener("click", inspectRendererCapabilities);
    viewportButton?.addEventListener("click", measureViewportDpr);
    boundsButton?.addEventListener("click", fitSceneBoundsForDashboard);
    raycastButton?.addEventListener("click", inspectSceneRaycastHit);
    canvas.addEventListener("pointermove", (event) => {
      const rect = canvas.getBoundingClientRect();
      if (!rect.width || !rect.height) return;
      const normalizedX = Math.min(0.999, Math.max(0, (event.clientX - rect.left) / rect.width));
      const nextIndex = Math.min(
        sceneNodes.length - 1,
        Math.max(0, Math.floor(normalizedX * sceneNodes.length)),
      );
      state.sceneNodeIndex = nextIndex;
      state.sceneReceiptId = "none";
      state.sceneReceiptState = "idle";
      syncSceneProof("pointer-hover");
    });

    const gl = canvas.getContext("webgl", {
      alpha: false,
      antialias: true,
      preserveDrawingBuffer: false,
    });
    const width = (canvas.width = Math.max(640, canvas.clientWidth * devicePixelRatio));
    const height = (canvas.height = Math.max(360, canvas.clientHeight * devicePixelRatio));
    inspectSceneCapabilities = () => {
      if (!gl) {
        state.sceneCapabilityStatus = "webgl-unavailable";
        state.sceneCapabilityReport = `webgl-unavailable:${state.sceneQualityProfile}:${state.scenePerformanceBand}`;
        return state.sceneCapabilityReport;
      }
      if (gl.isContextLost()) {
        state.sceneCapabilityStatus = "context-lost";
        state.sceneCapabilityReport = `webgl:context-lost:${state.sceneQualityProfile}`;
        return state.sceneCapabilityReport;
      }
      const attributes = gl.getContextAttributes();
      const extensions = gl.getSupportedExtensions() || [];
      const maxTextureSize = gl.getParameter(gl.MAX_TEXTURE_SIZE);
      const maxRenderbufferSize = gl.getParameter(gl.MAX_RENDERBUFFER_SIZE);
      state.sceneCapabilityStatus = "ready";
      state.sceneCapabilityReport = [
        "webgl",
        "source-owned-webgl",
        state.sceneQualityProfile,
        state.scenePerformanceBand,
        `aa${attributes?.antialias ? "1" : "0"}`,
        `tex${maxTextureSize}`,
        `rb${maxRenderbufferSize}`,
        `ext${extensions.length}`,
      ].join(":");
      return state.sceneCapabilityReport;
    };
    syncSceneProof("booting");

    if (!gl) {
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      readSceneFrameSample = () => {
        const sample = ctx.getImageData(
          Math.max(0, Math.floor(width / 2)),
          Math.max(0, Math.floor(height / 2)),
          1,
          1,
        ).data;
        return `rgba-${sample[0]}-${sample[1]}-${sample[2]}-${sample[3]}-${state.sceneQualityProfile}-${state.sceneMaterialPalette}`;
      };
      const render2d = (time) => {
        const node = activeSceneNode();
        const quality = activeQualityProfile();
        const materialPalette = activeMaterialPalette();
        const cameraRig = activeCameraRig();
        const speed =
          (state.scenePerformanceBand === "regressed" ? 0.35 : 1) *
          quality.motionMultiplier;
        ctx.clearRect(0, 0, width, height);
        const gradient = ctx.createLinearGradient(0, 0, width, height);
        const gradientColors = materialPalette.gradientTokens.map((tokenName) =>
          sceneTokenColor(tokenName),
        );
        gradient.addColorStop(0, gradientColors[0]);
        gradient.addColorStop(0.55, gradientColors[1]);
        gradient.addColorStop(1, gradientColors[2]);
        ctx.fillStyle = gradient;
        ctx.fillRect(0, 0, width, height);
        ctx.fillStyle = sceneTokenColor(
          materialPalette.overlayToken,
          materialPalette.overlayAlpha,
        );
        ctx.fillRect(
          width * (0.16 + cameraRig.offsetX),
          height * (0.18 + cameraRig.offsetY),
          width * 0.68 * cameraRig.scale,
          height * 0.64 * cameraRig.scale,
        );
        const activeNodeIndex = Math.max(0, state.sceneNodeIndex);
        const orbit = 0.18 + (activeNodeIndex % sceneNodes.length) * 0.12;
        const textColor = sceneTokenColor(materialPalette.textToken);
        ctx.fillStyle = textColor;
        ctx.globalAlpha = 0.36;
        ctx.beginPath();
        ctx.arc(
          width * (0.5 + cameraRig.offsetX),
          height * (0.5 + cameraRig.offsetY),
          Math.max(42, width * orbit) * cameraRig.scale,
          0,
          Math.PI * 2,
        );
        ctx.strokeStyle = textColor;
        ctx.lineWidth = Math.max(1, devicePixelRatio);
        ctx.stroke();
        ctx.globalAlpha = 1;
        if (scenePixelProofFrames < 3 || scenePixelProofFrames % 90 === 0) {
          const sample = readSceneFrameSample();
          markScenePixelProof(sample.includes("rgba-0-0-0-0") ? "blank" : "nonblank", sample);
        }
        scenePixelProofFrames += 1;
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
      uniform vec3 accentColor;
      void main() {
        gl_FragColor = vec4(
          0.08 + pulse * accentColor.r * 0.34,
          0.28 + pulse * accentColor.g * 0.72,
          0.32 + pulse * accentColor.b * 0.58,
          1.0
        );
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
    const accentColor = gl.getUniformLocation(program, "accentColor");
    readSceneFrameSample = () => {
      const sample = new Uint8Array(4);
      gl.readPixels(
        Math.max(0, Math.floor(width / 2)),
        Math.max(0, Math.floor(height / 2)),
        1,
        1,
        gl.RGBA,
        gl.UNSIGNED_BYTE,
        sample,
      );
      return `rgba-${sample[0]}-${sample[1]}-${sample[2]}-${sample[3]}-${state.sceneQualityProfile}-${state.sceneMaterialPalette}`;
    };
    const render = (time) => {
      const quality = activeQualityProfile();
      const materialPalette = activeMaterialPalette();
      const cameraRig = activeCameraRig();
      const speed =
        (state.scenePerformanceBand === "regressed" ? 0.35 : 1) *
        quality.motionMultiplier;
      const selectedOffset = Math.max(0, state.sceneNodeIndex) * 0.22;
      gl.viewport(0, 0, width, height);
      gl.clearColor(
        materialPalette.clear[0] * (quality.id === "cinematic" ? 1.2 : 1),
        materialPalette.clear[1] * (quality.id === "cinematic" ? 1.12 : 1),
        materialPalette.clear[2] * (quality.id === "cinematic" ? 1.18 : 1),
        1,
      );
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.uniform3f(
        accentColor,
        materialPalette.accent[0],
        materialPalette.accent[1],
        materialPalette.accent[2],
      );
      gl.uniform1f(turn, (time * speed) / 1800 + selectedOffset + cameraRig.turnOffset);
      gl.uniform1f(
        pulse,
        ((Math.sin((time * speed) / 500 + selectedOffset) + 1) / 2) *
          quality.pulseBoost *
          cameraRig.pulseMultiplier,
      );
      gl.drawArrays(gl.TRIANGLES, 0, 6);
      if (scenePixelProofFrames < 3 || scenePixelProofFrames % 90 === 0) {
        const sample = readSceneFrameSample();
        markScenePixelProof(sample.includes("rgba-0-0-0-0") ? "blank" : "nonblank", sample);
      }
      scenePixelProofFrames += 1;
      syncSceneProof("webgl-active");
      requestAnimationFrame(render);
    };
    requestAnimationFrame(render);
  }

  function bindApiActions() {
    const missionTrpc = $("#mission-trpc");
    const missionTrpcStatus = $("#mission-trpc-status");
    const missionTrpcDetail = $("#mission-trpc-detail");
    const missionTrpcReceipt = $("#mission-trpc-receipt");

    function setTrpcWorkflow(message, status, requestId) {
      updateMissionControl("tRPC state updated in the launch dashboard.");
      if (missionTrpc) {
        missionTrpc.dataset.trpcMutationState = status;
        missionTrpc.dataset.dxTrpcReceiptState = status;
        missionTrpc.dataset.dxTrpcRequestId = requestId;
      }
      if (missionTrpcStatus) {
        missionTrpcStatus.dataset.trpcMutationState = status;
        missionTrpcStatus.dataset.trpcRequestId = requestId;
        missionTrpcStatus.dataset.state = status === "error" ? "error" : "ok";
        missionTrpcStatus.textContent = message;
      }
      if (missionTrpcDetail) {
        missionTrpcDetail.textContent =
          status === "error"
            ? "The route boundary stayed visible; runtime execution still needs app-owned wiring."
            : "tRPC updated the launch dashboard through the health/query and mutation contract.";
      }
      if (missionTrpcReceipt) {
        missionTrpcReceipt.dataset.dxTrpcReceiptState = status;
        missionTrpcReceipt.dataset.dxTrpcRequestId = requestId;
        missionTrpcReceipt.textContent =
          status === "pending"
            ? "Preparing tRPC dashboard receipt..."
            : `Receipt ${requestId}: app-owned auth, limits, transport, and observability remain explicit.`;
      }
    }

    async function queryTrpcHealth() {
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
    }

    async function recordTrpcLaunchEvent() {
      const requestId = `dx-trpc-local-${++state.trpcRuns}`;
      setTrpcWorkflow("Recording safe local launchEvent validation...", "pending", requestId);
      try {
        const response = await fetch("/api/trpc/health", {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ event: "validated", route: "/", requestId }),
        });
        const payload = await response.json();
        setTrpcWorkflow(
          `${payload.procedure ?? "launchEvent"} accepted for /; invalidate health.query; ${requestId}`,
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
    }

    $("#mission-trpc-health-check")?.addEventListener("click", queryTrpcHealth);
    $("#mission-trpc-launch-event")?.addEventListener("click", recordTrpcLaunchEvent);

    const aiCard = $('[data-dx-component="launch-ai-assistant-dashboard-workflow"][data-dx-package="ai/vercel-ai"]');
    const aiStatus = $("#ai-status");
    const aiPrompt = $("#ai-prompt");
    const aiAction = $("#ai-action");
    const aiProviderButtons = Array.from(document.querySelectorAll("[data-dx-ai-provider-choice]"));
    const setAiProof = (
      message,
      responseState,
      configState = "missing-config",
      requestId = "idle",
    ) => {
      if (aiCard) {
        aiCard.dataset.dxAiProvider = state.aiProvider;
        aiCard.dataset.dxAiConfigState = configState;
        aiCard.dataset.dxAiLocalResponse = responseState;
        aiCard.dataset.dxAiRequestId = requestId;
      }
      if (aiStatus) {
        aiStatus.dataset.dxAiLocalResponse = responseState;
        aiStatus.dataset.dxAiRequestId = requestId;
        aiStatus.dataset.dxAiResponseState = responseState;
        aiStatus.dataset.state = responseState === "error" ? "error" : "ok";
        aiStatus.textContent = message;
      }
    };
    const syncAiPromptState = () => {
      const prompt =
        aiPrompt && "value" in aiPrompt ? String(aiPrompt.value || "").trim() : "";
      const promptState = prompt.length > 0 ? "ready" : "empty";
      if (aiPrompt) aiPrompt.dataset.dxAiPromptState = promptState;
      if (aiAction) aiAction.dataset.dxAiActionState = promptState === "ready" ? "ready" : "needs-prompt";
      return { prompt, promptState };
    };
    aiPrompt?.addEventListener("input", syncAiPromptState);
    syncAiPromptState();
    aiProviderButtons.forEach((button) => {
      button.addEventListener("click", () => {
        state.aiProvider = button.dataset.dxAiProviderChoice || "openai-compatible";
        aiProviderButtons.forEach((candidate) => {
          candidate.dataset.dxAiProviderSelected = candidate === button ? "true" : "false";
        });
        setAiProof(
          `${state.aiProvider} selected. Credentials remain app-owned until configured.`,
          "provider-selected",
          "missing-config",
          "provider-picker",
        );
      });
    });

    aiAction?.addEventListener("click", async () => {
      const { prompt, promptState } = syncAiPromptState();
      const requestId = `dx-ai-local-${++state.aiRuns}`;
      if (promptState === "empty") {
        setAiProof(
          "Enter a launch prompt before previewing the AI route contract.",
          "invalid-prompt",
          "missing-config",
          requestId,
        );
        return;
      }
      if (aiAction) aiAction.dataset.dxAiActionState = "pending";
      setAiProof(
        "Calling source-owned AI route with a safe prompt preview...",
        "pending",
        "missing-config",
        requestId,
      );
      try {
        const response = await fetch("/api/ai/chat", {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({
            message: prompt,
            provider: state.aiProvider,
            requestId,
          }),
        });
        const payload = await response.json();
        const missingConfig =
          payload.status === "missing-config" || payload.credentialsConfigured === false;
        setAiProof(
          response.ok
            ? `${payload.provider ?? state.aiProvider} boundary accepted ${requestId}; ${payload.appOwnedBoundary ?? "provider credentials stay app-owned."}`
            : missingConfig
              ? `${payload.provider ?? state.aiProvider} missing credentials for ${requestId}; ${payload.appOwnedBoundary ?? "set AI_PROVIDER_API_KEY before streaming."}`
              : `AI route returned ${response.status}; local response remains visible.`,
          response.ok ? "route-accepted" : missingConfig ? "missing-config" : "error",
          payload.credentialsConfigured ? "configured" : "missing-config",
          payload.requestId ?? requestId,
        );
        syncAiPromptState();
      } catch {
        setAiProof(
          `Local AI readiness ${requestId}: ${state.aiProvider} provider missing credentials; prompt length ${prompt.length}.`,
          "local-readiness",
          "missing-config",
          requestId,
        );
        syncAiPromptState();
      }
    });

  }

  function bindDrizzleDashboardData() {
    const card = $('[data-dx-component="launch-drizzle-data-workflow"]');
    const databaseDashboard = $('[data-dx-component="database-backend-proof"], [data-dx-component="database-backend-card"]');
    const status = $("#drizzle-status");
    const helper = $("#drizzle-helper");
    const publicApi = $("#drizzle-public-api");
    const statement = $("#drizzle-statement");
    const rows = $("#drizzle-rows");
    const metrics = $("#drizzle-metrics");
    const queryPlan = $("#drizzle-query-plan");
    const summary = $("#drizzle-dashboard-summary");
    if (!card || !status || !helper || !publicApi || !statement || !rows || !metrics || !summary) return;

    const readModels = {
      "launch-pipeline": {
        queryPlanId: "overview",
        helper: "readDrizzleDashboardOverview",
        publicApi: "readLaunchDatabaseStats + readDrizzleDashboardOverview",
        statement:
          "select count(*), count(distinct users.role), count(*) filter (where posts.status = ?) from users left join posts",
        summary:
          "Use the generated dashboard overview to drive the launch operations summary.",
        metrics: [
          ["Users", "2"],
          ["Published", "2"],
          ["Pending", "1"],
        ],
        rows: [
          ["dx-preview", "DX template", "Published content and adapter readiness are visible.", "ready"],
          ["forge-source", "Forge source packages", "Generated files stay source-owned and editable.", "ready"],
        ],
      },
      "published-content": {
        queryPlanId: "published-posts",
        helper: "listPublishedPostPreviews",
        publicApi: "select().from().innerJoin().where().toSQL()",
        statement:
          "select posts.title, users.name from posts inner join users on posts.author_id = users.id where posts.status = ?",
        summary:
          "Preview the content list the dashboard would read from the local SQLite store.",
        metrics: [
          ["Rows", "2"],
          ["Join", "author"],
          ["Status", "published"],
        ],
        rows: [
          ["dx-preview", "DX template", "essencefromexistence", "published"],
          ["forge-source", "Forge source-owned packages", "Friday lane", "published"],
        ],
      },
      "author-workload": {
        queryPlanId: "author-counts",
        helper: "listAuthorsWithPostCounts",
        publicApi: "db.$with + db.with + leftJoin + groupBy + toSQL",
        statement:
          "with post_counts as (...) select users.name, post_counts.total from users leftJoin post_counts",
        summary:
          "Preview the author workload panel before the app connects a SQLite file.",
        metrics: [
          ["Authors", "2"],
          ["CTE", "post_counts"],
          ["Window", "launch"],
        ],
        rows: [
          ["essence", "essencefromexistence", "owner", "2 launch records"],
          ["friday", "Friday", "orchestrator", "1 readiness record"],
        ],
      },
    };

    function syncDatabaseDashboardState(status, detail) {
      if (!databaseDashboard) return;
      databaseDashboard.dataset.dxBackendStatus = status;
      databaseDashboard.dataset.dxBackendDetail = detail;
      databaseDashboard.dataset.dxBackendPackage = "db/drizzle-sqlite";
      databaseDashboard.dataset.dxBackendWorkflow = "sqlite-read-model";
    }

    const render = (readModelId) => {
      const model = readModels[readModelId] ?? readModels["launch-pipeline"];
      const queryBox = $('[data-dx-drizzle-sql-preview]');
      card.dataset.dxDrizzleReadModel = readModelId;
      syncDatabaseDashboardState(
        "read-model-ready",
        `Drizzle read model ready: ${model.helper} loaded for ${readModelId}; apply it to update the local launch dashboard fixture.`,
      );
      helper.dataset.dxDrizzleHelper = model.helper;
      helper.textContent = `Helper: ${model.helper}`;
      publicApi.dataset.dxDrizzlePublicApi = model.publicApi;
      publicApi.textContent = model.publicApi;
      statement.textContent = model.statement;
      if (queryBox) {
        queryBox.dataset.dxDrizzleSqlPreview = readModelId;
        queryBox.dataset.dxDrizzleQueryPlanId = model.queryPlanId;
        queryBox.dataset.dxDrizzleQueryPlanExport = "readDrizzleDashboardQueryPlanById";
      }
      summary.dataset.dxDrizzleDashboardSummary = readModelId;
      summary.textContent = model.summary;
      metrics.dataset.dxDrizzleMetrics = readModelId;
      metrics.replaceChildren(
        ...model.metrics.map(([label, value]) => {
          const metric = document.createElement("span");
          metric.dataset.dxDrizzleMetric = label;
          metric.textContent = `${label}: ${value}`;
          return metric;
        }),
      );
      rows.replaceChildren(
        ...model.rows.map(([id, primary, secondary, metric]) => {
          const row = document.createElement("div");
          row.className = "route-card";
          row.dataset.dxDrizzleFixtureRow = id;
          row.innerHTML = `<strong>${primary}</strong><span>${secondary}</span><span>${metric}</span>`;
          return row;
        }),
      );
      document.querySelectorAll("[data-dx-drizzle-read-model-option]").forEach((button) => {
        button.dataset.active = button.dataset.dxDrizzleReadModelOption === readModelId ? "true" : "false";
      });
    };

    document.querySelectorAll("[data-dx-drizzle-action='select-read-model']").forEach((button) => {
      button.addEventListener("click", () => {
        const readModelId = button.dataset.dxDrizzleReadModelOption || "launch-pipeline";
        render(readModelId);
        card.dataset.dxDrizzleStatus = "read-model-ready";
        card.dataset.dxDrizzleReceiptState = "idle";
        status.dataset.state = "ready";
        status.dataset.dxDrizzleReceiptState = "idle";
        status.textContent = `Loaded ${readModelId}. Apply read model to update the safe launch dashboard fixture.`;
      });
    });

    queryPlan?.addEventListener("click", () => {
      const queryBox = $('[data-dx-drizzle-sql-preview]');
      if (queryBox) queryBox.dataset.dxDrizzleQueryPlanPreviewed = "true";
      const readModelId = card.dataset.dxDrizzleReadModel || "launch-pipeline";
      const model = readModels[readModelId] ?? readModels["launch-pipeline"];
      syncDatabaseDashboardState(
        "query-plan-previewed",
        `Drizzle query plan previewed: ${model.publicApi}; database execution remains app-owned.`,
      );
      status.textContent = "Drizzle .toSQL() query-plan preview is visible; execution remains app-owned.";
      updateMissionControl("Drizzle query-plan preview updated the launch dashboard.");
    });

    $("#drizzle-apply-read-model")?.addEventListener("click", () => {
      state.drizzleRuns += 1;
      const readModelId = card.dataset.dxDrizzleReadModel || "launch-pipeline";
      const model = readModels[readModelId] ?? readModels["launch-pipeline"];
      card.dataset.dxDrizzleStatus = "read-model-applied";
      card.dataset.dxDrizzleReceiptState = "ready";
      syncDatabaseDashboardState(
        "read-model-applied",
        `Drizzle ${model.helper} applied locally ${state.drizzleRuns} time${
          state.drizzleRuns === 1 ? "" : "s"
        }; production SQLite still needs app-owned better-sqlite3 runtime and migrations.`,
      );
      status.dataset.state = "ok";
      status.dataset.dxDrizzleReceiptState = "ready";
      status.textContent = `Drizzle read model applied ${state.drizzleRuns} time${
        state.drizzleRuns === 1 ? "" : "s"
      }. Production SQLite still needs the app-owned better-sqlite3 runtime, reviewed migrations, and database path.`;
      updateMissionControl("Drizzle read model updated the launch dashboard.");
    });

    render(card.dataset.dxDrizzleReadModel || "launch-pipeline");
  }

  function bindInstantDbRuntimeProof() {
    const card = $('[data-dx-component="instantdb-runtime-dashboard-workflow"]');
    const button = $("#instantdb-local-receipt");
    const receipt = $("#instantdb-runtime-receipt");
    if (!card || !button || !receipt) return;

    button.addEventListener("click", () => {
      state.instantdbReceiptRuns += 1;
      const receiptId = `dx-instantdb-local-${String(state.instantdbReceiptRuns).padStart(2, "0")}`;

      card.dataset.dxInstantRuntimeStatus = "local-receipt-ready";
      card.dataset.dxInstantReceiptState = "created";
      button.dataset.dxInstantSafeActionState = "created";
      receipt.dataset.dxInstantLocalReceipt = receiptId;
      receipt.dataset.dxInstantReceiptStatus = "missing-config";
      receipt.dataset.dxInstantRuntimeStatus = "local-receipt-ready";
      receipt.dataset.state = "ok";
      receipt.textContent = `${receiptId}: prepared local schema receipt for todos, presence, typing, files, and streams. Hosted realtime still waits for NEXT_PUBLIC_INSTANT_APP_ID and app-owned rules.`;

      updateMissionControl("InstantDB local realtime receipt updated the launch dashboard.");
    });
  }

  function bindSupabaseSchemaQueryWorkflow() {
    const card = $('[data-dx-component="supabase-schema-query-workflow"]');
    const readiness = $("#supabase-readiness");
    const button = $("#supabase-local-query");
    const result = $("#supabase-query-result");
    const profilePanel = $('[data-dx-component="supabase-profile-workflow"]');
    const loadProfile = $("#supabase-load-profile");
    const prepareUpsert = $("#supabase-prepare-upsert");
    const profileFixture = $("#supabase-profile-fixture");
    const profileReceipt = $("#supabase-profile-receipt");
    if (!card || !readiness || !button || !result) return;

    button.addEventListener("click", () => {
      state.supabaseQueryRuns += 1;
      const operation =
        "supabase.from('profiles').select('id, full_name, username, website')";
      card.dataset.dxSupabaseQueryOperation = operation;
      card.dataset.dxSupabaseQueryState = "ready";
      result.dataset.dxSupabaseQueryOperation = operation;
      result.dataset.dxSupabaseQueryState = "ready";
      result.dataset.dxSupabaseRows = "2";
      readiness.dataset.state = "ok";
      result.innerHTML = [
        "<strong>profiles local schema query prepared</strong>",
        "<span>essencedx - https://dx.local/profile</span>",
        "<span>friday - https://dx.local/friday</span>",
        `<span>Run ${state.supabaseQueryRuns}; hosted credentials remain app-owned.</span>`,
      ].join("");
      updateMissionControl("Supabase local query updated the launch dashboard.");
    });

    if (profilePanel && loadProfile && prepareUpsert && profileFixture && profileReceipt) {
      const profile = {
        id: "00000000-0000-4000-8000-000000000001",
        username: "essencedx",
        fullName: "essencefromexistence",
        website: "https://dx.local/profile",
      };

      loadProfile.addEventListener("click", () => {
        profilePanel.dataset.dxSupabaseProfileLoaded = "true";
        profileFixture.dataset.dxSupabaseProfileLoaded = "true";
        profileFixture.dataset.dxSupabaseProfileId = profile.id;
        profileFixture.dataset.state = "ok";
        profileFixture.innerHTML = [
          "<strong>Local profile fixture loaded</strong>",
          `<span>${profile.fullName} - @${profile.username}</span>`,
          `<span>${profile.website}</span>`,
        ].join("");
        updateMissionControl("Supabase profile fixture loaded locally.");
      });

      prepareUpsert.addEventListener("click", () => {
        profilePanel.dataset.dxSupabaseReceiptState = "missing-config";
        profileReceipt.dataset.dxSupabaseUpsertOperation =
          "upsertDxSupabaseProfile(userId, input)";
        profileReceipt.dataset.dxSupabaseUpsertStatus = "missing-config";
        profileReceipt.dataset.state = "ready";
        profileReceipt.innerHTML = [
          "<strong>upsertDxSupabaseProfile(userId, input)</strong>",
          "<span>Input: fullName, username, website, avatarUrl.</span>",
          "<span>Set NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY before live writes.</span>",
        ].join("");
        updateMissionControl("Supabase profile upsert receipt prepared.");
      });
    }
  }

  function bindMissionControl() {
    const dashboard = $('[data-dx-component="launch-operating-dashboard"]');
    const sync = $("[data-dx-dashboard-action='sync-from-runtime']");
    const status = $("#mission-dashboard-status");
    if (!dashboard || !sync || !status) return;

    const write = (selector, value) => {
      const node = $(selector);
      if (node) node.textContent = value;
    };
    function updateMissionControl() {
      const session = $('[data-dx-component="better-auth-boundary-review"]');
      const payment = $('[data-dx-component="launch-billing-checkout-workflow"]');
      const automation = $('[data-dx-component="launch-automation-connector-workflow"]');
      const automationMission = $('[data-dx-component="launch-automation-mission-summary"]');
      const automationWorkflow = $('[data-dx-component="launch-automation-dashboard-workflow"]');
      const automationSchema = $("#automation-schema-summary");
      const databaseSurface = $('[data-dx-component="database-backend-proof"], [data-dx-component="database-backend-card"]');
      const locale = $('[data-dx-component="next-intl-dashboard-locale-workflow"]');
      const wasm = $('[data-dx-component="wasm-bindgen-readiness-workflow"]');
      const motion = $('[data-dx-component="motion-animation-card"]');
      const authBoundaryReviewed =
        session?.dataset.dxAuthLocalSession === "reviewed";
      const paymentState = payment?.dataset.dxStripePreviewState || "idle";
      const automationState = automation?.dataset.dxAutomationReceiptStatus || "ready";
      const automationWorkflowStatus =
        automationWorkflow?.dataset.dxDashboardAutomationStatus ||
        automationSchema?.dataset.dxAutomationWorkflowStatus ||
        "metadata-ready";
      const automationNodeReadiness =
        automationWorkflow?.dataset.dxDashboardAutomationNodeReadiness ||
        automationSchema?.dataset.dxAutomationWorkflowNodeReadiness ||
        "metadata-ready";
      const automationAuthKinds =
        automationSchema?.dataset.dxAutomationAuthKinds || "none";
      const automationCredentialTypes =
        automationSchema?.dataset.dxAutomationCredentialTypes || "none";
      const automationRequiredEnv =
        automationSchema?.dataset.dxAutomationRequiredEnv || "DX_AUTOMATIONS_OPERATOR_APPROVAL";
      const automationToolReady =
        automationSchema?.dataset.dxAutomationUsableAsTool || "false";
      const automationTriggerNode =
        automationSchema?.dataset.dxAutomationTriggerNode || "false";
      const automationDashboardState =
        automationWorkflow?.dataset.dxDashboardAutomationRunReceipt === "created"
          ? "zed-handoff-created"
          : automationWorkflow?.dataset.dxDashboardAutomationReceipt === "created"
            ? "draft-created"
            : "idle";
      const databaseState = databaseSurface?.dataset.dxBackendStatus || "ready";
      const databaseDetail =
        databaseSurface?.dataset.dxBackendDetail ||
        "Supabase, Drizzle, InstantDB, and tRPC adapters visible.";
      const localeState = locale?.dataset.dxDashboardCopyLocale || state.locale;
      const intl = dashboardIntlMessages[localeState] ?? dashboardIntlMessages.en;
      if (locale) {
        locale.dataset.dxIntlReadinessCopy = intl.readiness;
        locale.dataset.dxIntlFormatPreview = intl.formatPreview;
        locale.dataset.dxIntlFormatSourceApi = intl.formatSourceApi;
        locale.dataset.dxIntlFormatTimeZone = intl.formatTimeZone;
        locale.dataset.dxIntlNumberPreview = intl.numberPreview;
        locale.dataset.dxIntlNumberSourceApi = intl.numberSourceApi;
        locale.dataset.dxIntlNumberCurrency = intl.numberCurrency;
      }
      const wasmRuns = wasm
        ? Number(wasm.querySelector("[data-dx-wasm-runs]")?.dataset.dxWasmRuns || state.wasmRuns || 0)
        : state.wasmRuns;
      const controls = $('[data-dx-component="shadcn-dashboard-controls-runtime"]');
      const controlsDensity =
        controls?.dataset.dxShadcnDashboardDensity || state.shadcnDashboardDensity;
      const controlsQueue =
        controls?.dataset.dxShadcnDashboardQueue || state.shadcnDashboardQueue;
      const motionStageId =
        motion?.dataset.dxMotionState ||
        (motionStages[state.motionStageIndex] || motionStages[0]).id;
      const motionStage =
        motionStages.find((stage) => stage.id === motionStageId) || motionStages[0];
      const motionReduced = motion?.dataset.dxMotionReduced || (state.motionReduced ? "preview" : "system");
      const settingsState =
        state.zodSettingsValidation === "idle" ? "Waiting" : state.zodSettingsValidation;
      const sceneWorkflowLabels = {
        "visual-runtime-health": "Runtime health",
        "render-budget": "Render budget",
        "web-preview-readiness": "Web Preview readiness",
      };
      const sceneWorkflowLabel = sceneWorkflowLabels[state.sceneWorkflowId] || "Runtime health";
      const sceneQualityLabel =
        state.sceneQualityProfile === "cinematic" ? "Cinematic" : "Web Preview";
      const sceneMaterialLabel =
        state.sceneMaterialPalette === "graphite" ? "Graphite review" : "Aurora launch";
      const sceneCameraLabel = {
        orbit: "Orbit",
        isometric: "Isometric",
        inspection: "Inspection",
      }[state.sceneCameraRig] || "Orbit";

      write(
        "#mission-session-status",
        authBoundaryReviewed ? "Reviewed" : "Not reviewed",
      );
      write(
        "#mission-session-detail",
        authBoundaryReviewed ? state.authEmail : "Sign-in readiness mounted; credentials stay app-owned.",
      );
      write("#mission-payment-status", paymentState);
      write(
        "#mission-payment-detail",
        paymentState === "ready"
          ? "Stripe-shaped checkout contact prepared."
          : "Stripe-shaped checkout contract waiting.",
      );
      write("#mission-settings-status", settingsState);
      write(
        "#mission-settings-detail",
        state.zodSettingsValidation === "valid"
          ? `${state.zodSettingsWorkspaceName || "The dashboard"} uses ${state.zodSettingsTheme} theme, ${state.zodSettingsLocale} locale, and score target ${
              state.zodSettingsLaunchScoreTarget || "unset"
            }; ${zodSettingsValidationContract.fieldErrorsApi} has ${state.zodSettingsFieldErrorCount} field groups.`
          : state.zodSettingsValidation === "invalid"
            ? `${zodSettingsValidationContract.publicApi} blocked ${state.zodSettingsIssueCount} settings issue${
                state.zodSettingsIssueCount === 1 ? "" : "s"
              }; ${zodSettingsValidationContract.fieldErrorsApi} grouped ${state.zodSettingsFieldErrorCount} field error${
                state.zodSettingsFieldErrorCount === 1 ? "" : "s"
              } before dashboard state changed.`
            : "Zod validation has not run yet.",
      );
      write("#mission-automation-status", automationState);
      write(
        "#mission-automation-detail",
        automationDashboardState === "zed-handoff-created"
          ? `Connector ${state.automationConnector}; Zed handoff ready and execution remains gated.`
          : `Connector ${state.automationConnector}; execution remains gated.`,
      );
      write(
        "#mission-automation-schema",
        `Auth schema: ${automationAuthKinds}; credentials: ${automationCredentialTypes}; required env: ${automationRequiredEnv}; workflow node: ${automationNodeReadiness}; tool ready: ${automationToolReady}.`,
      );
      if (automationMission) {
        automationMission.dataset.dxAutomationAuthKinds = automationAuthKinds;
        automationMission.dataset.dxAutomationCredentialTypes = automationCredentialTypes;
        automationMission.dataset.dxAutomationDashboardState = automationDashboardState;
        automationMission.dataset.dxAutomationRequiredEnv = automationRequiredEnv;
        automationMission.dataset.dxAutomationSelectedConnector = state.automationConnector;
        automationMission.dataset.dxAutomationTriggerNode = automationTriggerNode;
        automationMission.dataset.dxAutomationUsableAsTool = automationToolReady;
        automationMission.dataset.dxAutomationWorkflowNodeReadiness = automationNodeReadiness;
        automationMission.dataset.dxAutomationWorkflowStatus = automationWorkflowStatus;
      }
      write("#mission-database-status", databaseState);
      write(
        "#mission-database-detail",
        databaseDetail,
      );
      write("#mission-scene-status", sceneWorkflowLabel);
      write(
        "#mission-scene-detail",
        state.sceneReceiptState === "prepared"
          ? `${state.sceneReceiptId} prepared from the ${sceneQualityLabel} 3D render budget workflow with ${sceneMaterialLabel} and ${sceneCameraLabel}.`
          : `${sceneQualityLabel} renderer; ${sceneMaterialLabel} material; ${sceneCameraLabel} camera; ${state.scenePerformanceBand} budget; frame sample ${state.sceneFrameSample}; capability ${state.sceneCapabilityReport}; viewport ${state.sceneViewportReport}; bounds ${state.sceneBoundsReport}. Focus a scene node or prepare a render budget receipt.`,
      );
      write("#mission-motion-status", motionStage.label);
      write(
        "#mission-motion-detail",
        `${motionStage.progress}% ready. ${motionStage.detail}`,
      );
      write(
        "#mission-motion-policy",
        motionReduced === "preview"
          ? "Reduced motion preview is active for dashboard choreography review."
          : "Motion policy follows the system preference.",
      );
      write("#mission-locale-status", intl.status);
      write("#mission-locale-detail", intl.detail);
      write("#mission-locale-copy", intl.copy);
      write("#mission-locale-readiness", intl.readiness);
      write("#mission-locale-format", `Formatter preview: ${intl.formatPreview}.`);
      write("#mission-locale-price", `Localized plan price: ${intl.numberPreview}/mo.`);
      write("#mission-locale-plan", `Plan copy: ${intl.planLabel}.`);
      write("#mission-locale-support", `Support SLA: ${intl.supportSla}.`);
      write("#mission-locale-route", `Route preview: ${intl.routePreview}.`);
      const localePlan = $("#mission-locale-plan");
      const localeSupport = $("#mission-locale-support");
      const localeFormat = $("#mission-locale-format");
      const localePrice = $("#mission-locale-price");
      const localeRoute = $("#mission-locale-route");
      if (localePlan) localePlan.dataset.dxIntlPlanLabel = intl.planLabel;
      if (localeSupport) localeSupport.dataset.dxIntlSupportSla = intl.supportSla;
      if (localeFormat) {
        localeFormat.dataset.dxIntlFormatPreview = intl.formatPreview;
        localeFormat.dataset.dxIntlFormatSourceApi = intl.formatSourceApi;
        localeFormat.dataset.dxIntlFormatTimeZone = intl.formatTimeZone;
      }
      if (localePrice) {
        localePrice.dataset.dxIntlNumberPreview = intl.numberPreview;
        localePrice.dataset.dxIntlNumberSourceApi = intl.numberSourceApi;
        localePrice.dataset.dxIntlNumberCurrency = intl.numberCurrency;
      }
      if (localeRoute) localeRoute.dataset.dxIntlRoutePreview = intl.routePreview;
      write(
        "#mission-wasm-status",
        wasmRuns ? `${wasmRuns} local run${wasmRuns === 1 ? "" : "s"}` : "Idle",
      );
      write(
        "#mission-wasm-detail",
        wasmRuns
          ? "wasm-bindgen local compute updated the dashboard."
          : "WebAssembly local add check waiting.",
      );
      write("#mission-controls-status", controlsDensity === "compact" ? "Compact" : "Comfortable");
      write(
        "#mission-controls-detail",
        state.shadcnDashboardReceiptRuns
          ? `${controlsQueue} queue has ${state.shadcnDashboardReceiptRuns} shadcn receipt preview${
              state.shadcnDashboardReceiptRuns === 1 ? "" : "s"
            }.`
          : `shadcn/ui controls focused on ${controlsQueue}.`,
      );
      dashboard.dataset.dxDashboardState = "synced";
      dashboard.dataset.dxDashboardMotion = motionStage.id;
      dashboard.dataset.dxDashboardMotionOrder = state.motionOrder.join(",");
      dashboard.dataset.dxDashboardMotionReduced = motionReduced;
      dashboard.dataset.dxDashboardWasmRuns = String(wasmRuns);
      dashboard.dataset.dxDashboardControlsDensity = controlsDensity;
      dashboard.dataset.dxDashboardControlsQueue = controlsQueue;
      dashboard.dataset.dxDashboardSceneWorkflow = state.sceneWorkflowId;
      dashboard.dataset.dxDashboardSceneReceipt = state.sceneReceiptState;
      dashboard.dataset.dxDashboardSceneQualityProfile = state.sceneQualityProfile;
      dashboard.dataset.dxDashboardSceneMaterialPalette = state.sceneMaterialPalette;
      dashboard.dataset.dxDashboardSceneCameraRig = state.sceneCameraRig;
      dashboard.dataset.dxDashboardSceneFrameSample = state.sceneFrameSample;
      dashboard.dataset.dxDashboardLocale = localeState;
      dashboard.dataset.dxDashboardSettingsValidation = state.zodSettingsValidation;
      dashboard.dataset.dxDashboardSettingsIssueCount = String(state.zodSettingsIssueCount);
      dashboard.dataset.dxDashboardSettingsFieldErrorCount = String(
        state.zodSettingsFieldErrorCount,
      );
      dashboard.dataset.dxDashboardSettingsWorkspace = state.zodSettingsWorkspaceName || "none";
      dashboard.dataset.dxDashboardSettingsScoreTarget = String(
        state.zodSettingsLaunchScoreTarget || "none",
      );
      dashboard.dataset.dxDashboardAutomation = state.automationConnector;
      dashboard.dataset.dxDashboardAutomationState = automationDashboardState;
      dashboard.dataset.dxDashboardAutomationStatus = automationWorkflowStatus;
      dashboard.dataset.dxDashboardAutomationNodeReadiness = automationNodeReadiness;
      dashboard.dataset.dxDashboardDatabase = databaseState;
      document.documentElement.lang = localeState;
      status.dataset.dxDashboardStatus = "synced";
      status.dataset.state = "ok";
      status.textContent = "Dashboard metrics synced from the visible runtime proof surfaces.";
    }

    sync.addEventListener("click", updateMissionControl);
    updateMissionControl();
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
        updateMissionControl("WASM local compute updated the launch dashboard.");
      } catch (error) {
        card.dataset.dxWasmBindgenStatus = "error";
        status.dataset.state = "error";
        status.dataset.dxWasmAddResult = "error";
        status.textContent = `Local WebAssembly readiness check failed: ${error instanceof Error ? error.message : "unknown error"}.`;
        updateMissionControl("WASM local compute failed and the launch dashboard recorded the boundary.");
      }
    });
  }

  document.addEventListener("DOMContentLoaded", () => {
    bindMissionControl();
    bindIntlDashboard();
    bindLaunchDashboardSettings();
    bindShadcnDashboardControls();
    bindSession();
    bindPayment();
    bindSettingsForm();
    bindStateAndQuery();
    bindTemplateApp();
    bindMotion();
    bindFumadocsDocs();
    bindAutomations();
    bindScene();
    bindApiActions();
    bindSupabaseSchemaQueryWorkflow();
    bindInstantDbRuntimeProof();
    bindDrizzleDashboardData();
    bindMissionControl();
    bindWasm();
  });
})();
