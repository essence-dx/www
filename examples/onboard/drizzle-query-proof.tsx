"use client";

import * as React from "react";

import { dxDrizzlePackage } from "@/db/drizzle/metadata";

type DrizzleReadModelId =
  | "launch-pipeline"
  | "published-content"
  | "author-workload";

type DrizzleFixtureRow = {
  id: string;
  label: string;
  detail: string;
  metric: string;
};

type DrizzleReadModel = {
  id: DrizzleReadModelId;
  queryPlanId: "overview" | "published-posts" | "author-counts";
  label: string;
  helper: string;
  publicApi: string;
  sqlPreview: string;
  summary: string;
  metrics: readonly { label: string; value: string }[];
  rows: readonly DrizzleFixtureRow[];
};

type DrizzleReadReceipt = {
  modelId: DrizzleReadModelId;
  kind: "ready";
  message: string;
};

const drizzleReadModels: readonly DrizzleReadModel[] = [
  {
    id: "launch-pipeline",
    queryPlanId: "overview",
    label: "Launch pipeline",
    helper: dxDrizzlePackage.dashboardUsage.entryPoint,
    publicApi: "readLaunchDatabaseStats + readDrizzleDashboardOverview",
    sqlPreview:
      "select count(*), count(distinct users.role), count(*) filter (where posts.status = ?) from users left join posts",
    summary:
      "Use the generated dashboard overview to drive the launch operations summary.",
    metrics: [
      { label: "Users", value: "2" },
      { label: "Published", value: "2" },
      { label: "Pending", value: "1" },
    ],
    rows: [
      {
        id: "dx-preview",
        label: "DX launch template",
        detail: "Published content and adapter readiness are visible.",
        metric: "ready",
      },
      {
        id: "forge-source",
        label: "Forge source packages",
        detail: "Generated files stay source-owned and editable.",
        metric: "ready",
      },
    ],
  },
  {
    id: "published-content",
    queryPlanId: "published-posts",
    label: "Published content",
    helper: dxDrizzlePackage.joins.listPostPreviews,
    publicApi: "select().from().innerJoin().where().toSQL()",
    sqlPreview:
      "select posts.title, users.name from posts inner join users on posts.author_id = users.id where posts.status = ?",
    summary:
      "Preview the content list the dashboard would read from the local SQLite store.",
    metrics: [
      { label: "Rows", value: "2" },
      { label: "Join", value: "author" },
      { label: "Status", value: "published" },
    ],
    rows: [
      {
        id: "dx-preview",
        label: "DX launch template",
        detail: "essencefromexistence",
        metric: "published",
      },
      {
        id: "forge-source",
        label: "Forge source-owned packages",
        detail: "Friday lane",
        metric: "published",
      },
    ],
  },
  {
    id: "author-workload",
    queryPlanId: "author-counts",
    label: "Author workload",
    helper: dxDrizzlePackage.cteQueries.listAuthorsWithPostCounts,
    publicApi: "db.$with + db.with + leftJoin + groupBy + toSQL",
    sqlPreview:
      "with post_counts as (...) select users.name, post_counts.total from users left join post_counts",
    summary:
      "Preview the author workload panel before the app connects a SQLite file.",
    metrics: [
      { label: "Authors", value: "2" },
      { label: "CTE", value: "post_counts" },
      { label: "Window", value: "launch" },
    ],
    rows: [
      {
        id: "essence",
        label: "essencefromexistence",
        detail: "owner",
        metric: "2 launch records",
      },
      {
        id: "friday",
        label: "Friday",
        detail: "orchestrator",
        metric: "1 readiness record",
      },
    ],
  },
];

function findDrizzleReadModel(readModelId: DrizzleReadModelId) {
  return (
    drizzleReadModels.find((model) => model.id === readModelId) ??
    drizzleReadModels[0]
  );
}

function createDrizzleReadReceipt(
  readModel: DrizzleReadModel,
  runCount: number,
): DrizzleReadReceipt {
  return {
    modelId: readModel.id,
    kind: "ready",
    message: `${readModel.helper} prepared ${readModel.rows.length} local rows on run ${runCount}. Production SQLite reads still require ${dxDrizzlePackage.runtimeDependencies.join(
      " + ",
    )}, reviewed migrations, and an app-owned database path.`,
  };
}

export function LaunchDrizzleDashboardData() {
  const [activeModelId, setActiveModelId] =
    React.useState<DrizzleReadModelId>("launch-pipeline");
  const [runCount, setRunCount] = React.useState(0);
  const [planPreviewed, setPlanPreviewed] = React.useState(false);
  const [receipt, setReceipt] = React.useState<DrizzleReadReceipt | null>(null);
  const activeModel = findDrizzleReadModel(activeModelId);
  const status = receipt ? "read-model-applied" : "read-model-ready";

  return (
    <section
      className="grid gap-3 rounded-md border bg-background/60 p-3"
      data-dx-package="db/drizzle-sqlite"
      data-dx-component="launch-drizzle-data-workflow"
      data-dx-style-surface="database-orm"
      data-dx-dashboard-workflow="sqlite-read-model"
      data-dx-product-surface="launch-data-dashboard"
      data-dx-dashboard-target="mission-control-database"
      data-dx-source="examples/template/drizzle-query-proof.tsx"
      data-dx-drizzle-mission-control="database"
      data-dx-drizzle-status={status}
      data-dx-drizzle-read-model={activeModel.id}
      data-dx-drizzle-runtime-boundary="app-owned-sqlite"
      data-dx-drizzle-runtime-dependencies={dxDrizzlePackage.runtimeDependencies.join(",")}
      data-dx-drizzle-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json"
      data-dx-drizzle-receipt-state={receipt?.kind ?? "idle"}
      data-dx-node-modules="forbidden"
    >
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="min-w-0">
          <p className="flex items-center gap-2 text-sm font-medium">
            <dx-icon name="pack:database" aria-hidden="true" />
            Launch data read model
          </p>
          <p className="mt-1 text-xs leading-5 text-muted-foreground">
            Select the dashboard data view, inspect the Drizzle SQL plan, and
            apply a safe local fixture while the real SQLite runtime stays
            app-owned. The local readiness path uses no node_modules.
          </p>
        </div>
        <button
          className="rounded-md border px-3 py-2 text-xs font-medium"
          data-dx-drizzle-action="apply-read-model"
          type="button"
          onClick={() => {
            const nextRun = runCount + 1;
            setRunCount(nextRun);
            setReceipt(createDrizzleReadReceipt(activeModel, nextRun));
          }}
        >
          Apply read model
        </button>
      </div>

      <div className="flex flex-wrap gap-2" aria-label="Drizzle dashboard read models">
        {drizzleReadModels.map((model) => (
          <button
            key={model.id}
            className="rounded-md border px-3 py-2 text-xs data-[active=true]:bg-muted"
            data-active={model.id === activeModel.id}
            data-dx-drizzle-action="select-read-model"
            data-dx-drizzle-read-model-option={model.id}
            type="button"
            onClick={() => {
              setActiveModelId(model.id);
              setReceipt(null);
              setPlanPreviewed(false);
            }}
          >
            {model.label}
          </button>
        ))}
      </div>

      <div className="grid gap-2 sm:grid-cols-3">
        {activeModel.metrics.map((metric) => (
          <div
            key={metric.label}
            className="rounded-md border p-3"
            data-dx-drizzle-metric={metric.label}
          >
            <p className="text-xs text-muted-foreground">{metric.label}</p>
            <p className="text-lg font-semibold">{metric.value}</p>
          </div>
        ))}
      </div>

      <div
        className="grid gap-2 rounded-md bg-muted/70 p-3"
        data-dx-drizzle-sql-preview={activeModel.id}
        data-dx-drizzle-query-plan-id={activeModel.queryPlanId}
        data-dx-drizzle-query-plan-export={
          dxDrizzlePackage.dashboardUsage.queryPlanByIdEntryPoint
        }
        data-dx-drizzle-query-plan-previewed={
          planPreviewed ? "true" : "false"
        }
      >
        <div className="flex flex-wrap items-center justify-between gap-2">
          <span
            className="text-xs text-muted-foreground"
            data-dx-drizzle-helper={activeModel.helper}
          >
            Helper: {activeModel.helper}
          </span>
          <button
            className="rounded-md border px-3 py-2 text-xs font-medium"
            data-dx-drizzle-action="preview-query-plan"
            type="button"
            onClick={() => setPlanPreviewed(true)}
          >
            Preview query plan
          </button>
        </div>
        <p
          className="text-xs text-muted-foreground"
          data-dx-drizzle-public-api={activeModel.publicApi}
        >
          {activeModel.publicApi}
        </p>
        <code className="whitespace-pre-wrap break-words text-xs leading-5">
          {activeModel.sqlPreview}
        </code>
      </div>

      <div className="grid gap-2 sm:grid-cols-2">
        {activeModel.rows.map((row) => (
          <div
            key={row.id}
            className="rounded-md border bg-background p-3"
            data-dx-drizzle-fixture-row={row.id}
          >
            <p className="text-sm font-medium">{row.label}</p>
            <p className="text-xs text-muted-foreground">{row.detail}</p>
            <p className="mt-2 text-xs">{row.metric}</p>
          </div>
        ))}
      </div>

      <p
        className="rounded-md bg-muted px-3 py-2 text-xs leading-5 text-muted-foreground"
        data-dx-drizzle-dashboard-summary={activeModel.id}
      >
        {activeModel.summary}
      </p>

      <p
        className="rounded-md bg-muted px-3 py-2 text-xs leading-5 text-muted-foreground"
        data-dx-drizzle-receipt-state={receipt?.kind ?? "idle"}
        role="status"
      >
        {receipt
          ? receipt.message
          : `Ready to apply ${dxDrizzlePackage.packageId} to the launch dashboard without installing packages.`}
      </p>
    </section>
  );
}

export const LaunchDrizzleQueryProof = LaunchDrizzleDashboardData;
