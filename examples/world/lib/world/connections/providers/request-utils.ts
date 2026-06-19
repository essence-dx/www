import type { WorldConnectionEnv, WorldReadinessRequest } from "./types";

const emptySha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

function envValue(env: WorldConnectionEnv, name: string): string {
  return env[name]?.trim() ?? "";
}

export function bearerGet(url: string, token: string, extraHeaders: Record<string, string> = {}): WorldReadinessRequest {
  return {
    input: url,
    init: {
      method: "GET",
      headers: {
        Accept: "application/json",
        Authorization: `Bearer ${token}`,
        ...extraHeaders,
      },
    },
    endpoint: url,
  };
}

export function headerGet(url: string, headers: Record<string, string>): WorldReadinessRequest {
  return {
    input: url,
    init: {
      method: "GET",
      headers: {
        Accept: "application/json",
        ...headers,
      },
    },
    endpoint: url,
  };
}

function toHex(bytes: Uint8Array): string {
  return [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function utf8(value: string): Uint8Array {
  return new TextEncoder().encode(value);
}

async function hmacSha256(key: Uint8Array, value: string): Promise<Uint8Array> {
  const cryptoKey = await crypto.subtle.importKey("raw", key, { name: "HMAC", hash: "SHA-256" }, false, [
    "sign",
  ]);
  return new Uint8Array(await crypto.subtle.sign("HMAC", cryptoKey, utf8(value)));
}

async function sha256Hex(value: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", utf8(value));
  return toHex(new Uint8Array(digest));
}

function amzDate(now = new Date()): { stamp: string; shortDate: string } {
  const iso = now.toISOString().replace(/[:-]|\.\d{3}/g, "");
  return { stamp: iso, shortDate: iso.slice(0, 8) };
}

async function signingKey(secret: string, shortDate: string, region: string, service: string): Promise<Uint8Array> {
  const dateKey = await hmacSha256(utf8(`AWS4${secret}`), shortDate);
  const regionKey = await hmacSha256(dateKey, region);
  const serviceKey = await hmacSha256(regionKey, service);
  return hmacSha256(serviceKey, "aws4_request");
}

export async function signedS3HeadBucketRequest(options: {
  accessKey: string;
  secretKey: string;
  region: string;
  bucket: string;
  endpointHost: string;
  canonicalUri: string;
}): Promise<WorldReadinessRequest> {
  const method = "HEAD";
  const service = "s3";
  const { stamp, shortDate } = amzDate();
  const signedHeaders = "host;x-amz-content-sha256;x-amz-date";
  const canonicalHeaders = [
    `host:${options.endpointHost}`,
    `x-amz-content-sha256:${emptySha256}`,
    `x-amz-date:${stamp}`,
    "",
  ].join("\n");
  const canonicalRequest = [
    method,
    options.canonicalUri,
    "",
    canonicalHeaders,
    signedHeaders,
    emptySha256,
  ].join("\n");
  const credentialScope = `${shortDate}/${options.region}/${service}/aws4_request`;
  const stringToSign = [
    "AWS4-HMAC-SHA256",
    stamp,
    credentialScope,
    await sha256Hex(canonicalRequest),
  ].join("\n");
  const signature = toHex(await hmacSha256(await signingKey(options.secretKey, shortDate, options.region, service), stringToSign));
  const endpoint = `https://${options.endpointHost}${options.canonicalUri}`;

  return {
    input: endpoint,
    init: {
      method,
      headers: {
        Authorization: `AWS4-HMAC-SHA256 Credential=${options.accessKey}/${credentialScope}, SignedHeaders=${signedHeaders}, Signature=${signature}`,
        "x-amz-content-sha256": emptySha256,
        "x-amz-date": stamp,
      },
    },
    endpoint,
  };
}

export function readRequired(env: WorldConnectionEnv, names: readonly string[]): readonly string[] {
  return names.filter((name) => !envValue(env, name));
}

export function readPresent(env: WorldConnectionEnv, names: readonly string[]): readonly string[] {
  return names.filter((name) => Boolean(envValue(env, name)));
}

export function value(env: WorldConnectionEnv, name: string): string {
  return envValue(env, name);
}
