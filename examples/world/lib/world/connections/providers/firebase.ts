import type {
  WorldConnectionContext,
  WorldConnectionEnvStatus,
  WorldConnectionFetch,
  WorldConnectionProbe,
  WorldConnectionResult,
} from "../contracts";
import { connectionResult } from "../http";
import { readableEndpointLabel } from "../redaction";

export const firebaseFirestoreCrudRequiredEnv = ["FIREBASE_PROJECT_ID", "FIREBASE_API_KEY"] as const;
export const firebaseFirestoreCrudOptionalEnv = [
  "FIREBASE_AUTH_DOMAIN",
  "FIREBASE_STORAGE_BUCKET",
  "FIREBASE_MESSAGING_SENDER_ID",
  "FIREBASE_APP_ID",
  "FIREBASE_MEASUREMENT_ID",
  "FIREBASE_AUTH_ID_TOKEN",
  "GOOGLE_APPLICATION_CREDENTIALS",
] as const;

export type FirebaseFirestoreCrudState = "missing-config" | "live-validated" | "blocked";

export type FirebaseFirestoreCrudStepName =
  | "create-document"
  | "read-document"
  | "update-document"
  | "delete-document";

export type FirebaseFirestoreCrudStep = {
  name: FirebaseFirestoreCrudStepName;
  method: "GET" | "POST" | "PATCH" | "DELETE";
  endpoint: string;
  ok: boolean;
  httpStatus?: number;
  evidence: string;
};

export type FirebaseFirestoreCrudReceipt = {
  schema: "dx.examples.world.firebase-firestore-crud";
  providerId: "firebase-firestore";
  packageId: "database/firebase-firestore";
  category: "Firebase Firestore";
  status: FirebaseFirestoreCrudState;
  redaction: "secret-values-never-included";
  checkedAt: string;
  projectId: string;
  collection: string;
  documentId: string;
  requiredEnv: readonly string[];
  optionalEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  secretValues: [];
  liveProviderExecution: boolean;
  steps: readonly FirebaseFirestoreCrudStep[];
  nextAction: string;
};

export type FirebaseFirestoreCrudOptions = {
  env?: Record<string, string | undefined>;
  fetch?: WorldConnectionFetch;
  now?: () => Date;
  collection?: string;
  documentId?: string;
};

type FirebaseContext = {
  apiKey: string;
  projectId: string;
  checkedAt: string;
  collection: string;
  documentId: string;
  presentEnv: readonly string[];
  missingEnv: readonly string[];
};

type FirebaseStepInput = {
  name: FirebaseFirestoreCrudStepName;
  method: FirebaseFirestoreCrudStep["method"];
  url: URL;
  body?: Record<string, unknown>;
  expectedStatuses?: readonly number[];
  idToken?: string | null;
};

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

export const firebaseConnectionProbes: readonly WorldConnectionProbe[] = [
  {
    id: "firebase-firestore-document-readiness",
    providerId: "firebase-firestore",
    packageId: "database/firebase-firestore",
    name: "Firebase Firestore document readiness",
    category: "Firebase Firestore",
    kind: "http",
    endpoint: "env:FIREBASE_PROJECT_ID/firestore/documents",
    documentationUrl: "https://firebase.google.com/docs/firestore/use-rest-api",
    requiredEnv: firebaseFirestoreCrudRequiredEnv,
    optionalEnv: firebaseFirestoreCrudOptionalEnv,
    run: runFirebaseReadinessProbe,
  },
];

export async function runFirebaseFirestoreCrudSmoke(
  options: FirebaseFirestoreCrudOptions = {},
): Promise<FirebaseFirestoreCrudReceipt> {
  const env = options.env ?? process?.env ?? {};
  const fetchImpl = options.fetch ?? fetch;
  const context = buildFirebaseContext(env, options);

  if (context.missingEnv.length > 0) {
    return firebaseReceipt({
      context,
      status: "missing-config",
      liveProviderExecution: false,
      steps: [],
      nextAction: `Set ${context.missingEnv.join(", ")} before running Firebase Firestore CRUD proof.`,
    });
  }

  const idToken = readEnv(env, "FIREBASE_AUTH_ID_TOKEN");
  const createUrl = collectionUrl(context);
  createUrl.searchParams.set("documentId", context.documentId);
  const documentUrlValue = documentUrl(context);
  const updateUrl = documentUrl(context);
  updateUrl.searchParams.append("updateMask.fieldPaths", "title");
  updateUrl.searchParams.append("updateMask.fieldPaths", "counter");

  const steps: FirebaseFirestoreCrudStep[] = [];
  steps.push(
    await runFirebaseStep(fetchImpl, context, {
      name: "create-document",
      method: "POST",
      url: createUrl,
      idToken,
      body: firestoreDocument({
        schema: "dx.examples.world.firebase-firestore-crud-object",
        title: "created-from-dx-www",
        counter: 1,
      }),
    }),
  );
  steps.push(
    await runFirebaseStep(fetchImpl, context, {
      name: "read-document",
      method: "GET",
      url: documentUrlValue,
      idToken,
    }),
  );
  steps.push(
    await runFirebaseStep(fetchImpl, context, {
      name: "update-document",
      method: "PATCH",
      url: updateUrl,
      idToken,
      body: firestoreDocument({
        title: "updated-from-dx-www",
        counter: 2,
      }),
    }),
  );
  steps.push(
    await runFirebaseStep(fetchImpl, context, {
      name: "delete-document",
      method: "DELETE",
      url: documentUrl(context),
      idToken,
    }),
  );

  const ok = steps.every((step) => step.ok);

  return firebaseReceipt({
    context,
    status: ok ? "live-validated" : "blocked",
    liveProviderExecution: true,
    steps,
    nextAction: ok
      ? "Import this redacted Firebase Firestore CRUD receipt."
      : "Firebase Firestore rejected the public REST CRUD path. Configure rules/auth or provide an Admin/service-account adapter before claiming live CRUD.",
  });
}

async function runFirebaseReadinessProbe(
  context: WorldConnectionContext,
  envStatus: WorldConnectionEnvStatus,
): Promise<WorldConnectionResult> {
  const firebase = buildFirebaseContext(context.env, {
    now: () => new Date(context.checkedAt),
    collection: "dx_www_readiness",
    documentId: "readiness",
  });

  if (firebase.missingEnv.length > 0) {
    return connectionResult({
      probe: firebaseConnectionProbes[0],
      context,
      envStatus,
      state: "missing-config",
      ok: false,
      durationMs: 0,
      evidence: "required-env-missing",
      message: "Firebase Firestore env is missing; read-only document access was not attempted.",
    });
  }

  if (!context.allowNetwork) {
    return connectionResult({
      probe: firebaseConnectionProbes[0],
      context,
      envStatus,
      state: "configured-readiness",
      ok: false,
      durationMs: 0,
      evidence: "network-disabled",
      message: "Firebase Firestore env is present, but live network probing is disabled.",
    });
  }

  const startedAt = Date.now();
  const result = await runFirebaseStep(context.fetch, firebase, {
    name: "read-document",
    method: "GET",
    url: documentUrl(firebase),
    expectedStatuses: [200, 404],
    idToken: readEnv(context.env, "FIREBASE_AUTH_ID_TOKEN"),
  });

  return connectionResult({
    probe: {
      ...firebaseConnectionProbes[0],
      endpoint: result.endpoint,
    },
    context,
    envStatus,
    state: result.ok ? "live-validated" : "blocked",
    ok: result.ok,
    durationMs: Date.now() - startedAt,
    evidence: result.evidence,
    message: result.ok
      ? "Firebase Firestore read-only document request completed."
      : "Firebase Firestore read-only document request was rejected.",
    httpStatus: result.httpStatus,
  });
}

function buildFirebaseContext(
  env: Record<string, string | undefined>,
  options: FirebaseFirestoreCrudOptions,
): FirebaseContext {
  const checkedAt = (options.now?.() ?? new Date()).toISOString();
  const projectId = readEnv(env, "FIREBASE_PROJECT_ID");
  const apiKey = readEnv(env, "FIREBASE_API_KEY");
  const presentEnv = [...firebaseFirestoreCrudRequiredEnv, ...firebaseFirestoreCrudOptionalEnv].filter(
    (name) => Boolean(readEnv(env, name)),
  );
  const missingEnv = [
    ...(projectId ? [] : ["FIREBASE_PROJECT_ID"]),
    ...(apiKey ? [] : ["FIREBASE_API_KEY"]),
  ];
  const runId = checkedAt.replace(/\D/g, "").slice(0, 14) || "00000000000000";

  return {
    apiKey: apiKey ?? "",
    projectId: projectId ?? "preview-only",
    checkedAt,
    collection: options.collection ?? "dx_www_crud_smoke",
    documentId: options.documentId ?? `check-${runId}`,
    presentEnv,
    missingEnv,
  };
}

function readEnv(env: Record<string, string | undefined>, name: string): string | null {
  const value = env[name]?.trim();
  return value ? value : null;
}

function firestoreBaseUrl(context: FirebaseContext): URL {
  const url = new URL(
    `https://firestore.googleapis.com/v1/projects/${encodeURIComponent(
      context.projectId,
    )}/databases/(default)/documents/`,
  );
  url.searchParams.set("key", context.apiKey);
  return url;
}

function collectionUrl(context: FirebaseContext): URL {
  const url = firestoreBaseUrl(context);
  url.pathname = `${url.pathname}${encodeURIComponent(context.collection)}`;
  return url;
}

function documentUrl(context: FirebaseContext): URL {
  const url = collectionUrl(context);
  url.pathname = `${url.pathname}/${encodeURIComponent(context.documentId)}`;
  return url;
}

async function runFirebaseStep(
  fetchImpl: WorldConnectionFetch,
  context: FirebaseContext,
  step: FirebaseStepInput,
): Promise<FirebaseFirestoreCrudStep> {
  try {
    const response = await fetchImpl(step.url, {
      method: step.method,
      headers: firebaseHeaders(step.idToken),
      body: step.body ? JSON.stringify(step.body) : undefined,
    });
    const expectedStatuses = step.expectedStatuses ?? [200];
    const ok = expectedStatuses.includes(response.status);

    if (step.method === "GET" && response.body) {
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

function firebaseHeaders(idToken: string | null): Record<string, string> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };

  if (idToken) {
    headers.Authorization = `Bearer ${idToken}`;
  }

  return headers;
}

function firestoreDocument(values: {
  schema?: string;
  title: string;
  counter: number;
}): Record<string, unknown> {
  const fields: Record<string, unknown> = {
    title: { stringValue: values.title },
    counter: { integerValue: String(values.counter) },
  };

  if (values.schema) {
    fields.schema = { stringValue: values.schema };
  }

  return { fields };
}

function firebaseReceipt(input: {
  context: FirebaseContext;
  status: FirebaseFirestoreCrudState;
  liveProviderExecution: boolean;
  steps: readonly FirebaseFirestoreCrudStep[];
  nextAction: string;
}): FirebaseFirestoreCrudReceipt {
  return {
    schema: "dx.examples.world.firebase-firestore-crud",
    providerId: "firebase-firestore",
    packageId: "database/firebase-firestore",
    category: "Firebase Firestore",
    status: input.status,
    redaction: "secret-values-never-included",
    checkedAt: input.context.checkedAt,
    projectId: input.context.projectId,
    collection: input.context.collection,
    documentId: input.context.documentId,
    requiredEnv: firebaseFirestoreCrudRequiredEnv,
    optionalEnv: firebaseFirestoreCrudOptionalEnv,
    presentEnv: input.context.presentEnv,
    missingEnv: input.context.missingEnv,
    secretValues: [],
    liveProviderExecution: input.liveProviderExecution,
    steps: input.steps,
    nextAction: input.nextAction,
  };
}
