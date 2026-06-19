"use client";

import * as React from "react";

import { dxDrizzlePackage } from "@/db/drizzle/metadata";
import { dxInstantDbForgePackage } from "@/lib/instant/metadata";
import {
  readSupabasePublicConfig,
  type DxSupabasePublicConfig,
} from "@/lib/supabase/env";
import { dxSupabaseForgePackage } from "@/lib/supabase/metadata";
import {
  readDxSupabaseProfilesReadModel,
  type DxSupabaseProfilesReadModel,
} from "@/lib/supabase/profile-workflow";

import { LaunchDrizzleDashboardData } from "./drizzle-query-proof";

type DataBoundaryStatus =
  | {
      kind: "ready";
      config: DxSupabasePublicConfig;
      message: string;
    }
  | {
      kind: "missing-config";
      message: string;
    };

type SupabaseQueryState =
  | {
      kind: "idle";
      message: string;
    }
  | DxSupabaseProfilesReadModel;

function readDataBoundaryStatus(): DataBoundaryStatus {
  try {
    const config = readSupabasePublicConfig();
    return {
      kind: "ready",
      config,
      message: config.isLocal
        ? "Local Supabase endpoint is configured."
        : "Hosted Supabase endpoint is configured.",
    };
  } catch (error) {
    return {
      kind: "missing-config",
      message: error instanceof Error ? error.message : "Supabase config is missing.",
    };
  }
}

const databaseSurfaces = [
  {
    label: "SQLite read model",
    packageId: dxDrizzlePackage.packageId,
    detail: `${dxDrizzlePackage.dashboardUsage.entryPoint} + ${dxDrizzlePackage.dashboardUsage.queryPlanByIdEntryPoint}`,
  },
  {
    label: "Supabase client",
    packageId: dxSupabaseForgePackage.packageId,
    detail: `public env: ${dxSupabaseForgePackage.env.join(", ")}`,
  },
  {
    label: "InstantDB realtime",
    packageId: dxInstantDbForgePackage.packageId,
    detail: dxInstantDbForgePackage.discovery.realtimeHook,
  },
] as const;

const databaseOrmCapabilities = [
  {
    label: "SQLite migrations",
    helper: dxDrizzlePackage.migrations.helper,
    detail: `Default folder ${dxDrizzlePackage.migrations.defaultFolder}; SQL review and rollout stay app-owned.`,
  },
  {
    label: "SQLite views",
    helper: dxDrizzlePackage.views.listPublishedPostSummaries,
    detail: dxDrizzlePackage.views.publicApi,
  },
  {
    label: "SQLite relations",
    helper: dxDrizzlePackage.relationalQueries.listUsers,
    detail: dxDrizzlePackage.relationalQueries.publicApi,
  },
  {
    label: "SQLite joins",
    helper: dxDrizzlePackage.joins.listPostPreviews,
    detail: dxDrizzlePackage.joins.publicApi,
  },
  {
    label: "SQLite set ops",
    helper: dxDrizzlePackage.setOperations.listAudience,
    detail: dxDrizzlePackage.setOperations.publicApi,
  },
  {
    label: "SQLite CTEs",
    helper: dxDrizzlePackage.cteQueries.listAuthorsWithPostCounts,
    detail: dxDrizzlePackage.cteQueries.publicApi,
  },
  {
    label: "SQLite transactions",
    helper: dxDrizzlePackage.transactions.transactionHelper,
    detail: dxDrizzlePackage.transactions.publicApi,
  },
  {
    label: "SQLite prepared",
    helper: dxDrizzlePackage.preparedQueries.listUsers,
    detail: dxDrizzlePackage.preparedQueries.publicApi,
  },
  {
    label: "SQLite conflict writes",
    helper: dxDrizzlePackage.conflictWrites.upsertUser,
    detail: dxDrizzlePackage.conflictWrites.publicApi,
  },
  {
    label: "SQLite mutations",
    helper: dxDrizzlePackage.mutations.updateUser,
    detail: dxDrizzlePackage.mutations.publicApi,
  },
  {
    label: "SQLite analytics",
    helper: dxDrizzlePackage.analytics.readStats,
    detail: dxDrizzlePackage.analytics.publicApi,
  },
] as const;

export function LaunchDataStatus() {
  const [status, refresh] = React.useReducer(
    () => readDataBoundaryStatus(),
    undefined,
    readDataBoundaryStatus,
  );
  const [supabaseQuery, setSupabaseQuery] = React.useState<SupabaseQueryState>({
    kind: "idle",
    message: "Run the local schema query to preview the Supabase table shape.",
  });

  return (
    <div
      className="grid gap-3"
      data-dx-component="supabase-schema-query-workflow"
      data-dx-dashboard-card="database-supabase-client"
      data-dx-dashboard-workflow="supabase-schema-query"
      data-dx-data-status={status.kind}
      data-dx-edit-id="launch.database.supabase-client"
      data-dx-edit-kind="dashboard-workflow"
      data-dx-edit-ops="move_reorder_section,update_text_content,insert_icon_media"
      data-dx-node-modules="not-required"
      data-dx-package="supabase/client"
      data-dx-product-surface="launch-data-dashboard"
      data-dx-style-surface="backend-platform-client"
      data-dx-supabase-config-status={status.kind}
      data-dx-supabase-readiness="client-readiness"
      data-dx-supabase-query-state={supabaseQuery.kind}
      data-dx-supabase-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"
      data-dx-token-scope="supabase/client"
    >
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="text-sm font-medium">Data adapters</p>
          <p className="text-xs leading-5 text-muted-foreground">
            Drizzle owns the local schema, Supabase owns hosted client wiring,
            and InstantDB owns realtime launch-room state.
          </p>
        </div>
        <button
          className="rounded-md border px-3 py-2 text-xs font-medium"
          data-dx-supabase-interaction="config-refresh"
          type="button"
          onClick={refresh}
        >
          Recheck
        </button>
      </div>
      <div
        className="grid gap-3 rounded-md border p-3"
        data-dx-supabase-interaction="config-readiness"
      >
        <div className="flex flex-wrap items-center justify-between gap-2">
          <div>
            <p className="text-sm font-medium">Supabase client readiness</p>
            <p className="text-xs leading-5 text-muted-foreground">
              Public URL and publishable key are app-owned. Service-role secrets
              are intentionally absent from the browser slice.
            </p>
          </div>
          <button
            className="rounded-md border px-3 py-2 text-xs font-medium"
            data-dx-supabase-action="run-local-schema-query"
            type="button"
            onClick={() => setSupabaseQuery(readDxSupabaseProfilesReadModel())}
          >
            Run local query
          </button>
        </div>
        <p
          className="rounded-md bg-muted px-3 py-2 text-xs leading-5 text-muted-foreground"
          data-dx-supabase-config-message={status.kind}
          role={status.kind === "missing-config" ? "status" : undefined}
        >
          {status.message}
        </p>
        <div
          className="grid gap-2"
          data-dx-supabase-query-operation={
            supabaseQuery.kind === "ready"
              ? supabaseQuery.operation
              : "supabase.from('profiles').select('id, full_name, username, website')"
          }
          data-dx-supabase-query-state={supabaseQuery.kind}
          data-dx-supabase-table={
            supabaseQuery.kind === "ready" ? supabaseQuery.table : "profiles"
          }
        >
          <p className="text-xs text-muted-foreground">{supabaseQuery.message}</p>
          {supabaseQuery.kind === "ready" ? (
            <div className="grid gap-2 sm:grid-cols-2">
              {supabaseQuery.rows.map((row) => (
                <div
                  key={row.id}
                  className="rounded-md border p-3"
                  data-dx-supabase-row={row.id}
                  data-dx-supabase-row-username={row.username ?? ""}
                >
                  <p className="text-sm font-medium">{row.fullName}</p>
                  <p className="text-xs text-muted-foreground">
                    @{row.username} - {row.website}
                  </p>
                </div>
              ))}
            </div>
          ) : null}
        </div>
      </div>
      <LaunchDrizzleDashboardData />
      <section
        className="grid gap-3 rounded-md border bg-background/60 p-3"
        data-dx-component="database-orm-capability-matrix"
        data-dx-dashboard-workflow="sqlite-source-owned-capabilities"
        data-dx-package={dxDrizzlePackage.packageId}
        data-dx-product-surface="launch-data-dashboard"
        data-dx-style-surface="database-orm"
      >
        <div>
          <p className="text-sm font-medium">Database ORM capability matrix</p>
          <p className="text-xs leading-5 text-muted-foreground">
            Source-owned SQLite helpers are materialized for app review. Runtime
            database files, migrations, authorization, and dependency
            installation stay app-owned.
          </p>
        </div>
        <div className="grid gap-2 sm:grid-cols-2 xl:grid-cols-3">
          {databaseOrmCapabilities.map((surface) => (
            <div
              key={`${dxDrizzlePackage.packageId}:${surface.label}`}
              className="rounded-md border p-3"
              data-dx-drizzle-capability={surface.label}
              data-dx-drizzle-helper={surface.helper}
              data-dx-drizzle-runtime-proof="source-owned"
            >
              <p className="text-xs text-muted-foreground">{surface.label}</p>
              <p className="mt-1 text-sm font-medium">{surface.helper}</p>
              <p className="mt-2 text-xs leading-5 text-muted-foreground">
                {surface.detail}
              </p>
            </div>
          ))}
        </div>
      </section>
      <div className="grid gap-2 sm:grid-cols-2 xl:grid-cols-4 2xl:grid-cols-6">
        {databaseSurfaces.map((surface) => (
          <div
            key={`${surface.packageId}:${surface.label}`}
            className="rounded-md border p-3"
          >
            <p className="text-xs text-muted-foreground">{surface.label}</p>
            <p className="text-sm font-medium">{surface.packageId}</p>
            <p className="mt-2 text-xs leading-5 text-muted-foreground">
              {surface.detail}
            </p>
          </div>
        ))}
      </div>
      <p
        className="rounded-md bg-muted px-3 py-2 text-xs leading-5 text-muted-foreground"
        role={status.kind === "missing-config" ? "status" : undefined}
      >
        {status.message}
      </p>
    </div>
  );
}
