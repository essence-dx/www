import { bearerGet, headerGet, value } from "./request-utils";
import type { WorldConnectionProvider } from "./types";

function withProtocol(host: string, fallbackProtocol = "https"): string {
  if (/^https?:\/\//.test(host)) {
    return host.replace(/\/+$/, "");
  }

  return `${fallbackProtocol}://${host}`.replace(/\/+$/, "");
}

export const searchVectorConnectionProviders = [
  {
    id: "algolia",
    categoryId: "search",
    name: "Algolia",
    packageId: "search/algolia",
    requiredEnv: ["ALGOLIA_APP_ID", "ALGOLIA_ADMIN_API_KEY"],
    optionalEnv: ["NEXT_PUBLIC_ALGOLIA_SEARCH_API_KEY", "ALGOLIA_INDEX_PREFIX"],
    receiptSchema: "dx.forge.world.search",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://<app-id>.algolia.net/1/indexes?entriesPerPage=1&page=0",
      buildRequest: async (env) =>
        headerGet(`https://${value(env, "ALGOLIA_APP_ID")}.algolia.net/1/indexes?entriesPerPage=1&page=0`, {
          "x-algolia-api-key": value(env, "ALGOLIA_ADMIN_API_KEY"),
          "x-algolia-application-id": value(env, "ALGOLIA_APP_ID"),
        }),
    },
    nextAction: "Import a redacted Algolia index-list receipt before enabling hosted search claims.",
  },
  {
    id: "meilisearch",
    categoryId: "search",
    name: "Meilisearch",
    packageId: "search/meilisearch",
    requiredEnv: ["MEILISEARCH_HOST", "MEILISEARCH_API_KEY"],
    optionalEnv: ["MEILISEARCH_INDEX_PREFIX"],
    receiptSchema: "dx.forge.world.search",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET <MEILISEARCH_HOST>/indexes?limit=1",
      buildRequest: async (env) =>
        bearerGet(`${withProtocol(value(env, "MEILISEARCH_HOST"))}/indexes?limit=1`, value(env, "MEILISEARCH_API_KEY")),
    },
    nextAction: "Import a redacted Meilisearch index-list receipt before enabling search write or query claims.",
  },
  {
    id: "typesense",
    categoryId: "search",
    name: "Typesense",
    packageId: "search/typesense",
    requiredEnv: ["TYPESENSE_HOST", "TYPESENSE_API_KEY"],
    optionalEnv: ["TYPESENSE_PROTOCOL", "TYPESENSE_PORT", "TYPESENSE_COLLECTION_PREFIX"],
    receiptSchema: "dx.forge.world.search",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET <TYPESENSE_HOST>/collections",
      buildRequest: async (env) => {
        const protocol = value(env, "TYPESENSE_PROTOCOL") || "https";
        const base = withProtocol(value(env, "TYPESENSE_HOST"), protocol);
        const port = value(env, "TYPESENSE_PORT");
        const endpoint = port && !new URL(base).port ? `${base}:${port}/collections` : `${base}/collections`;
        return headerGet(endpoint, { "X-TYPESENSE-API-KEY": value(env, "TYPESENSE_API_KEY") });
      },
    },
    nextAction: "Import a redacted Typesense collection-list receipt before enabling collection indexing claims.",
  },
  {
    id: "pinecone",
    categoryId: "vector-search",
    name: "Pinecone",
    packageId: "vector/pinecone",
    requiredEnv: ["PINECONE_API_KEY", "PINECONE_INDEX"],
    optionalEnv: ["PINECONE_REGION"],
    receiptSchema: "dx.forge.world.vector",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.pinecone.io/indexes/<index>",
      buildRequest: async (env) =>
        headerGet(`https://api.pinecone.io/indexes/${encodeURIComponent(value(env, "PINECONE_INDEX"))}`, {
          "Api-Key": value(env, "PINECONE_API_KEY"),
        }),
    },
    nextAction: "Import a redacted Pinecone describe-index receipt before enabling vector query or upsert claims.",
  },
] satisfies readonly WorldConnectionProvider[];
