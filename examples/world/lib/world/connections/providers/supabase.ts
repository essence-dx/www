import type {
  WorldConnectionContext,
  WorldConnectionEnvStatus,
  WorldConnectionFetch,
  WorldConnectionProbe,
  WorldConnectionResult,
} from "../contracts";
import { connectionResult, runReadOnlyHttpProbe } from "../http";
import { readableEndpointLabel } from "../redaction";

export const supabaseCrudRequiredEnv = ["SUPABASE_URL", "SUPABASE_SECRET_KEY"] as const;
export const supabaseCrudOptionalEnv = [
  "NEXT_PUBLIC_SUPABASE_URL",
  "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
  "SUPABASE_SERVICE_ROLE_KEY",
] as const;

export type SupabaseCrudState = "missing-config" | "live-validated" | "blocked";

export type SupabaseCrudStepName =
  | "list-buckets"
  | "create-bucket"
  | "create-object"
  | "read-object"
  | "update-object"
  | "delete-object"
  | "delete-bucket";

export type SupabaseCrudStep = {
  name: SupabaseCrudStepName;
  method: "GET" | "POST" | "DELETE";
  endpoint: string;
  ok: boolean;
  httpStatus?: number;
  evidence: string;
};

export type SupabaseStorageCrudReceipt = {
  schema: "dx.examples.world.supabase-storage-crud";
  providerId: "supabase";
  packageId: "database/supabase";
  category: "Database and storage platform";
  status: SupabaseCrudState;
  redaction: "secret-values-never-included";
  checkedAt: string;
  projectRef: string;
  bucket: string;
  objectPath: string;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  secretValues: [];
  liveProviderExecution: boolean;
  steps: readonly SupabaseCrudStep[];
  nextAction: string;
};

export type SupabaseStorageCrudOptions = {
  env?: Record<string, string | undefined>;
  fetch?: WorldConnectionFetch;
  now?: () => Date;
  bucket?: string;
  objectPath?: string;
  payloadTag?: string;
};

type SupabaseContext = {
  url: URL;
  projectRef: string;
  secretKey: string;
  checkedAt: string;
  bucket: string;
  objectPath: string;
  presentEnv: readonly string[];
  missingEnv: readonly string[];
};

type SupabaseStepInput = {
  name: SupabaseCrudStepName;
  method: SupabaseCrudStep["method"];
  url: URL;
  headers: Record<string, string>;
  body?: string;
  expectedStatuses?: readonly number[];
};

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

export const supabaseConnectionProbes: readonly WorldConnectionProbe[] = [
  {
    id: "supabase-storage-buckets-readiness",
    providerId: "supabase",
    packageId: "database/supabase",
    name: "Supabase Storage bucket access",
    category: "Database and storage platform",
    kind: "http",
    endpoint: "env:SUPABASE_URL/storage/v1/bucket",
    documentationUrl: "https://supabase.com/docs/guides/storage",
    requiredEnv: supabaseCrudRequiredEnv,
    optionalEnv: supabaseCrudOptionalEnv,
    run: runSupabaseStorageReadinessProbe,
  },
];

export async function runSupabaseStorageCrudSmoke(
  options: SupabaseStorageCrudOptions = {},
): Promise<SupabaseStorageCrudReceipt> {
  const env = options.env ?? process?.env ?? {};
  const fetchImpl = options.fetch ?? fetch;
  const context = buildSupabaseContext(env, options);

  if (context.missingEnv.length > 0) {
    return {
      schema: "dx.examples.world.supabase-storage-crud",
      providerId: "supabase",
      packageId: "database/supabase",
      category: "Database and storage platform",
      status: "missing-config",
      redaction: "secret-values-never-included",
      checkedAt: context.checkedAt,
      projectRef: context.projectRef,
      bucket: context.bucket,
      objectPath: context.objectPath,
      requiredEnv: supabaseCrudRequiredEnv,
      optionalEnv: supabaseCrudOptionalEnv,
      presentEnv: context.presentEnv,
      missingEnv: context.missingEnv,
      secretValues: [],
      liveProviderExecution: false,
      steps: [],
      nextAction: `Set ${context.missingEnv.join(", ")} before running Supabase Storage CRUD proof.`,
    };
  }

  const steps: SupabaseCrudStep[] = [];
  const headers = supabaseHeaders(context.secretKey);
  const objectBody = JSON.stringify({
    schema: "dx.examples.world.supabase-storage-crud-object",
    operation: "create",
    tag: options.payloadTag ?? "dx-www",
    checkedAt: context.checkedAt,
  });
  const updatedBody = JSON.stringify({
    schema: "dx.examples.world.supabase-storage-crud-object",
    operation: "update",
    tag: options.payloadTag ?? "dx-www",
    checkedAt: context.checkedAt,
  });

  steps.push(
    await runSupabaseStep(fetchImpl, {
      name: "list-buckets",
      method: "GET",
      url: storageUrl(context, "bucket"),
      headers,
    }),
  );

  const createBucket = await runSupabaseStep(fetchImpl, {
    name: "create-bucket",
    method: "POST",
    url: storageUrl(context, "bucket"),
    headers: { ...headers, "Content-Type": "application/json" },
    body: JSON.stringify({ id: context.bucket, name: context.bucket, public: false }),
  });
  steps.push(createBucket);

  if (createBucket.ok) {
    steps.push(
      await runSupabaseStep(fetchImpl, {
        name: "create-object",
        method: "POST",
        url: storageUrl(context, `object/${context.bucket}/${context.objectPath}`),
        headers: { ...headers, "Content-Type": "application/json", "x-upsert": "true" },
        body: objectBody,
      }),
    );

    steps.push(
      await runSupabaseStep(fetchImpl, {
        name: "read-object",
        method: "GET",
        url: storageUrl(context, `object/${context.bucket}/${context.objectPath}`),
        headers,
      }),
    );

    steps.push(
      await runSupabaseStep(fetchImpl, {
        name: "update-object",
        method: "POST",
        url: storageUrl(context, `object/${context.bucket}/${context.objectPath}`),
        headers: { ...headers, "Content-Type": "application/json", "x-upsert": "true" },
        body: updatedBody,
      }),
    );

    steps.push(
      await runSupabaseStep(fetchImpl, {
        name: "delete-object",
        method: "DELETE",
        url: storageUrl(context, `object/${context.bucket}`),
        headers: { ...headers, "Content-Type": "application/json" },
        body: JSON.stringify({ prefixes: [context.objectPath] }),
      }),
    );

    steps.push(
      await runSupabaseStep(fetchImpl, {
        name: "delete-bucket",
        method: "DELETE",
        url: storageUrl(context, `bucket/${context.bucket}`),
        headers,
      }),
    );
  }

  const ok = steps.length === 7 && steps.every((step) => step.ok);

  return {
    schema: "dx.examples.world.supabase-storage-crud",
    providerId: "supabase",
    packageId: "database/supabase",
    category: "Database and storage platform",
    status: ok ? "live-validated" : "blocked",
    redaction: "secret-values-never-included",
    checkedAt: context.checkedAt,
    projectRef: context.projectRef,
    bucket: context.bucket,
    objectPath: context.objectPath,
    requiredEnv: supabaseCrudRequiredEnv,
    optionalEnv: supabaseCrudOptionalEnv,
    presentEnv: context.presentEnv,
    missingEnv: [],
    secretValues: [],
    liveProviderExecution: true,
    steps,
    nextAction: ok
      ? "Import this redacted Supabase Storage CRUD receipt."
      : "Review the failed Supabase Storage CRUD step before claiming live write proof.",
  };
}

async function runSupabaseStorageReadinessProbe(
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
): Promise<WorldConnectionResult> {
  const supabase = buildSupabaseContext(context.env, { now: () => new Date(context.checkedAt) });

  if (supabase.missingEnv.length > 0) {
    return connectionResult({
      probe: supabaseConnectionProbes[0],
      context,
      envStatus,
      state: "missing-config",
      ok: false,
      durationMs: 0,
      evidence: "required-env-missing",
      message: "Supabase Storage env is missing; live bucket access was not attempted.",
    });
  }

  return runReadOnlyHttpProbe(context, envStatus, {
    id: "supabase-storage-buckets-readiness",
    providerId: "supabase",
    packageId: "database/supabase",
    name: "Supabase Storage bucket access",
    category: "Database and storage platform",
    endpoint: storageUrl(supabase, "bucket").toString(),
    documentationUrl: "https://supabase.com/docs/guides/storage",
    headers: supabaseHeaders(supabase.secretKey),
    evidence: "supabase-storage-buckets-list",
  });
}

function buildSupabaseContext(
  env: Record<string, string | undefined>,
  options: SupabaseStorageCrudOptions,
): SupabaseContext {
  const checkedAt = (options.now?.() ?? new Date()).toISOString();
  const supabaseUrl = readEnv(env, "SUPABASE_URL") ?? readEnv(env, "NEXT_PUBLIC_SUPABASE_URL");
  const secretKey = readEnv(env, "SUPABASE_SECRET_KEY") ?? readEnv(env, "SUPABASE_SERVICE_ROLE_KEY");
  const presentEnv = [...supabaseCrudRequiredEnv, ...supabaseCrudOptionalEnv].filter((name) =>
    Boolean(readEnv(env, name)),
  );
  const missingEnv = [
    ...(supabaseUrl ? [] : ["SUPABASE_URL"]),
    ...(secretKey ? [] : ["SUPABASE_SECRET_KEY"]),
  ];
  const url = supabaseUrl ? normalizeSupabaseUrl(supabaseUrl) : new URL("https://preview-only.supabase.co");
  const projectRef = url.hostname.split(".")[0] ?? "preview-only";
  const runId = checkedAt.replace(/\D/g, "").slice(0, 14) || "00000000000000";

  return {
    url,
    projectRef,
    secretKey: secretKey ?? "",
    checkedAt,
    bucket: options.bucket ?? `dx-www-crud-${runId}`,
    objectPath: options.objectPath ?? `checks/${runId}-supabase-crud.json`,
    presentEnv,
    missingEnv,
  };
}

function readEnv(env: Record<string, string | undefined>, name: string): string | null {
  const value = env[name]?.trim();
  return value ? value : null;
}

function normalizeSupabaseUrl(value: string): URL {
  const url = new URL(value);

  if (url.protocol !== "https:") {
    throw new Error("SUPABASE_URL must be an HTTPS URL.");
  }

  url.pathname = "";
  url.search = "";
  url.hash = "";
  return url;
}

function supabaseHeaders(secretKey: string): Record<string, string> {
  return {
    apikey: secretKey,
    Authorization: `Bearer ${secretKey}`,
  };
}

function storageUrl(context: Pick<SupabaseContext, "url">, path: string): URL {
  return new URL(`/storage/v1/${path.replace(/^\/+/, "")}`, context.url);
}

async function runSupabaseStep(
  fetchImpl: WorldConnectionFetch,
  step: SupabaseStepInput,
): Promise<SupabaseCrudStep> {
  try {
    const response = await fetchImpl(step.url, {
      method: step.method,
      headers: step.headers,
      body: step.body,
    });
    const expectedStatuses = step.expectedStatuses ?? [200];
    const ok = expectedStatuses.includes(response.status);

    if (step.name === "read-object" && ok) {
      await response.text();
    }

    return {
      name: step.name,
      method: step.method,
      endpoint: readableEndpointLabel(step.url.toString()),
      ok,
      httpStatus: response.status,
      evidence: ok ? `${step.name}-ok` : `http-status-${response.status}`,
    };
  } catch (error) {
    return {
      name: step.name,
      method: step.method,
      endpoint: readableEndpointLabel(step.url.toString()),
      ok: false,
      evidence: error instanceof Error ? error.name : "unknown-error",
    };
  }
}
