export type WorldRuntime =
  | "browser"
  | "server"
  | "edge"
  | "server-or-edge"
  | "server-and-browser"
  | "development"
  | "ci";

export type WorldValidationState =
  | "preview-only"
  | "live-if-env-present"
  | "live-validated";

export type WorldSupportMode =
  | "source-owned-preview"
  | "missing-config"
  | "configured-readiness"
  | "live-validated";

export type WorldEnvScope = "server" | "browser-public" | "ci" | "agent-safe-name-only";

export type WorldEnvRequirement = {
  name: string;
  scope: WorldEnvScope;
  required: boolean;
  capability: string;
};

export type WorldIntegration = {
  id: string;
  packageId: string;
  name: string;
  categoryId: string;
  category: string;
  reason: string;
  runtime: WorldRuntime;
  surface: string;
  adapter: string;
  supportMode: WorldSupportMode;
  validation: WorldValidationState;
  env: readonly WorldEnvRequirement[];
  routeHandlers: readonly string[];
  receipts: readonly string[];
  nextAction: string;
  frameworkSuggestion?: string;
};

export type WorldCategory = {
  id: string;
  title: string;
  purpose: string;
  providers: readonly WorldIntegration[];
};

export type WorldProviderStatus = {
  id: string;
  packageId: string;
  name: string;
  category: string;
  state: WorldValidationState;
  supportMode: WorldSupportMode;
  requiredEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  routeHandlers: readonly string[];
  receipts: readonly string[];
  nextAction: string;
};

export type WorldStatus = {
  generatedBy: "examples/world";
  redaction: "secret-values-never-included";
  totals: {
    categories: number;
    providers: number;
    liveReady: number;
    envReady: number;
    previewOnly: number;
    missingEnv: number;
  };
  connections: {
    runner: string;
    receiptPath: string;
    probeCount: number;
    liveProbeMode: "read-only-when-env-present";
  };
  providers: readonly WorldProviderStatus[];
};
