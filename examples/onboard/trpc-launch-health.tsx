"use client";

import * as React from "react";

import {
  createLocalHealthCheck,
  createLocalLaunchEvent,
  trpcLaunchContract,
  type TrpcLaunchWorkflowResult,
} from "./trpc-launch-contract";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "dx-icon": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & { name: string },
        HTMLElement
      >;
    }
  }
}

export function TrpcLaunchHealth() {
  const [sequence, setSequence] = React.useState(0);
  const [result, setResult] = React.useState<TrpcLaunchWorkflowResult | null>(
    null,
  );

  function recordHealthCheck() {
    const nextSequence = sequence + 1;

    setSequence(nextSequence);
    setResult(createLocalHealthCheck(nextSequence));
  }

  function recordLaunchValidation() {
    const nextSequence = sequence + 1;

    setSequence(nextSequence);
    setResult(createLocalLaunchEvent(nextSequence));
  }

  return (
    <section
      className="grid gap-3 rounded-md border bg-card p-3 text-sm text-card-foreground"
      data-api-state={result ? "local-validated" : "runtime-pending"}
      data-dx-package="api/trpc"
      data-dx-component="trpc-launch-health-workflow"
      data-dx-dashboard-card="typed-api-health"
      data-dx-dashboard-workflow="typed-api-health"
      data-dx-dashboard-flow="typed-api-readiness"
      data-dx-trpc-workflow="launch-api-readiness"
      data-dx-style-surface="theme-token"
      data-trpc-workflow="template-visible"
      data-trpc-route={trpcLaunchContract.route}
      data-trpc-endpoint={trpcLaunchContract.endpoint}
      data-trpc-procedure={`${trpcLaunchContract.procedures.health},${trpcLaunchContract.procedures.launchEvent}`}
      data-trpc-runtime-boundary="source-owned-runtime-pending"
    >
      <div className="grid gap-1">
        <div className="flex items-center gap-2 text-xs font-medium uppercase tracking-normal text-foreground">
          <dx-icon name="api:trpc" aria-hidden="true" />
          <span>Type-Safe API launch health</span>
        </div>
        <p
          className="text-muted-foreground"
          data-trpc-readiness-state={result ? "validated" : "ready"}
        >
          Route handler: {trpcLaunchContract.route}; local workflow validates
          the health query and launch event contracts before runtime
          dependencies or route execution are allowed.
        </p>
      </div>

      <dl
        className="grid gap-2 text-xs text-muted-foreground sm:grid-cols-2"
        data-trpc-source-surface="public-api"
      >
        <div>
          <dt className="text-muted-foreground">Query</dt>
          <dd data-trpc-query={trpcLaunchContract.procedures.health}>
            {trpcLaunchContract.sourceApis[2]}
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground">Mutation</dt>
          <dd data-trpc-mutation={trpcLaunchContract.procedures.launchEvent}>
            {trpcLaunchContract.sourceApis[3]}
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground">Cache action</dt>
          <dd data-trpc-cache-action="invalidate-health-query">
            {trpcLaunchContract.sourceApis[4]}
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground">Adapter</dt>
          <dd data-trpc-adapter="fetchRequestHandler">
            {trpcLaunchContract.sourceApis[1]}
          </dd>
        </div>
      </dl>

      <div className="flex flex-wrap items-center gap-3">
        <button
          type="button"
          className="rounded-md border px-3 py-2 text-xs font-medium text-foreground transition hover:bg-muted"
          data-dx-trpc-action="check-health"
          data-trpc-interaction="health-query"
          onClick={recordHealthCheck}
        >
          Check health contract
        </button>
        <button
          type="button"
          className="rounded-md border px-3 py-2 text-xs font-medium text-foreground transition hover:bg-muted"
          data-dx-trpc-action="prepare-launch-event"
          data-trpc-interaction="local-launch-event-mutation"
          onClick={recordLaunchValidation}
        >
          Run launchEvent validation
        </button>
        <span
          className="text-xs text-muted-foreground"
          data-trpc-client-ready="local-contract"
          data-trpc-node-modules="not-required-for-workflow"
        >
          Browser workflow uses local typed state; the live route stays app-owned.
        </span>
      </div>

      <p
        className="rounded-md bg-muted px-3 py-2 text-xs text-muted-foreground"
        data-api-mutation={result?.status ?? "idle"}
        data-dx-trpc-receipt-state={result?.status ?? "idle"}
        data-dx-trpc-request-id={result?.requestId ?? "not-run"}
        data-trpc-procedure-result={result?.procedure ?? "idle"}
        data-trpc-query-state={
          result?.procedure === trpcLaunchContract.procedures.health
            ? result.status
            : "idle"
        }
        data-trpc-mutation-state={result?.status ?? "idle"}
        data-trpc-request-id={result?.requestId ?? "not-run"}
        role={result ? "status" : undefined}
      >
        {result?.procedure === trpcLaunchContract.procedures.health
          ? `${result.procedure} ready on ${result.route}; ${result.cacheAction}; ${result.requestId}`
          : result
            ? `${result.event} on ${result.route}; ${result.cacheAction}; ${result.requestId}`
            : "Waiting for a local Type-Safe API workflow action."}
      </p>
    </section>
  );
}
