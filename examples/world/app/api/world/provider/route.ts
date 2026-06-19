import {
  checkWorldConnectionProvider,
  worldConnectionProviders,
  type WorldConnectionStatus,
} from "../../../../lib/world/connections/providers";
import { runOperationsProviderProbe } from "../../../../lib/world/connections/providers/operations";
import {
  findAiRealtimeWorkflowProvider,
  probeAiRealtimeWorkflowProvider,
} from "../../../../lib/world/connections/providers/ai-realtime-workflows";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

const operationsProviderIds = new Set([
  "redis-valkey",
  "upstash-redis",
  "cloudflare-kv",
  "posthog",
  "plausible",
  "vercel-analytics",
  "sentry",
  "datadog",
  "opentelemetry",
]);

function statusCodeFor(status: WorldConnectionStatus | string): number {
  if (status === "unknown-provider") return 404;
  if (status === "missing-config") return 428;
  if (status === "provider-error") return 502;
  if (status === "blocked") return 424;
  return 200;
}

export async function GET(request: Request) {
  const requestUrl = new URL(request.url);
  const providerId = requestUrl.searchParams.get("provider");
  const mode = requestUrl.searchParams.get("mode");

  if (!providerId) {
    return Response.json({
      schema: "dx.examples.world.provider-readiness",
      ok: true,
      status: "source-owned-preview",
      providerSelection: "query-provider-runtime-boundary",
      providerCatalog: "examples/world/lib/world/registry.ts",
      connectionRunner: "examples/world/lib/world/connections/runner.ts",
      connectionProviderCatalog: "examples/world/lib/world/connections/providers/index.ts",
      routeContractCatalog: "examples/world/lib/world/routes.ts",
      redaction: "secret-values-never-included",
      credentialState: "provider-query-missing",
      liveProviderExecution: false,
      providers: worldConnectionProviders.map((provider) => ({
        id: provider.id,
        packageId: provider.packageId,
        categoryId: provider.categoryId,
        method: provider.readiness.method,
        endpoint: provider.readiness.endpointLabel,
        requiredEnv: provider.requiredEnv,
      })),
      nextAction: "Select a provider, add Env Firewall values, then run the TypeScript connection runner.",
    });
  }

  const aiRealtimeWorkflowProvider = findAiRealtimeWorkflowProvider(providerId);

  if (aiRealtimeWorkflowProvider) {
    const result = await probeAiRealtimeWorkflowProvider(
      aiRealtimeWorkflowProvider.providerId,
      {
        env: process?.env ?? {},
        fetch: mode === "live-read-only" ? fetch : undefined,
      },
    );

    return Response.json(
      {
        ...result,
        ok: result.status === "live-validated" || result.status === "configured-readiness",
        credentialState:
          result.status === "missing-config" ? "missing-config" : "redacted-env-names-only",
        connectionRunner:
          "examples/world/lib/world/connections/providers/ai-realtime-workflows.ts",
      },
      { status: statusCodeFor(result.status) },
    );
  }

  if (operationsProviderIds.has(providerId)) {
    const probe = await runOperationsProviderProbe(providerId);

    if (!probe) {
      return Response.json(
        {
          schema: "dx.examples.world.provider-readiness",
          ok: false,
          status: "unknown-provider",
          providerId,
          requiredEnv: [],
          presentEnv: [],
          missingEnv: [],
          secretValues: [],
          credentialState: "unknown-provider",
          liveProviderExecution: false,
          redaction: "secret-values-never-included",
          nextAction: "Choose a registered operations provider probe.",
        },
        { status: 404 },
      );
    }

    return Response.json(
      {
        schema: "dx.examples.world.provider-readiness",
        ok: probe.state === "live-validated" || probe.state === "configured-readiness",
        status: probe.state,
        providerId: probe.providerId,
        packageId: probe.packageId,
        category: probe.category,
        method: "GET",
        endpoint: probe.endpoint,
        requiredEnv: probe.requiredEnv,
        presentEnv: probe.presentEnv,
        missingEnv: probe.missingEnv,
        secretValues: [],
        receiptSchema: probe.evidence,
        redaction: probe.redaction,
        credentialState: probe.state,
        liveProviderExecution: probe.state === "live-validated",
        operationProbe: probe,
        connectionRunner: "examples/world/lib/world/connections/providers/operations.ts",
        nextAction: probe.message,
      },
      { status: probe.state === "missing-config" ? 428 : probe.state === "blocked" ? 502 : 200 },
    );
  }

  const catalogResult = await checkWorldConnectionProvider(providerId);
  const operationsProbe = catalogResult.status === "unknown-provider"
    ? await runOperationsProviderProbe(providerId)
    : undefined;
  const result = operationsProbe
    ? {
        schema: "dx.examples.world.provider-readiness",
        providerId: operationsProbe.providerId,
        packageId: operationsProbe.packageId,
        status: operationsProbe.state === "blocked" ? "provider-error" : operationsProbe.state,
        method: "GET",
        endpoint: operationsProbe.endpoint,
        requiredEnv: operationsProbe.requiredEnv,
        presentEnv: operationsProbe.presentEnv,
        missingEnv: operationsProbe.missingEnv,
        secretValues: [],
        liveProviderExecution: operationsProbe.state === "live-validated",
        httpStatus: operationsProbe.httpStatus,
        redaction: "secret-values-never-included",
        nextAction: operationsProbe.message,
      }
    : catalogResult;

  return Response.json(
    {
      ...result,
      ok: result.status === "live-validated" || result.status === "configured-readiness",
      credentialState: result.status === "missing-config" ? "missing-config" : "redacted-env-names-only",
      connectionRunner: "examples/world/lib/world/connections/runner.ts",
    },
    { status: statusCodeFor(result.status) },
  );
}
