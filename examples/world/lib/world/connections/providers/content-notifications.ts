import type { WorldConnectionProbe } from "../contracts";
import { configuredReadinessProbe, httpProbe } from "./shared";

export const contentNotificationConnectionProbes: readonly WorldConnectionProbe[] = [
  httpProbe(
    {
      id: "cloudflare-token-verify",
      providerId: "cloudflare",
      packageId: "deployment/cloudflare",
      name: "Cloudflare token",
      category: "Deployment",
      kind: "http",
      endpoint: "https://api.cloudflare.com/client/v4/user/tokens/verify",
      documentationUrl: "https://developers.cloudflare.com/api/resources/user/",
      requiredEnv: ["CLOUDFLARE_API_TOKEN"],
      optionalEnv: ["CLOUDFLARE_ACCOUNT_ID"],
    },
    (env) => ({
      endpoint: "https://api.cloudflare.com/client/v4/user/tokens/verify",
      headers: {
        Authorization: `Bearer ${env.CLOUDFLARE_API_TOKEN ?? ""}`,
      },
      expectedStatuses: [200],
      evidence: "token-verified",
    }),
  ),
  httpProbe(
    {
      id: "fly-app-readiness",
      providerId: "fly-io",
      packageId: "deployment/fly-io",
      name: "Fly.io app",
      category: "Deployment",
      kind: "http",
      endpoint: "https://api.fly.io/graphql",
      documentationUrl: "https://fly.io/docs/flyctl/auth-token/",
      requiredEnv: ["FLY_API_TOKEN", "FLY_APP_NAME"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.fly.io/graphql",
      method: "POST",
      headers: {
        Authorization: `Bearer ${env.FLY_API_TOKEN ?? ""}`,
        "Content-Type": "application/json",
      },
      body: {
        query: "query DxWorldFlyApp($name: String!) { app(name: $name) { id name status } }",
        variables: { name: env.FLY_APP_NAME ?? "" },
      },
      expectedStatuses: [200],
      evidence: "fly-app-metadata-query",
    }),
  ),
  httpProbe(
    {
      id: "netlify-user-readiness",
      providerId: "netlify",
      packageId: "deployment/netlify",
      name: "Netlify user",
      category: "Deployment",
      kind: "http",
      endpoint: "https://api.netlify.com/api/v1/user",
      documentationUrl: "https://docs.netlify.com/api/get-started/",
      requiredEnv: ["NETLIFY_AUTH_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.netlify.com/api/v1/user",
      headers: {
        Authorization: `Bearer ${env.NETLIFY_AUTH_TOKEN ?? ""}`,
      },
      expectedStatuses: [200],
      evidence: "netlify-user-readable",
    }),
  ),
  httpProbe(
    {
      id: "sanity-dataset-query",
      providerId: "sanity",
      packageId: "content/sanity",
      name: "Sanity dataset query",
      category: "Content",
      kind: "http",
      endpoint: "env:SANITY_PROJECT_ID/data/query",
      documentationUrl: "https://www.sanity.io/docs/http-api",
      requiredEnv: ["SANITY_PROJECT_ID", "SANITY_DATASET"],
      optionalEnv: ["SANITY_API_TOKEN"],
    },
    (env) => {
      const headers: Record<string, string> = {};

      if (env.SANITY_API_TOKEN) {
        headers.Authorization = `Bearer ${env.SANITY_API_TOKEN}`;
      }

      return {
        endpoint: `https://${env.SANITY_PROJECT_ID ?? ""}.api.sanity.io/v2021-06-07/data/query/${encodeURIComponent(
          env.SANITY_DATASET ?? "",
        )}?query=*%5B0...1%5D%7B_id%7D`,
        headers,
        expectedStatuses: [200],
        evidence: "sanity-dataset-readable",
      };
    },
  ),
  httpProbe(
    {
      id: "strapi-content-readiness",
      providerId: "strapi",
      packageId: "content/strapi",
      name: "Strapi content",
      category: "Content",
      kind: "http",
      endpoint: "env:STRAPI_URL/api",
      documentationUrl: "https://docs.strapi.io/cms/api/rest",
      requiredEnv: ["STRAPI_URL", "STRAPI_API_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: `${(env.STRAPI_URL ?? "").replace(/\/+$/, "")}/api`,
      headers: {
        Authorization: `Bearer ${env.STRAPI_API_TOKEN ?? ""}`,
      },
      expectedStatuses: [200],
      evidence: "strapi-api-readable",
    }),
  ),
  httpProbe(
    {
      id: "contentful-content-types",
      providerId: "contentful",
      packageId: "content/contentful",
      name: "Contentful content types",
      category: "Content",
      kind: "http",
      endpoint: "env:CONTENTFUL_SPACE_ID/content_types",
      documentationUrl: "https://www.contentful.com/developers/docs/references/content-delivery-api/",
      requiredEnv: ["CONTENTFUL_SPACE_ID", "CONTENTFUL_ENVIRONMENT", "CONTENTFUL_DELIVERY_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: `https://cdn.contentful.com/spaces/${encodeURIComponent(
        env.CONTENTFUL_SPACE_ID ?? "",
      )}/environments/${encodeURIComponent(env.CONTENTFUL_ENVIRONMENT ?? "")}/content_types?limit=1`,
      headers: {
        Authorization: `Bearer ${env.CONTENTFUL_DELIVERY_TOKEN ?? ""}`,
      },
      expectedStatuses: [200],
      evidence: "contentful-content-types-readable",
    }),
  ),
  httpProbe(
    {
      id: "resend-domains",
      providerId: "resend",
      packageId: "notifications/resend",
      name: "Resend domains",
      category: "Notifications",
      kind: "http",
      endpoint: "https://api.resend.com/domains",
      documentationUrl: "https://resend.com/docs/api-reference/domains/list-domains",
      requiredEnv: ["RESEND_API_KEY"],
      optionalEnv: ["RESEND_FROM_EMAIL"],
    },
    (env) => ({
      endpoint: "https://api.resend.com/domains",
      headers: {
        Authorization: `Bearer ${env.RESEND_API_KEY ?? ""}`,
        "User-Agent": "dx-www-world-example",
      },
      expectedStatuses: [200],
      evidence: "domains-readable",
    }),
  ),
  httpProbe(
    {
      id: "twilio-account-readiness",
      providerId: "twilio",
      packageId: "notifications/twilio",
      name: "Twilio account",
      category: "Notifications",
      kind: "http",
      endpoint: "env:TWILIO_ACCOUNT_SID",
      documentationUrl: "https://www.twilio.com/docs/usage/api",
      requiredEnv: ["TWILIO_ACCOUNT_SID", "TWILIO_AUTH_TOKEN"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: `https://api.twilio.com/2010-04-01/Accounts/${encodeURIComponent(
        env.TWILIO_ACCOUNT_SID ?? "",
      )}.json`,
      headers: {
        Authorization: `Basic ${btoa(`${env.TWILIO_ACCOUNT_SID ?? ""}:${env.TWILIO_AUTH_TOKEN ?? ""}`)}`,
      },
      expectedStatuses: [200],
      evidence: "twilio-account-readable",
    }),
  ),
  configuredReadinessProbe(
    {
      id: "fcm-local-readiness",
      providerId: "firebase-cloud-messaging",
      packageId: "notifications/firebase-cloud-messaging",
      name: "Firebase Cloud Messaging",
      category: "Notifications",
      kind: "env",
      endpoint: "env:FCM_PROJECT_ID",
      documentationUrl: "https://firebase.google.com/docs/cloud-messaging",
      requiredEnv: ["FCM_PROJECT_ID", "FCM_SERVICE_ACCOUNT_JSON"],
      optionalEnv: [],
    },
    "FCM env is present. Live push validation requires an operator-approved test token, so the template stays in configured readiness.",
  ),
];
