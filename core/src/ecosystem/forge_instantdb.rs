pub(crate) const INSTANTDB_REACT_VERSION: &str = "0.0.0-dx.0";

pub(crate) fn instantdb_react_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/instant/env.ts", INSTANTDB_ENV_TS),
        ("js/instant/schema.ts", INSTANTDB_SCHEMA_TS),
        ("js/instant/client.ts", INSTANTDB_CLIENT_TS),
        ("js/instant/next-client.tsx", INSTANTDB_NEXT_CLIENT_TSX),
        ("js/instant/next-server.ts", INSTANTDB_NEXT_SERVER_TS),
        ("js/instant/queries.ts", INSTANTDB_QUERIES_TS),
        ("js/instant/status.ts", INSTANTDB_STATUS_TS),
        ("js/instant/subscriptions.ts", INSTANTDB_SUBSCRIPTIONS_TS),
        ("js/instant/pagination.ts", INSTANTDB_PAGINATION_TS),
        ("js/instant/diagnostics.ts", INSTANTDB_DIAGNOSTICS_TS),
        ("js/instant/mutations.ts", INSTANTDB_MUTATIONS_TS),
        ("js/instant/rules.ts", INSTANTDB_RULES_TS),
        ("js/instant/perms.ts", INSTANTDB_PERMS_TS),
        ("js/instant/auth.ts", INSTANTDB_AUTH_TS),
        ("js/instant/oauth.ts", INSTANTDB_OAUTH_TS),
        ("js/instant/storage.ts", INSTANTDB_STORAGE_TS),
        ("js/instant/streams.ts", INSTANTDB_STREAMS_TS),
        ("js/instant/sync-table.ts", INSTANTDB_SYNC_TABLE_TS),
        (
            "js/instant/dashboard-workflow.ts",
            INSTANTDB_DASHBOARD_WORKFLOW_TS,
        ),
        ("js/instant/route.ts", INSTANTDB_ROUTE_TS),
        ("js/instant/metadata.ts", INSTANTDB_METADATA_TS),
        (
            "js/components/instant/instant-todos.tsx",
            INSTANTDB_TODOS_TSX,
        ),
        (
            "js/components/instant/instant-cursors.tsx",
            INSTANTDB_CURSORS_TSX,
        ),
        (
            "js/components/instant/instant-auth-boundary.tsx",
            INSTANTDB_AUTH_BOUNDARY_TSX,
        ),
        (
            "js/components/dashboard/instantdb-dashboard-workflow.tsx",
            INSTANTDB_DASHBOARD_WORKFLOW_TSX,
        ),
        (
            "js/app/api/instant/route.ts",
            INSTANTDB_API_INSTANT_ROUTE_TS,
        ),
        ("js/app/instant/page.tsx", INSTANTDB_PAGE_TSX),
        ("js/instant/README.md", INSTANTDB_README_MD),
    ]
}

const INSTANTDB_ENV_TS: &str = r#"export type DxInstantEnv = Record<string, string | undefined>;

export type DxInstantConfig = {
  appId: string;
  apiURI?: string;
  devtool?: boolean;
  disableValidation?: boolean;
  firstPartyPath?: string;
  queryCacheLimit?: number;
  useDateObjects: true;
  verbose?: boolean;
  websocketURI?: string;
};

export function defaultInstantEnv(): DxInstantEnv {
  return (globalThis as unknown as { process?: { env?: DxInstantEnv } }).process?.env ?? {};
}

export function readInstantConfig(
  env: DxInstantEnv = defaultInstantEnv(),
): DxInstantConfig {
  const config: DxInstantConfig = {
    appId: requiredEnv(env, "NEXT_PUBLIC_INSTANT_APP_ID"),
    useDateObjects: true,
  };

  const apiURI = optionalEnv(env, "NEXT_PUBLIC_INSTANT_API_URI");
  if (apiURI) {
    config.apiURI = apiURI;
  }

  const firstPartyPath = optionalEnv(env, "NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH");
  if (firstPartyPath) {
    config.firstPartyPath = firstPartyPath;
  }

  const websocketURI = optionalEnv(env, "NEXT_PUBLIC_INSTANT_WEBSOCKET_URI");
  if (websocketURI) {
    config.websocketURI = websocketURI;
  }

  const devtool = optionalBooleanEnv(env, "NEXT_PUBLIC_INSTANT_DEVTOOL");
  if (devtool !== undefined) {
    config.devtool = devtool;
  }

  const disableValidation = optionalBooleanEnv(env, "NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION");
  if (disableValidation !== undefined) {
    config.disableValidation = disableValidation;
  }

  const queryCacheLimit = optionalNumberEnv(env, "NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT");
  if (queryCacheLimit !== undefined) {
    config.queryCacheLimit = queryCacheLimit;
  }

  const verbose = optionalBooleanEnv(env, "NEXT_PUBLIC_INSTANT_VERBOSE");
  if (verbose !== undefined) {
    config.verbose = verbose;
  }

  return config;
}

function optionalEnv(env: DxInstantEnv, key: string): string | undefined {
  const value = env[key]?.trim();
  return value ? value : undefined;
}

function optionalBooleanEnv(env: DxInstantEnv, key: string): boolean | undefined {
  const value = optionalEnv(env, key);
  if (value === undefined) {
    return undefined;
  }

  if (["1", "true", "yes", "on"].includes(value.toLowerCase())) {
    return true;
  }

  if (["0", "false", "no", "off"].includes(value.toLowerCase())) {
    return false;
  }

  throw new Error(`Invalid boolean InstantDB env var: ${key}`);
}

function optionalNumberEnv(env: DxInstantEnv, key: string): number | undefined {
  const value = optionalEnv(env, key);
  if (value === undefined) {
    return undefined;
  }

  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed < 0) {
    throw new Error(`Invalid non-negative number InstantDB env var: ${key}`);
  }

  return parsed;
}

function requiredEnv(env: DxInstantEnv, key: string): string {
  const value = optionalEnv(env, key);
  if (!value) {
    throw new Error(`Missing required InstantDB env var: ${key}`);
  }
  return value;
}
"#;

const INSTANTDB_SCHEMA_TS: &str = r#"import { i } from "@instantdb/react";

const schema = i.schema({
  entities: {
    todos: i.entity({
      text: i.string(),
      done: i.boolean(),
      createdAt: i.number(),
      details: i.json().optional(),
    }),
    labels: i.entity({
      name: i.string().unique().indexed(),
    }),
  },
  links: {
    todoLabels: {
      forward: {
        on: "todos",
        has: "many",
        label: "labels",
      },
      reverse: {
        on: "labels",
        has: "many",
        label: "todos",
      },
    },
  },
  rooms: {
    launch: {
      presence: i.entity({
        name: i.string().optional(),
        "launch-input": i.boolean().optional(),
        "dx-launch-cursors": i.json().optional(),
      }),
      topics: {
        launchPing: i.entity({
          message: i.string(),
          sentAt: i.number(),
        }),
      },
    },
  },
});

type InstantLaunchSchemaShape = typeof schema;
export interface InstantLaunchSchema extends InstantLaunchSchemaShape {}

export default schema as InstantLaunchSchema;
"#;

const INSTANTDB_CLIENT_TS: &str = r#"import { init } from "@instantdb/react";

import { readInstantConfig, type DxInstantEnv } from "./env";
import schema from "./schema";

export type DxInstantClient = ReturnType<typeof createDxInstantClient>;

export function createDxInstantClient(env?: DxInstantEnv) {
  return init({
    ...readInstantConfig(env),
    schema,
  });
}

export function createDxInstantCapabilities(client: DxInstantClient) {
  return {
    auth: client.auth,
    storage: client.storage,
    streams: client.streams,
  };
}

export const db = createDxInstantClient();
export const instantCapabilities = createDxInstantCapabilities(db);
export const launchRoom = db.room("launch");
"#;

const INSTANTDB_NEXT_CLIENT_TSX: &str = r#""use client";

import { init, InstantSuspenseProvider } from "@instantdb/react/nextjs";
import type { ReactNode } from "react";

import { readInstantConfig, type DxInstantEnv } from "./env";
import { instantLaunchTodosQuery } from "./queries";
import schema from "./schema";

export type DxInstantLaunchSsrUser =
  | Parameters<typeof InstantSuspenseProvider>[0]["user"]
  | null;

export function createDxInstantNextClient(env?: DxInstantEnv) {
  return init({
    ...readInstantConfig(env),
    schema,
  });
}

export const nextDb = createDxInstantNextClient();

export function InstantLaunchSuspenseProvider({
  children,
  nonce,
  user,
}: {
  children: ReactNode;
  nonce?: string;
  user?: DxInstantLaunchSsrUser;
}) {
  return (
    <InstantSuspenseProvider db={nextDb} nonce={nonce} user={user}>
      {children}
    </InstantSuspenseProvider>
  );
}

export function useInstantLaunchTodosSuspense() {
  return nextDb.useSuspenseQuery(instantLaunchTodosQuery);
}
"#;

const INSTANTDB_NEXT_SERVER_TS: &str = r#"import { getUnverifiedUserFromInstantCookie } from "@instantdb/react/nextjs";

import { readInstantConfig, type DxInstantEnv } from "./env";

export function getInstantLaunchSsrUser(env?: DxInstantEnv) {
  const { appId } = readInstantConfig(env);
  return getUnverifiedUserFromInstantCookie(appId);
}
"#;

const INSTANTDB_QUERIES_TS: &str = r#"import { db } from "./client";

export const instantLaunchTodosQuery = {
  todos: {
    labels: {},
  },
} as const;

export function queryInstantLaunchTodosSnapshot() {
  return db.queryOnce(instantLaunchTodosQuery);
}

export function useInstantLaunchDeviceId() {
  return db.useLocalId("dx-launch-device");
}
"#;

const INSTANTDB_STATUS_TS: &str = r#"import { db } from "./client";

export function useInstantLaunchConnectionStatus() {
  return db.useConnectionStatus();
}

export function getInstantLaunchDeviceId() {
  return db.getLocalId("dx-launch-device");
}
"#;

const INSTANTDB_SUBSCRIPTIONS_TS: &str = r#"import { db } from "./client";
import { instantLaunchTodosQuery } from "./queries";

export type DxInstantTodosSubscriptionCallback = Parameters<
  typeof db.core.subscribeQuery
>[1];

export type DxInstantAuthSubscriptionCallback = Parameters<
  typeof db.core.subscribeAuth
>[0];

export type DxInstantConnectionSubscriptionCallback = Parameters<
  typeof db.core.subscribeConnectionStatus
>[0];

export function subscribeInstantLaunchTodos(
  callback: DxInstantTodosSubscriptionCallback,
) {
  return db.core.subscribeQuery(instantLaunchTodosQuery, callback);
}

export function subscribeInstantLaunchAuth(
  callback: DxInstantAuthSubscriptionCallback,
) {
  return db.core.subscribeAuth(callback);
}

export function subscribeInstantLaunchConnectionStatus(
  callback: DxInstantConnectionSubscriptionCallback,
) {
  return db.core.subscribeConnectionStatus(callback);
}
"#;

const INSTANTDB_PAGINATION_TS: &str = r#"import { db } from "./client";

export function instantLaunchTodosPageQuery(pageSize = 20) {
  return {
    todos: {
      $: {
        limit: pageSize,
        order: {
          createdAt: "desc",
        },
      },
    },
  } as const;
}

export function useInstantLaunchTodosInfinite(pageSize = 20) {
  return db.useInfiniteQuery(instantLaunchTodosPageQuery(pageSize));
}

export function subscribeInstantLaunchTodosInfinite(
  callback: Parameters<typeof db.core.subscribeInfiniteQuery>[1],
  pageSize = 20,
) {
  return db.core.subscribeInfiniteQuery(
    instantLaunchTodosPageQuery(pageSize),
    callback,
  );
}
"#;

const INSTANTDB_DIAGNOSTICS_TS: &str = r#"import {
  InstantAPIError,
  InstantError,
  setInstantWarningsEnabled,
} from "@instantdb/react";

export function setInstantLaunchWarningsEnabled(enabled: boolean) {
  setInstantWarningsEnabled(enabled);
}

export function isInstantLaunchError(error: unknown): error is InstantError {
  return error instanceof InstantError;
}

export function isInstantLaunchApiError(error: unknown): error is InstantAPIError {
  return error instanceof InstantAPIError;
}

export function formatInstantLaunchError(error: unknown) {
  if (error instanceof InstantAPIError) {
    return error.body?.message ?? error.message;
  }

  if (error instanceof InstantError) {
    return error.message;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "Unknown InstantDB error";
}
"#;

const INSTANTDB_MUTATIONS_TS: &str = r#"import { id, lookup, type InstaQLEntity } from "@instantdb/react";

import { db } from "./client";
import type { InstantLaunchSchema } from "./schema";

export type InstantTodo = InstaQLEntity<InstantLaunchSchema, "todos">;
export type InstantLabel = InstaQLEntity<InstantLaunchSchema, "labels">;

export const instantLaunchLabelName = "Launch";

export function addInstantTodo(text: string) {
  const trimmed = text.trim();
  if (!trimmed) {
    return;
  }

  db.transact(
    db.tx.todos[id()].create({
      text: trimmed,
      done: false,
      createdAt: Date.now(),
    }),
  );
}

export function toggleInstantTodo(todo: InstantTodo) {
  db.transact(db.tx.todos[todo.id].update({ done: !todo.done }, { upsert: false }));
}

export function deleteInstantTodo(todo: InstantTodo) {
  db.transact(db.tx.todos[todo.id].delete());
}

export function toggleAllInstantTodos(todos: InstantTodo[]) {
  if (todos.length === 0) {
    return;
  }

  const done = !todos.every((todo) => todo.done);
  db.transact(
    todos.map((todo) => db.tx.todos[todo.id].update({ done }, { upsert: false })),
  );
}

export function clearCompletedInstantTodos(todos: InstantTodo[]) {
  const chunks = todos
    .filter((todo) => todo.done)
    .map((todo) => db.tx.todos[todo.id].delete());

  if (chunks.length === 0) {
    return;
  }

  db.transact(chunks);
}

export function labelInstantTodoForLaunch(todo: InstantTodo) {
  db.transact([
    db.tx.labels[lookup("name", instantLaunchLabelName)].update({
      name: instantLaunchLabelName,
    }),
    db.tx.todos[todo.id].link({
      labels: lookup("name", instantLaunchLabelName),
    }),
  ]);
}

export function unlabelInstantTodoForLaunch(todo: InstantTodo) {
  db.transact(
    db.tx.todos[todo.id].unlink({
      labels: lookup("name", instantLaunchLabelName),
    }),
  );
}

export function mergeInstantTodoLaunchDetails(todo: InstantTodo) {
  db.transact(
    db.tx.todos[todo.id].merge({
      details: {
        launch: {
          touchedAt: Date.now(),
          source: "dx-www",
        },
      },
    }),
  );
}
"#;

const INSTANTDB_RULES_TS: &str = r#"import { lookup } from "@instantdb/react";

import { db } from "./client";
import { instantLaunchTodosQuery } from "./queries";

export type DxInstantRuleParams = Record<string, string | number | boolean | null>;

export function queryInstantLaunchTodosWithRuleParams(ruleParams: DxInstantRuleParams) {
  return db.queryOnce(instantLaunchTodosQuery, { ruleParams });
}

export function updateInstantLaunchTodoWithRuleParams(
  todoId: string,
  ruleParams: DxInstantRuleParams,
  done: boolean,
) {
  return db.transact(db.tx.todos[todoId].ruleParams(ruleParams).update({ done }));
}

export function updateInstantLaunchTodoByText(text: string, done: boolean) {
  const value = text.trim();
  if (!value) {
    throw new Error("InstantDB todo lookup text is required");
  }

  return db.transact(db.tx.todos[lookup("text", value)].update({ done }));
}
"#;

const INSTANTDB_PERMS_TS: &str = r#"import type { InstantRules } from "@instantdb/react";

const rules = {
  todos: {
    allow: {
      view: "true",
      create: "auth.id != null",
      update: "auth.id != null",
      delete: "auth.id != null",
    },
  },
  $files: {
    allow: {
      view: "auth.id != null",
      create: "auth.id != null",
      update: "auth.id != null",
      delete: "auth.id != null",
    },
  },
} satisfies InstantRules;

export default rules;
"#;

const INSTANTDB_AUTH_TS: &str = r#"import { db } from "./client";

export type DxInstantMagicCodeRequest = {
  email: string;
};

export type DxInstantMagicCodeVerification = DxInstantMagicCodeRequest & {
  code: string;
};

export function sendInstantLaunchMagicCode(input: DxInstantMagicCodeRequest) {
  const email = normalizeInstantEmail(input.email);
  return db.auth.sendMagicCode({ email });
}

export function verifyInstantLaunchMagicCode(input: DxInstantMagicCodeVerification) {
  const email = normalizeInstantEmail(input.email);
  const code = input.code.trim();
  if (!code) {
    throw new Error("InstantDB magic code is required");
  }

  return db.auth.signInWithMagicCode({ email, code });
}

export function signInInstantLaunchGuest() {
  return db.auth.signInAsGuest();
}

export function signOutInstantLaunchUser() {
  return db.auth.signOut();
}

export function getInstantLaunchAuth() {
  return db.getAuth();
}

export async function requireInstantLaunchUser() {
  const user = await getInstantLaunchAuth();
  if (!user) {
    throw new Error("InstantDB authenticated user is required");
  }

  return user;
}

function normalizeInstantEmail(email: string) {
  const normalized = email.trim().toLowerCase();
  if (!normalized) {
    throw new Error("InstantDB auth email is required");
  }

  return normalized;
}
"#;

const INSTANTDB_OAUTH_TS: &str = r#"import { db } from "./client";

export type DxInstantOAuthUrlInput = {
  clientName: string;
  redirectURL: string;
  extraFields?: Record<string, unknown>;
};

export type DxInstantIdTokenInput = {
  clientName: string;
  idToken: string;
  nonce?: string;
};

export type DxInstantOAuthCodeInput = {
  code: string;
  codeVerifier?: string;
};

export function createInstantLaunchAuthorizationUrl(input: DxInstantOAuthUrlInput) {
  return db.auth.createAuthorizationURL(input);
}

export function signInInstantLaunchWithIdToken(input: DxInstantIdTokenInput) {
  return db.auth.signInWithIdToken(input);
}

export function exchangeInstantLaunchOAuthCode(input: DxInstantOAuthCodeInput) {
  return db.auth.exchangeOAuthCode(input);
}

export function signInInstantLaunchWithToken(token: string) {
  return db.auth.signInWithToken(token);
}

export function instantLaunchIssuerUri() {
  return db.auth.issuerURI();
}
"#;

const INSTANTDB_STORAGE_TS: &str = r#"import { lookup } from "@instantdb/react";
import { db } from "./client";

export type DxInstantStorageUploadInput = {
  path: string;
  file: File | Blob;
  contentType?: string;
  contentDisposition?: string;
};

export type DxInstantFileRecord = {
  id: string;
  path: string;
  url?: string;
  size?: number;
  contentType?: string;
  createdAt?: Date | string;
  updatedAt?: Date | string;
  [key: string]: unknown;
};

type DxInstantFileQuery = {
  $files: {
    $: {
      where: {
        path: string;
      };
    };
  };
};

type DxInstantFileQueryResult = {
  data: {
    $files: DxInstantFileRecord[];
  };
};

type DxInstantFileClient = {
  queryOnce(query: DxInstantFileQuery): Promise<DxInstantFileQueryResult>;
  transact(chunk: unknown): Promise<unknown>;
  tx: {
    $files: Record<string, { delete(): unknown }>;
  };
};

function fileClient() {
  return db as unknown as DxInstantFileClient;
}

export function instantLaunchStoragePath(fileName: string) {
  const safeName = fileName.trim().replaceAll("\\", "/").split("/").filter(Boolean).join("-");
  if (!safeName) {
    throw new Error("InstantDB storage file name is required");
  }

  return `launch/${safeName}`;
}

export async function uploadInstantLaunchFile(input: DxInstantStorageUploadInput) {
  return db.storage.uploadFile(input.path, input.file, {
    contentType: input.contentType,
    contentDisposition: input.contentDisposition,
  });
}

export function instantLaunchFileQuery(path: string): DxInstantFileQuery {
  return {
    $files: {
      $: {
        where: {
          path,
        },
      },
    },
  };
}

export async function queryInstantLaunchFile(path: string) {
  const result = await fileClient().queryOnce(instantLaunchFileQuery(path));
  return result.data.$files[0] ?? null;
}

export async function deleteInstantLaunchFile(path: string) {
  return fileClient().transact(
    fileClient().tx.$files[lookup("path", path) as unknown as string].delete(),
  );
}
"#;

const INSTANTDB_STREAMS_TS: &str = r#"import { db } from "./client";

export type DxInstantStreamTarget = {
  clientId: string;
  streamId?: string;
};

export function createInstantLaunchWriteStream(target: DxInstantStreamTarget) {
  return db.streams.createWriteStream({
    clientId: target.clientId,
  });
}

export function createInstantLaunchReadStream(target: DxInstantStreamTarget) {
  return db.streams.createReadStream({
    clientId: target.clientId,
    streamId: target.streamId,
  });
}

export async function writeInstantLaunchText(
  target: DxInstantStreamTarget,
  text: string,
) {
  const stream = createInstantLaunchWriteStream(target);
  const writer = stream.getWriter();
  try {
    await writer.write(text);
  } finally {
    await writer.close();
  }
}
"#;

const INSTANTDB_SYNC_TABLE_TS: &str = r#"import {
  SyncTableCallbackEventType,
  type StoreInterfaceStoreName,
  type SyncTableCallbackEvent,
} from "@instantdb/react";

import { db } from "./client";
import { instantLaunchTodosQuery } from "./queries";
import type { InstantLaunchSchema } from "./schema";

export type InstantLaunchTodosSyncQuery = typeof instantLaunchTodosQuery;
export type DxInstantSyncStoreName = StoreInterfaceStoreName;

export type DxInstantSyncTableEvent = SyncTableCallbackEvent<
  InstantLaunchSchema,
  InstantLaunchTodosSyncQuery,
  true
>;

export type DxInstantSyncTableCallback = (
  event: DxInstantSyncTableEvent,
) => void;

export type DxInstantSyncTableUnsubscribe = (
  opts?: { keepSubscription?: boolean | null } | null,
) => void;

export function subscribeInstantLaunchSyncTable(
  callback: DxInstantSyncTableCallback,
): DxInstantSyncTableUnsubscribe {
  const query = instantLaunchTodosQuery;
  return db.core._syncTableExperimental(
    query,
    callback,
  ) as DxInstantSyncTableUnsubscribe;
}

export function summarizeInstantLaunchSyncTableEvent(
  event: DxInstantSyncTableEvent,
) {
  switch (event.type) {
    case SyncTableCallbackEventType.InitialSyncBatch:
      return `Initial sync loaded ${event.batch.length} todos`;
    case SyncTableCallbackEventType.InitialSyncComplete:
      return "Initial sync complete";
    case SyncTableCallbackEventType.LoadFromStorage:
      return `Loaded ${event.data.todos.length} todos from local storage`;
    case SyncTableCallbackEventType.SyncTransaction:
      return `Synced ${event.added.length} added, ${event.updated.length} updated, ${event.removed.length} removed todos`;
    case SyncTableCallbackEventType.Error:
      return `Sync Table error: ${event.error.message}`;
    default:
      return assertNeverInstantSyncTableEvent(event);
  }
}

function assertNeverInstantSyncTableEvent(event: never): never {
  throw new Error(`Unknown InstantDB Sync Table event: ${String(event)}`);
}
"#;

const INSTANTDB_DASHBOARD_WORKFLOW_TS: &str = r#"export type InstantDashboardSurfaceId =
  | "realtime-todos"
  | "presence-room"
  | "auth-storage-streams"
  | "sync-table-events";

export type InstantDashboardSurface = {
  id: InstantDashboardSurfaceId;
  label: string;
  publicApi: string;
  dashboardUse: string;
  appBoundary: string;
};

export type InstantDashboardReceipt = {
  packageId: "instantdb/react";
  surfaceId: InstantDashboardSurfaceId;
  status: "missing-config";
  receiptId: string;
  nextAction: string;
};

export type InstantDashboardDxCheckStatus =
  | "present"
  | "stale"
  | "missing-receipt"
  | "blocked"
  | "unsupported-surface";

export type InstantDashboardDxCheckVisibility = {
  schema: "dx.forge.package.dx_check_visibility";
  officialPackageName: "Realtime App Database";
  packageId: "instantdb/react";
  currentStatus: InstantDashboardDxCheckStatus;
  receiptStatus: InstantDashboardDxCheckStatus;
  receiptPath: string;
  statusLegend: readonly {
    status: InstantDashboardDxCheckStatus;
    meaning: string;
  }[];
  monitoredSurfaces: readonly {
    surfaceId: string;
    status: InstantDashboardDxCheckStatus;
    receiptPath: string;
    files: readonly string[];
    sourceMarkers: readonly string[];
  }[];
};

export const instantDashboardInspectedSourceFiles = [
  "client/packages/react/package.json",
  "client/packages/react/src/index.ts",
  "client/packages/core/src/index.ts",
  "client/packages/react-common/src/InstantReactAbstractDatabase.tsx",
  "client/sandbox/react-nextjs/pages/play/sync-table.tsx",
] as const;

export const instantDashboardSelectedSurfaces = [
  "realtime-todos",
  "presence-room",
  "auth-storage-streams",
  "sync-table-events",
  "dashboard-workflow",
] as const;

export const instantDashboardDxCheckVisibility = {
  schema: "dx.forge.package.dx_check_visibility",
  officialPackageName: "Realtime App Database",
  packageId: "instantdb/react",
  currentStatus: "present",
  receiptStatus: "present",
  receiptPath:
    "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
  statusLegend: [
    {
      status: "present",
      meaning:
        "selected Realtime App Database surfaces, source markers, and receipt are present",
    },
    {
      status: "stale",
      meaning:
        "materialized Realtime App Database files or hashes no longer match the receipt",
    },
    {
      status: "missing-receipt",
      meaning:
        "selected Realtime App Database surfaces exist without the dashboard workflow receipt",
    },
    {
      status: "blocked",
      meaning:
        "app-owned Instant configuration or hosted runtime proof is required before claiming more",
    },
    {
      status: "unsupported-surface",
      meaning:
        "a requested Realtime App Database surface is outside the selected upstream-backed set",
    },
  ],
  monitoredSurfaces: [
    {
      surfaceId: "instantdb-runtime-dashboard-workflow",
      status: "present",
      receiptPath:
        "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
      files: [
        "tools/launch/runtime-template/pages/index.html",
        "tools/launch/runtime-template/assets/launch-runtime.ts",
      ],
      sourceMarkers: [
        "data-dx-package=\"instantdb/react\"",
        "data-dx-component=\"instantdb-runtime-dashboard-workflow\"",
        "data-dx-instant-action=\"prepare-local-schema-receipt\"",
      ],
    },
    {
      surfaceId: "dashboard-instantdb-workflow",
      status: "present",
      receiptPath:
        "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
      files: [
        "examples/dashboard/src/lib/instantdbDashboard.ts",
        "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx",
        "components/dashboard/instantdb-dashboard-workflow.tsx",
      ],
      sourceMarkers: [
        "data-dx-package=\"instantdb/react\"",
        "data-dx-component=\"dashboard-instantdb-workflow\"",
        "data-dx-instant-dashboard-workflow=\"realtime-boundary\"",
      ],
    },
    {
      surfaceId: "sync-table-events",
      status: "present",
      receiptPath:
        "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
      files: ["lib/instant/sync-table.ts"],
      sourceMarkers: [
        "SyncTableCallbackEventType",
        "StoreInterfaceStoreName",
        "db.core._syncTableExperimental",
      ],
    },
  ],
} as const satisfies InstantDashboardDxCheckVisibility;

export const instantDashboardPackage = {
  packageId: "instantdb/react",
  officialPackageName: "Realtime App Database",
  aliases: ["@instantdb/react", "instantdb", "db/instantdb"],
  upstreamPackage: "@instantdb/react",
  upstreamVersion: "0.0.0",
  sourceMirror: "G:/WWW/inspirations/instantdb",
  requiredEnv: ["NEXT_PUBLIC_INSTANT_APP_ID"],
  inspectedSourceFiles: instantDashboardInspectedSourceFiles,
  selectedSurfaces: instantDashboardSelectedSurfaces,
  dxCheckVisibility: instantDashboardDxCheckVisibility,
  honestyLabel: "ADAPTER-BOUNDARY",
  exportedFiles: [
    "lib/instant/client.ts",
    "lib/instant/schema.ts",
    "lib/instant/sync-table.ts",
    "lib/instant/dashboard-workflow.ts",
    "components/dashboard/instantdb-dashboard-workflow.tsx",
    "components/launch/instantdb-status.tsx",
  ],
  receiptPaths: [
    ".dx/forge/docs/instantdb-react.md",
    ".dx/forge/receipts/*-instantdb-react.json",
    "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
  ],
  provenance:
    "Inspected the local InstantDB React export map, init(), i.schema, room hooks, auth, storage, streams, SyncTableCallbackEventType, db.core._syncTableExperimental, and Next SSR helpers before exposing this dashboard workflow.",
  appOwnedBoundaries: [
    "Instant dashboard app id",
    "rules and auth policy",
    "production schema and unique indexes",
    "file access rules",
    "stream lifecycle and topic payload policy",
    "experimental Sync Table subscriptions and local store retention",
  ],
} as const;

export const instantDashboardSurfaces: readonly InstantDashboardSurface[] = [
  {
    id: "realtime-todos",
    label: "Realtime todos",
    publicApi: "init + i.schema + db.useQuery + db.transact + db.tx",
    dashboardUse:
      "Read and mutate launch todo state after the app owns the Instant app id and rules.",
    appBoundary: "NEXT_PUBLIC_INSTANT_APP_ID and deployed schema",
  },
  {
    id: "presence-room",
    label: "Presence room",
    publicApi:
      "db.room + db.rooms.usePresence + db.rooms.useSyncPresence + db.rooms.useTypingIndicator",
    dashboardUse:
      "Show dashboard reviewers, typing readiness, and collaborative cursor state.",
    appBoundary: "room naming, payload policy, and authenticated visibility",
  },
  {
    id: "auth-storage-streams",
    label: "Auth, storage, streams",
    publicApi:
      "db.auth + db.storage.uploadFile + db.streams.createWriteStream + createInstantRouteHandler",
    dashboardUse:
      "Expose account, file, stream, and first-party route readiness without browser secrets.",
    appBoundary: "auth providers, file rules, stream retention, and route mounting",
  },
  {
    id: "sync-table-events",
    label: "Sync Table events",
    publicApi:
      "SyncTableCallbackEventType + db.core._syncTableExperimental + StoreInterfaceStoreName",
    dashboardUse:
      "Surface source-owned event summaries for local table sync without claiming hosted execution.",
    appBoundary:
      "local persistence store, subscription lifetime, and runtime validation",
  },
];

export function getInstantDashboardSurface(
  surfaceId: InstantDashboardSurfaceId,
): InstantDashboardSurface {
  return (
    instantDashboardSurfaces.find((surface) => surface.id === surfaceId) ??
    instantDashboardSurfaces[0]
  );
}

export function createInstantDashboardReceipt(
  surfaceId: InstantDashboardSurfaceId,
): InstantDashboardReceipt {
  return {
    packageId: instantDashboardPackage.packageId,
    surfaceId,
    status: "missing-config",
    receiptId: `instantdb-dashboard-${surfaceId}`,
    nextAction:
      "Create the Instant app, set NEXT_PUBLIC_INSTANT_APP_ID, review rules, then pass the real db client into the dashboard workflow.",
  };
}
"#;

const INSTANTDB_ROUTE_TS: &str = r#"import { createInstantRouteHandler } from "@instantdb/react";

import { readInstantConfig, type DxInstantEnv } from "./env";

function createInstantMissingConfigResponse(error: unknown) {
  const message =
    error instanceof Error
      ? error.message
      : "Missing required InstantDB env var NEXT_PUBLIC_INSTANT_APP_ID";

  return Response.json(
    {
      schema: "dx.www.template.instantdb_route_handler",
      status: "provider-gated",
      packageId: "instantdb/react",
      missingEnv: ["NEXT_PUBLIC_INSTANT_APP_ID"],
      runtimeProof: false,
      networkCalls: false,
      hostedCredentials: false,
      error: message,
      boundary:
        "Configure NEXT_PUBLIC_INSTANT_APP_ID before enabling the hosted InstantDB route handler.",
    },
    { status: 501, headers: { "cache-control": "no-store" } },
  );
}

export function createDxInstantRouteHandlers(env?: DxInstantEnv) {
  try {
    const { appId } = readInstantConfig(env);
    return createInstantRouteHandler({ appId });
  } catch (error) {
    return {
      GET: () => createInstantMissingConfigResponse(error),
      POST: () => createInstantMissingConfigResponse(error),
    };
  }
}
"#;

const INSTANTDB_API_INSTANT_ROUTE_TS: &str = r#"import { createDxInstantRouteHandlers } from "@/lib/instant/route";

export const { GET, POST } = createDxInstantRouteHandlers();
"#;

const INSTANTDB_METADATA_TS: &str = r#"export const dxInstantDbForgePackage = {
  packageId: "instantdb/react",
  officialPackageName: "Realtime App Database",
  aliases: ["@instantdb/react", "instantdb", "db/instantdb"],
  upstreamPackage: "@instantdb/react",
  upstreamVersion: "0.0.0",
  upstreamCorePackage: "@instantdb/core",
  forgeVersion: "0.0.0-dx.0",
  sourceMirror: "G:/WWW/inspirations/instantdb",
  provenance:
    "DX inspected the local InstantDB React package export map, react-common room hooks, storage/stream APIs, SyncTableCallbackEventType, db.core._syncTableExperimental, and Next SSR helpers before curating this source-owned package slice.",
  inspectedSourceFiles: [
    "client/packages/react/package.json",
    "client/packages/react/src/index.ts",
    "client/packages/core/src/index.ts",
    "client/packages/react-common/src/InstantReactAbstractDatabase.tsx",
    "client/sandbox/react-nextjs/pages/play/sync-table.tsx",
  ],
  selectedSurfaces: [
    "realtime-todos",
    "presence-room",
    "auth-storage-streams",
    "sync-table-events",
    "dashboard-workflow",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: [
      "present",
      "stale",
      "missing-receipt",
      "blocked",
      "unsupported-surface",
    ],
    receiptPath: "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
    monitoredSurfaces: [
      "instantdb-runtime-dashboard-workflow",
      "dashboard-instantdb-workflow",
      "sync-table-events",
    ],
  },
  honestyLabel: "ADAPTER-BOUNDARY",
  runtimeLimitations: [
    "ADAPTER-BOUNDARY: hosted realtime behavior requires app-owned Instant configuration.",
    "SOURCE-ONLY: Sync Table event helpers are generated from upstream source, but hosted runtime proof is deferred.",
  ],
  sourceSurface: [
    "init",
    "@instantdb/react/nextjs init",
    "InstantConfig.apiURI",
    "InstantConfig.devtool",
    "InstantConfig.disableValidation",
    "InstantConfig.firstPartyPath",
    "InstantConfig.queryCacheLimit",
    "InstantConfig.websocketURI",
    "InstantConfig.verbose",
    "InstantSuspenseProvider",
    "getUnverifiedUserFromInstantCookie",
    "i.schema",
    "InstantRules",
    "id",
    "lookup",
    "links",
    "db.useQuery",
    "db.useSuspenseQuery",
    "db.queryOnce",
    "db.useLocalId",
    "db.getLocalId",
    "db.useConnectionStatus",
    "db.core.subscribeQuery",
    "db.useInfiniteQuery",
    "db.core.subscribeInfiniteQuery",
    "db.core.subscribeAuth",
    "db.core.subscribeConnectionStatus",
    "db.transact",
    "db.transact([...])",
    "db.tx",
    "db.tx.*.create",
    "db.tx.*.update(..., { upsert: false })",
    "db.tx.*.link",
    "db.tx.*.unlink",
    "db.tx.*.merge",
    "db.tx.*.ruleParams",
    "db.room",
    "db.rooms.usePresence",
    "db.rooms.useSyncPresence",
    "Cursors",
    "db.rooms.useTopicEffect",
    "db.rooms.usePublishTopic",
    "db.rooms.useTypingIndicator",
    "db.useAuth",
    "db.useUser",
    "db.getAuth",
    "db.SignedIn",
    "db.SignedOut",
    "db.auth",
    "db.auth.sendMagicCode",
    "db.auth.signInWithMagicCode",
    "db.auth.signInAsGuest",
    "db.auth.signOut",
    "db.auth.createAuthorizationURL",
    "db.auth.signInWithIdToken",
    "db.auth.exchangeOAuthCode",
    "db.auth.signInWithToken",
    "db.auth.issuerURI",
    "db.storage",
    "db.storage.uploadFile",
    "db.queryOnce({ $files })",
    "db.tx.$files[lookup(\"path\", path)].delete()",
    "db.streams",
    "db.streams.createReadStream",
    "db.streams.createWriteStream",
    "SyncTableCallbackEventType",
    "SyncTableCallbackEvent",
    "StoreInterfaceStoreName",
    "db.core._syncTableExperimental",
    "createInstantRouteHandler",
    "InstantAPIError",
    "InstantError",
    "setInstantWarningsEnabled",
  ],
  env: [
    "NEXT_PUBLIC_INSTANT_APP_ID",
    "NEXT_PUBLIC_INSTANT_API_URI",
    "NEXT_PUBLIC_INSTANT_DEVTOOL",
    "NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION",
    "NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH",
    "NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT",
    "NEXT_PUBLIC_INSTANT_WEBSOCKET_URI",
    "NEXT_PUBLIC_INSTANT_VERBOSE",
  ],
  requiredEnv: ["NEXT_PUBLIC_INSTANT_APP_ID"],
  requiredDependencies: [
    {
      name: "@instantdb/react",
      version: "^0",
      required: true,
    },
    {
      name: "react",
      version: "^18 || ^19",
      required: true,
    },
  ],
  materializedFiles: [
    "lib/instant/env.ts",
    "lib/instant/schema.ts",
    "lib/instant/client.ts",
    "lib/instant/next-client.tsx",
    "lib/instant/next-server.ts",
    "lib/instant/queries.ts",
    "lib/instant/status.ts",
    "lib/instant/subscriptions.ts",
    "lib/instant/pagination.ts",
    "lib/instant/diagnostics.ts",
    "lib/instant/mutations.ts",
    "lib/instant/rules.ts",
    "lib/instant/perms.ts",
    "lib/instant/auth.ts",
    "lib/instant/oauth.ts",
    "lib/instant/storage.ts",
    "lib/instant/streams.ts",
    "lib/instant/sync-table.ts",
    "lib/instant/dashboard-workflow.ts",
    "lib/instant/route.ts",
    "lib/instant/metadata.ts",
    "components/instant/instant-todos.tsx",
    "components/instant/instant-cursors.tsx",
    "components/instant/instant-auth-boundary.tsx",
    "components/dashboard/instantdb-dashboard-workflow.tsx",
    "app/api/instant/route.ts",
    "app/instant/page.tsx",
  ],
  exportedFiles: [
    "lib/instant/env.ts",
    "lib/instant/schema.ts",
    "lib/instant/client.ts",
    "lib/instant/next-client.tsx",
    "lib/instant/next-server.ts",
    "lib/instant/queries.ts",
    "lib/instant/status.ts",
    "lib/instant/subscriptions.ts",
    "lib/instant/pagination.ts",
    "lib/instant/diagnostics.ts",
    "lib/instant/mutations.ts",
    "lib/instant/rules.ts",
    "lib/instant/perms.ts",
    "lib/instant/auth.ts",
    "lib/instant/oauth.ts",
    "lib/instant/storage.ts",
    "lib/instant/streams.ts",
    "lib/instant/sync-table.ts",
    "lib/instant/dashboard-workflow.ts",
    "lib/instant/route.ts",
    "lib/instant/metadata.ts",
    "components/instant/instant-todos.tsx",
    "components/instant/instant-cursors.tsx",
    "components/instant/instant-auth-boundary.tsx",
    "components/dashboard/instantdb-dashboard-workflow.tsx",
    "components/launch/instantdb-status.tsx",
    "examples/dashboard/src/lib/instantdbDashboard.ts",
    "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx",
    "tools/launch/runtime-template/pages/index.html#instantdb-runtime-dashboard-workflow",
    "tools/launch/runtime-template/assets/launch-runtime.ts#bindInstantDbRuntimeProof",
  ],
  receiptPaths: [
    ".dx/forge/docs/instantdb-react.md",
    ".dx/forge/receipts/*-instantdb-react.json",
    "docs/packages/instantdb-react.md",
  ],
  appOwnedBoundaries: [
    "Instant dashboard app provisioning",
    "Rules and auth policy",
    "Production schema and unique indexes",
    "File access rules and storage retention",
    "Stream lifecycle and topic payload policy",
    "Experimental Sync Table subscriptions and local store retention",
    "NEXT_PUBLIC_INSTANT_APP_ID and optional endpoint overrides",
  ],
  discovery: {
    dxAdd: "dx add instantdb/react --write",
    config: "readInstantConfig(env)",
    clientFactory: "createDxInstantClient()",
    nextClientFactory: "createDxInstantNextClient()",
    suspenseProvider: "InstantLaunchSuspenseProvider",
    suspenseQuery: "useInstantLaunchTodosSuspense()",
    ssrUser: "getInstantLaunchSsrUser()",
    clientCapabilities: "db.auth + db.storage + db.streams",
    realtimeHook: "db.useQuery({ todos: {} })",
    queryOnce: "queryInstantLaunchTodosSnapshot()",
    localId: "useInstantLaunchDeviceId()",
    connectionStatus: "useInstantLaunchConnectionStatus()",
    deviceId: "getInstantLaunchDeviceId()",
    subscriptions: "subscribeInstantLaunchTodos(callback)",
    pagination: "useInstantLaunchTodosInfinite(pageSize)",
    diagnostics: "formatInstantLaunchError(error)",
    ruleParams: "queryInstantLaunchTodosWithRuleParams(ruleParams)",
    permissions: "rules satisfies InstantRules",
    lookupMutation: "updateInstantLaunchTodoByText(text, done)",
    cursorsComponent: "InstantLaunchCursors",
    presenceSync: "db.rooms.useSyncPresence(launchRoom, presence)",
    topicPublish: "db.rooms.usePublishTopic(launchRoom, \"launchPing\")",
    topicEffect: "db.rooms.useTopicEffect(launchRoom, \"launchPing\", handler)",
    typingIndicator: "db.rooms.useTypingIndicator(launchRoom, \"launch-input\")",
    authState: "db.useAuth()",
    authSnapshot: "getInstantLaunchAuth()",
    authBoundary: "InstantLaunchAuthBoundary + InstantLaunchUserBadge",
    authHelpers: "sendInstantLaunchMagicCode(email) + verifyInstantLaunchMagicCode(code)",
    oauthHelpers: "createInstantLaunchAuthorizationUrl(clientName, redirectURL)",
    storageHelper: "uploadInstantLaunchFile(file)",
    fileLookupHelper: "queryInstantLaunchFile(path)",
    streamHelper: "createInstantLaunchWriteStream(clientId)",
    syncTable: "subscribeInstantLaunchSyncTable(callback)",
    routeHandler: "createDxInstantRouteHandlers().POST(request)",
    firstPartyRoute: "app/api/instant/route.ts",
    mutationSurface: "db.transact(db.tx.todos[id()].create(...))",
    strictUpdateMutation: "toggleInstantTodo(todo)",
    batchMutation: "clearCompletedInstantTodos(todos)",
    bulkStrictUpdateMutation: "toggleAllInstantTodos(todos)",
    linkMutation: "labelInstantTodoForLaunch(todo)",
    mergeMutation: "mergeInstantTodoLaunchDetails(todo)",
  },
} as const;

export type DxInstantDbForgePackage = typeof dxInstantDbForgePackage;
"#;

const INSTANTDB_TODOS_TSX: &str = r#""use client";

import * as React from "react";

import { InstantLaunchUserBadge } from "@/components/instant/instant-auth-boundary";
import { db, instantCapabilities, launchRoom } from "@/lib/instant/client";
import {
  addInstantTodo,
  clearCompletedInstantTodos,
  deleteInstantTodo,
  instantLaunchLabelName,
  labelInstantTodoForLaunch,
  mergeInstantTodoLaunchDetails,
  toggleInstantTodo,
  toggleAllInstantTodos,
  unlabelInstantTodoForLaunch,
  type InstantTodo,
} from "@/lib/instant/mutations";
import { instantLaunchTodosQuery, useInstantLaunchDeviceId } from "@/lib/instant/queries";

export function InstantLaunchTodos() {
  const [text, setText] = React.useState("");
  const [lastPing, setLastPing] = React.useState("No topic pings yet");
  const query = db.useQuery(instantLaunchTodosQuery);
  const auth = db.useAuth();
  const deviceId = useInstantLaunchDeviceId();
  const presenceName = auth.user?.email?.split("@")[0] ?? deviceId ?? "guest";
  db.rooms.useSyncPresence(launchRoom, { name: presenceName }, [presenceName]);
  const { peers } = db.rooms.usePresence(launchRoom);
  const publishLaunchPing = db.rooms.usePublishTopic(launchRoom, "launchPing");
  const typing = db.rooms.useTypingIndicator(launchRoom, "launch-input", {
    stopOnEnter: true,
  });

  db.rooms.useTopicEffect(launchRoom, "launchPing", (event) => {
    setLastPing(`${event.message} at ${new Date(event.sentAt).toLocaleTimeString()}`);
  });

  function submit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    addInstantTodo(text);
    setText("");
  }

  if (query.isLoading) {
    return <p className="text-sm text-muted-foreground">Loading realtime todos...</p>;
  }

  if (query.error) {
    return <p role="alert" className="text-sm text-destructive">{query.error.message}</p>;
  }

  const todos = query.data.todos;
  const onlineCount = Object.keys(peers).length + 1;
  const peerNames = Object.values(peers)
    .map((peer) => peer.name)
    .filter((name): name is string => Boolean(name));
  const typingCount = typing.active.length;

  return (
    <section className="grid gap-4">
      <div className="flex items-center justify-between gap-3">
        <h2 className="text-lg font-semibold">Realtime launch todos</h2>
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <span className="rounded-md border px-2 py-1">{onlineCount} online</span>
          <span className="rounded-md border px-2 py-1">
            <InstantLaunchUserBadge fallback={auth.user ? "signed in" : "guest"} />
          </span>
          <span className="rounded-md border px-2 py-1">
            {deviceId ? "device ready" : "device pending"}
          </span>
        </div>
      </div>
      <p className="text-xs text-muted-foreground">
        Auth, storage, and streams are available from the real InstantDB client:
        {instantCapabilities.auth && instantCapabilities.storage && instantCapabilities.streams
          ? " ready"
          : " unavailable"}
      </p>
      <div className="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
        <button
          className="rounded-md border px-2 py-1 hover:text-foreground"
          onClick={() =>
            publishLaunchPing({
              message: "Launch ping",
              sentAt: Date.now(),
            })
          }
          type="button"
        >
          Ping room
        </button>
        <span>{lastPing}</span>
        {peerNames.length > 0 ? <span>Peers: {peerNames.join(", ")}</span> : null}
        <span>{typingCount} typing</span>
      </div>
      <form className="flex gap-2" onSubmit={submit}>
        <input
          className="min-w-0 flex-1 rounded-md border px-3 py-2 text-sm outline-none"
          name="todo"
          onChange={(event) => setText(event.currentTarget.value)}
          onBlur={typing.inputProps.onBlur}
          onKeyDown={typing.inputProps.onKeyDown}
          placeholder="Add a launch task"
          value={text}
        />
        <button
          className="rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50"
          disabled={!text.trim()}
          type="submit"
        >
          Add
        </button>
      </form>
      <div className="flex justify-end">
        <button
          className="rounded-md border px-2 py-1 text-xs text-muted-foreground hover:text-foreground disabled:opacity-50"
          disabled={todos.length === 0}
          onClick={() => toggleAllInstantTodos(todos)}
          type="button"
        >
          Toggle all
        </button>
        <button
          className="rounded-md border px-2 py-1 text-xs text-muted-foreground hover:text-foreground disabled:opacity-50"
          disabled={!todos.some((todo) => todo.done)}
          onClick={() => clearCompletedInstantTodos(todos)}
          type="button"
        >
          Clear completed
        </button>
      </div>
      <ul className="grid gap-2">
        {todos.map((todo: InstantTodo) => (
          <li
            className="flex items-center justify-between gap-3 rounded-md border px-3 py-2"
            key={todo.id}
          >
            <label className="flex min-w-0 flex-1 items-center gap-2 text-sm">
              <input
                checked={todo.done}
                onChange={() => toggleInstantTodo(todo)}
                type="checkbox"
              />
              <span className={todo.done ? "truncate line-through" : "truncate"}>
                {todo.text}
              </span>
              {todo.labels?.some((label) => label.name === instantLaunchLabelName) ? (
                <span className="rounded-sm border px-1.5 py-0.5 text-[11px] text-muted-foreground">
                  {instantLaunchLabelName}
                </span>
              ) : null}
            </label>
            <div className="flex shrink-0 items-center gap-2">
              {todo.labels?.some((label) => label.name === instantLaunchLabelName) ? (
                <button
                  className="text-xs text-muted-foreground hover:text-foreground"
                  onClick={() => unlabelInstantTodoForLaunch(todo)}
                  type="button"
                >
                  Unmark
                </button>
              ) : (
                <button
                  className="text-xs text-muted-foreground hover:text-foreground"
                  onClick={() => labelInstantTodoForLaunch(todo)}
                  type="button"
                >
                  Mark launch
                </button>
              )}
              <button
                className="text-xs text-muted-foreground hover:text-foreground"
                onClick={() => mergeInstantTodoLaunchDetails(todo)}
                type="button"
              >
                Touch
              </button>
              <button
                className="text-xs text-muted-foreground hover:text-foreground"
                onClick={() => deleteInstantTodo(todo)}
                type="button"
              >
                Delete
              </button>
            </div>
          </li>
        ))}
      </ul>
    </section>
  );
}
"#;

const INSTANTDB_CURSORS_TSX: &str = r##""use client";

import { Cursors } from "@instantdb/react";
import type { ReactNode } from "react";

import { launchRoom } from "@/lib/instant/client";

export type InstantLaunchCursorsProps = {
  children: ReactNode;
  className?: string;
  color?: string;
  spaceId?: string;
};

export const DX_INSTANT_CURSOR_COLOR_TOKEN =
  "var(--dx-instant-cursor-color, var(--dx-ui-primary-bg))";

export function InstantLaunchCursors({
  children,
  className,
  color = DX_INSTANT_CURSOR_COLOR_TOKEN,
  spaceId = "dx-launch-cursors",
}: InstantLaunchCursorsProps) {
  return (
    <Cursors
      className={className}
      room={launchRoom}
      spaceId={spaceId}
      userCursorColor={color}
    >
      {children}
    </Cursors>
  );
}
"##;

const INSTANTDB_AUTH_BOUNDARY_TSX: &str = r#""use client";

import type { ReactNode } from "react";

import { db } from "@/lib/instant/client";

type InstantLaunchUser = ReturnType<typeof db.useUser>;

export type InstantLaunchAuthBoundaryProps = {
  children: ReactNode;
  signedOut?: ReactNode;
};

export type InstantLaunchSignedInProps = {
  children: (user: InstantLaunchUser) => ReactNode;
  fallback?: ReactNode;
};

export type InstantLaunchUserBadgeProps = {
  fallback?: ReactNode;
};

export function InstantLaunchAuthBoundary({
  children,
  signedOut = null,
}: InstantLaunchAuthBoundaryProps) {
  return (
    <>
      <db.SignedIn>{children}</db.SignedIn>
      <db.SignedOut>{signedOut}</db.SignedOut>
    </>
  );
}

export function InstantLaunchSignedIn({
  children,
  fallback = null,
}: InstantLaunchSignedInProps) {
  return (
    <InstantLaunchAuthBoundary signedOut={fallback}>
      <InstantLaunchUser>{children}</InstantLaunchUser>
    </InstantLaunchAuthBoundary>
  );
}

export function InstantLaunchUserBadge({
  fallback = "guest",
}: InstantLaunchUserBadgeProps) {
  return (
    <InstantLaunchSignedIn fallback={fallback}>
      {(user) => <span>{user.email ?? user.id}</span>}
    </InstantLaunchSignedIn>
  );
}

function InstantLaunchUser({
  children,
}: {
  children: (user: InstantLaunchUser) => ReactNode;
}) {
  const user = db.useUser();
  return <>{children(user)}</>;
}
"#;

const INSTANTDB_DASHBOARD_WORKFLOW_TSX: &str = r#""use client";

import * as React from "react";

import {
  createInstantDashboardReceipt,
  getInstantDashboardSurface,
  instantDashboardPackage,
  instantDashboardSurfaces,
  type InstantDashboardReceipt,
  type InstantDashboardSurfaceId,
} from "@/lib/instant/dashboard-workflow";

type DxIconProps = React.HTMLAttributes<HTMLElement> & {
  name: string;
};

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "dx-icon": DxIconProps;
    }
  }
}

export function InstantDbDashboardWorkflow() {
  const [surfaceId, setSurfaceId] =
    React.useState<InstantDashboardSurfaceId>("realtime-todos");
  const [receipt, setReceipt] = React.useState<InstantDashboardReceipt | null>(
    null,
  );
  const activeSurface = getInstantDashboardSurface(surfaceId);

  return (
    <section
      className="grid gap-4 rounded-md border bg-card p-4 text-card-foreground"
      data-dx-package="instantdb/react"
      data-dx-component="dashboard-instantdb-workflow"
      data-dx-instant-dashboard-workflow="realtime-boundary"
      data-dx-instant-dashboard-surface={activeSurface.id}
      data-dx-instant-dashboard-status={receipt ? receipt.status : "missing-config"}
      data-dx-instant-dashboard-dx-check-schema={
        instantDashboardPackage.dxCheckVisibility.schema
      }
      data-dx-instant-dashboard-dx-check-status={
        instantDashboardPackage.dxCheckVisibility.currentStatus
      }
      data-dx-node-modules="forbidden"
    >
      <header className="flex items-start gap-3">
        <dx-icon name="pack:database" aria-label="Realtime App Database" />
        <div className="grid gap-1">
          <h2 className="text-base font-semibold">Realtime App Database workflow</h2>
          <p className="text-sm text-muted-foreground">
            Prepare realtime todos, room presence, auth, storage, streams, and
            first-party routes behind app-owned Instant credentials.
          </p>
        </div>
      </header>

      <div
        className="flex flex-wrap gap-2"
        data-dx-instant-dashboard-interaction="surface-picker"
      >
        {instantDashboardSurfaces.map((surface) => (
          <button
            key={surface.id}
            type="button"
            className="rounded-md border bg-background px-3 py-2 text-sm text-foreground"
            data-dx-instant-dashboard-action="select-surface"
            data-dx-instant-dashboard-option={surface.id}
            data-dx-instant-dashboard-selected={
              surface.id === activeSurface.id ? "true" : "false"
            }
            onClick={() => {
              setSurfaceId(surface.id);
              setReceipt(null);
            }}
          >
            {surface.label}
          </button>
        ))}
      </div>

      <dl
        className="grid gap-3 text-sm"
        data-dx-instant-dashboard-readiness="source-owned"
      >
        <div>
          <dt className="text-muted-foreground">Public API</dt>
          <dd data-dx-instant-dashboard-public-api={activeSurface.publicApi}>
            {activeSurface.publicApi}
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground">Required env</dt>
          <dd data-dx-instant-dashboard-required-env="NEXT_PUBLIC_INSTANT_APP_ID">
            {instantDashboardPackage.requiredEnv.join(", ")}
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground">App-owned boundary</dt>
          <dd data-dx-instant-dashboard-boundary={activeSurface.appBoundary}>
            {activeSurface.appBoundary}
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground">dx-check</dt>
          <dd
            data-dx-instant-dashboard-dx-check-receipt={
              instantDashboardPackage.dxCheckVisibility.receiptPath
            }
          >
            {instantDashboardPackage.dxCheckVisibility.currentStatus}
          </dd>
        </div>
      </dl>

      <p
        className="rounded-md bg-muted p-3 text-sm text-muted-foreground"
        data-dx-instant-dashboard-source-mirror={instantDashboardPackage.sourceMirror}
      >
        {activeSurface.dashboardUse}
      </p>

      <button
        type="button"
        className="w-fit rounded-md border bg-background px-3 py-2 text-sm font-medium text-foreground"
        data-dx-instant-dashboard-action="prepare-local-receipt"
        onClick={() => setReceipt(createInstantDashboardReceipt(activeSurface.id))}
      >
        Prepare Realtime App Database receipt
      </button>

      <output
        className="rounded-md border bg-background p-3 text-sm text-muted-foreground"
        data-dx-instant-dashboard-receipt={receipt ? receipt.receiptId : "none"}
        data-dx-instant-dashboard-receipt-status={receipt ? receipt.status : "idle"}
        data-dx-instant-dashboard-exported-files={
          instantDashboardPackage.exportedFiles.length
        }
      >
        {receipt
          ? `${receipt.receiptId}: ${receipt.nextAction}`
          : `Source mirror: ${instantDashboardPackage.sourceMirror}. No live Realtime App Database call runs until the app id and rules are configured.`}
      </output>
    </section>
  );
}
"#;

const INSTANTDB_PAGE_TSX: &str = r#"import { InstantLaunchTodos } from "@/components/instant/instant-todos";

export default function InstantLaunchPage() {
  return (
    <main className="mx-auto grid min-h-screen max-w-xl content-center p-6">
      <InstantLaunchTodos />
    </main>
  );
}
"#;

const INSTANTDB_README_MD: &str = r#"# Realtime App Database

Package id: `instantdb/react`. Upstream package: `@instantdb/react`.

This package materializes a small source-owned launch slice around the real `@instantdb/react` public API. It does not reimplement InstantDB networking, storage, transactions, auth, or presence.

## Owned Surface

- `env.ts` reads `NEXT_PUBLIC_INSTANT_APP_ID`.
- `env.ts` also supports optional `NEXT_PUBLIC_INSTANT_API_URI`, `NEXT_PUBLIC_INSTANT_WEBSOCKET_URI`, `NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH`, `NEXT_PUBLIC_INSTANT_DEVTOOL`, `NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION`, `NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT`, and `NEXT_PUBLIC_INSTANT_VERBOSE` overrides for local/runtime endpoint ownership.
- `schema.ts` defines a typed `todos` entity and `launch` presence room with `i.schema`.
- `schema.ts` also includes a `labels` entity and `todoLabels` relationship so the launch template exercises real InstantDB links.
- `client.ts` creates the real InstantDB React client with `init`.
- `client.ts` also exposes `createDxInstantCapabilities` for the real `db.auth`, `db.storage`, and `db.streams` client APIs.
- `next-client.tsx` creates the real `@instantdb/react/nextjs` client, wraps `InstantSuspenseProvider`, and exposes `db.useSuspenseQuery` for DX-WWW App Router templates.
- `next-server.ts` exposes `getUnverifiedUserFromInstantCookie` behind an app-owned `getInstantLaunchSsrUser` helper.
- `queries.ts` exposes the launch todo query, `db.queryOnce`, and `db.useLocalId` for snapshot reads and local device identity.
- `status.ts` exposes `db.useConnectionStatus` and `db.getLocalId` for operational readiness and async device identity reads.
- `subscriptions.ts` exposes `db.core.subscribeQuery`, `db.core.subscribeAuth`, and `db.core.subscribeConnectionStatus` for non-React listeners.
- `pagination.ts` exposes `db.useInfiniteQuery` and `db.core.subscribeInfiniteQuery` helpers for launch todo pagination.
- `diagnostics.ts` exposes `InstantAPIError`, `InstantError`, and `setInstantWarningsEnabled` based helpers for app-owned error handling.
- `mutations.ts` writes through `id`, `lookup`, explicit `.create()`, strict `.update(..., { upsert: false })`, `db.tx`, `db.transact`, relationship `.link()` / `.unlink()`, nested `.merge()`, and batched `db.transact([...])` cleanup.
- `rules.ts` exposes `ruleParams` query/transaction helpers and a `lookup` mutation helper for permission-scoped app flows.
- `perms.ts` provides a small `InstantRules`-typed starter policy for todos and `$files`; apps must replace it with their deployed product rules.
- `auth.ts` wraps real `db.auth.sendMagicCode`, `db.auth.signInWithMagicCode`, `db.auth.signInAsGuest`, and `db.auth.signOut` calls behind app-owned launch helper names.
- `oauth.ts` wraps real `db.auth.createAuthorizationURL`, `db.auth.signInWithIdToken`, `db.auth.exchangeOAuthCode`, `db.auth.signInWithToken`, and `db.auth.issuerURI` calls for app-owned OAuth/provider flows.
- `storage.ts` wraps real `db.storage.uploadFile`, queries `$files` through `db.queryOnce`, and deletes files with the recommended `db.transact(db.tx.$files[lookup("path", path)].delete())` pattern.
- `streams.ts` wraps real `db.streams.createWriteStream` and `db.streams.createReadStream` calls for launch-owned text streams.
- `sync-table.ts` wraps the upstream `SyncTableCallbackEventType`, `SyncTableCallbackEvent`, `StoreInterfaceStoreName`, and `db.core._syncTableExperimental` surface so apps can summarize local sync-table events while runtime proof remains app-owned.
- `route.ts` wraps real `createInstantRouteHandler` so apps can mount the Instant auth sync POST handler in their DX-WWW-owned route tree.
- `app/api/instant/route.ts` mounts the real Instant auth sync POST handler for `NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH=/api/instant`.
- `instant-cursors.tsx` wraps the real `Cursors` component for launch-room collaborative cursor presence.
- `instant-auth-boundary.tsx` wraps real `db.SignedIn`, `db.SignedOut`, and `db.useUser` auth-boundary APIs.
- `auth.ts` also exposes `db.getAuth` for one-time auth reads and a small app-owned `requireInstantLaunchUser` boundary.
- `instant-todos.tsx` reads through `db.useQuery`, checks auth state through `db.useAuth`, reads a local device id through `db.useLocalId`, syncs this client with `db.rooms.useSyncPresence`, tracks peers through `db.rooms.usePresence`, broadcasts launch pings through `db.rooms.usePublishTopic`, receives room events through `db.rooms.useTopicEffect`, and exposes typing state through `db.rooms.useTypingIndicator`.
- `metadata.ts` gives DX CLI, Zed, and launch templates a stable discovery record.
- `metadata.ts` records the `instantdb/react` package id, aliases, source mirror, curated provenance, exported files, materialized files, required env, app-owned boundaries, and receipt paths.
- `app/instant/page.tsx` is a tiny template mount point.
- The `/launch` dashboard proof in `examples/template/template-shell.tsx` mounts `examples/template/instantdb-status.tsx` as `data-dx-component="launch-instantdb-dashboard-workflow"` with a visible missing-config workflow and safe local schema receipt action when `NEXT_PUBLIC_INSTANT_APP_ID` is not configured.
- The live runtime bridge in `tools/launch/runtime-template/pages/index.html` exposes `data-dx-component="instantdb-runtime-dashboard-workflow"` and `data-dx-instant-action="prepare-local-schema-receipt"` so the governed browser route can show InstantDB without a template-local `node_modules` folder.
- `lib/instant/dashboard-workflow.ts` and `components/dashboard/instantdb-dashboard-workflow.tsx` materialize a reusable app-local realtime-boundary dashboard surface for generated starters and `dx add instantdb/react --write`.
- `examples/dashboard/src/lib/instantdbDashboard.ts` and `examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx` expose the starter dashboard workflow for realtime todos, presence rooms, auth, storage, streams, route readiness, and a safe local receipt.
- `docs/packages/instantdb-react.md` is the front-facing package handoff for real APIs, dashboard markers, no-node-modules behavior, and app-owned boundaries.

## App-Owned Work

Install and version `@instantdb/react`, create the app in the Instant dashboard, set `NEXT_PUBLIC_INSTANT_APP_ID`, and replace the starter schema with your product schema before production. Applications still own Instant auth flows, endpoint selection, local/runtime endpoint security, OAuth provider setup, redirect validation, token issuance, authenticated route design, SSR layout placement, Suspense fallback design, stream lifecycle, file access rules, storage bucket policy, stream usage, deployed rule definitions, unique lookup attribute design, deployed data access review, experimental Sync Table subscriptions, local store retention, and topic payload policy for production events.
"#;
