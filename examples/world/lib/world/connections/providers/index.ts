import { aiConnectionProbes } from "./ai";
import {
  authConnectionProbes,
  authProviderDefinitions,
  probeAuthProvider,
  readAuthProvider,
} from "./auth";
import { commerceConnectionProviders } from "./commerce";
import { contentDeployConnectionProviders } from "./content-deploy";
import { contentNotificationConnectionProbes } from "./content-notifications";
import {
  databaseConnectionProbes,
  databaseProviderDefinitions,
  probeDatabaseProvider,
  readDatabaseProvider,
} from "./database";
import { developerToolConnectionProbes } from "./developer-tools";
import {
  firebaseConnectionProbes,
  firebaseFirestoreCrudOptionalEnv,
  firebaseFirestoreCrudRequiredEnv,
  runFirebaseFirestoreCrudSmoke,
} from "./firebase";
import { mediaConnectionProviders } from "./media";
import {
  neonConnectionProbes,
  neonCrudAcceptedEnv,
  neonCrudOptionalEnv,
  neonCrudRequiredEnv,
  runNeonDatabaseCrudSmoke,
} from "./neon";
import { operationConnectionProbes } from "./operations";
import { paymentConnectionProbes } from "./payments";
import { readPresent, readRequired } from "./request-utils";
import { searchVectorConnectionProviders } from "./search-vector";
import {
  runSupabaseStorageCrudSmoke,
  supabaseConnectionProbes,
  supabaseCrudOptionalEnv,
  supabaseCrudRequiredEnv,
} from "./supabase";
import { tursoConnectionProbes } from "./turso";
import {
  runTursoDatabaseCrudSmoke,
  tursoCrudOptionalEnv,
  tursoCrudRequiredEnv,
} from "./turso-crud";
import { vercelConnectionProbes } from "./vercel";
import type { WorldConnectionProbe } from "../contracts";
import type {
  WorldConnectionCheckOptions,
  WorldConnectionCheckResult,
  WorldConnectionEnv,
  WorldConnectionProvider,
  WorldConnectionProviderId,
  WorldConnectionStatus,
  WorldFetch,
} from "./types";

declare const process:
  | {
      env?: WorldConnectionEnv;
    }
  | undefined;

export {
  aiConnectionProbes,
  authConnectionProbes,
  authProviderDefinitions,
  contentDeployConnectionProviders,
  contentNotificationConnectionProbes,
  databaseConnectionProbes,
  databaseProviderDefinitions,
  developerToolConnectionProbes,
  firebaseConnectionProbes,
  firebaseFirestoreCrudOptionalEnv,
  firebaseFirestoreCrudRequiredEnv,
  operationConnectionProbes,
  paymentConnectionProbes,
  neonConnectionProbes,
  neonCrudAcceptedEnv,
  neonCrudOptionalEnv,
  neonCrudRequiredEnv,
  probeAuthProvider,
  probeDatabaseProvider,
  readAuthProvider,
  readDatabaseProvider,
  runFirebaseFirestoreCrudSmoke,
  runNeonDatabaseCrudSmoke,
  runSupabaseStorageCrudSmoke,
  runTursoDatabaseCrudSmoke,
  supabaseConnectionProbes,
  supabaseCrudOptionalEnv,
  supabaseCrudRequiredEnv,
  tursoConnectionProbes,
  tursoCrudOptionalEnv,
  tursoCrudRequiredEnv,
  vercelConnectionProbes,
};

export type { WorldAuthProviderId } from "./auth";
export type { WorldDatabaseProviderId } from "./database";

export const worldConnectionProbes = [
  ...tursoConnectionProbes,
  ...authConnectionProbes,
  ...databaseConnectionProbes,
  ...neonConnectionProbes,
  ...supabaseConnectionProbes,
  ...firebaseConnectionProbes,
  ...paymentConnectionProbes,
  ...aiConnectionProbes,
  ...contentNotificationConnectionProbes,
  ...operationConnectionProbes,
  ...developerToolConnectionProbes,
  ...vercelConnectionProbes,
] satisfies readonly WorldConnectionProbe[];

export const worldConnectionProviders = [
  ...commerceConnectionProviders,
  ...mediaConnectionProviders,
  ...searchVectorConnectionProviders,
  ...contentDeployConnectionProviders,
] satisfies readonly WorldConnectionProvider[];

const providersById = new Map<string, WorldConnectionProvider>(
  worldConnectionProviders.map((provider) => [provider.id, provider]),
);

function defaultEnv(): WorldConnectionEnv {
  return process?.env ?? {};
}

function defaultFetch(): WorldFetch {
  return fetch;
}

function baseResult(provider: WorldConnectionProvider, env: WorldConnectionEnv): Omit<
  WorldConnectionCheckResult,
  "status" | "liveProviderExecution" | "nextAction"
> {
  return {
    schema: "dx.examples.world.connection_provider_readiness",
    providerId: provider.id,
    packageId: provider.packageId,
    categoryId: provider.categoryId,
    method: provider.readiness.method,
    endpoint: provider.readiness.endpointLabel,
    requiredEnv: provider.requiredEnv,
    presentEnv: readPresent(env, provider.requiredEnv),
    missingEnv: readRequired(env, provider.requiredEnv),
    secretValues: [],
    receiptSchema: provider.receiptSchema,
    redaction: provider.secretRedaction,
  };
}

function isSuccess(status: number): boolean {
  return status >= 200 && status < 300;
}

export async function checkWorldConnectionProvider(
  providerId: WorldConnectionProviderId | string,
  options: WorldConnectionCheckOptions = {},
): Promise<WorldConnectionCheckResult> {
  const provider = providersById.get(providerId);

  if (!provider) {
    return {
      schema: "dx.examples.world.connection_provider_readiness",
      providerId,
      status: "unknown-provider",
      requiredEnv: [],
      presentEnv: [],
      missingEnv: [],
      secretValues: [],
      liveProviderExecution: false,
      redaction: "secret-values-never-included",
      nextAction: "Choose one of the registered world connection providers.",
    };
  }

  const env = options.env ?? defaultEnv();
  const result = baseResult(provider, env);

  if (result.missingEnv.length > 0) {
    return {
      ...result,
      status: "missing-config",
      liveProviderExecution: false,
      nextAction: "Provide missing Env Firewall keys before live validation.",
    };
  }

  if (!provider.readiness.buildRequest) {
    return {
      ...result,
      status: "configured-readiness",
      liveProviderExecution: false,
      nextAction: provider.nextAction,
    };
  }

  const request = await provider.readiness.buildRequest(env);
  const response = await (options.fetch ?? defaultFetch())(request.input, request.init);

  return {
    ...result,
    status: isSuccess(response.status) ? "live-validated" : "provider-error",
    endpoint: provider.readiness.endpointLabel,
    httpStatus: response.status,
    liveProviderExecution: true,
    nextAction: isSuccess(response.status)
      ? provider.nextAction
      : "Review provider permissions and import only a successful redacted live-proof receipt.",
  };
}

export type {
  WorldConnectionCheckOptions,
  WorldConnectionCheckResult,
  WorldConnectionProvider,
  WorldConnectionProviderId,
  WorldConnectionStatus,
} from "./types";
