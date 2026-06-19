import { bearerGet, headerGet, value } from "./request-utils";
import type { WorldConnectionEnv, WorldConnectionProvider, WorldReadinessRequest } from "./types";

function basicAuth(user: string, password: string): string {
  return `Basic ${btoa(`${user}:${password}`)}`;
}

function normalizedBaseUrl(rawValue: string): string {
  const withProtocol = /^https?:\/\//.test(rawValue) ? rawValue : `https://${rawValue}`;
  return withProtocol.replace(/\/+$/, "");
}

function strapiReadinessPath(env: WorldConnectionEnv): string {
  const configuredPath = value(env, "STRAPI_READINESS_PATH") || "/api";
  return configuredPath.startsWith("/") ? configuredPath : `/${configuredPath}`;
}

function sanityDatasetQuery(env: WorldConnectionEnv): WorldReadinessRequest {
  const projectId = encodeURIComponent(value(env, "SANITY_PROJECT_ID"));
  const dataset = encodeURIComponent(value(env, "SANITY_DATASET"));
  const query = encodeURIComponent("*[0]{_id}");
  const endpoint = `https://${projectId}.api.sanity.io/v2021-10-21/data/query/${dataset}?query=${query}`;
  const token = value(env, "SANITY_API_TOKEN");

  return headerGet(endpoint, token ? { Authorization: `Bearer ${token}` } : {});
}

function strapiContentReadiness(env: WorldConnectionEnv): WorldReadinessRequest {
  const endpoint = `${normalizedBaseUrl(value(env, "STRAPI_URL"))}${strapiReadinessPath(env)}`;

  return bearerGet(endpoint, value(env, "STRAPI_API_TOKEN"));
}

function contentfulContentTypes(env: WorldConnectionEnv): WorldReadinessRequest {
  const spaceId = encodeURIComponent(value(env, "CONTENTFUL_SPACE_ID"));
  const environment = encodeURIComponent(value(env, "CONTENTFUL_ENVIRONMENT"));
  const endpoint = `https://cdn.contentful.com/spaces/${spaceId}/environments/${environment}/content_types?limit=1`;

  return bearerGet(endpoint, value(env, "CONTENTFUL_DELIVERY_TOKEN"));
}

function twilioAccountReadiness(env: WorldConnectionEnv): WorldReadinessRequest {
  const accountSid = encodeURIComponent(value(env, "TWILIO_ACCOUNT_SID"));

  return headerGet(`https://api.twilio.com/2010-04-01/Accounts/${accountSid}.json`, {
    Authorization: basicAuth(value(env, "TWILIO_ACCOUNT_SID"), value(env, "TWILIO_AUTH_TOKEN")),
  });
}

export const contentDeployConnectionProviders = [
  {
    id: "vercel",
    categoryId: "deployment",
    name: "Vercel",
    packageId: "deployment/vercel",
    requiredEnv: ["VERCEL_TOKEN"],
    optionalEnv: ["VERCEL_PROJECT_ID", "VERCEL_TEAM_ID"],
    receiptSchema: "dx.forge.world.deploy",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.vercel.com/v2/user",
      buildRequest: async (env) => bearerGet("https://api.vercel.com/v2/user", value(env, "VERCEL_TOKEN")),
    },
    nextAction:
      "Import a redacted Vercel user receipt or local CLI identity proof before claiming provider deployment readiness.",
  },
  {
    id: "cloudflare",
    categoryId: "deployment",
    name: "Cloudflare",
    packageId: "deployment/cloudflare",
    requiredEnv: ["CLOUDFLARE_API_TOKEN"],
    optionalEnv: ["CLOUDFLARE_ACCOUNT_ID", "CLOUDFLARE_PROJECT_NAME"],
    receiptSchema: "dx.forge.world.deploy",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.cloudflare.com/client/v4/user/tokens/verify",
      buildRequest: async (env) =>
        bearerGet("https://api.cloudflare.com/client/v4/user/tokens/verify", value(env, "CLOUDFLARE_API_TOKEN")),
    },
    nextAction: "Import a redacted Cloudflare token verification receipt before enabling Pages or Workers deploy claims.",
  },
  {
    id: "fly-io",
    categoryId: "deployment",
    name: "Fly.io",
    packageId: "deployment/fly-io",
    requiredEnv: ["FLY_API_TOKEN", "FLY_APP_NAME"],
    optionalEnv: [],
    receiptSchema: "dx.forge.world.deploy",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.machines.dev/v1/apps/<app>",
      buildRequest: async (env) =>
        bearerGet(
          `https://api.machines.dev/v1/apps/${encodeURIComponent(value(env, "FLY_APP_NAME"))}`,
          value(env, "FLY_API_TOKEN"),
        ),
    },
    nextAction: "Import a redacted Fly app metadata receipt before claiming portable server deployment readiness.",
  },
  {
    id: "netlify",
    categoryId: "deployment",
    name: "Netlify",
    packageId: "deployment/netlify",
    requiredEnv: ["NETLIFY_AUTH_TOKEN"],
    optionalEnv: ["NETLIFY_SITE_ID"],
    receiptSchema: "dx.forge.world.deploy",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.netlify.com/api/v1/user",
      buildRequest: async (env) => bearerGet("https://api.netlify.com/api/v1/user", value(env, "NETLIFY_AUTH_TOKEN")),
    },
    nextAction: "Import a redacted Netlify user receipt before claiming hosted deploy readiness.",
  },
  {
    id: "sanity",
    categoryId: "content",
    name: "Sanity",
    packageId: "content/sanity",
    requiredEnv: ["SANITY_PROJECT_ID", "SANITY_DATASET"],
    optionalEnv: ["SANITY_API_TOKEN"],
    receiptSchema: "dx.forge.world.schema",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://<project>.api.sanity.io/v2021-10-21/data/query/<dataset>",
      buildRequest: async (env) => sanityDatasetQuery(env),
    },
    nextAction: "Import a redacted Sanity dataset query receipt before enabling draft preview or webhook claims.",
  },
  {
    id: "strapi",
    categoryId: "content",
    name: "Strapi",
    packageId: "content/strapi",
    requiredEnv: ["STRAPI_URL", "STRAPI_API_TOKEN"],
    optionalEnv: ["STRAPI_READINESS_PATH", "STRAPI_WEBHOOK_SECRET"],
    receiptSchema: "dx.forge.world.schema",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET <STRAPI_URL>/api",
      buildRequest: async (env) => strapiContentReadiness(env),
    },
    nextAction: "Import a redacted Strapi content API receipt before enabling CMS sync or webhook claims.",
  },
  {
    id: "contentful",
    categoryId: "content",
    name: "Contentful",
    packageId: "content/contentful",
    requiredEnv: ["CONTENTFUL_SPACE_ID", "CONTENTFUL_ENVIRONMENT", "CONTENTFUL_DELIVERY_TOKEN"],
    optionalEnv: ["CONTENTFUL_PREVIEW_TOKEN", "CONTENTFUL_WEBHOOK_SECRET"],
    receiptSchema: "dx.forge.world.schema",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://cdn.contentful.com/spaces/<space>/environments/<environment>/content_types?limit=1",
      buildRequest: async (env) => contentfulContentTypes(env),
    },
    nextAction: "Import a redacted Contentful content-type receipt before enabling preview or revalidation claims.",
  },
  {
    id: "resend",
    categoryId: "notifications",
    name: "Resend",
    packageId: "notifications/resend",
    requiredEnv: ["RESEND_API_KEY"],
    optionalEnv: ["RESEND_FROM_EMAIL", "RESEND_WEBHOOK_SECRET"],
    receiptSchema: "dx.forge.world.webhook",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.resend.com/domains",
      buildRequest: async (env) => bearerGet("https://api.resend.com/domains", value(env, "RESEND_API_KEY")),
    },
    nextAction: "Import a redacted Resend domain receipt before enabling transactional email send claims.",
  },
  {
    id: "twilio",
    categoryId: "notifications",
    name: "Twilio",
    packageId: "notifications/twilio",
    requiredEnv: ["TWILIO_ACCOUNT_SID", "TWILIO_AUTH_TOKEN"],
    optionalEnv: ["TWILIO_MESSAGING_SERVICE_SID", "TWILIO_FROM_NUMBER", "TWILIO_WEBHOOK_SECRET"],
    receiptSchema: "dx.forge.world.webhook",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "GET https://api.twilio.com/2010-04-01/Accounts/<sid>.json",
      buildRequest: async (env) => twilioAccountReadiness(env),
    },
    nextAction: "Import a redacted Twilio account receipt before enabling SMS send or webhook claims.",
  },
  {
    id: "firebase-cloud-messaging",
    categoryId: "notifications",
    name: "Firebase Cloud Messaging",
    packageId: "notifications/firebase-cloud-messaging",
    requiredEnv: ["FCM_PROJECT_ID", "FCM_SERVICE_ACCOUNT_JSON"],
    optionalEnv: ["NEXT_PUBLIC_FCM_SENDER_ID", "FCM_WEB_PUSH_CERTIFICATE_KEY"],
    receiptSchema: "dx.forge.world.preview-only",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "env:FCM_PROJECT_ID",
    },
    nextAction:
      "FCM has no safe read-only delivery endpoint; require an operator-approved test token before live send proof.",
  },
] satisfies readonly WorldConnectionProvider[];
