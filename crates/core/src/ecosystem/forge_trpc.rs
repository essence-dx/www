pub(super) const TRPC_NEXT_VERSION: &str = "11.17.0-dx.10";

pub(super) fn trpc_next_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/lib/trpc/context.ts", TRPC_CONTEXT_TS),
        ("js/lib/trpc/transformer.ts", TRPC_TRANSFORMER_TS),
        ("js/lib/trpc/server.ts", TRPC_SERVER_TS),
        ("js/lib/trpc/errors.ts", TRPC_ERRORS_TS),
        ("js/lib/trpc/response-meta.ts", TRPC_RESPONSE_META_TS),
        ("js/lib/trpc/router.ts", TRPC_ROUTER_TS),
        ("js/lib/trpc/route-handler.ts", TRPC_ROUTE_HANDLER_TS),
        ("js/lib/trpc/server-caller.ts", TRPC_SERVER_CALLER_TS),
        ("js/app/api/trpc/[trpc]/route.ts", TRPC_APP_ROUTE_TS),
        ("js/lib/trpc/http.ts", TRPC_HTTP_TS),
        ("js/lib/trpc/client.ts", TRPC_CLIENT_TS),
        ("js/lib/trpc/subscriptions.ts", TRPC_SUBSCRIPTIONS_TS),
        ("js/lib/trpc/streaming-client.ts", TRPC_STREAMING_CLIENT_TS),
        ("js/lib/trpc/provider.tsx", TRPC_PROVIDER_TSX),
        (
            "js/lib/trpc/dashboard-workflow.ts",
            TRPC_DASHBOARD_WORKFLOW_TS,
        ),
        (
            "js/components/dashboard/trpc-dashboard-workflow.tsx",
            TRPC_DASHBOARD_WORKFLOW_TSX,
        ),
        ("js/lib/trpc/metadata.ts", TRPC_METADATA_TS),
        (
            "js/examples/template/trpc-launch-contract.ts",
            TRPC_LAUNCH_CONTRACT_TS,
        ),
        (
            "js/examples/template/trpc-launch-health.tsx",
            TRPC_LAUNCH_HEALTH_TSX,
        ),
        (
            "js/examples/template/trpc-server-readiness.ts",
            TRPC_LAUNCH_SERVER_READINESS_TS,
        ),
        (
            "js/examples/template/trpc-subscription-status.tsx",
            TRPC_LAUNCH_SUBSCRIPTION_STATUS_TSX,
        ),
        (
            "js/examples/template/trpc-error-status.tsx",
            TRPC_LAUNCH_ERROR_STATUS_TSX,
        ),
        (
            "js/examples/template/trpc-streaming-client-status.tsx",
            TRPC_LAUNCH_STREAMING_CLIENT_STATUS_TSX,
        ),
        (
            "js/examples/template/trpc-response-meta.ts",
            TRPC_LAUNCH_RESPONSE_META_TS,
        ),
        (
            "js/examples/template/trpc-infinite-feed.tsx",
            TRPC_LAUNCH_INFINITE_FEED_TSX,
        ),
        (
            "js/examples/template/trpc-transformer-status.ts",
            TRPC_LAUNCH_TRANSFORMER_STATUS_TS,
        ),
        (
            "js/examples/template/trpc-request-policy.ts",
            TRPC_LAUNCH_REQUEST_POLICY_TS,
        ),
        ("js/lib/trpc/README.md", TRPC_README_MD),
    ]
}

const TRPC_CONTEXT_TS: &str = r#"export type DxTrpcSession = {
  userId: string;
  email?: string;
  role?: "admin" | "member" | (string & {});
};

export type DxTrpcContext = {
  requestId: string;
  headers: Headers;
  session: DxTrpcSession | null;
};

export type CreateDxTrpcContextOptions = {
  req: Request;
  requestId?: string;
  session?: DxTrpcSession | null;
};

export async function createDxTrpcContext({
  req,
  requestId,
  session = null,
}: CreateDxTrpcContextOptions): Promise<DxTrpcContext> {
  return {
    requestId: requestId ?? req.headers.get("x-request-id") ?? createRequestId(),
    headers: req.headers,
    session,
  };
}

function createRequestId(): string {
  if (globalThis.crypto?.randomUUID) {
    return globalThis.crypto.randomUUID();
  }

  return `req_${Date.now().toString(36)}_${Math.random().toString(36).slice(2)}`;
}
"#;

const TRPC_TRANSFORMER_TS: &str = r#"import type {
  TRPCCombinedDataTransformer,
  TRPCDataTransformer,
} from "@trpc/server";

export type DxTrpcTransformer =
  | TRPCCombinedDataTransformer
  | TRPCDataTransformer;

export const dxTrpcIdentityDataTransformer: TRPCDataTransformer = {
  serialize(data) {
    return data;
  },
  deserialize(data) {
    return data;
  },
};

export function createDxTrpcTransformer(
  transformer: DxTrpcTransformer = dxTrpcIdentityDataTransformer,
): TRPCCombinedDataTransformer {
  if ("input" in transformer) {
    return transformer;
  }

  return {
    input: transformer,
    output: transformer,
  };
}

export const dxTrpcTransformer = createDxTrpcTransformer();

export const dxTrpcTransformerPolicy = {
  defaultTransformer: "identity",
  appOwned:
    "SuperJSON, Temporal, Decimal, Date, Map/Set, custom class registration, and serializer dependency installation",
  contract:
    "Use the same transformer on initTRPC.create() and every client link.",
} as const;
"#;

const TRPC_SERVER_TS: &str = r#"import { initTRPC, TRPCError } from "@trpc/server";

import type { DxTrpcContext } from "./context";
import { formatDxTrpcError } from "./errors";
import { dxTrpcTransformer } from "./transformer";

const t = initTRPC.context<DxTrpcContext>().create({
  errorFormatter: formatDxTrpcError,
  transformer: dxTrpcTransformer,
});

export const router = t.router;
export const mergeRouters = t.mergeRouters;
export const publicProcedure = t.procedure;
export const createCallerFactory = t.createCallerFactory;

export const protectedProcedure = publicProcedure.use(({ ctx, next }) => {
  if (!ctx.session?.userId) {
    throw new TRPCError({ code: "UNAUTHORIZED" });
  }

  return next({
    ctx: {
      ...ctx,
      session: ctx.session,
    },
  });
});
"#;

const TRPC_ERRORS_TS: &str = r#"import {
  TRPCError,
  type TRPCDefaultErrorData,
  type TRPCErrorFormatter,
  type TRPCErrorShape,
  type TRPC_ERROR_CODE_KEY,
} from "@trpc/server";
import { getHTTPStatusCodeFromError } from "@trpc/server/http";

import type { DxTrpcContext } from "./context";

export type DxTrpcErrorData = TRPCDefaultErrorData & {
  requestId?: string;
  publicMessage: string;
};

export type DxTrpcErrorShape = TRPCErrorShape<DxTrpcErrorData>;

export const formatDxTrpcError: TRPCErrorFormatter<
  DxTrpcContext,
  DxTrpcErrorShape
> = ({ shape, error, ctx }) => ({
  ...shape,
  data: {
    ...shape.data,
    requestId: ctx?.requestId,
    publicMessage: dxTrpcPublicErrorMessage(error),
    httpStatus: getHTTPStatusCodeFromError(error),
  },
});

export function createDxTrpcError(
  code: TRPC_ERROR_CODE_KEY,
  message?: string,
): TRPCError {
  return new TRPCError({
    code,
    message: message ?? dxTrpcPublicMessageForCode(code),
  });
}

export function dxTrpcHttpStatus(error: TRPCError): number {
  return getHTTPStatusCodeFromError(error);
}

export function dxTrpcPublicErrorMessage(error: TRPCError): string {
  return dxTrpcPublicMessageForCode(error.code);
}

export function dxTrpcPublicMessageForCode(code: TRPC_ERROR_CODE_KEY): string {
  switch (code) {
    case "UNAUTHORIZED":
      return "Sign in to continue.";
    case "FORBIDDEN":
      return "You do not have access to this action.";
    case "NOT_FOUND":
      return "That API resource was not found.";
    case "BAD_REQUEST":
    case "UNPROCESSABLE_CONTENT":
      return "Check the request and try again.";
    case "TOO_MANY_REQUESTS":
      return "Slow down and try again soon.";
    default:
      return "The API request could not be completed.";
  }
}
"#;

const TRPC_RESPONSE_META_TS: &str = r#"import type {
  ResponseMeta,
  ResponseMetaFn,
} from "@trpc/server/http";

import type { AppRouter } from "./router";

export type DxTrpcResponseMetaFn = ResponseMetaFn<AppRouter>;
export type DxTrpcResponseMetaOptions = Parameters<DxTrpcResponseMetaFn>[0];

export type DxTrpcPublicCacheOptions = {
  publicPathPrefix?: string;
  sMaxAge?: number;
  staleWhileRevalidate?: number;
  vary?: string;
};

export function createDxTrpcResponseMeta({
  publicPathPrefix = "public.",
  sMaxAge = 1,
  staleWhileRevalidate = 60,
  vary = "accept, trpc-accept",
}: DxTrpcPublicCacheOptions = {}): DxTrpcResponseMetaFn {
  return ({ ctx, eagerGeneration, errors, paths, type }) => {
    if (ctx?.session || errors.length > 0 || type !== "query" || eagerGeneration) {
      return dxTrpcNoStoreResponseMeta();
    }

    const allPublicPaths =
      paths?.length &&
      paths.every(
        (path) =>
          path === publicPathPrefix || path.startsWith(publicPathPrefix),
      );

    if (!allPublicPaths) {
      return dxTrpcNoStoreResponseMeta();
    }

    return dxTrpcPublicCacheResponseMeta({
      sMaxAge,
      staleWhileRevalidate,
      vary,
    });
  };
}

export function dxTrpcPublicCacheResponseMeta({
  sMaxAge = 1,
  staleWhileRevalidate = 60,
  vary = "accept, trpc-accept",
}: Omit<DxTrpcPublicCacheOptions, "publicPathPrefix"> = {}): ResponseMeta {
  return {
    headers: new Headers([
      [
        "cache-control",
        `s-maxage=${sMaxAge}, stale-while-revalidate=${staleWhileRevalidate}`,
      ],
      ["vary", vary],
    ]),
  };
}

export function dxTrpcNoStoreResponseMeta(): ResponseMeta {
  return {
    headers: new Headers([["cache-control", "no-store"]]),
  };
}

export const dxTrpcResponseMetaPolicy = {
  defaultCacheControl: "no-store",
  publicQueryCache: "only cache anonymous, error-free query batches whose paths share the public prefix",
  nextAppRouterCacheControl:
    "Next.js App Router may override Cache-Control; use native revalidate or unstable_cache when route-level caching must be guaranteed.",
  appOwned:
    "cache taxonomy, CDN behavior, auth policy, path naming, and deployed header verification",
} as const;
"#;

const TRPC_ROUTER_TS: &str = r#"import { tracked } from "@trpc/server";
import type {
  inferRouterError,
  inferRouterInputs,
  inferRouterOutputs,
} from "@trpc/server";
import { z } from "zod";

import { createCallerFactory, publicProcedure, router } from "./server";

const launchEventRows = [
  {
    id: "launch-001",
    event: "viewed" as const,
    route: "/launch",
    summary: "Launch surface opened",
  },
  {
    id: "launch-002",
    event: "validated" as const,
    route: "/launch",
    summary: "Template package checks passed",
  },
  {
    id: "launch-003",
    event: "submitted" as const,
    route: "/launch",
    summary: "Lead form handoff accepted",
  },
  {
    id: "launch-004",
    event: "validated" as const,
    route: "/launch/api",
    summary: "API readiness verified",
  },
] as const;

export const appRouter = router({
  health: publicProcedure.query(({ ctx }) => ({
    status: "ok" as const,
    requestId: ctx.requestId,
    serverTime: new Date().toISOString(),
  })),
  launchReadiness: publicProcedure
    .input(
      z
        .object({
          template: z.string().min(1).max(80).default("dx-www"),
        })
        .optional(),
    )
    .query(({ input }) => ({
      template: input?.template ?? "dx-www",
      api: "trpc" as const,
      ready: true,
    })),
  launchEvents: publicProcedure
    .input(
      z.object({
        cursor: z.number().int().min(0).default(0),
        limit: z.number().int().min(1).max(10).default(2),
      }),
    )
    .query(({ input }) => {
      const start = input.cursor;
      const end = start + input.limit;
      const items = launchEventRows.slice(start, end);

      return {
        items,
        nextCursor: end < launchEventRows.length ? end : undefined,
        total: launchEventRows.length,
      };
    }),
  launchEvent: publicProcedure
    .input(
      z.object({
        event: z.enum(["viewed", "validated", "submitted"]),
        route: z.string().min(1).max(120).default("/launch"),
      }),
    )
    .mutation(({ ctx, input }) => ({
      accepted: true as const,
      event: input.event,
      route: input.route,
      requestId: ctx.requestId,
      serverTime: new Date().toISOString(),
    })),
  launchFeed: publicProcedure
    .input(
      z
        .object({
          stream: z.enum(["readiness", "activity"]).default("readiness"),
          lastEventId: z.coerce.number().int().min(0).optional(),
        })
        .optional(),
    )
    .subscription(async function* ({ input, signal }) {
      let count = input?.lastEventId ?? 0;

      while (!signal?.aborted && count < 3) {
        count += 1;
        yield tracked(String(count), {
          stream: input?.stream ?? "readiness",
          count,
          status: count === 1 ? ("connected" as const) : ("heartbeat" as const),
          serverTime: new Date().toISOString(),
        });

        await waitForLaunchFeedTick(signal);
      }
    }),
});

export type AppRouter = typeof appRouter;
export type AppRouterError = inferRouterError<AppRouter>;
export type AppRouterInputs = inferRouterInputs<AppRouter>;
export type AppRouterOutputs = inferRouterOutputs<AppRouter>;
export type LaunchEventInput = AppRouterInputs["launchEvent"];
export type LaunchEventOutput = AppRouterOutputs["launchEvent"];
export type LaunchEventsInput = AppRouterInputs["launchEvents"];
export type LaunchEventsOutput = AppRouterOutputs["launchEvents"];
export type LaunchFeedInput = AppRouterInputs["launchFeed"];
export type LaunchFeedOutput = AppRouterOutputs["launchFeed"];

export const createCaller = createCallerFactory(appRouter);

function waitForLaunchFeedTick(signal?: AbortSignal): Promise<void> {
  if (signal?.aborted) return Promise.resolve();

  return new Promise((resolve) => {
    const timeout = setTimeout(resolve, 1000);

    signal?.addEventListener(
      "abort",
      () => {
        clearTimeout(timeout);
        resolve();
      },
      { once: true },
    );
  });
}
"#;

const TRPC_ROUTE_HANDLER_TS: &str = r#"import { fetchRequestHandler } from "@trpc/server/adapters/fetch";

import {
  createDxTrpcContext,
  type DxTrpcSession,
} from "./context";
import { dxTrpcHttpStatus } from "./errors";
import {
  createDxTrpcResponseMeta,
  type DxTrpcResponseMetaFn,
} from "./response-meta";
import { appRouter } from "./router";

export const DX_TRPC_ENDPOINT = "/api/trpc";

export type DxTrpcRouteOptions = {
  endpoint?: string;
  responseMeta?: DxTrpcResponseMetaFn;
  resolveSession?: (req: Request) => DxTrpcSession | null | Promise<DxTrpcSession | null>;
  onInternalError?: (event: { error: unknown; path?: string; httpStatus?: number }) => void;
};

export function createDxTrpcRouteHandler(options: DxTrpcRouteOptions = {}) {
  return async function dxTrpcRouteHandler(req: Request) {
    return fetchRequestHandler({
      endpoint: options.endpoint ?? DX_TRPC_ENDPOINT,
      req,
      router: appRouter,
      responseMeta: options.responseMeta ?? createDxTrpcResponseMeta(),
      createContext: async () =>
        createDxTrpcContext({
          req,
          session: (await options.resolveSession?.(req)) ?? null,
        }),
      onError({ error, path }) {
        if (error.code !== "INTERNAL_SERVER_ERROR") return;

        if (options.onInternalError) {
          options.onInternalError({
            error,
            path,
            httpStatus: dxTrpcHttpStatus(error),
          });
          return;
        }

        console.error("tRPC internal error", error);
      },
    });
  };
}

export const dxTrpcRouteHandler = createDxTrpcRouteHandler();
"#;

const TRPC_SERVER_CALLER_TS: &str = r#"import {
  createDxTrpcContext,
  type DxTrpcSession,
} from "./context";
import {
  createCaller,
  type AppRouterOutputs,
} from "./router";

export type DxTrpcServerCallerOptions = {
  headers?: HeadersInit;
  requestId?: string;
  session?: DxTrpcSession | null;
};

export type DxTrpcServerCaller = Awaited<
  ReturnType<typeof createDxTrpcServerCaller>
>;

export type LaunchReadinessOutput = AppRouterOutputs["launchReadiness"];

export async function createDxTrpcServerCaller({
  headers,
  requestId,
  session = null,
}: DxTrpcServerCallerOptions = {}) {
  const req = new Request("http://dx.local/api/trpc", { headers });

  return createCaller(await createDxTrpcContext({
    req,
    requestId,
    session,
  }));
}

export async function readDxTrpcLaunchReadiness(
  options: DxTrpcServerCallerOptions & { template?: string } = {},
): Promise<LaunchReadinessOutput> {
  const {
    template = "www-template",
    ...callerOptions
  } = options;
  const caller = await createDxTrpcServerCaller(callerOptions);

  return await caller.launchReadiness({ template });
}
"#;

const TRPC_APP_ROUTE_TS: &str = r#"import { dxTrpcRouteHandler } from "DX_TRPC_ROUTE_HANDLER_IMPORT";

export const GET = dxTrpcRouteHandler;
export const POST = dxTrpcRouteHandler;
"#;

const TRPC_HTTP_TS: &str = r#"import type {
  HTTPBatchLinkOptions,
  HTTPHeaders,
  Operation,
  TRPCFetch,
} from "@trpc/client";

import type { AppRouter } from "./router";

export type DxTrpcBatchLinkOptions = Omit<
  Pick<
    HTTPBatchLinkOptions<AppRouter["_def"]["_config"]["$types"]>,
    "fetch" | "headers" | "maxItems" | "maxURLLength" | "methodOverride"
  >,
  "fetch"
> & {
  fetch?: TRPCFetch;
};

export type DxTrpcRequestHeadersOptions = {
  getAuthorization?: () => string | null | Promise<string | null>;
  headers?: HTTPHeaders;
  requestId?: string;
  template?: string;
};

export type DxTrpcHttpLinkOptions = Omit<
  DxTrpcBatchLinkOptions,
  "headers"
> &
  DxTrpcRequestHeadersOptions;

export const dxTrpcHttpBatchPolicy = {
  maxItems: 10,
  maxURLLength: 8_000,
  methodOverride: undefined as "POST" | undefined,
  appOwned:
    "auth token source, request id propagation, URL limits, proxy limits, fetch runtime, and cross-origin headers",
} as const;

export function createDxTrpcRequestHeaders({
  getAuthorization,
  headers,
  requestId,
  template = "dx-www",
}: DxTrpcRequestHeadersOptions = {}) {
  return async function dxTrpcRequestHeaders({
    opList,
  }: {
    opList: Operation[];
  }): Promise<HTTPHeaders> {
    const authorization = await getAuthorization?.();

    return {
      ...normalizeDxTrpcHeaders(headers),
      "x-dx-www-template": template,
      ...(requestId ? { "x-request-id": requestId } : {}),
      ...(authorization ? { authorization } : {}),
      "x-trpc-operation-count": String(opList.length),
    };
  };
}

export function createDxTrpcHttpLinkOptions(
  options: DxTrpcHttpLinkOptions = {},
): DxTrpcBatchLinkOptions {
  return {
    fetch: options.fetch,
    headers:
      options.headers || options.requestId || options.template || options.getAuthorization
        ? createDxTrpcRequestHeaders(options)
        : undefined,
    maxItems: options.maxItems ?? dxTrpcHttpBatchPolicy.maxItems,
    maxURLLength: options.maxURLLength ?? dxTrpcHttpBatchPolicy.maxURLLength,
    methodOverride: options.methodOverride,
  };
}

function normalizeDxTrpcHeaders(
  headers?: HTTPHeaders,
): Record<string, string[] | string | undefined> {
  if (!headers) {
    return {};
  }

  if (typeof Headers !== "undefined" && headers instanceof Headers) {
    return Object.fromEntries(headers.entries());
  }

  if (
    typeof (headers as { [Symbol.iterator]?: unknown })[Symbol.iterator] ===
    "function"
  ) {
    return Object.fromEntries(headers as Iterable<[string, string]>);
  }

  return headers as Record<string, string[] | string | undefined>;
}
"#;

const TRPC_CLIENT_TS: &str = r#"import { createTRPCClient, httpBatchLink } from "@trpc/client";

import type { AppRouter } from "./router";
import {
  createDxTrpcHttpLinkOptions,
  type DxTrpcHttpLinkOptions,
} from "./http";
import {
  dxTrpcTransformer,
  type DxTrpcTransformer,
} from "./transformer";

export type DxTrpcClientOptions = DxTrpcHttpLinkOptions & {
  baseUrl?: string;
  transformer?: DxTrpcTransformer;
  url?: string;
};

export function createDxTrpcClient(options: DxTrpcClientOptions = {}) {
  return createTRPCClient<AppRouter>({
    links: [
      httpBatchLink({
        ...createDxTrpcHttpLinkOptions(options),
        transformer: options.transformer ?? dxTrpcTransformer,
        url: options.url ?? dxTrpcUrl(options.baseUrl),
      }),
    ],
  });
}

export type DxTrpcClient = ReturnType<typeof createDxTrpcClient>;

export function dxTrpcUrl(baseUrl = getDxTrpcBaseUrl()): string {
  return `${baseUrl.replace(/\/$/, "")}/api/trpc`;
}

export function getDxTrpcBaseUrl(): string {
  if (typeof window !== "undefined") {
    return "";
  }

  const env = (
    globalThis as {
      process?: { env?: Record<string, string | undefined> };
    }
  ).process?.env ?? {};

  if (env.VERCEL_URL) {
    return `https://${env.VERCEL_URL}`;
  }

  return `http://localhost:${env.PORT ?? "3000"}`;
}
"#;

const TRPC_SUBSCRIPTIONS_TS: &str = r#"import {
  createTRPCClient,
  httpBatchLink,
  httpSubscriptionLink,
  splitLink,
} from "@trpc/client";

import {
  dxTrpcUrl,
  type DxTrpcClientOptions,
} from "./client";
import { createDxTrpcHttpLinkOptions } from "./http";
import type { AppRouter } from "./router";
import { dxTrpcTransformer } from "./transformer";

export type DxTrpcConnectionParams = Record<string, string> | null;

export type DxTrpcSubscriptionClientOptions = DxTrpcClientOptions & {
  connectionParams?:
    | DxTrpcConnectionParams
    | (() => DxTrpcConnectionParams | Promise<DxTrpcConnectionParams>);
  EventSource?: typeof EventSource;
  subscriptionUrl?: string;
};

export function createDxTrpcSubscriptionClient(
  options: DxTrpcSubscriptionClientOptions = {},
) {
  const url = options.url ?? dxTrpcUrl(options.baseUrl);
  const subscriptionUrl = options.subscriptionUrl ?? url;

  return createTRPCClient<AppRouter>({
    links: [
      splitLink({
        condition: (op) => op.type === "subscription",
        true: httpSubscriptionLink({
          url: subscriptionUrl,
          transformer: options.transformer ?? dxTrpcTransformer,
          ...(options.EventSource ? { EventSource: options.EventSource } : {}),
          ...(options.connectionParams
            ? { connectionParams: options.connectionParams }
            : {}),
        }),
        false: httpBatchLink({
          ...createDxTrpcHttpLinkOptions(options),
          url,
          transformer: options.transformer ?? dxTrpcTransformer,
        }),
      }),
    ],
  });
}

export type DxTrpcSubscriptionClient = ReturnType<
  typeof createDxTrpcSubscriptionClient
>;

export const dxTrpcSubscriptionTransport = {
  link: "httpSubscriptionLink",
  splitBy: "operation.type === subscription",
  eventSource: "app-owned EventSource or runtime default",
  resume: "tracked(id, data) event id support",
} as const;
"#;

const TRPC_STREAMING_CLIENT_TS: &str = r#"import {
  createTRPCClient,
  httpBatchStreamLink,
  httpSubscriptionLink,
  loggerLink,
  splitLink,
  type LoggerLinkOptions,
  type TRPCLink,
} from "@trpc/client";

import {
  dxTrpcUrl,
  type DxTrpcClientOptions,
} from "./client";
import { createDxTrpcHttpLinkOptions } from "./http";
import type { AppRouter } from "./router";
import type { DxTrpcConnectionParams } from "./subscriptions";
import { dxTrpcTransformer } from "./transformer";

export type DxTrpcLoggerOptions = Omit<
  LoggerLinkOptions<AppRouter>,
  "enabled"
> & {
  enabled?: boolean;
  logErrors?: boolean;
};

export type DxTrpcStreamingClientOptions = DxTrpcClientOptions & {
  connectionParams?:
    | DxTrpcConnectionParams
    | (() => DxTrpcConnectionParams | Promise<DxTrpcConnectionParams>);
  enableLogger?: boolean;
  enableSubscriptions?: boolean;
  EventSource?: typeof EventSource;
  logger?: DxTrpcLoggerOptions;
  streamHeader?: "trpc-accept" | "accept";
  subscriptionUrl?: string;
};

export function createDxTrpcLoggerLink(
  options: DxTrpcLoggerOptions = {},
): TRPCLink<AppRouter> {
  const {
    enabled = false,
    logErrors = true,
    ...loggerOptions
  } = options;

  return loggerLink<AppRouter>({
    colorMode: "none",
    ...loggerOptions,
    enabled(operation) {
      if (enabled) return true;

      return (
        logErrors &&
        operation.direction === "down" &&
        operation.result instanceof Error
      );
    },
  });
}

export function createDxTrpcStreamingClient(
  options: DxTrpcStreamingClientOptions = {},
) {
  return createTRPCClient<AppRouter>({
    links: createDxTrpcStreamingLinks(options),
  });
}

export type DxTrpcStreamingClient = ReturnType<
  typeof createDxTrpcStreamingClient
>;

export function createDxTrpcStreamingLinks(
  options: DxTrpcStreamingClientOptions = {},
): TRPCLink<AppRouter>[] {
  const url = options.url ?? dxTrpcUrl(options.baseUrl);
  const streamHeader =
    options.streamHeader ?? dxTrpcStreamingTransport.streamHeader;
  const links: TRPCLink<AppRouter>[] = [];

  if (options.enableLogger || options.logger?.enabled) {
    links.push(createDxTrpcLoggerLink(options.logger ?? { enabled: true }));
  }

  if (options.enableSubscriptions) {
    links.push(
      splitLink({
        condition: (op) => op.type === "subscription",
        true: httpSubscriptionLink({
          url: options.subscriptionUrl ?? url,
          transformer: options.transformer ?? dxTrpcTransformer,
          ...(options.EventSource ? { EventSource: options.EventSource } : {}),
          ...(options.connectionParams
            ? { connectionParams: options.connectionParams }
            : {}),
        }),
        false: httpBatchStreamLink({
          ...createDxTrpcHttpLinkOptions(options),
          url,
          transformer: options.transformer ?? dxTrpcTransformer,
          streamHeader,
        }),
      }),
    );
    return links;
  }

  links.push(
    httpBatchStreamLink({
      ...createDxTrpcHttpLinkOptions(options),
      url,
      transformer: options.transformer ?? dxTrpcTransformer,
      streamHeader,
    }),
  );
  return links;
}

export const dxTrpcStreamingTransport = {
  link: "httpBatchStreamLink",
  logger: "loggerLink",
  streamHeader: "trpc-accept",
  subscriptions: "splitLink + httpSubscriptionLink",
  appOwned:
    "stream compatibility, proxy buffering, log redaction, and production observability policy",
} as const;
"#;

const TRPC_PROVIDER_TSX: &str = r#""use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createTRPCContext } from "@trpc/tanstack-react-query";
import * as React from "react";

import {
  createDxTrpcClient,
  type DxTrpcClient,
  type DxTrpcClientOptions,
} from "./client";
import { createDxTrpcStreamingClient } from "./streaming-client";
import { createDxTrpcSubscriptionClient } from "./subscriptions";
import type { AppRouter } from "./router";

export const {
  TRPCProvider: DxTrpcInnerProvider,
  useTRPC,
  useTRPCClient,
} = createTRPCContext<AppRouter>();

export type DxTrpcProviderProps = {
  baseUrl?: string;
  children: React.ReactNode;
  client?: DxTrpcClient;
  enableLogger?: boolean;
  enableSubscriptions?: boolean;
  fetch?: DxTrpcClientOptions["fetch"];
  getAuthorization?: DxTrpcClientOptions["getAuthorization"];
  headers?: DxTrpcClientOptions["headers"];
  maxItems?: DxTrpcClientOptions["maxItems"];
  maxURLLength?: DxTrpcClientOptions["maxURLLength"];
  methodOverride?: DxTrpcClientOptions["methodOverride"];
  queryClient?: QueryClient;
  requestId?: string;
  streamHeader?: "trpc-accept" | "accept";
  subscriptionUrl?: string;
  template?: string;
  transformer?: DxTrpcClientOptions["transformer"];
  transport?: "batch" | "stream" | "subscription";
  url?: string;
};

export function DxTrpcProvider({
  baseUrl,
  children,
  client,
  enableLogger = false,
  enableSubscriptions = false,
  fetch,
  getAuthorization,
  headers,
  maxItems,
  maxURLLength,
  methodOverride,
  queryClient,
  requestId,
  streamHeader,
  subscriptionUrl,
  template,
  transformer,
  transport = "batch",
  url,
}: DxTrpcProviderProps) {
  const [ownedQueryClient] = React.useState(
    () => queryClient ?? new QueryClient(),
  );
  const resolvedTransport = enableSubscriptions ? "subscription" : transport;
  const [ownedTrpcClient] = React.useState(
    () => {
      if (client) return client;

      const clientOptions = {
        baseUrl,
        fetch,
        getAuthorization,
        headers,
        maxItems,
        maxURLLength,
        methodOverride,
        requestId,
        subscriptionUrl,
        template,
        transformer,
        url,
      };

      if (resolvedTransport === "subscription") {
        return createDxTrpcSubscriptionClient(clientOptions);
      }

      if (transport === "stream") {
        return createDxTrpcStreamingClient({
          ...clientOptions,
          enableLogger,
          streamHeader,
        });
      }

      return createDxTrpcClient(clientOptions);
    },
  );

  return (
    <QueryClientProvider client={ownedQueryClient}>
      <DxTrpcInnerProvider
        queryClient={ownedQueryClient}
        trpcClient={ownedTrpcClient}
      >
        {children}
      </DxTrpcInnerProvider>
    </QueryClientProvider>
  );
}
"#;

const TRPC_DASHBOARD_WORKFLOW_TS: &str = r#"export type TrpcDashboardProcedureId =
  | "health.query"
  | "launchEvent.mutation"
  | "launchEvents.infiniteQuery"
  | "launchFeed.subscription";

export type TrpcDashboardProcedure = {
  id: TrpcDashboardProcedureId;
  label: string;
  sourceApi: string;
  dashboardUse: string;
  appOwnedBoundary: string;
};

export type TrpcDashboardReceipt = {
  packageId: "api/trpc";
  procedure: TrpcDashboardProcedureId;
  requestId: string;
  status: "local-receipt";
  cacheAction: string;
  nextAction: string;
};

export const trpcDashboardPackage = {
  packageId: "api/trpc",
  officialDxPackageName: "Type-Safe API",
  packageDisplayName: "Type-Safe API",
  upstreamPackageName: "@trpc/server",
  aliases: [
    "trpc",
    "trpc/next",
    "@trpc/server",
    "@trpc/client",
    "@trpc/tanstack-react-query",
  ],
  sourceMirror: "G:/WWW/inspirations/trpc",
  provenance: {
    upstreamRepo: "trpc/trpc",
    upstreamVersion: "11.17.0",
    inspectedSource: [
      "packages/server/src/unstable-core-do-not-import/initTRPC.ts",
      "packages/server/src/adapters/fetch/fetchRequestHandler.ts",
      "packages/server/src/unstable-core-do-not-import/http/resolveResponse.ts",
      "packages/client/src/createTRPCClient.ts",
      "packages/client/src/links/httpBatchLink.ts",
      "packages/client/src/links/httpBatchStreamLink.ts",
      "packages/client/src/links/httpSubscriptionLink.ts",
      "packages/client/src/links/splitLink.ts",
      "packages/tanstack-react-query/src/internals/createOptionsProxy.ts",
    ],
  },
  requiredEnv: [],
  dashboardEntryPoint: "TrpcDashboardWorkflow",
  publicApis: [
    "initTRPC.context().create()",
    "fetchRequestHandler",
    "createTRPCClient",
    "httpBatchLink",
    "queryOptions",
    "mutationOptions",
    "infiniteQueryOptions",
    "subscriptionOptions",
  ],
  exportedFiles: [
    "lib/trpc/context.ts",
    "lib/trpc/router.ts",
    "lib/trpc/route-handler.ts",
    "lib/trpc/http.ts",
    "lib/trpc/client.ts",
    "lib/trpc/provider.tsx",
    "lib/trpc/dashboard-workflow.ts",
    "components/dashboard/trpc-dashboard-workflow.tsx",
    "lib/trpc/metadata.ts",
    "examples/template/trpc-launch-contract.ts",
    "examples/template/trpc-launch-health.tsx",
  ],
  receiptPaths: [
    ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
    ".dx/forge/receipts/api-trpc.json",
    ".dx/forge/template-readiness/launch-route.json",
    ".dx/forge/template-readiness/launch-runtime-checklist.json",
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
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
    monitoredSurfaces: [
      "trpc-launch-dashboard-workflow",
      "trpc-starter-dashboard-workflow",
      "trpc-route-handler",
    ],
    dxCheckMetrics: [
      "type_safe_api_receipt_present",
      "type_safe_api_receipt_stale",
      "type_safe_api_missing_receipt",
      "type_safe_api_blocked_surface",
      "type_safe_api_unsupported_surface",
      "type_safe_api_hash_manifest_present",
      "type_safe_api_hash_mismatch",
    ],
  },
  appOwnedBoundaries: [
    "domain router implementation",
    "authorization and session context",
    "runtime dependency installation",
    "transport, cache, and stream policy",
  ],
} as const;

export const trpcDashboardProcedures: readonly TrpcDashboardProcedure[] = [
  {
    id: "health.query",
    label: "Health query",
    sourceApi: "fetchRequestHandler + queryOptions",
    dashboardUse: "Read the typed health contract through /api/trpc/health.",
    appOwnedBoundary: "route mounting, request context, and production cache policy",
  },
  {
    id: "launchEvent.mutation",
    label: "Launch event",
    sourceApi: "createTRPCClient + mutationOptions",
    dashboardUse: "Prepare a typed event mutation receipt without writing external state.",
    appOwnedBoundary: "persistence, audit logging, rate limiting, and authorization",
  },
  {
    id: "launchEvents.infiniteQuery",
    label: "Launch feed",
    sourceApi: "infiniteQueryOptions",
    dashboardUse: "Describe the cursor-paginated feed contract for operator activity.",
    appOwnedBoundary: "cursor semantics, retention, and database-backed event storage",
  },
  {
    id: "launchFeed.subscription",
    label: "Live feed",
    sourceApi: "subscriptionOptions",
    dashboardUse: "Expose the subscription-ready surface without opening a stream locally.",
    appOwnedBoundary: "fan-out, stream pacing, runtime transport, and auth policy",
  },
];

export function getTrpcDashboardProcedure(
  procedureId: TrpcDashboardProcedureId,
): TrpcDashboardProcedure {
  return (
    trpcDashboardProcedures.find((procedure) => procedure.id === procedureId) ??
    trpcDashboardProcedures[0]
  );
}

export function createTrpcDashboardReceipt(
  procedure: TrpcDashboardProcedureId,
  sequence = 1,
): TrpcDashboardReceipt {
  return {
    packageId: trpcDashboardPackage.packageId,
    procedure,
    requestId: `dx-trpc-dashboard-${String(sequence).padStart(2, "0")}`,
    status: "local-receipt",
    cacheAction:
      procedure === "health.query"
        ? "queryOptions can hydrate a health cache once the route is mounted"
        : "query client invalidation remains app-owned until runtime wiring",
    nextAction:
      "Install the tRPC runtime dependencies, mount the app router, and connect auth/session context before executing network calls.",
  };
}
"#;

const TRPC_DASHBOARD_WORKFLOW_TSX: &str = r#""use client";

import * as React from "react";

import {
  createTrpcDashboardReceipt,
  getTrpcDashboardProcedure,
  trpcDashboardPackage,
  trpcDashboardProcedures,
  type TrpcDashboardProcedureId,
  type TrpcDashboardReceipt,
} from "@/lib/trpc/dashboard-workflow";

export type TrpcDashboardWorkflowProps = {
  initialProcedure?: TrpcDashboardProcedureId;
};

export function TrpcDashboardWorkflow({
  initialProcedure = "health.query",
}: TrpcDashboardWorkflowProps) {
  const [procedureId, setProcedureId] =
    React.useState<TrpcDashboardProcedureId>(initialProcedure);
  const [sequence, setSequence] = React.useState(1);
  const [receipt, setReceipt] = React.useState<TrpcDashboardReceipt | null>(null);
  const activeProcedure = getTrpcDashboardProcedure(procedureId);

  return (
    <section
      className="rounded-lg border border-border bg-card p-4 text-card-foreground shadow-sm"
      data-dx-package="api/trpc"
      data-dx-component="dashboard-trpc-workflow"
      data-dx-dashboard-workflow="typed-api-boundary"
      data-dx-style-surface="theme-token"
      data-dx-trpc-procedure={activeProcedure.id}
      data-dx-trpc-runtime={receipt ? receipt.status : "ready-for-runtime"}
      data-dx-node-modules="forbidden"
    >
      <header className="flex items-start gap-3">
        <dx-icon name="api:trpc" aria-label="tRPC" />
        <div>
          <h2 className="text-lg font-semibold text-card-foreground">
            Type-Safe API workflow
          </h2>
          <p className="text-sm text-muted-foreground">
            Prepare dashboard procedures against the source-owned upstream tRPC
            router, client, and TanStack Query boundary.
          </p>
        </div>
      </header>

      <div
        className="mt-4 flex flex-wrap gap-2"
        data-dx-trpc-interaction="procedure-picker"
      >
        {trpcDashboardProcedures.map((procedure) => (
          <button
            key={procedure.id}
            type="button"
            className={
              procedure.id === activeProcedure.id
                ? "rounded-md border border-primary bg-primary px-3 py-2 text-sm text-primary-foreground"
                : "rounded-md border border-border bg-background px-3 py-2 text-sm text-foreground"
            }
            data-dx-trpc-action="select-procedure"
            data-dx-trpc-procedure-option={procedure.id}
            data-dx-trpc-selected={
              procedure.id === activeProcedure.id ? "true" : "false"
            }
            onClick={() => {
              setProcedureId(procedure.id);
              setReceipt(null);
            }}
          >
            {procedure.label}
          </button>
        ))}
      </div>

      <dl className="mt-4 grid gap-3 text-sm" data-dx-trpc-readiness="app-owned">
        <div>
          <dt className="font-medium text-card-foreground">Source API</dt>
          <dd
            className="text-muted-foreground"
            data-dx-trpc-source-api={activeProcedure.sourceApi}
          >
            {activeProcedure.sourceApi}
          </dd>
        </div>
        <div>
          <dt className="font-medium text-card-foreground">Dashboard use</dt>
          <dd
            className="text-muted-foreground"
            data-dx-trpc-dashboard-use={activeProcedure.id}
          >
            {activeProcedure.dashboardUse}
          </dd>
        </div>
        <div>
          <dt className="font-medium text-card-foreground">Required env</dt>
          <dd className="text-muted-foreground" data-dx-trpc-required-env="none">
            None for local receipt; runtime packages are app-owned.
          </dd>
        </div>
        <div>
          <dt className="font-medium text-card-foreground">Boundary</dt>
          <dd
            className="text-muted-foreground"
            data-dx-trpc-boundary={activeProcedure.appOwnedBoundary}
          >
            {activeProcedure.appOwnedBoundary}
          </dd>
        </div>
      </dl>

      <button
        type="button"
        className="mt-4 inline-flex items-center gap-2 rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground"
        data-dx-trpc-action="prepare-local-receipt"
        onClick={() => {
          setReceipt(createTrpcDashboardReceipt(activeProcedure.id, sequence));
          setSequence(sequence + 1);
        }}
      >
        <dx-icon name="api:trpc" aria-hidden="true" />
        Prepare API receipt
      </button>

      <p
        className="mt-3 text-sm text-muted-foreground"
        data-dx-trpc-receipt-state={receipt ? receipt.status : "idle"}
        data-dx-trpc-request-id={receipt?.requestId ?? "idle"}
        data-dx-trpc-source-mirror={trpcDashboardPackage.sourceMirror}
      >
        {receipt
          ? `${receipt.packageId} ${receipt.procedure} prepared ${receipt.requestId}. ${receipt.cacheAction}. ${receipt.nextAction}`
          : `Source mirror: ${trpcDashboardPackage.sourceMirror}. Choose a procedure and prepare a local receipt before runtime wiring.`}
      </p>
    </section>
  );
}

export default TrpcDashboardWorkflow;
"#;

const TRPC_METADATA_TS: &str = r#"export const dxTrpcForgePackage = {
  packageId: "api/trpc",
  officialDxPackageName: "Type-Safe API",
  packageDisplayName: "Type-Safe API",
  upstreamPackageName: "@trpc/server",
  aliases: [
    "trpc",
    "trpc/next",
    "@trpc/server",
    "@trpc/client",
    "@trpc/tanstack-react-query",
  ],
  sourceMirror: "G:/WWW/inspirations/trpc",
  provenance: {
    upstreamRepo: "trpc/trpc",
    upstreamPackages: [
      "packages/server",
      "packages/client",
      "packages/tanstack-react-query",
    ],
    studiedFiles: [
      "packages/server/src/@trpc/server/index.ts",
      "packages/server/src/adapters/fetch/fetchRequestHandler.ts",
      "packages/client/src/links/httpBatchLink.ts",
      "packages/client/src/links/httpBatchStreamLink.ts",
      "packages/client/src/links/httpSubscriptionLink.ts",
      "packages/tanstack-react-query/src/index.ts",
      "examples/lazy-load/src/server/routers/_app.ts",
    ],
  },
  upstreamPackages: [
    {
      name: "@trpc/server",
      version: "11.17.0",
      required: true,
    },
    {
      name: "@trpc/client",
      version: "11.17.0",
      required: true,
    },
    {
      name: "@trpc/tanstack-react-query",
      version: "11.17.0",
      required: true,
    },
    {
      name: "@tanstack/react-query",
      version: "^5.80.3",
      required: true,
    },
    {
      name: "zod",
      version: "^4",
      required: true,
    },
  ],
  requiredEnv: [],
  exportedFiles: [
    "lib/trpc/context.ts",
    "lib/trpc/transformer.ts",
    "lib/trpc/server.ts",
    "lib/trpc/errors.ts",
    "lib/trpc/response-meta.ts",
    "lib/trpc/router.ts",
    "lib/trpc/route-handler.ts",
    "lib/trpc/server-caller.ts",
    "app/api/trpc/[trpc]/route.ts",
    "lib/trpc/http.ts",
    "lib/trpc/client.ts",
    "lib/trpc/subscriptions.ts",
    "lib/trpc/streaming-client.ts",
    "lib/trpc/provider.tsx",
    "lib/trpc/dashboard-workflow.ts",
    "components/dashboard/trpc-dashboard-workflow.tsx",
    "lib/trpc/metadata.ts",
  ],
  sourceSurface: [
    "initTRPC.context().create()",
    "errorFormatter from initTRPC.create()",
    "router, procedure, mergeRouters, and createCallerFactory",
    "TRPCCombinedDataTransformer and TRPCDataTransformer from @trpc/server",
    "TRPCErrorFormatter, TRPCErrorShape, and inferRouterError from @trpc/server",
    "getHTTPStatusCodeFromError from @trpc/server/http",
    "responseMeta, ResponseMeta, and ResponseMetaFn from @trpc/server/http",
    "inferRouterInputs and inferRouterOutputs from @trpc/server",
    "fetchRequestHandler from @trpc/server/adapters/fetch",
    "createCallerFactory server callers for RSC/server actions",
    "createTRPCClient and httpBatchLink from @trpc/client",
    "HTTPBatchLinkOptions, HTTPHeaders, TRPCFetch, maxItems, maxURLLength, methodOverride, and headers from @trpc/client",
    "httpSubscriptionLink and splitLink from @trpc/client",
    "httpBatchStreamLink and loggerLink from @trpc/client",
    "createTRPCContext from @trpc/tanstack-react-query",
    "mutationOptions from @trpc/tanstack-react-query",
    "infiniteQueryOptions, infiniteQueryKey, and infiniteQueryFilter from @trpc/tanstack-react-query",
    "subscriptionOptions and useSubscription from @trpc/tanstack-react-query",
    "dashboard procedure readiness and local receipts from the DX-owned starter workflow",
  ],
  materializedFiles: [
    "lib/trpc/context.ts",
    "lib/trpc/transformer.ts",
    "lib/trpc/server.ts",
    "lib/trpc/errors.ts",
    "lib/trpc/response-meta.ts",
    "lib/trpc/router.ts",
    "lib/trpc/route-handler.ts",
    "lib/trpc/server-caller.ts",
    "app/api/trpc/[trpc]/route.ts",
    "lib/trpc/http.ts",
    "lib/trpc/client.ts",
    "lib/trpc/subscriptions.ts",
    "lib/trpc/streaming-client.ts",
    "lib/trpc/provider.tsx",
    "lib/trpc/dashboard-workflow.ts",
    "components/dashboard/trpc-dashboard-workflow.tsx",
    "lib/trpc/metadata.ts",
    "examples/template/trpc-launch-contract.ts",
    "examples/template/trpc-launch-health.tsx",
    "examples/template/trpc-server-readiness.ts",
    "examples/template/trpc-subscription-status.tsx",
    "examples/template/trpc-error-status.tsx",
    "examples/template/trpc-streaming-client-status.tsx",
    "examples/template/trpc-response-meta.ts",
    "examples/template/trpc-infinite-feed.tsx",
    "examples/template/trpc-transformer-status.ts",
    "examples/template/trpc-request-policy.ts",
    "lib/trpc/README.md",
  ],
  discovery: {
    dxAdd: "dx add trpc --write",
    dxDryRun: "dx add trpc --dry-run --format json",
    icon: "api:trpc",
    route: "app/api/trpc/[trpc]/route.ts",
    provider: "DxTrpcProvider",
    routerType: "AppRouter",
    transformer: "dxTrpcTransformer",
    transformerFactory: "createDxTrpcTransformer()",
    errorFormatter: "formatDxTrpcError",
    errorShape: "AppRouterError",
    responseMeta: "createDxTrpcResponseMeta()",
    routerInputsType: "AppRouterInputs",
    routerOutputsType: "AppRouterOutputs",
    hook: "useTRPC()",
    mutation: "launchEvent",
    infiniteQuery: "launchEvents.infiniteQueryOptions",
    infiniteQueryKey: "launchEvents.infiniteQueryKey",
    infiniteQueryFilter: "launchEvents.infiniteQueryFilter",
    subscription: "launchFeed",
    subscriptionClient: "createDxTrpcSubscriptionClient()",
    subscriptionHook: "launchFeed.subscriptionOptions",
    streamingClient: "createDxTrpcStreamingClient()",
    diagnosticsLink: "createDxTrpcLoggerLink()",
    requestPolicy: "createDxTrpcHttpLinkOptions()",
    serverCaller: "createDxTrpcServerCaller()",
    serverSnapshot: "readDxTrpcLaunchReadiness()",
    dashboardWorkflowApiFile: "lib/trpc/dashboard-workflow.ts",
    dashboardWorkflowComponent: "components/dashboard/trpc-dashboard-workflow.tsx",
  },
  dashboardWorkflow: {
    route: "/launch",
    component: "launch-trpc-api-dashboard-workflow",
    visibleSelector:
      '[data-dx-package="api/trpc"][data-dx-component="launch-trpc-api-dashboard-workflow"]',
    workflowMarker: 'data-dx-dashboard-workflow="typed-api-readiness"',
    interactionMarker: 'data-dx-trpc-action="prepare-launch-event"',
    icon: '<dx-icon name="api:trpc" />',
    localAction: "safe launchEvent validation",
    runtimeBoundary: "source-owned-runtime-pending",
    launchDashboard: {
      component: "launch-trpc-api-dashboard-workflow",
      visibleSelector:
        '[data-dx-package="api/trpc"][data-dx-component="launch-trpc-api-dashboard-workflow"]',
      workflowMarker: 'data-dx-dashboard-workflow="typed-api-readiness"',
      dashboardCard: 'data-dx-dashboard-card="typed-api"',
      healthAction: 'data-dx-trpc-action="check-health"',
      launchEventAction: 'data-dx-trpc-action="prepare-launch-event"',
      runtimePage: "tools/launch/runtime-template/pages/index.html#mission-control",
    },
    starterDashboard: {
      component: "TrpcDashboardWorkflow",
      visibleSelector:
        '[data-dx-package="api/trpc"][data-dx-component="dashboard-trpc-workflow"]',
      workflowMarker: 'data-dx-dashboard-workflow="typed-api-boundary"',
      interactionMarker: 'data-dx-trpc-action="prepare-local-receipt"',
      icon: '<dx-icon name="api:trpc" />',
      localAction: "safe typed procedure receipt",
    },
  },
  appOwnedBoundaries: [
    "domain routers",
    "authorization and session policy",
    "request limits and request id propagation",
    "auth token source and cross-origin headers",
    "runtime dependency installation",
    "subscription fan-out and stream pacing",
    "EventSource/runtime policy",
    "serializer dependency selection and custom type registration",
    "cache taxonomy, CDN/proxy behavior, and deployed header verification",
    "audit logging, rate limiting, log redaction, and production observability",
  ],
  receiptPaths: [
    ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
    ".dx/forge/receipts/api-trpc.json",
    ".dx/forge/template-readiness/launch-route.json",
    ".dx/forge/template-readiness/launch-runtime-checklist.json",
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
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
    monitoredSurfaces: [
      "trpc-launch-dashboard-workflow",
      "trpc-starter-dashboard-workflow",
      "trpc-route-handler",
    ],
    dxCheckMetrics: [
      "type_safe_api_receipt_present",
      "type_safe_api_receipt_stale",
      "type_safe_api_missing_receipt",
      "type_safe_api_blocked_surface",
      "type_safe_api_unsupported_surface",
      "type_safe_api_hash_manifest_present",
      "type_safe_api_hash_mismatch",
    ],
  },
} as const;

export type DxTrpcForgePackageMetadata = typeof dxTrpcForgePackage;
"#;

const TRPC_LAUNCH_CONTRACT_TS: &str = r#"export const trpcLaunchContract = {
  packageId: "api/trpc",
  officialName: "Type-Safe API",
  upstreamPackage: "@trpc/server",
  route: "/api/trpc/[trpc]",
  endpoint: "/api/trpc",
  procedures: {
    health: "health.query",
    launchEvent: "launchEvent.mutation",
  },
  sourceApis: [
    "initTRPC.create",
    "fetchRequestHandler",
    "trpc.health.queryOptions()",
    "trpc.launchEvent.mutationOptions()",
    "trpc.health.queryFilter()",
  ],
} as const;

export type LaunchEventResult = {
  cacheAction: "invalidate health.query";
  event: "validated";
  procedure: typeof trpcLaunchContract.procedures.launchEvent;
  requestId: string;
  route: "/launch";
  status: "accepted";
};

export type HealthCheckResult = {
  cacheAction: "hydrate health.query";
  procedure: typeof trpcLaunchContract.procedures.health;
  requestId: string;
  route: "/api/trpc/health";
  status: "ready";
};

export type TrpcLaunchWorkflowResult = HealthCheckResult | LaunchEventResult;

export function createLocalHealthCheck(sequence: number): HealthCheckResult {
  return {
    cacheAction: "hydrate health.query",
    procedure: trpcLaunchContract.procedures.health,
    requestId: `dx-trpc-health-${String(sequence).padStart(2, "0")}`,
    route: "/api/trpc/health",
    status: "ready",
  };
}

export function createLocalLaunchEvent(sequence: number): LaunchEventResult {
  return {
    cacheAction: "invalidate health.query",
    event: "validated",
    procedure: trpcLaunchContract.procedures.launchEvent,
    requestId: `dx-trpc-local-${String(sequence).padStart(2, "0")}`,
    route: "/launch",
    status: "accepted",
  };
}
"#;

const TRPC_LAUNCH_HEALTH_TSX: &str = r#""use client";

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
"#;

const TRPC_LAUNCH_SERVER_READINESS_TS: &str = r#"import { readDxTrpcLaunchReadiness } from "@/lib/trpc/server-caller";

export async function loadTrpcLaunchReadiness() {
  const snapshot = await readDxTrpcLaunchReadiness({
    template: "www-template",
    requestId: "dx-www-template",
  });

  return {
    packageId: "api/trpc" as const,
    api: snapshot.api,
    ready: snapshot.ready,
    template: snapshot.template,
  };
}
"#;

const TRPC_LAUNCH_SUBSCRIPTION_STATUS_TSX: &str = r#""use client";

import { useSubscription } from "@trpc/tanstack-react-query";
import * as React from "react";

import { useTRPC } from "@/lib/trpc/provider";
import type { LaunchFeedOutput } from "@/lib/trpc/router";

export type TrpcLaunchSubscriptionStatusProps = {
  enabled?: boolean;
};

export function TrpcLaunchSubscriptionStatus({
  enabled = true,
}: TrpcLaunchSubscriptionStatusProps) {
  const trpc = useTRPC();
  const [events, setEvents] = React.useState<LaunchFeedOutput[]>([]);
  const subscription = useSubscription(
    trpc.launchFeed.subscriptionOptions(
      { stream: "readiness" },
      {
        enabled,
        onData(data) {
          setEvents((current) => [data, ...current].slice(0, 3));
        },
      },
    ),
  );

  if (subscription.status === "error") {
    return (
      <p data-trpc-subscription="error" role="alert">
        {subscription.error.message}
      </p>
    );
  }

  if (subscription.status === "idle") {
    return <p data-trpc-subscription="idle">Subscription paused.</p>;
  }

  if (subscription.status === "connecting") {
    return <p data-trpc-subscription="connecting">Connecting launch feed...</p>;
  }

  return (
    <section data-trpc-subscription="pending">
      <p>Launch feed receiving {events.length} event(s).</p>
      <ol>
        {events.map((event) => (
          <li key={`${event.count}-${event.serverTime}`}>
            {event.status} #{event.count}
          </li>
        ))}
      </ol>
    </section>
  );
}
"#;

const TRPC_LAUNCH_ERROR_STATUS_TSX: &str = r#""use client";

import { TRPCClientError } from "@trpc/client";

import type { AppRouterError } from "@/lib/trpc/router";

type AppRouterErrorData = AppRouterError["data"];

export type TrpcErrorStatusProps = {
  error?: unknown;
};

export function TrpcErrorStatus({ error }: TrpcErrorStatusProps) {
  if (!(error instanceof TRPCClientError)) {
    return <p data-trpc-error-code="none">No tRPC error.</p>;
  }

  const data = error.data as AppRouterErrorData | undefined;
  const publicMessage = data?.publicMessage ?? error.message;

  return (
    <p
      data-trpc-error-code={data?.code ?? "UNKNOWN"}
      data-trpc-error-http-status={data?.httpStatus ?? "unknown"}
      role="alert"
    >
      {publicMessage}
    </p>
  );
}
"#;

const TRPC_LAUNCH_STREAMING_CLIENT_STATUS_TSX: &str = r#""use client";

import { DxTrpcProvider } from "@/lib/trpc/provider";
import { dxTrpcStreamingTransport } from "@/lib/trpc/streaming-client";

import { TrpcLaunchHealth } from "./trpc-launch-health";

export function TrpcStreamingClientStatus() {
  return (
    <DxTrpcProvider transport="stream" enableLogger>
      <section
        data-trpc-transport="httpBatchStreamLink"
        data-trpc-logger={dxTrpcStreamingTransport.logger}
      >
        <TrpcLaunchHealth />
      </section>
    </DxTrpcProvider>
  );
}
"#;

const TRPC_LAUNCH_RESPONSE_META_TS: &str = r#"import { createDxTrpcRouteHandler } from "@/lib/trpc/route-handler";
import {
  createDxTrpcResponseMeta,
  dxTrpcResponseMetaPolicy,
} from "@/lib/trpc/response-meta";

export const launchResponseMeta = createDxTrpcResponseMeta({
  publicPathPrefix: "health",
  sMaxAge: 1,
  staleWhileRevalidate: 60,
});

export const launchTrpcRouteHandler = createDxTrpcRouteHandler({
  responseMeta: launchResponseMeta,
});

export const trpcResponseMetaReadiness = {
  packageId: "api/trpc" as const,
  responseMeta: "createDxTrpcResponseMeta",
  defaultCacheControl: dxTrpcResponseMetaPolicy.defaultCacheControl,
  appRouterCaveat: dxTrpcResponseMetaPolicy.nextAppRouterCacheControl,
};
"#;

const TRPC_LAUNCH_INFINITE_FEED_TSX: &str = r#""use client";

import { useInfiniteQuery, useQueryClient } from "@tanstack/react-query";

import { useTRPC } from "@/lib/trpc/provider";

export function TrpcInfiniteFeed() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const queryKey = trpc.launchEvents.infiniteQueryKey({ limit: 2 });
  const feed = useInfiniteQuery(
    trpc.launchEvents.infiniteQueryOptions(
      { limit: 2 },
      {
        getNextPageParam(lastPage) {
          return lastPage.nextCursor;
        },
      },
    ),
  );

  function refreshFeed() {
    void queryClient.invalidateQueries(
      trpc.launchEvents.infiniteQueryFilter({ limit: 2 }),
    );
  }

  if (feed.isPending) {
    return <p data-trpc-infinite-feed="loading">Loading launch events...</p>;
  }

  if (feed.isError) {
    return (
      <p data-trpc-infinite-feed="error" role="alert">
        {feed.error.message}
      </p>
    );
  }

  const events = feed.data.pages.flatMap((page) => page.items);

  return (
    <section
      data-trpc-infinite-feed="ready"
      data-trpc-infinite-key={queryKey[0].join(".")}
    >
      <p>Loaded {events.length} launch event(s).</p>
      <ol>
        {events.map((event) => (
          <li key={event.id}>
            {event.event} on {event.route}: {event.summary}
          </li>
        ))}
      </ol>
      <button
        type="button"
        onClick={() => void feed.fetchNextPage()}
        disabled={!feed.hasNextPage || feed.isFetchingNextPage}
      >
        {feed.isFetchingNextPage
          ? "Loading more..."
          : feed.hasNextPage
            ? "Load more events"
            : "All launch events loaded"}
      </button>
      <button type="button" onClick={refreshFeed}>
        Refresh events
      </button>
    </section>
  );
}
"#;

const TRPC_LAUNCH_TRANSFORMER_STATUS_TS: &str = r#"import {
  createDxTrpcTransformer,
  dxTrpcIdentityDataTransformer,
  dxTrpcTransformerPolicy,
} from "@/lib/trpc/transformer";

export const launchTransformer = createDxTrpcTransformer(
  dxTrpcIdentityDataTransformer,
);

export const trpcTransformerReadiness = {
  packageId: "api/trpc" as const,
  transformer: "dxTrpcTransformer",
  defaultTransformer: dxTrpcTransformerPolicy.defaultTransformer,
  contract: dxTrpcTransformerPolicy.contract,
  appOwned: dxTrpcTransformerPolicy.appOwned,
};
"#;

const TRPC_LAUNCH_REQUEST_POLICY_TS: &str = r#"import {
  createDxTrpcHttpLinkOptions,
  createDxTrpcRequestHeaders,
  dxTrpcHttpBatchPolicy,
} from "@/lib/trpc/http";

export const launchRequestHeaders = createDxTrpcRequestHeaders({
  headers: {
    "x-dx-www-template": "www-template",
  },
  requestId: "dx-www-template",
  template: "www-template",
});

export const launchRequestPolicy = createDxTrpcHttpLinkOptions({
  headers: {
    "x-dx-www-template": "www-template",
  },
  maxItems: 5,
  maxURLLength: dxTrpcHttpBatchPolicy.maxURLLength,
  requestId: "dx-www-template",
  template: "www-template",
});

export const trpcRequestPolicyReadiness = {
  packageId: "api/trpc" as const,
  requestPolicy: "createDxTrpcHttpLinkOptions",
  header: "x-dx-www-template",
  maxItems: launchRequestPolicy.maxItems,
  maxURLLength: launchRequestPolicy.maxURLLength,
  appOwned: dxTrpcHttpBatchPolicy.appOwned,
};
"#;

const TRPC_README_MD: &str = r#"# Type-Safe API

Type-Safe API is the official DX Forge package lane for typed API work. It materializes a small source-owned adapter around the real upstream tRPC 11 public APIs. It does not reimplement tRPC, fake router typing, or run package-manager lifecycle scripts.

## Owned Files

- `lib/trpc/context.ts` creates a request context with headers, request id, and optional session.
- `lib/trpc/transformer.ts` owns the app-replaceable data transformer boundary used by server and client links.
- `lib/trpc/server.ts` owns the single `initTRPC.context().create()` call plus base procedures.
- `lib/trpc/errors.ts` owns a typed `errorFormatter`, public error copy, and HTTP status helpers.
- `lib/trpc/response-meta.ts` owns conservative `responseMeta` helpers for cache/status headers.
- `lib/trpc/router.ts` provides a typed launch router, `AppRouter`, inferred input/output helpers, a launch-safe mutation, a cursor-paginated query, and a tiny tracked subscription.
- `lib/trpc/route-handler.ts` adapts the router through `fetchRequestHandler`.
- `lib/trpc/server-caller.ts` creates typed in-process callers for App Router, RSC, and server actions.
- `app/api/trpc/[trpc]/route.ts` mounts the handler for Next App Router.
- `lib/trpc/http.ts` owns app-replaceable request headers, batch limits, URL limits, method override, and fetch runtime options for HTTP links.
- `lib/trpc/client.ts` creates the typed client with `httpBatchLink`.
- `lib/trpc/subscriptions.ts` creates an opt-in typed client with `splitLink` plus `httpSubscriptionLink`.
- `lib/trpc/streaming-client.ts` creates an opt-in `httpBatchStreamLink` client with optional `loggerLink` diagnostics.
- `lib/trpc/provider.tsx` wires TanStack Query and `@trpc/tanstack-react-query`.
- `lib/trpc/dashboard-workflow.ts` owns the starter dashboard procedure readiness metadata and local receipt helper.
- `components/dashboard/trpc-dashboard-workflow.tsx` renders the dashboard workflow with theme-token classes, DX icons, stable `data-dx-trpc-*` markers, and a safe local receipt action.
- `lib/trpc/metadata.ts` gives DX CLI, Zed, and launch templates a stable discovery record.
- `examples/template/trpc-launch-contract.ts` owns the launch route/procedure contract and local `launchEvent` receipt helper used by the visible workflow.
- `examples/template/trpc-launch-health.tsx` renders the no-node_modules typed API workflow used by `/launch`, with stable DX markers, a local typed `launchEvent` interaction, and honest route-runtime boundaries.
- `examples/template/trpc-server-readiness.ts` proves the server caller path without HTTP roundtrips or fake runtime evidence.
- `examples/template/trpc-subscription-status.tsx` proves the `subscriptionOptions` plus `useSubscription` path against the launch feed.
- `examples/template/trpc-error-status.tsx` proves typed client error rendering from the app router error shape.
- `examples/template/trpc-streaming-client-status.tsx` proves the streaming transport/provider path without requiring a local server.
- `examples/template/trpc-response-meta.ts` proves opt-in `responseMeta` wiring for route handlers.
- `examples/template/trpc-infinite-feed.tsx` proves cursor pagination through `infiniteQueryOptions`, `infiniteQueryKey`, and `infiniteQueryFilter`.
- `examples/template/trpc-transformer-status.ts` proves the shared transformer contract without adding a serializer dependency.
- `examples/template/trpc-request-policy.ts` proves request header and batch policy wiring without shipping credentials or app secrets.

## Metadata And Dashboard Workflow

`lib/trpc/metadata.ts` publishes the official DX package name `Type-Safe API`, canonical package id `api/trpc`, upstream package provenance, aliases, `G:/WWW/inspirations/trpc` source mirror, studied upstream files, exported files, required env, app-owned boundaries, receipt paths, and dashboard workflow selectors. The starter dashboard consumes the package through `TrpcDashboardWorkflow`, not just a catalog card: the component exposes `data-dx-dashboard-workflow="typed-api-boundary"`, `data-dx-package="api/trpc"`, `data-dx-component="dashboard-trpc-workflow"`, a `data-dx-trpc-action="prepare-local-receipt"` button, and `<dx-icon name="api:trpc" />` while staying on theme-token classes. The launch template consumes `TrpcLaunchHealth` from the account/data dashboard as `data-dx-component="launch-trpc-api-dashboard-workflow"` with health-query and launchEvent actions wired into mission control.

## Required App Dependencies

Install or provide `@trpc/server`, `@trpc/client`, `@trpc/tanstack-react-query`, `@tanstack/react-query`, `zod`, React, and Next in the host app. Forge owns these adapter files and receipts; it does not vendor the tRPC runtime.

## Template Usage

Wrap your app shell once:

```tsx
import { DxTrpcProvider } from "@/lib/trpc/provider";

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <DxTrpcProvider>{children}</DxTrpcProvider>
      </body>
    </html>
  );
}
```

Use the typed options proxy in client components:

```tsx
"use client";

import { TrpcLaunchHealth } from "@/examples/template/trpc-launch-health";

export function LaunchApiPanel() {
  return <TrpcLaunchHealth />;
}
```

Read the same router from server code when you need typed data without an HTTP roundtrip:

```ts
import { readDxTrpcLaunchReadiness } from "@/lib/trpc/server-caller";

export async function loadLaunchReadiness() {
  return readDxTrpcLaunchReadiness({ template: "www-template" });
}
```

Enable HTTP subscriptions when a Client Component needs a live launch feed:

```tsx
import { DxTrpcProvider } from "@/lib/trpc/provider";
import { TrpcLaunchSubscriptionStatus } from "@/examples/template/trpc-subscription-status";

export function LaunchFeedPanel() {
  return (
    <DxTrpcProvider enableSubscriptions>
      <TrpcLaunchSubscriptionStatus />
    </DxTrpcProvider>
  );
}
```

Use cursor pagination for launch activity feeds:

```tsx
"use client";

import { TrpcInfiniteFeed } from "@/examples/template/trpc-infinite-feed";

export function LaunchActivityPanel() {
  return <TrpcInfiniteFeed />;
}
```

Swap the transformer file when your app needs richer payloads:

```ts
import { createDxTrpcTransformer } from "@/lib/trpc/transformer";
import SuperJSON from "superjson";

export const dxTrpcTransformer = createDxTrpcTransformer(SuperJSON);
```

Set request headers and batch limits at the provider boundary:

```tsx
import { DxTrpcProvider } from "@/lib/trpc/provider";

export function LaunchApiProvider({ children }: { children: React.ReactNode }) {
  return (
    <DxTrpcProvider
      maxItems={5}
      requestId="dx-www-template"
      template="www-template"
    >
      {children}
    </DxTrpcProvider>
  );
}
```

Render safe tRPC errors with the formatted router error shape:

```tsx
"use client";

import { TrpcErrorStatus } from "@/examples/template/trpc-error-status";

export function LaunchErrorPanel({ error }: { error?: unknown }) {
  return <TrpcErrorStatus error={error} />;
}
```

Opt into streaming query/mutation transport and diagnostics when the app is ready for JSONL responses:

```tsx
"use client";

import { DxTrpcProvider } from "@/lib/trpc/provider";
import { TrpcLaunchHealth } from "@/examples/template/trpc-launch-health";

export function StreamingLaunchApiPanel() {
  return (
    <DxTrpcProvider transport="stream" enableLogger>
      <TrpcLaunchHealth />
    </DxTrpcProvider>
  );
}
```

Attach conservative response metadata when a route needs explicit cache boundaries:

```ts
import { createDxTrpcRouteHandler } from "@/lib/trpc/route-handler";
import { createDxTrpcResponseMeta } from "@/lib/trpc/response-meta";

export const GET = createDxTrpcRouteHandler({
  responseMeta: createDxTrpcResponseMeta({ publicPathPrefix: "health" }),
});
```

Keep business procedures in application routers. This slice is the launch-safe baseline for typed server procedures, route handling, data transformation, response metadata, request headers and batch policy, client batching/streaming, inferred router input/output/error types, formatted public error payloads, TanStack Query query/mutation/infinite-query/subscription provider wiring, and HTTP subscription transport discovery.

## App-Owned Boundaries

Applications own production domain routers, authorization, sessions, request limits, request id propagation, auth token source, cross-origin header policy, proxy URL limits, fetch runtime selection, persistence, audit logging, rate limiting, pagination cursor semantics, event retention, serializer dependency selection, custom type registration, payload compatibility review, subscription fan-out, stream pacing, EventSource/runtime policy, JSONL/proxy compatibility, cache taxonomy, CDN/proxy behavior, deployed header verification, log redaction policy, production observability, error redaction policy, public error copy, and runtime dependency installation. The included `launchEvent` mutation, `launchEvents` paginated query, `launchFeed` subscription, identity transformer, public error formatter, request policy helper, response metadata helper, and streaming client helper are intentionally small: they prove the public mutation/inference/infinite-query/subscription/transformer/error/request-policy/streaming/cache API shape without pretending to provide durable analytics, production realtime infrastructure, a production event store, application-specific serializer policy, credential handling, or application-specific incident policy. Note that Next.js App Router may override `Cache-Control`; use native `revalidate` or `unstable_cache` when route-level caching must be guaranteed.
"#;
