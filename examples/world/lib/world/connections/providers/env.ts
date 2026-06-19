import type {
  DataIdentityConnectionResult,
  DataIdentityProviderDefinition,
  DataIdentityProviderKind,
  DataIdentityProbe,
  DataIdentityProbeContext,
} from "./data-identity-types";

type RuntimeProcess = {
  process?: {
    env?: Record<string, string | undefined>;
  };
};

export function readProviderEnv(
  context: DataIdentityProbeContext,
): Record<string, string | undefined> {
  return context.env ?? (globalThis as RuntimeProcess).process?.env ?? {};
}

export function readEnvValue(
  env: Record<string, string | undefined>,
  name: string,
): string | null {
  const value = env[name]?.trim();
  return value ? value : null;
}

export function presentEnvNames(
  env: Record<string, string | undefined>,
  names: readonly string[],
): readonly string[] {
  return names.filter((name) => readEnvValue(env, name) !== null);
}

export function missingEnvNames(
  env: Record<string, string | undefined>,
  names: readonly string[],
): readonly string[] {
  return names.filter((name) => readEnvValue(env, name) === null);
}

export function checkedAt(context: DataIdentityProbeContext): string {
  return (context.now?.() ?? new Date()).toISOString();
}

export function configuredProbe(
  context: DataIdentityProbeContext,
  message: string,
): DataIdentityProbe {
  return {
    kind: "configured-readiness",
    checkedAt: checkedAt(context),
    live: false,
    endpointKind: "not-probed",
    message,
  };
}

export function buildProviderResult<ProviderId extends string>({
  blockers = [],
  context,
  definition,
  nextAction,
  probe,
  status,
}: {
  blockers?: readonly string[];
  context: DataIdentityProbeContext;
  definition: DataIdentityProviderDefinition<ProviderId>;
  nextAction: string;
  probe: DataIdentityProbe;
  status: DataIdentityConnectionResult<ProviderId>["status"];
}): DataIdentityConnectionResult<ProviderId> {
  const env = readProviderEnv(context);
  const envNames = [...definition.requiredEnv, ...definition.optionalEnv];

  return {
    schema: "dx.examples.world.provider-connection",
    providerId: definition.id,
    providerName: definition.name,
    kind: definition.kind,
    category: definition.category,
    status,
    requiredEnv: definition.requiredEnv,
    optionalEnv: definition.optionalEnv,
    presentEnv: presentEnvNames(env, envNames),
    missingEnv: missingEnvNames(env, definition.requiredEnv),
    receiptSchemas: definition.receiptSchemas,
    appOwnedBoundary: definition.appOwnedBoundary,
    redaction: "secret-values-never-included",
    probe,
    blockers,
    nextAction,
  };
}

export function missingConfigResult<ProviderId extends string>(
  definition: DataIdentityProviderDefinition<ProviderId>,
  context: DataIdentityProbeContext,
): DataIdentityConnectionResult<ProviderId> {
  return buildProviderResult({
    context,
    definition,
    nextAction: `Set ${missingEnvNames(readProviderEnv(context), definition.requiredEnv).join(
      ", ",
    )} before running live ${labelForKind(definition.kind)} validation.`,
    probe: {
      kind: "missing-env",
      checkedAt: checkedAt(context),
      live: false,
      endpointKind: "not-probed",
      message: "Required provider environment is missing.",
    },
    status: "missing-config",
  });
}

export function isSafeReadinessUrl(value: string): boolean {
  try {
    const url = new URL(value);
    return (
      url.protocol === "https:" &&
      url.username === "" &&
      url.password === "" &&
      url.search === "" &&
      url.hash === ""
    );
  } catch {
    return false;
  }
}

export function isLocalReadinessUrl(value: string): boolean {
  try {
    const url = new URL(value);
    return (
      (url.protocol === "http:" || url.protocol === "https:") &&
      ["localhost", "127.0.0.1", "::1"].includes(url.hostname) &&
      url.username === "" &&
      url.password === "" &&
      url.search === "" &&
      url.hash === ""
    );
  } catch {
    return false;
  }
}

function labelForKind(kind: DataIdentityProviderKind): string {
  return kind === "auth" ? "identity" : "database";
}
