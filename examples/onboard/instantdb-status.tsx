"use client";

import * as React from "react";

import type {
  getInstantLaunchAuth,
  requireInstantLaunchUser,
  sendInstantLaunchMagicCode,
  signInInstantLaunchGuest,
  signOutInstantLaunchUser,
  verifyInstantLaunchMagicCode,
} from "@/lib/instant/auth";
import type {
  InstantLaunchAuthBoundary,
  InstantLaunchUserBadge,
} from "@/components/instant/instant-auth-boundary";
import type { InstantLaunchCursors } from "@/components/instant/instant-cursors";
import type {
  db as defaultInstantClient,
  launchRoom as defaultLaunchRoom,
} from "@/lib/instant/client";
import type { formatInstantLaunchError } from "@/lib/instant/diagnostics";
import type { readInstantConfig } from "@/lib/instant/env";
import type {
  InstantLaunchSuspenseProvider,
  useInstantLaunchTodosSuspense,
} from "@/lib/instant/next-client";
import type { getInstantLaunchSsrUser } from "@/lib/instant/next-server";
import type { queryInstantLaunchTodosSnapshot } from "@/lib/instant/queries";
import type { createDxInstantRouteHandlers } from "@/lib/instant/route";
import type {
  queryInstantLaunchTodosWithRuleParams,
  updateInstantLaunchTodoByText,
} from "@/lib/instant/rules";
import type {
  createInstantLaunchAuthorizationUrl,
  exchangeInstantLaunchOAuthCode,
  instantLaunchIssuerUri,
  signInInstantLaunchWithIdToken,
  signInInstantLaunchWithToken,
} from "@/lib/instant/oauth";
import type {
  subscribeInstantLaunchTodosInfinite,
  useInstantLaunchTodosInfinite,
} from "@/lib/instant/pagination";
import type {
  getInstantLaunchDeviceId,
  useInstantLaunchConnectionStatus,
} from "@/lib/instant/status";
import type {
  deleteInstantLaunchFile,
  queryInstantLaunchFile,
  uploadInstantLaunchFile,
} from "@/lib/instant/storage";
import type { createInstantLaunchWriteStream } from "@/lib/instant/streams";
import type {
  subscribeInstantLaunchAuth,
  subscribeInstantLaunchConnectionStatus,
  subscribeInstantLaunchTodos,
} from "@/lib/instant/subscriptions";

type DxInstantClient = typeof defaultInstantClient;
type DxInstantRoom = typeof defaultLaunchRoom;
type LaunchInstantCursors = typeof InstantLaunchCursors;
type LaunchInstantDiagnostics = typeof formatInstantLaunchError;
type LaunchInstantConfigHelpers = {
  readConfig: typeof readInstantConfig;
};
type LaunchInstantAuthHelpers = {
  getAuth: typeof getInstantLaunchAuth;
  requireUser: typeof requireInstantLaunchUser;
  sendMagicCode: typeof sendInstantLaunchMagicCode;
  signInAsGuest: typeof signInInstantLaunchGuest;
  signOut: typeof signOutInstantLaunchUser;
  verifyMagicCode: typeof verifyInstantLaunchMagicCode;
};
type LaunchInstantAuthBoundary = {
  boundary: typeof InstantLaunchAuthBoundary;
  userBadge: typeof InstantLaunchUserBadge;
};
type LaunchInstantSnapshotQuery = typeof queryInstantLaunchTodosSnapshot;
type LaunchInstantRouteHandlers = ReturnType<typeof createDxInstantRouteHandlers>;
type LaunchInstantRuleHelpers = {
  queryWithRuleParams: typeof queryInstantLaunchTodosWithRuleParams;
  updateByText: typeof updateInstantLaunchTodoByText;
};
type LaunchInstantOAuthHelpers = {
  createAuthorizationUrl: typeof createInstantLaunchAuthorizationUrl;
  exchangeOAuthCode: typeof exchangeInstantLaunchOAuthCode;
  issuerUri: typeof instantLaunchIssuerUri;
  signInWithIdToken: typeof signInInstantLaunchWithIdToken;
  signInWithToken: typeof signInInstantLaunchWithToken;
};
type LaunchInstantPaginationHelpers = {
  subscribeInfiniteTodos: typeof subscribeInstantLaunchTodosInfinite;
  useInfiniteTodos: typeof useInstantLaunchTodosInfinite;
};
type LaunchInstantSsrHelpers = {
  getSsrUser: typeof getInstantLaunchSsrUser;
  provider: typeof InstantLaunchSuspenseProvider;
  useSuspenseTodos: typeof useInstantLaunchTodosSuspense;
};
type LaunchInstantStatusHelpers = {
  getDeviceId: typeof getInstantLaunchDeviceId;
  useConnectionStatus: typeof useInstantLaunchConnectionStatus;
};
type LaunchInstantSubscriptionHelpers = {
  subscribeAuth: typeof subscribeInstantLaunchAuth;
  subscribeConnectionStatus: typeof subscribeInstantLaunchConnectionStatus;
  subscribeTodos: typeof subscribeInstantLaunchTodos;
};
type LaunchInstantStorageHelpers = {
  deleteFile: typeof deleteInstantLaunchFile;
  queryFile: typeof queryInstantLaunchFile;
  uploadFile: typeof uploadInstantLaunchFile;
};
type LaunchInstantStreamWriter = typeof createInstantLaunchWriteStream;
type LaunchInstantQuery = ReturnType<DxInstantClient["useQuery"]>;
type LaunchInstantAuth = ReturnType<DxInstantClient["useAuth"]>;
type InstantLocalReceipt = {
  id: string;
  status: "ready";
  schema: "todos+labels";
  boundary: "local-preview";
  nextStep: "set NEXT_PUBLIC_INSTANT_APP_ID";
};
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

export type LaunchInstantStatusProps = {
  authBoundary?: LaunchInstantAuthBoundary;
  authHelpers?: LaunchInstantAuthHelpers;
  configHelpers?: LaunchInstantConfigHelpers;
  cursors?: LaunchInstantCursors;
  db?: DxInstantClient;
  diagnostics?: LaunchInstantDiagnostics;
  oauthHelpers?: LaunchInstantOAuthHelpers;
  paginationHelpers?: LaunchInstantPaginationHelpers;
  room?: DxInstantRoom;
  ruleHelpers?: LaunchInstantRuleHelpers;
  snapshotQuery?: LaunchInstantSnapshotQuery;
  ssrHelpers?: LaunchInstantSsrHelpers;
  statusHelpers?: LaunchInstantStatusHelpers;
  subscriptionHelpers?: LaunchInstantSubscriptionHelpers;
  routeHandlers?: LaunchInstantRouteHandlers;
  storageHelpers?: LaunchInstantStorageHelpers;
  streamWriter?: LaunchInstantStreamWriter;
};

export function LaunchInstantStatus({
  authBoundary,
  authHelpers,
  configHelpers,
  cursors,
  db,
  diagnostics,
  oauthHelpers,
  paginationHelpers,
  routeHandlers,
  room,
  ruleHelpers,
  snapshotQuery,
  ssrHelpers,
  statusHelpers,
  subscriptionHelpers,
  storageHelpers,
  streamWriter,
}: LaunchInstantStatusProps) {
  if (!db) {
    return <LaunchInstantMissingConfigStatus />;
  }

  return (
    <LaunchInstantLiveStatus
      db={db}
      authBoundary={authBoundary}
      authHelpers={authHelpers}
      configHelpers={configHelpers}
      cursors={cursors}
      diagnostics={diagnostics}
      oauthHelpers={oauthHelpers}
      paginationHelpers={paginationHelpers}
      routeHandlers={routeHandlers}
      room={room}
      ruleHelpers={ruleHelpers}
      snapshotQuery={snapshotQuery}
      ssrHelpers={ssrHelpers}
      statusHelpers={statusHelpers}
      subscriptionHelpers={subscriptionHelpers}
      storageHelpers={storageHelpers}
      streamWriter={streamWriter}
    />
  );
}

function LaunchInstantMissingConfigStatus() {
  const [receipt, setReceipt] = React.useState<InstantLocalReceipt | null>(null);

  return (
    <section
      className="grid gap-3 rounded-md border bg-card p-3 text-card-foreground"
      data-dx-component="instantdb-dashboard-workflow"
      data-dx-instant-readiness="dashboard-readiness"
      data-dx-instant-receipt-state={receipt ? "created" : "idle"}
      data-dx-instant-status="client-required"
      data-dx-package="instantdb/react"
      data-dx-style-surface="realtime-app-database"
    >
      <div className="flex items-start justify-between gap-3">
        <div className="grid gap-1">
          <p className="text-sm font-medium">InstantDB realtime boundary</p>
          <p className="text-xs leading-5 text-muted-foreground">
            Realtime queries, auth, rooms, storage, and streams are source-owned.
            Live sync waits for the app-owned Instant app id.
          </p>
        </div>
        <DxIcon
          aria-hidden="true"
          className="size-4 shrink-0 text-muted-foreground"
          name="pack:database"
        />
      </div>
      <div
        className="grid gap-2 rounded-md bg-muted p-3 text-xs leading-5 text-muted-foreground"
        data-dx-instant-required-env="NEXT_PUBLIC_INSTANT_APP_ID"
        data-dx-instant-safe-boundary="missing-config"
      >
        <span>Missing env: NEXT_PUBLIC_INSTANT_APP_ID</span>
        <span>
          Safe local preview uses the same launch schema names without connecting
          to InstantDB or storing credentials.
        </span>
      </div>
      <button
        className="w-fit rounded-md border bg-background px-3 py-2 text-xs font-medium text-foreground"
        data-dx-instant-action="prepare-local-schema-receipt"
        type="button"
        onClick={() => setReceipt(createInstantLocalReceipt())}
      >
        Prepare local receipt
      </button>
      {receipt ? (
        <output
          className="rounded-md border bg-background p-3 text-xs leading-5 text-muted-foreground"
          data-dx-instant-local-receipt={receipt.id}
          data-dx-instant-local-schema={receipt.schema}
          data-dx-instant-next-step={receipt.nextStep}
          data-dx-instant-receipt-status={receipt.status}
        >
          Local receipt ready for {receipt.schema}; live sync remains gated until
          {` ${receipt.nextStep}`}.
        </output>
      ) : null}
    </section>
  );
}

function DxIcon(props: DxIconProps) {
  return <dx-icon {...props} />;
}

function createInstantLocalReceipt(): InstantLocalReceipt {
  return {
    id: "instantdb-local-preview",
    status: "ready",
    schema: "todos+labels",
    boundary: "local-preview",
    nextStep: "set NEXT_PUBLIC_INSTANT_APP_ID",
  };
}

function LaunchInstantLiveStatus({
  authBoundary,
  authHelpers,
  configHelpers,
  cursors,
  db,
  diagnostics,
  oauthHelpers,
  paginationHelpers,
  routeHandlers,
  room,
  ruleHelpers,
  snapshotQuery,
  ssrHelpers,
  statusHelpers,
  subscriptionHelpers,
  storageHelpers,
  streamWriter,
}: Required<Pick<LaunchInstantStatusProps, "db">> &
  Pick<
    LaunchInstantStatusProps,
    | "authBoundary"
    | "authHelpers"
    | "configHelpers"
    | "cursors"
    | "diagnostics"
    | "oauthHelpers"
    | "paginationHelpers"
    | "routeHandlers"
    | "room"
    | "ruleHelpers"
    | "snapshotQuery"
    | "ssrHelpers"
    | "statusHelpers"
    | "subscriptionHelpers"
    | "storageHelpers"
    | "streamWriter"
  >) {
  const query = db.useQuery({ todos: {} });
  const auth = db.useAuth();
  const connectionStatus = db.useConnectionStatus();
  const deviceId = db.useLocalId("dx-launch-device");

  if (query.isLoading) {
    return (
      <p
        className="text-sm text-muted-foreground"
        data-dx-instant-status="loading"
      >
        Checking realtime data...
      </p>
    );
  }

  if (query.error) {
    return (
      <p className="text-sm text-destructive" role="alert">
        {query.error.message}
      </p>
    );
  }

  if (room) {
    return (
      <LaunchInstantPresenceStatus
        auth={auth}
        authBoundary={authBoundary}
        authHelpers={authHelpers}
        configHelpers={configHelpers}
        cursors={cursors}
        diagnostics={diagnostics}
        db={db}
        oauthHelpers={oauthHelpers}
        paginationHelpers={paginationHelpers}
        query={query}
        routeHandlers={routeHandlers}
        room={room}
        ruleHelpers={ruleHelpers}
        snapshotQuery={snapshotQuery}
        ssrHelpers={ssrHelpers}
        statusHelpers={statusHelpers}
        subscriptionHelpers={subscriptionHelpers}
        storageHelpers={storageHelpers}
        streamWriter={streamWriter}
      />
    );
  }

  return (
    <LaunchInstantReadinessLine
      auth={auth}
      authBoundary={authBoundary}
      authHelpers={authHelpers}
      configHelpers={configHelpers}
      cursors={cursors}
      diagnostics={diagnostics}
      oauthHelpers={oauthHelpers}
      paginationHelpers={paginationHelpers}
      peers={null}
      routeHandlers={routeHandlers}
      ruleHelpers={ruleHelpers}
      connectionStatus={connectionStatus}
      deviceId={deviceId}
      snapshotQuery={snapshotQuery}
      ssrHelpers={ssrHelpers}
      statusHelpers={statusHelpers}
      subscriptionHelpers={subscriptionHelpers}
      storageHelpers={storageHelpers}
      streamWriter={streamWriter}
      todoCount={query.data.todos.length}
      typingCount={null}
    />
  );
}

function LaunchInstantReadinessLine({
  auth,
  authBoundary,
  authHelpers,
  configHelpers,
  connectionStatus,
  cursors,
  diagnostics,
  deviceId,
  oauthHelpers,
  paginationHelpers,
  peers,
  routeHandlers,
  ruleHelpers,
  snapshotQuery,
  ssrHelpers,
  statusHelpers,
  subscriptionHelpers,
  storageHelpers,
  streamWriter,
  todoCount,
  typingCount,
}: {
  auth: LaunchInstantAuth;
  authBoundary?: LaunchInstantAuthBoundary;
  authHelpers?: LaunchInstantAuthHelpers;
  configHelpers?: LaunchInstantConfigHelpers;
  connectionStatus: ReturnType<DxInstantClient["useConnectionStatus"]>;
  cursors?: LaunchInstantCursors;
  diagnostics?: LaunchInstantDiagnostics;
  deviceId: string | null;
  oauthHelpers?: LaunchInstantOAuthHelpers;
  paginationHelpers?: LaunchInstantPaginationHelpers;
  peers: number | null;
  routeHandlers?: LaunchInstantRouteHandlers;
  ruleHelpers?: LaunchInstantRuleHelpers;
  snapshotQuery?: LaunchInstantSnapshotQuery;
  ssrHelpers?: LaunchInstantSsrHelpers;
  statusHelpers?: LaunchInstantStatusHelpers;
  subscriptionHelpers?: LaunchInstantSubscriptionHelpers;
  storageHelpers?: LaunchInstantStorageHelpers;
  streamWriter?: LaunchInstantStreamWriter;
  todoCount: number;
  typingCount: number | null;
}) {
  const authLabel = auth.user ? "signed in" : "guest";
  const authBoundaryLabel = authBoundary
    ? "auth boundary wired"
    : "auth boundary app-owned";
  const authHelpersLabel = authHelpers
    ? "auth action helpers wired"
    : "auth actions app-owned";
  const authSnapshotLabel = authHelpers
    ? "getAuth helper wired"
    : "getAuth helper app-owned";
  const connectionLabel = `connection: ${connectionStatus}`;
  const configLabel = configHelpers
    ? "config env boundary wired"
    : "config env boundary app-owned";
  const cursorLabel = cursors ? "cursor wrapper wired" : "cursor wrapper app-owned";
  const diagnosticsLabel = diagnostics
    ? "diagnostics helper wired"
    : "diagnostics app-owned";
  const deviceLabel = deviceId ? "local id ready" : "local id pending";
  const oauthLabel = oauthHelpers ? "oauth helpers wired" : "oauth helpers app-owned";
  const paginationLabel = paginationHelpers
    ? "pagination helpers wired"
    : "pagination helpers app-owned";
  const peerLabel = peers === null ? "presence room not connected" : `peers: ${peers}`;
  const snapshotLabel = snapshotQuery
    ? "queryOnce snapshot wired"
    : "queryOnce snapshot app-owned";
  const ssrLabel = ssrHelpers
    ? "next SSR helpers wired"
    : "next SSR helpers app-owned";
  const statusHelpersLabel = statusHelpers
    ? "status helpers wired"
    : "status helpers app-owned";
  const subscriptionLabel = subscriptionHelpers
    ? "subscription helpers wired"
    : "subscription helpers app-owned";
  const storageLabel = storageHelpers
    ? "storage helpers wired"
    : "storage helper app-owned";
  const routeLabel = routeHandlers
    ? "route POST handler wired"
    : "route handler app-owned";
  const ruleLabel = ruleHelpers
    ? "rule helpers wired"
    : "rule helpers app-owned";
  const streamLabel = streamWriter
    ? "stream writer helper wired"
    : "stream helper app-owned";
  const typingLabel = typingCount === null ? "typing unavailable" : `${typingCount} typing`;

  return (
    <p className="text-sm text-muted-foreground" data-dx-instant-status="ready">
      Realtime todos: {todoCount}; {peerLabel};{" "}
      <span data-dx-instant-auth={authLabel}>{authLabel}</span>;{" "}
      <span data-dx-instant-auth-boundary={authBoundaryLabel}>
        {authBoundaryLabel}
      </span>;{" "}
      <span data-dx-instant-auth-actions={authHelpersLabel}>
        {authHelpersLabel}
      </span>;{" "}
      <span data-dx-instant-auth-snapshot={authSnapshotLabel}>
        {authSnapshotLabel}
      </span>;{" "}
      <span data-dx-instant-connection={connectionLabel}>{connectionLabel}</span>;{" "}
      <span data-dx-instant-config={configLabel}>{configLabel}</span>;{" "}
      <span data-dx-instant-cursors={cursorLabel}>{cursorLabel}</span>;{" "}
      <span data-dx-instant-diagnostics={diagnosticsLabel}>
        {diagnosticsLabel}
      </span>;{" "}
      <span data-dx-instant-local-id={deviceLabel}>{deviceLabel}</span>;{" "}
      <span data-dx-instant-oauth={oauthLabel}>{oauthLabel}</span>;{" "}
      <span data-dx-instant-pagination={paginationLabel}>
        {paginationLabel}
      </span>;{" "}
      <span data-dx-instant-query-once={snapshotLabel}>{snapshotLabel}</span>;{" "}
      <span data-dx-instant-route={routeLabel}>{routeLabel}</span>;{" "}
      <span data-dx-instant-rules={ruleLabel}>{ruleLabel}</span>;{" "}
      <span data-dx-instant-ssr={ssrLabel}>{ssrLabel}</span>;{" "}
      <span data-dx-instant-status-helpers={statusHelpersLabel}>
        {statusHelpersLabel}
      </span>;{" "}
      <span data-dx-instant-storage={storageLabel}>{storageLabel}</span>;{" "}
      <span data-dx-instant-subscriptions={subscriptionLabel}>
        {subscriptionLabel}
      </span>;{" "}
      <span data-dx-instant-streams={streamLabel}>{streamLabel}</span>;{" "}
      <span data-dx-instant-typing={typingLabel}>{typingLabel}</span>
    </p>
  );
}

function LaunchInstantPresenceStatus({
  auth,
  authBoundary,
  authHelpers,
  configHelpers,
  cursors,
  diagnostics,
  db,
  oauthHelpers,
  paginationHelpers,
  query,
  routeHandlers,
  room,
  ruleHelpers,
  snapshotQuery,
  ssrHelpers,
  statusHelpers,
  subscriptionHelpers,
  storageHelpers,
  streamWriter,
}: {
  auth: LaunchInstantAuth;
  authBoundary?: LaunchInstantAuthBoundary;
  authHelpers?: LaunchInstantAuthHelpers;
  configHelpers?: LaunchInstantConfigHelpers;
  cursors?: LaunchInstantCursors;
  diagnostics?: LaunchInstantDiagnostics;
  db: DxInstantClient;
  oauthHelpers?: LaunchInstantOAuthHelpers;
  paginationHelpers?: LaunchInstantPaginationHelpers;
  query: LaunchInstantQuery;
  routeHandlers?: LaunchInstantRouteHandlers;
  room: DxInstantRoom;
  ruleHelpers?: LaunchInstantRuleHelpers;
  snapshotQuery?: LaunchInstantSnapshotQuery;
  ssrHelpers?: LaunchInstantSsrHelpers;
  statusHelpers?: LaunchInstantStatusHelpers;
  subscriptionHelpers?: LaunchInstantSubscriptionHelpers;
  storageHelpers?: LaunchInstantStorageHelpers;
  streamWriter?: LaunchInstantStreamWriter;
}) {
  const presence = db.rooms.usePresence(room);
  const connectionStatus = db.useConnectionStatus();
  const deviceId = db.useLocalId("dx-launch-device");
  const presenceName = auth.user?.email?.split("@")[0] ?? deviceId ?? "guest";
  db.rooms.useSyncPresence(room, { name: presenceName }, [presenceName]);
  const typing = db.rooms.useTypingIndicator(room, "launch-input", {
    writeOnly: true,
  });

  return (
    <LaunchInstantReadinessLine
      auth={auth}
      authBoundary={authBoundary}
      authHelpers={authHelpers}
      configHelpers={configHelpers}
      connectionStatus={connectionStatus}
      cursors={cursors}
      diagnostics={diagnostics}
      deviceId={deviceId}
      oauthHelpers={oauthHelpers}
      paginationHelpers={paginationHelpers}
      peers={Object.keys(presence.peers).length + 1}
      routeHandlers={routeHandlers}
      ruleHelpers={ruleHelpers}
      snapshotQuery={snapshotQuery}
      ssrHelpers={ssrHelpers}
      statusHelpers={statusHelpers}
      subscriptionHelpers={subscriptionHelpers}
      storageHelpers={storageHelpers}
      streamWriter={streamWriter}
      todoCount={query.data.todos.length}
      typingCount={typing.active.length}
    />
  );
}
