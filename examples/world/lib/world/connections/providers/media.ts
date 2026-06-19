import { signedS3HeadBucketRequest, value } from "./request-utils";
import type { WorldConnectionProvider } from "./types";

export const mediaConnectionProviders = [
  {
    id: "aws-s3",
    categoryId: "storage",
    name: "AWS S3",
    packageId: "storage/aws-s3",
    requiredEnv: ["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY", "AWS_REGION", "S3_BUCKET"],
    optionalEnv: ["S3_PUBLIC_BASE_URL"],
    receiptSchema: "dx.forge.world.storage",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "HEAD",
      endpointLabel: "HEAD https://<bucket>.s3.<region>.amazonaws.com/",
      buildRequest: async (env) =>
        signedS3HeadBucketRequest({
          accessKey: value(env, "AWS_ACCESS_KEY_ID"),
          secretKey: value(env, "AWS_SECRET_ACCESS_KEY"),
          region: value(env, "AWS_REGION"),
          bucket: value(env, "S3_BUCKET"),
          endpointHost: `${value(env, "S3_BUCKET")}.s3.${value(env, "AWS_REGION")}.amazonaws.com`,
          canonicalUri: "/",
        }),
    },
    nextAction: "Import a redacted S3 HeadBucket receipt before enabling signed uploads or CDN cache claims.",
  },
  {
    id: "cloudflare-r2",
    categoryId: "storage",
    name: "Cloudflare R2",
    packageId: "storage/cloudflare-r2",
    requiredEnv: ["R2_ACCOUNT_ID", "R2_ACCESS_KEY_ID", "R2_SECRET_ACCESS_KEY", "R2_BUCKET"],
    optionalEnv: ["R2_PUBLIC_BASE_URL"],
    receiptSchema: "dx.forge.world.storage",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "HEAD",
      endpointLabel: "HEAD https://<account>.r2.cloudflarestorage.com/<bucket>",
      buildRequest: async (env) =>
        signedS3HeadBucketRequest({
          accessKey: value(env, "R2_ACCESS_KEY_ID"),
          secretKey: value(env, "R2_SECRET_ACCESS_KEY"),
          region: "auto",
          bucket: value(env, "R2_BUCKET"),
          endpointHost: `${value(env, "R2_ACCOUNT_ID")}.r2.cloudflarestorage.com`,
          canonicalUri: `/${encodeURIComponent(value(env, "R2_BUCKET"))}`,
        }),
    },
    nextAction: "Import a redacted R2 HeadBucket receipt before enabling remote object install or upload claims.",
  },
  {
    id: "vercel-blob",
    categoryId: "storage",
    name: "Vercel Blob",
    packageId: "storage/vercel-blob",
    requiredEnv: ["BLOB_READ_WRITE_TOKEN"],
    optionalEnv: ["VERCEL_BLOB_PUBLIC_BASE_URL"],
    receiptSchema: "dx.forge.world.storage",
    secretRedaction: "secret-values-never-included",
    readiness: {
      method: "GET",
      endpointLabel: "@vercel/blob list({ limit: 1 })",
      sdkOperation: "list",
    },
    nextAction: "Run the @vercel/blob list readiness operation in the app runtime and import a redacted receipt.",
  },
] satisfies readonly WorldConnectionProvider[];
