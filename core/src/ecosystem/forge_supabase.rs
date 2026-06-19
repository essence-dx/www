pub(crate) fn supabase_client_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/supabase/env.ts",
            r#"export type DxSupabaseEnv = Record<string, string | undefined>;

export type DxSupabasePublicConfig = {
  url: string;
  publishableKey: string;
  isLocal: boolean;
};

export function defaultSupabaseEnv(): DxSupabaseEnv {
  return (globalThis as unknown as { process?: { env?: DxSupabaseEnv } }).process?.env ?? {};
}

export function readSupabasePublicConfig(
  env: DxSupabaseEnv = defaultSupabaseEnv(),
): DxSupabasePublicConfig {
  const url = assertSupabasePublicUrl(requiredEnv(env, "NEXT_PUBLIC_SUPABASE_URL"));
  const publishableKey = assertSupabasePublishableKey(
    requiredEnv(env, "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"),
  );

  return {
    url,
    publishableKey,
    isLocal: isLocalSupabaseUrl(url),
  };
}

export function isLocalSupabaseUrl(value: string): boolean {
  try {
    const url = new URL(value);
    return url.hostname === "localhost" || url.hostname === "127.0.0.1";
  } catch {
    return false;
  }
}

export function requiredEnv(env: DxSupabaseEnv, key: string): string {
  const value = env[key]?.trim();
  if (!value) {
    throw new Error(`Missing required Supabase env var: ${key}`);
  }
  return value;
}

export function assertSupabasePublicUrl(value: string): string {
  let url: URL;
  try {
    url = new URL(value);
  } catch {
    throw new Error("NEXT_PUBLIC_SUPABASE_URL must be a valid URL.");
  }

  if (!["http:", "https:"].includes(url.protocol)) {
    throw new Error("NEXT_PUBLIC_SUPABASE_URL must use http or https.");
  }

  return value;
}

export function assertSupabasePublishableKey(value: string): string {
  if (/service[_-]?role|secret/i.test(value)) {
    throw new Error(
      "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY must be a publishable key, not a service-role secret.",
    );
  }

  return value;
}
"#,
        ),
        (
            "js/supabase/browser.ts",
            r#""use client";

import { createBrowserClient } from "@supabase/ssr";
import type { SupabaseClient } from "@supabase/supabase-js";

import {
  readSupabasePublicConfig,
  type DxSupabaseEnv,
} from "./env";

export type DxSupabaseBrowserOptions = NonNullable<
  Parameters<typeof createBrowserClient>[2]
>;

export function createDxSupabaseBrowserClient<Database = never>(
  env?: DxSupabaseEnv,
  options?: DxSupabaseBrowserOptions,
): SupabaseClient<Database> {
  const config = readSupabasePublicConfig(env);

  return createBrowserClient<Database>(
    config.url,
    config.publishableKey,
    options,
  ) as SupabaseClient<Database>;
}
"#,
        ),
        (
            "js/supabase/avatar-storage.ts",
            r#""use client";

import { createDxSupabaseBrowserClient } from "./browser";

export const DX_SUPABASE_AVATAR_BUCKET = "avatars";

export type DxSupabaseAvatarUploadOptions = {
  userId: string;
  file: File;
  bucket?: string;
  cacheControl?: string;
  upsert?: boolean;
  random?: () => number;
};

export type DxSupabaseAvatarStorageOptions = {
  bucket?: string;
};

export type DxSupabaseAvatarUploadResult = {
  path: string;
  fullPath: string | null;
  publicUrl: string;
};

export async function uploadDxSupabaseAvatar(
  options: DxSupabaseAvatarUploadOptions,
): Promise<DxSupabaseAvatarUploadResult> {
  const file = assertDxSupabaseAvatarFile(options.file);
  const bucket = assertDxSupabaseAvatarBucket(
    options.bucket ?? DX_SUPABASE_AVATAR_BUCKET,
  );
  const path = createDxSupabaseAvatarPath(
    options.userId,
    file.name,
    options.random,
  );
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .upload(path, file, {
      cacheControl: options.cacheControl ?? "3600",
      upsert: options.upsert ?? false,
    });

  if (error) {
    throw error;
  }

  const storedPath = data?.path ?? path;
  return {
    path: storedPath,
    fullPath:
      data && "fullPath" in data && typeof data.fullPath === "string"
        ? data.fullPath
        : null,
    publicUrl: getDxSupabaseAvatarPublicUrl(storedPath, { bucket }),
  };
}

export async function downloadDxSupabaseAvatarUrl(
  path: string,
  options: DxSupabaseAvatarStorageOptions = {},
): Promise<string> {
  const bucket = assertDxSupabaseAvatarBucket(
    options.bucket ?? DX_SUPABASE_AVATAR_BUCKET,
  );
  const safePath = assertDxSupabaseAvatarPath(path);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .download(safePath);

  if (error) {
    throw error;
  }

  return URL.createObjectURL(data);
}

export function getDxSupabaseAvatarPublicUrl(
  path: string,
  options: DxSupabaseAvatarStorageOptions = {},
): string {
  const bucket = assertDxSupabaseAvatarBucket(
    options.bucket ?? DX_SUPABASE_AVATAR_BUCKET,
  );
  const safePath = assertDxSupabaseAvatarPath(path);
  const supabase = createDxSupabaseBrowserClient();
  const { data } = supabase.storage.from(bucket).getPublicUrl(safePath);
  return data.publicUrl;
}

export function revokeDxSupabaseAvatarUrl(url: string) {
  if (url.startsWith("blob:")) {
    URL.revokeObjectURL(url);
  }
}

export function createDxSupabaseAvatarPath(
  userId: string,
  fileName: string,
  random: () => number = Math.random,
): string {
  const safeUserId = assertDxSupabaseAvatarUserId(userId);
  const extension = readDxSupabaseAvatarExtension(fileName);
  const token = Math.floor(random() * 1_000_000_000)
    .toString(36)
    .padStart(6, "0");

  return assertDxSupabaseAvatarPath(
    `${safeUserId}/${Date.now().toString(36)}-${token}.${extension}`,
  );
}

function assertDxSupabaseAvatarFile(file: File): File {
  if (!file) {
    throw new Error("Supabase avatar file is required.");
  }
  if (file.size <= 0) {
    throw new Error("Supabase avatar file must not be empty.");
  }
  if (file.type && !file.type.startsWith("image/")) {
    throw new Error("Supabase avatar file must be an image.");
  }
  return file;
}

function assertDxSupabaseAvatarBucket(bucket: string): string {
  const value = bucket.trim();
  if (!/^[A-Za-z0-9._-]+$/.test(value)) {
    throw new Error("Supabase avatar bucket must be a simple bucket id.");
  }
  return value;
}

function assertDxSupabaseAvatarUserId(userId: string): string {
  const value = userId.trim();
  if (!/^[A-Za-z0-9_-]+$/.test(value)) {
    throw new Error("Supabase avatar userId must be a simple path segment.");
  }
  return value;
}

function assertDxSupabaseAvatarPath(path: string): string {
  const value = path.trim();
  if (
    !value ||
    value.startsWith("/") ||
    path.includes("\\") ||
    path.includes("..") ||
    path.includes("://")
  ) {
    throw new Error("Supabase avatar path must be a relative storage path.");
  }
  return value;
}

function readDxSupabaseAvatarExtension(fileName: string): string {
  const extension = fileName.split(".").pop()?.toLowerCase() ?? "";
  return /^[a-z0-9]+$/.test(extension) ? extension : "png";
}
"#,
        ),
        (
            "js/supabase/signed-storage.ts",
            r#""use client";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseStorageTransformOptions = {
  width?: number;
  height?: number;
  resize?: "cover" | "contain" | "fill";
  format?: "origin" | "webp";
  quality?: number;
};

export type DxSupabaseSignedDownloadOptions = {
  bucket: string;
  path: string;
  expiresIn?: number;
  download?: boolean | string;
  transform?: DxSupabaseStorageTransformOptions;
};

export type DxSupabaseSignedDownloadBatchOptions = {
  bucket: string;
  paths: string[];
  expiresIn?: number;
  download?: boolean | string;
};

export type DxSupabaseSignedUploadOptions = {
  bucket: string;
  path: string;
  upsert?: boolean;
};

export type DxSupabaseSignedUploadFileOptions = {
  bucket: string;
  path: string;
  token: string;
  file: Blob | ArrayBuffer | File;
  cacheControl?: string;
  contentType?: string;
};

export type DxSupabaseSignedDownloadResult = {
  signedUrl: string;
  path: string;
  expiresIn: number;
};

export type DxSupabaseSignedDownloadBatchItem = {
  path: string;
  signedUrl: string | null;
  error: string | null;
  expiresIn: number;
};

export type DxSupabaseSignedUploadResult = {
  path: string;
  token: string;
  signedUrl: string;
};

type DxSupabaseSignedUrlResponse = {
  path?: string | null;
  signedUrl?: string | null;
  error?: string | { message?: string | null } | null;
};

export async function createDxSupabaseSignedDownloadUrl(
  options: DxSupabaseSignedDownloadOptions,
): Promise<DxSupabaseSignedDownloadResult> {
  const bucket = assertDxSupabaseSignedStorageBucket(options.bucket);
  const path = assertDxSupabaseSignedStoragePath(options.path);
  const expiresIn = assertDxSupabaseSignedStorageExpiresIn(
    options.expiresIn ?? 60 * 60,
  );
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .createSignedUrl(path, expiresIn, {
      download: options.download,
      transform: options.transform,
    });

  if (error) {
    throw error;
  }

  return {
    signedUrl: data.signedUrl,
    path,
    expiresIn,
  };
}

export async function createDxSupabaseSignedDownloadUrls(
  options: DxSupabaseSignedDownloadBatchOptions,
): Promise<DxSupabaseSignedDownloadBatchItem[]> {
  const bucket = assertDxSupabaseSignedStorageBucket(options.bucket);
  const paths = assertDxSupabaseSignedStoragePaths(options.paths);
  const expiresIn = assertDxSupabaseSignedStorageExpiresIn(
    options.expiresIn ?? 60 * 60,
  );
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .createSignedUrls(paths, expiresIn, {
      download: options.download,
    });

  if (error) {
    throw error;
  }

  return (data ?? []).map((item, index) =>
    toDxSupabaseSignedDownloadBatchItem(
      item as DxSupabaseSignedUrlResponse,
      paths[index] ?? "",
      expiresIn,
    ),
  );
}

export async function createDxSupabaseSignedUploadUrl(
  options: DxSupabaseSignedUploadOptions,
): Promise<DxSupabaseSignedUploadResult> {
  const bucket = assertDxSupabaseSignedStorageBucket(options.bucket);
  const path = assertDxSupabaseSignedStoragePath(options.path);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .createSignedUploadUrl(path, {
      upsert: options.upsert,
    });

  if (error) {
    throw error;
  }

  return {
    path: data.path,
    token: data.token,
    signedUrl: data.signedUrl,
  };
}

export async function uploadDxSupabaseToSignedUrl(
  options: DxSupabaseSignedUploadFileOptions,
): Promise<string> {
  const bucket = assertDxSupabaseSignedStorageBucket(options.bucket);
  const path = assertDxSupabaseSignedStoragePath(options.path);
  const token = assertDxSupabaseSignedStorageToken(options.token);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .uploadToSignedUrl(path, token, options.file, {
      cacheControl: options.cacheControl,
      contentType: options.contentType,
    });

  if (error) {
    throw error;
  }

  return data.path;
}

function assertDxSupabaseSignedStorageBucket(bucket: string): string {
  const value = bucket.trim();
  if (!/^[A-Za-z0-9._-]+$/.test(value)) {
    throw new Error("Supabase signed storage bucket must be a simple bucket id.");
  }
  return value;
}

function assertDxSupabaseSignedStoragePath(path: string): string {
  const value = path.trim();
  if (
    !value ||
    value.startsWith("/") ||
    path.includes("\\") ||
    path.includes("..") ||
    path.includes("://")
  ) {
    throw new Error("Supabase signed storage path must be a relative object path.");
  }
  return value;
}

function assertDxSupabaseSignedStoragePaths(paths: string[]): string[] {
  if (!Array.isArray(paths) || paths.length === 0) {
    throw new Error("Supabase signed storage paths must include at least one object path.");
  }
  return paths.map(assertDxSupabaseSignedStoragePath);
}

function assertDxSupabaseSignedStorageExpiresIn(expiresIn: number): number {
  if (!Number.isSafeInteger(expiresIn) || expiresIn <= 0) {
    throw new Error("Supabase signed storage expiresIn must be a positive integer.");
  }
  return expiresIn;
}

function assertDxSupabaseSignedStorageToken(token: string): string {
  const value = token.trim();
  if (!value) {
    throw new Error("Supabase signed upload token is required.");
  }
  return value;
}

function toDxSupabaseSignedDownloadBatchItem(
  item: DxSupabaseSignedUrlResponse,
  fallbackPath: string,
  expiresIn: number,
): DxSupabaseSignedDownloadBatchItem {
  return {
    path: item.path ?? fallbackPath,
    signedUrl: item.signedUrl ?? null,
    error: readDxSupabaseSignedStorageItemError(item.error),
    expiresIn,
  };
}

function readDxSupabaseSignedStorageItemError(
  error: DxSupabaseSignedUrlResponse["error"],
): string | null {
  if (!error) {
    return null;
  }

  if (typeof error === "string") {
    return error;
  }

  if (typeof error.message === "string" && error.message) {
    return error.message;
  }

  return "Supabase Storage could not sign this object.";
}
"#,
        ),
        (
            "js/supabase/storage-objects.ts",
            r#""use client";

import type { DxSupabaseStorageTransformOptions } from "./signed-storage";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseStorageObjectMetadata = Record<string, unknown> | null;

export type DxSupabaseStorageObject = {
  name: string;
  id?: string | null;
  updated_at?: string | null;
  created_at?: string | null;
  last_accessed_at?: string | null;
  metadata?: DxSupabaseStorageObjectMetadata;
};

export type DxSupabaseStorageSortOptions = {
  column: "name" | "updated_at" | "created_at" | "last_accessed_at";
  order?: "asc" | "desc";
};

export type DxSupabaseStorageListOptions = {
  bucket: string;
  prefix?: string;
  limit?: number;
  offset?: number;
  sortBy?: DxSupabaseStorageSortOptions;
  search?: string;
};

export type DxSupabaseStorageDownloadOptions = {
  bucket: string;
  path: string;
  download?: boolean | string;
  transform?: DxSupabaseStorageTransformOptions;
};

export type DxSupabaseStorageRemoveOptions = {
  bucket: string;
  paths: string[];
};

export type DxSupabaseStorageTransferOptions = {
  bucket: string;
  fromPath: string;
  toPath: string;
  destinationBucket?: string;
  upsert?: boolean;
};

export type DxSupabaseStorageTransferResult = {
  path: string | null;
  data: unknown;
};

export async function listDxSupabaseStorageObjects(
  options: DxSupabaseStorageListOptions,
): Promise<DxSupabaseStorageObject[]> {
  const bucket = assertDxSupabaseStorageBucket(options.bucket);
  const prefix = assertDxSupabaseStoragePrefix(options.prefix ?? "");
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage.from(bucket).list(prefix, {
    limit: assertDxSupabaseStorageListLimit(options.limit ?? 100),
    offset: assertDxSupabaseStorageOffset(options.offset ?? 0),
    sortBy: options.sortBy,
    search: options.search,
  });

  if (error) {
    throw error;
  }

  return (data ?? []) as DxSupabaseStorageObject[];
}

export async function downloadDxSupabaseStorageObject(
  options: DxSupabaseStorageDownloadOptions,
): Promise<Blob> {
  const bucket = assertDxSupabaseStorageBucket(options.bucket);
  const path = assertDxSupabaseStoragePath(options.path);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage.from(bucket).download(path, {
    download: options.download,
    transform: options.transform,
  });

  if (error) {
    throw error;
  }

  return data;
}

export async function removeDxSupabaseStorageObjects(
  options: DxSupabaseStorageRemoveOptions,
): Promise<DxSupabaseStorageObject[]> {
  const bucket = assertDxSupabaseStorageBucket(options.bucket);
  const paths = assertDxSupabaseStoragePaths(options.paths);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage.from(bucket).remove(paths);

  if (error) {
    throw error;
  }

  return (data ?? []) as DxSupabaseStorageObject[];
}

export async function copyDxSupabaseStorageObject(
  options: DxSupabaseStorageTransferOptions,
): Promise<DxSupabaseStorageTransferResult> {
  const bucket = assertDxSupabaseStorageBucket(options.bucket);
  const fromPath = assertDxSupabaseStoragePath(options.fromPath);
  const toPath = assertDxSupabaseStoragePath(options.toPath);
  const destinationBucket = options.destinationBucket
    ? assertDxSupabaseStorageBucket(options.destinationBucket)
    : undefined;
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .copy(fromPath, toPath, {
      destinationBucket,
      upsert: options.upsert,
    });

  if (error) {
    throw error;
  }

  return toDxSupabaseStorageTransferResult(data);
}

export async function moveDxSupabaseStorageObject(
  options: DxSupabaseStorageTransferOptions,
): Promise<DxSupabaseStorageTransferResult> {
  const bucket = assertDxSupabaseStorageBucket(options.bucket);
  const fromPath = assertDxSupabaseStoragePath(options.fromPath);
  const toPath = assertDxSupabaseStoragePath(options.toPath);
  const destinationBucket = options.destinationBucket
    ? assertDxSupabaseStorageBucket(options.destinationBucket)
    : undefined;
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.storage
    .from(bucket)
    .move(fromPath, toPath, {
      destinationBucket,
    });

  if (error) {
    throw error;
  }

  return toDxSupabaseStorageTransferResult(data);
}

function toDxSupabaseStorageTransferResult(
  data: unknown,
): DxSupabaseStorageTransferResult {
  return {
    path: readDxSupabaseStorageResultPath(data),
    data,
  };
}

function readDxSupabaseStorageResultPath(data: unknown): string | null {
  if (
    data &&
    typeof data === "object" &&
    "path" in data &&
    typeof data.path === "string"
  ) {
    return data.path;
  }

  return null;
}

function assertDxSupabaseStorageBucket(bucket: string): string {
  const value = bucket.trim();
  if (!/^[A-Za-z0-9._-]+$/.test(value)) {
    throw new Error("Supabase storage bucket must be a simple bucket id.");
  }
  return value;
}

function assertDxSupabaseStoragePrefix(prefix: string): string {
  const value = prefix.trim();
  if (
    value.startsWith("/") ||
    prefix.includes("\\") ||
    prefix.includes("..") ||
    prefix.includes("://")
  ) {
    throw new Error("Supabase storage prefix must be a relative folder path.");
  }
  return value;
}

function assertDxSupabaseStoragePath(path: string): string {
  const value = path.trim();
  if (
    !value ||
    value.startsWith("/") ||
    path.includes("\\") ||
    path.includes("..") ||
    path.includes("://")
  ) {
    throw new Error("Supabase storage path must be a relative object path.");
  }
  return value;
}

function assertDxSupabaseStoragePaths(paths: string[]): string[] {
  if (!paths.length) {
    throw new Error("Supabase storage remove requires at least one object path.");
  }
  if (paths.length > 1000) {
    throw new Error("Supabase storage remove accepts at most 1000 object paths.");
  }
  return paths.map(assertDxSupabaseStoragePath);
}

function assertDxSupabaseStorageListLimit(limit: number): number {
  if (!Number.isSafeInteger(limit) || limit <= 0 || limit > 1000) {
    throw new Error("Supabase storage list limit must be between 1 and 1000.");
  }
  return limit;
}

function assertDxSupabaseStorageOffset(offset: number): number {
  if (!Number.isSafeInteger(offset) || offset < 0) {
    throw new Error("Supabase storage list offset must be zero or greater.");
  }
  return offset;
}
"#,
        ),
        (
            "js/supabase/database-rows.ts",
            r#""use client";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseRowValue =
  | string
  | number
  | boolean
  | null
  | readonly DxSupabaseRowValue[]
  | { readonly [key: string]: DxSupabaseRowValue };

export type DxSupabaseRowData = Record<string, DxSupabaseRowValue>;

export type DxSupabaseRowFilterOperator =
  | "eq"
  | "neq"
  | "gt"
  | "gte"
  | "lt"
  | "lte"
  | "like"
  | "ilike"
  | "is"
  | "in"
  | "contains"
  | "containedBy";

export type DxSupabaseRowFilter = {
  column: string;
  operator?: DxSupabaseRowFilterOperator;
  value: DxSupabaseRowValue;
};

export type DxSupabaseRowOrFilter = {
  expression: string;
  referencedTable?: string;
};

export type DxSupabaseRowsResult<Row extends DxSupabaseRowData> = {
  rows: Row[];
  count: number | null;
};

export type DxSupabaseRowRange = {
  from: number;
  to: number;
};

export type DxSupabaseRowOrder = {
  column: string;
  ascending?: boolean;
  nullsFirst?: boolean;
};

export type DxSupabaseSelectRowsOptions = {
  table: string;
  columns?: string;
  filters?: readonly DxSupabaseRowFilter[];
  or?: readonly DxSupabaseRowOrFilter[];
  order?: DxSupabaseRowOrder;
  range?: DxSupabaseRowRange;
  limit?: number;
  count?: "exact" | "planned" | "estimated";
};

export type DxSupabaseSelectSingleRowOptions = Omit<
  DxSupabaseSelectRowsOptions,
  "count" | "range" | "limit"
> & {
  required?: boolean;
};

export type DxSupabaseInsertRowsOptions<Row extends DxSupabaseRowData> = {
  table: string;
  rows: Row | readonly Row[];
  columns?: string;
};

export type DxSupabaseUpsertRowsOptions<Row extends DxSupabaseRowData> =
  DxSupabaseInsertRowsOptions<Row> & {
    onConflict?: string;
    ignoreDuplicates?: boolean;
  };

export type DxSupabaseMutateRowsOptions<Row extends DxSupabaseRowData> = {
  table: string;
  values: Partial<Row>;
  filters: readonly DxSupabaseRowFilter[];
  columns?: string;
};

export type DxSupabaseDeleteRowsOptions = {
  table: string;
  filters: readonly DxSupabaseRowFilter[];
  columns?: string;
};

export async function selectDxSupabaseRows<Row extends DxSupabaseRowData>(
  options: DxSupabaseSelectRowsOptions,
): Promise<DxSupabaseRowsResult<Row>> {
  const table = assertDxSupabaseRowTable(options.table);
  const columns = assertDxSupabaseSelectColumns(options.columns ?? "*");
  const supabase = createDxSupabaseBrowserClient();
  let query = supabase.from(table).select(columns, {
    count: options.count,
  });

  query = applyDxSupabaseRowFilters(query, options.filters ?? []);
  query = applyDxSupabaseRowOrFilters(query, options.or ?? []);

  if (options.order) {
    query = query.order(assertDxSupabaseRowColumn(options.order.column), {
      ascending: options.order.ascending,
      nullsFirst: options.order.nullsFirst,
    });
  }

  if (typeof options.limit === "number") {
    query = query.limit(assertDxSupabaseRowLimit(options.limit));
  }

  if (options.range) {
    const range = assertDxSupabaseRowRange(options.range);
    query = query.range(range.from, range.to);
  }

  const { data, error, count } = await query;
  if (error) throw error;
  return { rows: (data ?? []) as Row[], count: count ?? null };
}

export async function selectSingleDxSupabaseRow<Row extends DxSupabaseRowData>(
  options: DxSupabaseSelectSingleRowOptions,
): Promise<Row | null> {
  const table = assertDxSupabaseRowTable(options.table);
  const columns = assertDxSupabaseSelectColumns(options.columns ?? "*");
  const supabase = createDxSupabaseBrowserClient();
  let query = supabase.from(table).select(columns);

  query = applyDxSupabaseRowFilters(query, options.filters ?? []);
  query = applyDxSupabaseRowOrFilters(query, options.or ?? []);

  if (options.order) {
    query = query.order(assertDxSupabaseRowColumn(options.order.column), {
      ascending: options.order.ascending,
      nullsFirst: options.order.nullsFirst,
    });
  }

  const { data, error } = await (options.required ? query.single() : query.maybeSingle());
  if (error) throw error;
  return (data ?? null) as Row | null;
}

export async function insertDxSupabaseRows<Row extends DxSupabaseRowData>(
  options: DxSupabaseInsertRowsOptions<Row>,
): Promise<DxSupabaseRowsResult<Row>> {
  const table = assertDxSupabaseRowTable(options.table);
  const columns = assertDxSupabaseSelectColumns(options.columns ?? "*");
  const rows = normalizeDxSupabaseRows(options.rows);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error, count } = await supabase.from(table).insert(rows).select(columns);
  if (error) throw error;
  return { rows: (data ?? []) as Row[], count: count ?? null };
}

export async function upsertDxSupabaseRows<Row extends DxSupabaseRowData>(
  options: DxSupabaseUpsertRowsOptions<Row>,
): Promise<DxSupabaseRowsResult<Row>> {
  const table = assertDxSupabaseRowTable(options.table);
  const columns = assertDxSupabaseSelectColumns(options.columns ?? "*");
  const rows = normalizeDxSupabaseRows(options.rows);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error, count } = await supabase
    .from(table).upsert(rows, {
      onConflict: options.onConflict,
      ignoreDuplicates: options.ignoreDuplicates,
    })
    .select(columns);

  if (error) throw error;
  return { rows: (data ?? []) as Row[], count: count ?? null };
}

export async function updateDxSupabaseRows<Row extends DxSupabaseRowData>(
  options: DxSupabaseMutateRowsOptions<Row>,
): Promise<DxSupabaseRowsResult<Row>> {
  const table = assertDxSupabaseRowTable(options.table);
  const columns = assertDxSupabaseSelectColumns(options.columns ?? "*");
  const values = assertDxSupabaseRowValues(options.values);
  const filters = assertDxSupabaseRequiredFilters(options.filters, "update");
  const supabase = createDxSupabaseBrowserClient();
  let query = supabase.from(table).update(values);

  query = applyDxSupabaseRowFilters(query, filters);

  const { data, error, count } = await query.select(columns);
  if (error) throw error;
  return { rows: (data ?? []) as Row[], count: count ?? null };
}

export async function deleteDxSupabaseRows<Row extends DxSupabaseRowData>(
  options: DxSupabaseDeleteRowsOptions,
): Promise<DxSupabaseRowsResult<Row>> {
  const table = assertDxSupabaseRowTable(options.table);
  const columns = assertDxSupabaseSelectColumns(options.columns ?? "*");
  const filters = assertDxSupabaseRequiredFilters(options.filters, "delete");
  const supabase = createDxSupabaseBrowserClient();
  let query = supabase.from(table).delete();

  query = applyDxSupabaseRowFilters(query, filters);

  const { data, error, count } = await query.select(columns);
  if (error) throw error;
  return { rows: (data ?? []) as Row[], count: count ?? null };
}

type DxSupabaseRowPostgrestQuery = {
  eq(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  neq(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  gt(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  gte(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  lt(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  lte(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  like(column: string, value: string): DxSupabaseRowPostgrestQuery;
  ilike(column: string, value: string): DxSupabaseRowPostgrestQuery;
  is(column: string, value: boolean | null): DxSupabaseRowPostgrestQuery;
  in(
    column: string,
    values: readonly DxSupabaseRowValue[],
  ): DxSupabaseRowPostgrestQuery;
  contains(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  containedBy(column: string, value: DxSupabaseRowValue): DxSupabaseRowPostgrestQuery;
  or(
    expression: string,
    options?: { referencedTable?: string },
  ): DxSupabaseRowPostgrestQuery;
};

function applyDxSupabaseRowFilters<Query>(
  query: Query,
  filters: readonly DxSupabaseRowFilter[],
): Query {
  let next = query as Query & DxSupabaseRowPostgrestQuery;
  for (const filter of filters) {
    next = applyDxSupabaseRowFilter(next, filter) as Query & DxSupabaseRowPostgrestQuery;
  }
  return next as Query;
}

function applyDxSupabaseRowFilter<Query extends DxSupabaseRowPostgrestQuery>(
  query: Query,
  filter: DxSupabaseRowFilter,
): Query {
  const column = assertDxSupabaseRowColumn(filter.column);

  switch (filter.operator ?? "eq") {
    case "eq":
      return query.eq(column, filter.value) as Query;
    case "neq":
      return query.neq(column, filter.value) as Query;
    case "gt":
      return query.gt(column, filter.value) as Query;
    case "gte":
      return query.gte(column, filter.value) as Query;
    case "lt":
      return query.lt(column, filter.value) as Query;
    case "lte":
      return query.lte(column, filter.value) as Query;
    case "like":
      return query.like(column, assertDxSupabasePatternFilterValue(filter.value, "like")) as Query;
    case "ilike":
      return query.ilike(column, assertDxSupabasePatternFilterValue(filter.value, "ilike")) as Query;
    case "is":
      return query.is(column, assertDxSupabaseIsFilterValue(filter.value)) as Query;
    case "in":
      return query.in(column, assertDxSupabaseRowArrayFilter(filter.value, "in")) as Query;
    case "contains":
      return query.contains(column, filter.value) as Query;
    case "containedBy":
      return query.containedBy(column, filter.value) as Query;
  }
}

function applyDxSupabaseRowOrFilters<Query>(
  query: Query,
  filters: readonly DxSupabaseRowOrFilter[],
): Query {
  let next = query as Query & DxSupabaseRowPostgrestQuery;
  for (const filter of filters) {
    next = next.or(assertDxSupabaseOrFilter(filter.expression), {
      referencedTable: filter.referencedTable
        ? assertDxSupabaseRowTable(filter.referencedTable)
        : undefined,
    }) as Query & DxSupabaseRowPostgrestQuery;
  }
  return next as Query;
}

function normalizeDxSupabaseRows<Row extends DxSupabaseRowData>(
  rows: Row | readonly Row[],
): Row[] {
  const normalized = Array.isArray(rows) ? [...rows] : [rows];
  if (!normalized.length) {
    throw new Error("Supabase row mutation requires at least one row.");
  }
  return normalized.map((row) => assertDxSupabaseRowObject(row));
}

function assertDxSupabaseRowValues<Row extends DxSupabaseRowData>(
  values: Partial<Row>,
): Partial<Row> {
  return assertDxSupabaseRowObject(values, "Supabase row update requires values.");
}

function assertDxSupabaseRowObject<Row extends Partial<DxSupabaseRowData>>(
  row: Row,
  message = "Supabase row mutation requires an object row.",
): Row {
  if (!row || typeof row !== "object" || Array.isArray(row)) {
    throw new Error(message);
  }
  if (!Object.keys(row).length) {
    throw new Error(message);
  }
  return row;
}

function assertDxSupabaseRequiredFilters(
  filters: readonly DxSupabaseRowFilter[],
  action: "update" | "delete",
): DxSupabaseRowFilter[] {
  if (!filters.length) {
    throw new Error(`Supabase ${action} requires at least one filter.`);
  }
  return filters.map((filter) => ({
    column: assertDxSupabaseRowColumn(filter.column),
    value: filter.value,
  }));
}

function assertDxSupabaseRowTable(table: string): string {
  const value = table.trim();
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(value)) {
    throw new Error("Supabase table name must be a simple identifier.");
  }
  return value;
}

function assertDxSupabaseRowColumn(column: string): string {
  const value = column.trim();
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(value)) {
    throw new Error("Supabase row column must be a simple identifier.");
  }
  return value;
}

function assertDxSupabaseSelectColumns(columns: string): string {
  const value = columns.trim();
  if (
    !value ||
    value.length > 500 ||
    value.includes(";") ||
    value.includes("--") ||
    value.includes("/*") ||
    value.includes("*/")
  ) {
    throw new Error("Supabase select columns must be a safe select expression.");
  }
  return value;
}

function assertDxSupabaseRowLimit(limit: number): number {
  if (!Number.isSafeInteger(limit) || limit < 1 || limit > 1_000) {
    throw new Error("Supabase row limit must be a safe integer between 1 and 1000.");
  }
  return limit;
}

function assertDxSupabasePatternFilterValue(
  value: DxSupabaseRowValue,
  operator: "like" | "ilike",
): string {
  if (typeof value !== "string" || !value.trim()) {
    throw new Error(`Supabase ${operator} filter requires a non-empty string.`);
  }
  return value;
}

function assertDxSupabaseIsFilterValue(value: DxSupabaseRowValue): boolean | null {
  if (value !== null && typeof value !== "boolean") {
    throw new Error("Supabase is filter requires a boolean or null value.");
  }
  return value;
}

function assertDxSupabaseRowArrayFilter(
  value: DxSupabaseRowValue,
  operator: "in",
): readonly DxSupabaseRowValue[] {
  if (!Array.isArray(value) || !value.length) {
    throw new Error(`Supabase ${operator} filter requires at least one value.`);
  }
  return value;
}

function assertDxSupabaseOrFilter(expression: string): string {
  const value = expression.trim();
  if (
    !value ||
    value.length > 500 ||
    value.includes(";") ||
    value.includes("--") ||
    value.includes("/*") ||
    value.includes("*/")
  ) {
    throw new Error("Supabase or filter must be a safe PostgREST expression.");
  }
  return value;
}

function assertDxSupabaseRowRange(range: DxSupabaseRowRange): DxSupabaseRowRange {
  if (
    !Number.isSafeInteger(range.from) ||
    !Number.isSafeInteger(range.to) ||
    range.from < 0 ||
    range.to < range.from
  ) {
    throw new Error("Supabase row range must be a valid inclusive range.");
  }
  return range;
}
"#,
        ),
        (
            "js/supabase/rpc.ts",
            r#""use client";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseRpcArgs = Record<string, unknown>;

export type DxSupabaseRpcOptions<
  Args extends DxSupabaseRpcArgs = DxSupabaseRpcArgs,
> = {
  name: string;
  args?: Args;
  get?: boolean;
  head?: boolean;
  count?: "exact" | "planned" | "estimated";
};

export type DxSupabaseRpcResult<TData> = {
  data: TData | null;
  count: number | null;
};

type DxSupabaseRpcDatabase = {
  public: {
    Tables: Record<string, never>;
    Views: Record<string, never>;
    Functions: Record<
      string,
      {
        Args: DxSupabaseRpcArgs;
        Returns: unknown;
      }
    >;
    Enums: Record<string, never>;
    CompositeTypes: Record<string, never>;
  };
};

export async function callDxSupabaseRpc<
  TData = unknown,
  Args extends DxSupabaseRpcArgs = DxSupabaseRpcArgs,
>(
  options: DxSupabaseRpcOptions<Args>,
): Promise<DxSupabaseRpcResult<TData>> {
  const supabase = createDxSupabaseBrowserClient<DxSupabaseRpcDatabase>();
  const { data, error, count } = await supabase.rpc(
    assertDxSupabaseRpcName(options.name),
    options.args ?? ({} as Args),
    {
      get: options.get,
      head: options.head,
      count: options.count,
    },
  );

  if (error) {
    throw error;
  }

  return {
    data: data as TData | null,
    count: count ?? null,
  };
}

function assertDxSupabaseRpcName(name: string): string {
  const value = name.trim();
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(value)) {
    throw new Error("Supabase RPC name must be a simple Postgres function identifier.");
  }
  return value;
}
"#,
        ),
        (
            "js/supabase/server.ts",
            r#"import { createServerClient } from "@supabase/ssr";
import type { SupabaseClient } from "@supabase/supabase-js";
import { cookies } from "next/headers";

import {
  readSupabasePublicConfig,
  type DxSupabaseEnv,
} from "./env";

export type DxSupabaseServerOptions = NonNullable<
  Parameters<typeof createServerClient>[2]
>;

export async function createDxSupabaseServerClient<Database = never>(
  env?: DxSupabaseEnv,
  options: Omit<DxSupabaseServerOptions, "cookies"> = {},
): Promise<SupabaseClient<Database>> {
  const config = readSupabasePublicConfig(env);
  const cookieStore = await cookies();

  return createServerClient<Database>(config.url, config.publishableKey, {
    ...options,
    cookies: {
      getAll() {
        return cookieStore.getAll();
      },
      setAll(cookiesToSet) {
        try {
          cookiesToSet.forEach(({ name, value, options }) => {
            cookieStore.set(name, value, options);
          });
        } catch {
          // Server Components cannot set cookies; route handlers and actions can.
        }
      },
    },
  }) as SupabaseClient<Database>;
}
"#,
        ),
        (
            "js/supabase/auth-guard.ts",
            r#"import { redirect } from "next/navigation";
import type { AuthError, SupabaseClient, User } from "@supabase/supabase-js";

import { createDxSupabaseServerClient } from "./server";

export const DX_SUPABASE_DEFAULT_LOGIN_PATH = "/login";

export type DxSupabaseAuthGuardOptions = {
  redirectTo?: string;
};

export type DxSupabaseServerUserResult<Database = never> =
  | {
      supabase: SupabaseClient<Database>;
      user: User;
      error: null;
      authenticated: true;
    }
  | {
      supabase: SupabaseClient<Database>;
      user: null;
      error: AuthError | null;
      authenticated: false;
    };

export type DxSupabaseAuthenticatedUser<Database = never> = Extract<
  DxSupabaseServerUserResult<Database>,
  { authenticated: true }
>;

export async function getDxSupabaseServerUser<Database = never>(): Promise<
  DxSupabaseServerUserResult<Database>
> {
  const supabase = await createDxSupabaseServerClient<Database>();
  const { data, error } = await supabase.auth.getUser();
  const user = data.user ?? null;

  if (error || !user) {
    return {
      supabase,
      user: null,
      error,
      authenticated: false,
    };
  }

  return {
    supabase,
    user,
    error: null,
    authenticated: true,
  };
}

export async function requireDxSupabaseServerUser<Database = never>(
  options: DxSupabaseAuthGuardOptions = {},
): Promise<DxSupabaseAuthenticatedUser<Database>> {
  const result = await getDxSupabaseServerUser<Database>();

  if (result.authenticated) {
    return result;
  }

  redirect(readDxSupabaseLoginRedirect(options));
}

export function readDxSupabaseLoginRedirect(
  options: DxSupabaseAuthGuardOptions = {},
): `/${string}` {
  return assertDxSupabaseLocalLoginPath(
    options.redirectTo ?? DX_SUPABASE_DEFAULT_LOGIN_PATH,
  );
}

function assertDxSupabaseLocalLoginPath(path: string): `/${string}` {
  const value = path.trim();

  if (
    !value.startsWith("/") ||
    value.startsWith("//") ||
    value.includes("://") ||
    value.includes("\\")
  ) {
    throw new Error("Supabase auth guard redirect must be an app-local path.");
  }

  return value as `/${string}`;
}
"#,
        ),
        (
            "js/supabase/proxy.ts",
            r#"import { createServerClient } from "@supabase/ssr";
import { NextResponse, type NextRequest } from "next/server";

import {
  readSupabasePublicConfig,
  type DxSupabaseEnv,
} from "./env";

export type DxSupabaseSessionProxyOptions = {
  env?: DxSupabaseEnv;
};

type DxSupabaseProxyHeaderMap = Record<string, string> | undefined;

export async function updateDxSupabaseSession(
  request: NextRequest,
  options: DxSupabaseSessionProxyOptions = {},
): Promise<NextResponse> {
  const config = readSupabasePublicConfig(options.env);
  let supabaseResponse = NextResponse.next({
    request,
  });

  const supabase = createServerClient(config.url, config.publishableKey, {
    cookies: {
      getAll() {
        return request.cookies.getAll();
      },
      setAll(cookiesToSet, headers?: DxSupabaseProxyHeaderMap) {
        cookiesToSet.forEach(({ name, value }) => request.cookies.set(name, value));
        supabaseResponse = NextResponse.next({
          request,
        });
        cookiesToSet.forEach(({ name, value, options }) =>
          supabaseResponse.cookies.set(name, value, options),
        );
        Object.entries(headers ?? {}).forEach(([key, value]) =>
          supabaseResponse.headers.set(key, value),
        );
      },
    },
  });

  await supabase.auth.getClaims();

  return supabaseResponse;
}

export const dxSupabaseSessionProxyConfig = {
  matcher: [
    "/((?!_next/static|_next/image|favicon.ico|.*\\.(?:svg|png|jpg|jpeg|gif|webp)$).*)",
  ],
} as const;
"#,
        ),
        (
            "js/supabase/auth-actions.ts",
            r#""use server";

import { revalidatePath } from "next/cache";
import { redirect } from "next/navigation";

import { createDxSupabaseServerClient } from "./server";

export type DxSupabaseAuthActionOptions = {
  successPath?: string;
  failurePath?: string;
  revalidate?: string;
};

const DEFAULT_AUTH_OPTIONS: Required<DxSupabaseAuthActionOptions> = {
  successPath: "/account",
  failurePath: "/login",
  revalidate: "/",
};

export async function dxSupabaseSignInWithPassword(
  formData: FormData,
  options: DxSupabaseAuthActionOptions = {},
) {
  const merged = { ...DEFAULT_AUTH_OPTIONS, ...options };
  const paths = readSupabaseAuthActionPaths(merged);
  const supabase = await createDxSupabaseServerClient();
  const { error } = await supabase.auth.signInWithPassword(readEmailPassword(formData));

  if (error) {
    redirect(withMessage(paths.failurePath, error.message));
  }

  revalidatePath(paths.revalidate, "layout");
  redirect(paths.successPath);
}

export async function dxSupabaseSignUpWithPassword(
  formData: FormData,
  options: DxSupabaseAuthActionOptions = {},
) {
  const merged = { ...DEFAULT_AUTH_OPTIONS, ...options };
  const paths = readSupabaseAuthActionPaths(merged);
  const supabase = await createDxSupabaseServerClient();
  const { error } = await supabase.auth.signUp(readEmailPassword(formData));

  if (error) {
    redirect(withMessage(paths.failurePath, error.message));
  }

  revalidatePath(paths.revalidate, "layout");
  redirect(paths.successPath);
}

export async function dxSupabaseSignOut(
  options: DxSupabaseAuthActionOptions = {},
) {
  const merged = { ...DEFAULT_AUTH_OPTIONS, successPath: "/", ...options };
  const paths = readSupabaseAuthActionPaths(merged);
  const supabase = await createDxSupabaseServerClient();
  await supabase.auth.signOut();
  revalidatePath(paths.revalidate, "layout");
  redirect(paths.successPath);
}

function readEmailPassword(formData: FormData) {
  const email = String(formData.get("email") ?? "").trim();
  const password = String(formData.get("password") ?? "");

  if (!email || !password) {
    throw new Error("Email and password are required.");
  }

  return { email, password };
}

function readSupabaseAuthActionPaths(
  options: Required<DxSupabaseAuthActionOptions>,
) {
  return {
    successPath: assertSupabaseLocalRedirectPath(options.successPath, "successPath"),
    failurePath: assertSupabaseLocalRedirectPath(options.failurePath, "failurePath"),
    revalidate: assertSupabaseLocalRedirectPath(options.revalidate, "revalidate"),
  };
}

function assertSupabaseLocalRedirectPath(path: string, field: string): string {
  if (!path.startsWith("/") || path.startsWith("//")) {
    throw new Error(`Supabase auth ${field} must be an app-local path.`);
  }

  const url = new URL(path, "http://dx.local");
  if (url.origin !== "http://dx.local") {
    throw new Error(`Supabase auth ${field} must be an app-local path.`);
  }

  return `${url.pathname}${url.search}${url.hash}`;
}

function withMessage(path: string, message: string): string {
  const url = new URL(path, "http://dx.local");
  url.searchParams.set("message", message);
  return `${url.pathname}${url.search}`;
}
"#,
        ),
        (
            "js/supabase/auth-callback.ts",
            r#"import { NextResponse, type NextRequest } from "next/server";

import { createDxSupabaseServerClient } from "./server";

export type DxSupabaseAuthCallbackOptions = {
  successPath?: string;
  errorPath?: string;
  nextParam?: string;
};

const DEFAULT_AUTH_CALLBACK_OPTIONS: Required<DxSupabaseAuthCallbackOptions> = {
  successPath: "/account",
  errorPath: "/login",
  nextParam: "next",
};

export async function handleDxSupabaseAuthCallback(
  request: NextRequest,
  options: DxSupabaseAuthCallbackOptions = {},
): Promise<NextResponse> {
  const requestUrl = new URL(request.url);
  const code = requestUrl.searchParams.get("code");
  const paths = readSupabaseCallbackPaths(requestUrl, {
    ...DEFAULT_AUTH_CALLBACK_OPTIONS,
    ...options,
  });

  if (code) {
    const supabase = await createDxSupabaseServerClient();
    const { error } = await supabase.auth.exchangeCodeForSession(code);

    if (!error) {
      return NextResponse.redirect(new URL(paths.successPath, requestUrl.origin));
    }
  }

  return NextResponse.redirect(
    withCallbackMessage(
      paths.errorPath,
      "Unable to confirm Supabase auth callback.",
      requestUrl.origin,
    ),
  );
}

function readSupabaseCallbackPaths(
  requestUrl: URL,
  options: Required<DxSupabaseAuthCallbackOptions>,
) {
  const fallbackSuccessPath = assertSupabaseLocalCallbackPath(
    options.successPath,
    "successPath",
  );
  const requestedPath = requestUrl.searchParams.get(options.nextParam);

  return {
    successPath:
      requestedPath && isSupabaseLocalCallbackPath(requestedPath)
        ? normalizeSupabaseCallbackPath(requestedPath)
        : fallbackSuccessPath,
    errorPath: assertSupabaseLocalCallbackPath(options.errorPath, "errorPath"),
  };
}

function assertSupabaseLocalCallbackPath(path: string, field: string): string {
  if (!isSupabaseLocalCallbackPath(path)) {
    throw new Error(`Supabase auth callback ${field} must be an app-local path.`);
  }

  return normalizeSupabaseCallbackPath(path);
}

function isSupabaseLocalCallbackPath(path: string): boolean {
  if (!path.startsWith("/") || path.startsWith("//")) {
    return false;
  }

  try {
    return new URL(path, "http://dx.local").origin === "http://dx.local";
  } catch {
    return false;
  }
}

function normalizeSupabaseCallbackPath(path: string): string {
  const url = new URL(path, "http://dx.local");
  return `${url.pathname}${url.search}${url.hash}`;
}

function withCallbackMessage(path: string, message: string, origin: string): URL {
  const url = new URL(path, origin);
  url.searchParams.set("message", message);
  return url;
}
"#,
        ),
        (
            "js/supabase/auth-confirm.ts",
            r#"import type { EmailOtpType } from "@supabase/supabase-js";
import { NextResponse, type NextRequest } from "next/server";

import { createDxSupabaseServerClient } from "./server";

export type DxSupabaseAuthConfirmOptions = {
  successPath?: string;
  errorPath?: string;
  nextParam?: string;
};

const DEFAULT_AUTH_CONFIRM_OPTIONS: Required<DxSupabaseAuthConfirmOptions> = {
  successPath: "/account",
  errorPath: "/auth/error",
  nextParam: "next",
};

const DX_SUPABASE_EMAIL_OTP_TYPES = new Set([
  "signup",
  "invite",
  "magiclink",
  "recovery",
  "email_change",
  "email",
]);

export async function handleDxSupabaseAuthConfirm(
  request: NextRequest,
  options: DxSupabaseAuthConfirmOptions = {},
): Promise<NextResponse> {
  const requestUrl = new URL(request.url);
  const tokenHash = requestUrl.searchParams.get("token_hash");
  const type = readDxSupabaseEmailOtpType(requestUrl.searchParams.get("type"));
  const paths = readSupabaseConfirmPaths(requestUrl, {
    ...DEFAULT_AUTH_CONFIRM_OPTIONS,
    ...options,
  });

  if (tokenHash && type) {
    const supabase = await createDxSupabaseServerClient();
    const { error } = await supabase.auth.verifyOtp({
      type,
      token_hash: tokenHash,
    });

    if (!error) {
      return NextResponse.redirect(new URL(paths.successPath, requestUrl.origin));
    }

    return NextResponse.redirect(
      withConfirmMessage(paths.errorPath, error.message, requestUrl.origin),
    );
  }

  return NextResponse.redirect(
    withConfirmMessage(
      paths.errorPath,
      "Missing Supabase auth confirmation token.",
      requestUrl.origin,
    ),
  );
}

function readSupabaseConfirmPaths(
  requestUrl: URL,
  options: Required<DxSupabaseAuthConfirmOptions>,
) {
  const fallbackSuccessPath = assertSupabaseLocalConfirmPath(
    options.successPath,
    "successPath",
  );
  const requestedPath = requestUrl.searchParams.get(options.nextParam);

  return {
    successPath:
      requestedPath && isSupabaseLocalConfirmPath(requestedPath)
        ? normalizeSupabaseConfirmPath(requestedPath)
        : fallbackSuccessPath,
    errorPath: assertSupabaseLocalConfirmPath(options.errorPath, "errorPath"),
  };
}

function readDxSupabaseEmailOtpType(value: string | null): EmailOtpType | null {
  if (value && DX_SUPABASE_EMAIL_OTP_TYPES.has(value)) {
    return value as EmailOtpType;
  }

  return null;
}

function assertSupabaseLocalConfirmPath(path: string, field: string): string {
  if (!isSupabaseLocalConfirmPath(path)) {
    throw new Error(`Supabase auth confirm ${field} must be an app-local path.`);
  }

  return normalizeSupabaseConfirmPath(path);
}

function isSupabaseLocalConfirmPath(path: string): boolean {
  if (!path.startsWith("/") || path.startsWith("//")) {
    return false;
  }

  try {
    return new URL(path, "http://dx.local").origin === "http://dx.local";
  } catch {
    return false;
  }
}

function normalizeSupabaseConfirmPath(path: string): string {
  const url = new URL(path, "http://dx.local");
  return `${url.pathname}${url.search}${url.hash}`;
}

function withConfirmMessage(path: string, message: string, origin: string): URL {
  const url = new URL(path, origin);
  url.searchParams.set("message", message);
  return url;
}
"#,
        ),
        (
            "js/supabase/auth-otp.ts",
            r#""use client";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseOtpSignInOptions = {
  email: string;
  callbackPath?: string;
  nextPath?: string;
  nextParam?: string;
  shouldCreateUser?: boolean;
  captchaToken?: string;
};

const DEFAULT_OTP_OPTIONS = {
  callbackPath: "/auth/confirm",
  nextPath: "/account",
  nextParam: "next",
};

export async function signInWithDxSupabaseOtp(
  options: DxSupabaseOtpSignInOptions,
) {
  const email = assertDxSupabaseOtpEmail(options.email);
  const supabase = createDxSupabaseBrowserClient();

  return supabase.auth.signInWithOtp({
    email,
    options: {
      emailRedirectTo: buildDxSupabaseOtpRedirectTo(options),
      shouldCreateUser: options.shouldCreateUser,
      captchaToken: options.captchaToken,
    },
  });
}

export function buildDxSupabaseOtpRedirectTo(
  options: Pick<
    DxSupabaseOtpSignInOptions,
    "callbackPath" | "nextPath" | "nextParam"
  > = {},
): string {
  const callbackPath = assertSupabaseLocalOtpPath(
    options.callbackPath ?? DEFAULT_OTP_OPTIONS.callbackPath,
    "callbackPath",
  );
  const nextPath = assertSupabaseLocalOtpPath(
    options.nextPath ?? DEFAULT_OTP_OPTIONS.nextPath,
    "nextPath",
  );
  const nextParam = assertSupabaseOtpNextParam(
    options.nextParam ?? DEFAULT_OTP_OPTIONS.nextParam,
  );
  const url = new URL(callbackPath, window.location.origin);

  url.searchParams.set(nextParam, nextPath);
  return url.toString();
}

function assertDxSupabaseOtpEmail(email: string): string {
  const value = email.trim();
  if (!value || !value.includes("@")) {
    throw new Error("Supabase OTP email is required.");
  }
  return value;
}

function assertSupabaseLocalOtpPath(path: string, field: string): string {
  if (!path.startsWith("/") || path.startsWith("//")) {
    throw new Error(`Supabase OTP ${field} must be an app-local path.`);
  }

  const url = new URL(path, "http://dx.local");
  if (url.origin !== "http://dx.local") {
    throw new Error(`Supabase OTP ${field} must be an app-local path.`);
  }

  return `${url.pathname}${url.search}${url.hash}`;
}

function assertSupabaseOtpNextParam(value: string): string {
  if (!/^[A-Za-z0-9_-]+$/.test(value)) {
    throw new Error("Supabase OTP nextParam must be a simple query key.");
  }

  return value;
}
"#,
        ),
        (
            "js/supabase/auth-mfa.ts",
            r#""use client";

import type {
  Factor,
  MFAChallengeAndVerifyParams,
  MFAEnrollParams,
  MFAUnenrollParams,
  MFAVerifyParams,
} from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseMfaFactor = Factor;
export type DxSupabaseMfaAssuranceLevel = "aal1" | "aal2";

export type DxSupabaseMfaFactors = {
  totp: DxSupabaseMfaFactor[];
  phone: DxSupabaseMfaFactor[];
  all: DxSupabaseMfaFactor[];
};

export type DxSupabaseMfaAssurance = {
  currentLevel: DxSupabaseMfaAssuranceLevel | null;
  nextLevel: DxSupabaseMfaAssuranceLevel | null;
  currentAuthenticationMethods: unknown[];
};

type DxSupabaseTotpEnrollParams = Extract<
  MFAEnrollParams,
  { factorType: "totp" }
>;
type DxSupabasePhoneEnrollParams = Extract<
  MFAEnrollParams,
  { factorType: "phone" }
>;

export type DxSupabaseTotpEnrollOptions = Omit<
  DxSupabaseTotpEnrollParams,
  "factorType"
>;

export type DxSupabasePhoneEnrollOptions = Omit<
  DxSupabasePhoneEnrollParams,
  "factorType" | "phone"
> & {
  phone: string;
};

export type DxSupabaseMfaChallengeOptions = {
  factorId: string;
};

export type DxSupabaseMfaVerifyOptions = Pick<
  MFAVerifyParams,
  "factorId" | "challengeId" | "code"
>;

export async function listDxSupabaseMfaFactors(): Promise<DxSupabaseMfaFactors> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.listFactors();

  if (error) {
    throw error;
  }

  const totp = data.totp ?? [];
  const phone = data.phone ?? [];

  return {
    totp,
    phone,
    all: [...totp, ...phone],
  };
}

export async function getDxSupabaseMfaAssuranceLevel(): Promise<DxSupabaseMfaAssurance> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.getAuthenticatorAssuranceLevel();

  if (error) {
    throw error;
  }

  return {
    currentLevel: toDxSupabaseMfaAssuranceLevel(data.currentLevel),
    nextLevel: toDxSupabaseMfaAssuranceLevel(data.nextLevel),
    currentAuthenticationMethods: data.currentAuthenticationMethods ?? [],
  };
}

export async function enrollDxSupabaseTotpFactor(
  options: DxSupabaseTotpEnrollOptions = {},
) {
  const params: DxSupabaseTotpEnrollParams = {
    ...options,
    factorType: "totp",
  };
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.enroll(params);

  if (error) {
    throw error;
  }

  return data;
}

export async function enrollDxSupabasePhoneFactor(
  options: DxSupabasePhoneEnrollOptions,
) {
  const params: DxSupabasePhoneEnrollParams = {
    ...options,
    factorType: "phone",
    phone: assertDxSupabaseMfaPhone(options.phone),
  };
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.enroll(params);

  if (error) {
    throw error;
  }

  return data;
}

export async function challengeDxSupabaseMfa(
  options: DxSupabaseMfaChallengeOptions,
) {
  const factorId = assertDxSupabaseMfaFactorId(options.factorId);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.challenge({ factorId });

  if (error) {
    throw error;
  }

  return data;
}

export async function verifyDxSupabaseMfa(
  options: DxSupabaseMfaVerifyOptions,
) {
  const factorId = assertDxSupabaseMfaFactorId(options.factorId);
  const challengeId = assertDxSupabaseMfaFactorId(options.challengeId);
  const code = assertDxSupabaseMfaCode(options.code);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.verify({
    factorId,
    challengeId,
    code,
  });

  if (error) {
    throw error;
  }

  return data;
}

export async function challengeAndVerifyDxSupabaseMfa(
  options: MFAChallengeAndVerifyParams,
) {
  const params: MFAChallengeAndVerifyParams = {
    factorId: assertDxSupabaseMfaFactorId(options.factorId),
    code: assertDxSupabaseMfaCode(options.code),
  };
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.challengeAndVerify({
    factorId: params.factorId,
    code: params.code,
  });

  if (error) {
    throw error;
  }

  return data;
}

export async function unenrollDxSupabaseMfaFactor(
  options: MFAUnenrollParams,
) {
  const factorId = assertDxSupabaseMfaFactorId(options.factorId);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.mfa.unenroll({ factorId });

  if (error) {
    throw error;
  }

  return data;
}

function toDxSupabaseMfaAssuranceLevel(
  value: string | null | undefined,
): DxSupabaseMfaAssuranceLevel | null {
  return value === "aal1" || value === "aal2" ? value : null;
}

function assertDxSupabaseMfaFactorId(factorId: string): string {
  const value = factorId.trim();

  if (!/^[A-Za-z0-9_-]+$/.test(value)) {
    throw new Error("Supabase MFA factor id must be a simple identifier.");
  }

  return value;
}

function assertDxSupabaseMfaCode(code: string): string {
  const value = code.trim().replace(/\s+/g, "");

  if (!/^[0-9]{4,10}$/.test(value)) {
    throw new Error("Supabase MFA code must be 4 to 10 digits.");
  }

  return value;
}

function assertDxSupabaseMfaPhone(phone: string): string {
  const value = phone.trim();

  if (!/^\+[1-9][0-9]{7,14}$/.test(value)) {
    throw new Error("Supabase MFA phone must be an E.164 phone number.");
  }

  return value;
}
"#,
        ),
        (
            "js/supabase/realtime-postgres.ts",
            r#""use client";

import type {
  RealtimeChannel,
  RealtimePostgresChangesPayload,
} from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseRealtimePostgresEvent =
  | "*"
  | "INSERT"
  | "UPDATE"
  | "DELETE";

export type DxSupabaseRealtimePostgresOptions<
  Row extends Record<string, unknown> = Record<string, unknown>,
> = {
  channel?: string;
  schema?: string;
  table: string;
  event?: DxSupabaseRealtimePostgresEvent;
  filter?: string;
  onChange: (payload: RealtimePostgresChangesPayload<Row>) => void;
  onStatus?: (status: string, error?: Error) => void;
};

export type DxSupabaseRealtimePostgresSubscription = {
  channel: RealtimeChannel;
  unsubscribe: () => Promise<"ok" | "timed out" | "error">;
};

export function subscribeToDxSupabasePostgresChanges<
  Row extends Record<string, unknown> = Record<string, unknown>,
>(
  options: DxSupabaseRealtimePostgresOptions<Row>,
): DxSupabaseRealtimePostgresSubscription {
  const supabase = createDxSupabaseBrowserClient();
  const channelName = assertDxSupabaseRealtimeChannel(
    options.channel ?? `db:${options.schema ?? "public"}:${options.table}`,
  );
  const channel = supabase.channel(channelName)
    .on(
      "postgres_changes",
      {
        event: options.event ?? "*",
        schema: options.schema ?? "public",
        table: assertDxSupabaseRealtimeTable(options.table),
        filter: options.filter,
      },
      (payload) => options.onChange(payload as RealtimePostgresChangesPayload<Row>),
    )
    .subscribe((status, error) => {
      options.onStatus?.(status, error);
    });

  return {
    channel,
    unsubscribe() {
      return supabase.removeChannel(channel);
    },
  };
}

function assertDxSupabaseRealtimeChannel(channel: string): string {
  const value = channel.trim();
  if (!value || value === "realtime" || value.includes("\\")) {
    throw new Error(
      "Supabase Realtime channel must be non-empty and must not be 'realtime'.",
    );
  }
  return value;
}

function assertDxSupabaseRealtimeTable(table: string): string {
  const value = table.trim();
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(value)) {
    throw new Error("Supabase Realtime table must be a simple table name.");
  }
  return value;
}
"#,
        ),
        (
            "js/supabase/realtime-broadcast.ts",
            r#""use client";

import type { RealtimeChannel } from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseBroadcastPayload = Record<string, unknown>;

export type DxSupabaseRealtimeBroadcastConfig = {
  private?: boolean;
  self?: boolean;
  ack?: boolean;
};

export type DxSupabaseRealtimeBroadcastMessage<
  Payload = DxSupabaseBroadcastPayload,
> = {
  payload: Payload;
};

export type DxSupabaseRealtimeBroadcastOptions<
  Payload = DxSupabaseBroadcastPayload,
> = {
  channel: string;
  event: string;
  config?: DxSupabaseRealtimeBroadcastConfig;
  onMessage: (message: DxSupabaseRealtimeBroadcastMessage<Payload>) => void;
  onStatus?: (status: string, error?: Error) => void;
};

export type DxSupabaseRealtimeBroadcastSubscription<
  Payload = DxSupabaseBroadcastPayload,
> = {
  channel: RealtimeChannel;
  send: (payload: Payload) => Promise<"ok" | "timed out" | "error">;
  unsubscribe: () => Promise<"ok" | "timed out" | "error">;
};

export function subscribeToDxSupabaseBroadcast<
  Payload = DxSupabaseBroadcastPayload,
>(
  options: DxSupabaseRealtimeBroadcastOptions<Payload>,
): DxSupabaseRealtimeBroadcastSubscription<Payload> {
  const supabase = createDxSupabaseBrowserClient();
  const channelName = assertDxSupabaseBroadcastChannel(options.channel);
  const event = assertDxSupabaseBroadcastEvent(options.event);
  const channel = supabase
    .channel(channelName, {
      config: {
        private: options.config?.private ?? true,
        broadcast: {
          self: options.config?.self ?? true,
          ack: options.config?.ack ?? true,
        },
      },
    })
    .on("broadcast", { event }, (message) => {
      options.onMessage(message as DxSupabaseRealtimeBroadcastMessage<Payload>);
    })
    .subscribe((status, error) => {
      options.onStatus?.(status, error);
    });

  return {
    channel,
    send(payload: Payload) {
      return channel.send({
        type: "broadcast",
        event,
        payload,
      });
    },
    unsubscribe() {
      return supabase.removeChannel(channel);
    },
  };
}

export async function sendDxSupabaseBroadcast<
  Payload = DxSupabaseBroadcastPayload,
>(options: {
  channel: RealtimeChannel;
  event: string;
  payload: Payload;
}): Promise<"ok" | "timed out" | "error"> {
  return options.channel.send({
    type: "broadcast",
    event: assertDxSupabaseBroadcastEvent(options.event),
    payload: options.payload,
  });
}

function assertDxSupabaseBroadcastChannel(channel: string): string {
  const value = channel.trim();
  if (
    !value ||
    value === "realtime" ||
    value.includes("\\") ||
    value.includes("://")
  ) {
    throw new Error(
      "Supabase broadcast channel must be non-empty and must not be 'realtime'.",
    );
  }
  return value;
}

function assertDxSupabaseBroadcastEvent(event: string): string {
  const value = event.trim();
  if (!/^[A-Za-z0-9_.:-]+$/.test(value)) {
    throw new Error("Supabase broadcast event must be a simple event name.");
  }
  return value;
}
"#,
        ),
        (
            "js/supabase/realtime-presence.ts",
            r#""use client";

import type { RealtimeChannel } from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabasePresencePayload = Record<string, unknown>;

export type DxSupabasePresenceState<
  Presence = DxSupabasePresencePayload,
> = Record<string, Presence[]>;

export type DxSupabasePresenceJoinEvent<
  Presence = DxSupabasePresencePayload,
> = {
  key: string;
  newPresences: Presence[];
};

export type DxSupabasePresenceLeaveEvent<
  Presence = DxSupabasePresencePayload,
> = {
  key: string;
  leftPresences: Presence[];
};

export type DxSupabaseRealtimePresenceOptions<
  Presence = DxSupabasePresencePayload,
> = {
  channel: string;
  key?: string;
  initialPresence?: Presence;
  onSync?: (state: DxSupabasePresenceState<Presence>) => void;
  onJoin?: (event: DxSupabasePresenceJoinEvent<Presence>) => void;
  onLeave?: (event: DxSupabasePresenceLeaveEvent<Presence>) => void;
  onStatus?: (status: string, error?: Error) => void;
};

export type DxSupabaseRealtimePresenceSubscription<
  Presence = DxSupabasePresencePayload,
> = {
  channel: RealtimeChannel;
  presenceState: () => DxSupabasePresenceState<Presence>;
  track: (presence: Presence) => Promise<"ok" | "timed out" | "error">;
  untrack: () => Promise<"ok" | "timed out" | "error">;
  unsubscribe: () => Promise<"ok" | "timed out" | "error">;
};

export function subscribeToDxSupabasePresence<
  Presence = DxSupabasePresencePayload,
>(
  options: DxSupabaseRealtimePresenceOptions<Presence>,
): DxSupabaseRealtimePresenceSubscription<Presence> {
  const supabase = createDxSupabaseBrowserClient();
  const channelName = assertDxSupabasePresenceChannel(options.channel);
  const channelOptions = options.key
    ? {
        config: {
          presence: {
            key: assertDxSupabasePresenceKey(options.key),
          },
        },
      }
    : undefined;
  const channel = supabase
    .channel(channelName, channelOptions)
    .on("presence", { event: "sync" }, () => {
      options.onSync?.(
        channel.presenceState() as DxSupabasePresenceState<Presence>,
      );
    })
    .on("presence", { event: "join" }, (event) => {
      options.onJoin?.({
        key: event.key,
        newPresences: event.newPresences as Presence[],
      });
    })
    .on("presence", { event: "leave" }, (event) => {
      options.onLeave?.({
        key: event.key,
        leftPresences: event.leftPresences as Presence[],
      });
    })
    .subscribe((status, error) => {
      options.onStatus?.(status, error);
      if (status === "SUBSCRIBED" && options.initialPresence) {
        void channel.track(options.initialPresence);
      }
    });

  return {
    channel,
    presenceState() {
      return channel.presenceState() as DxSupabasePresenceState<Presence>;
    },
    track(presence: Presence) {
      return channel.track(presence);
    },
    untrack() {
      return channel.untrack();
    },
    unsubscribe() {
      return supabase.removeChannel(channel);
    },
  };
}

function assertDxSupabasePresenceChannel(channel: string): string {
  const value = channel.trim();
  if (
    !value ||
    value === "realtime" ||
    value.includes("\\") ||
    value.includes("://")
  ) {
    throw new Error(
      "Supabase presence channel must be non-empty and must not be 'realtime'.",
    );
  }
  return value;
}

function assertDxSupabasePresenceKey(key: string): string {
  const value = key.trim();
  if (!value || value.includes("\\") || value.includes("://")) {
    throw new Error("Supabase presence key must be a simple user/session key.");
  }
  return value;
}
"#,
        ),
        (
            "js/supabase/realtime-auth.ts",
            r#""use client";

import type { Session, SupabaseClient } from "@supabase/supabase-js";

import {
  getDxSupabaseBrowserSession,
  readDxSupabaseAccessToken,
} from "./auth-session";
import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseRealtimeAuthOptions = {
  accessToken?: string;
  session?: Session | null;
  allowMissingSession?: boolean;
};

export type DxSupabaseRealtimeAuthResult<Database = never> =
  | {
      supabase: SupabaseClient<Database>;
      accessToken: string;
      authenticated: true;
    }
  | {
      supabase: SupabaseClient<Database>;
      accessToken: null;
      authenticated: false;
    };

export type DxSupabasePrivateRealtimeChannelOptions = {
  self?: boolean;
  ack?: boolean;
  presenceKey?: string;
};

export type DxSupabasePrivateRealtimeChannelConfig = {
  config: {
    private: true;
    broadcast: {
      self: boolean;
      ack: boolean;
    };
    presence?: {
      key: string;
    };
  };
};

export async function applyDxSupabaseRealtimeAuth<Database = never>(
  options: DxSupabaseRealtimeAuthOptions = {},
): Promise<DxSupabaseRealtimeAuthResult<Database>> {
  const accessToken = await resolveDxSupabaseRealtimeAccessToken(options);
  const supabase = createDxSupabaseBrowserClient<Database>();

  if (!accessToken) {
    if (options.allowMissingSession) {
      return {
        supabase,
        accessToken: null,
        authenticated: false,
      };
    }

    throw new Error(
      "Supabase Realtime private channels require an authenticated session.",
    );
  }

  supabase.realtime.setAuth(accessToken);

  return {
    supabase,
    accessToken,
    authenticated: true,
  };
}

export async function resolveDxSupabaseRealtimeAccessToken(
  options: Pick<DxSupabaseRealtimeAuthOptions, "accessToken" | "session"> = {},
): Promise<string | null> {
  if (options.accessToken) {
    return assertDxSupabaseRealtimeAccessToken(options.accessToken);
  }

  if ("session" in options) {
    const providedSession = options.session ?? null;
    return readDxSupabaseAccessToken(providedSession);
  }

  const session = await getDxSupabaseBrowserSession();
  return readDxSupabaseAccessToken(session);
}

export function buildDxSupabasePrivateRealtimeChannelConfig(
  options: DxSupabasePrivateRealtimeChannelOptions = {},
): DxSupabasePrivateRealtimeChannelConfig {
  const config: DxSupabasePrivateRealtimeChannelConfig["config"] = {
    private: true,
    broadcast: {
      self: options.self ?? true,
      ack: options.ack ?? true,
    },
  };

  if (options.presenceKey) {
    config.presence = {
      key: assertDxSupabaseRealtimePresenceKey(options.presenceKey),
    };
  }

  return {
    config,
  };
}

function assertDxSupabaseRealtimeAccessToken(accessToken: string): string {
  const value = accessToken.trim();

  if (!value || value.split(".").length < 2) {
    throw new Error("Supabase Realtime access token must be a JWT-like token.");
  }

  return value;
}

function assertDxSupabaseRealtimePresenceKey(key: string): string {
  const value = key.trim();

  if (!value || value.includes("\\") || value.includes("://")) {
    throw new Error("Supabase Realtime presence key must be a simple key.");
  }

  return value;
}
"#,
        ),
        (
            "js/supabase/edge-functions.ts",
            r#""use client";

import {
  FunctionsFetchError,
  FunctionsHttpError,
  FunctionsRelayError,
  type FunctionRegion,
} from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseFunctionMethod =
  | "GET"
  | "POST"
  | "PUT"
  | "PATCH"
  | "DELETE";

export type DxSupabaseFunctionInvokeOptions<Body = unknown> = {
  body?: Body;
  headers?: Record<string, string>;
  method?: DxSupabaseFunctionMethod;
  region?: FunctionRegion;
};

export type DxSupabaseFunctionErrorKind =
  | "http"
  | "relay"
  | "fetch"
  | "unknown";

export type DxSupabaseFunctionInvokeResult<TResponse> =
  | { data: TResponse | null; error: null }
  | { data: null; error: DxSupabaseFunctionInvokeError };

export class DxSupabaseFunctionInvokeError extends Error {
  readonly kind: DxSupabaseFunctionErrorKind;
  readonly context: Response | null;
  readonly cause: unknown;

  constructor(
    kind: DxSupabaseFunctionErrorKind,
    message: string,
    cause: unknown,
    context: Response | null = null,
  ) {
    super(message);
    this.name = "DxSupabaseFunctionInvokeError";
    this.kind = kind;
    this.cause = cause;
    this.context = context;
  }
}

export async function invokeDxSupabaseFunction<
  TResponse = unknown,
  Body = unknown,
>(
  name: string,
  options: DxSupabaseFunctionInvokeOptions<Body> = {},
): Promise<TResponse | null> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.functions.invoke<TResponse>(
    assertDxSupabaseFunctionName(name),
    {
      body: options.body,
      headers: options.headers,
      method: options.method,
      region: options.region,
    },
  );

  if (error) {
    throw toDxSupabaseFunctionInvokeError(error);
  }

  return data;
}

export async function tryInvokeDxSupabaseFunction<
  TResponse = unknown,
  Body = unknown,
>(
  name: string,
  options: DxSupabaseFunctionInvokeOptions<Body> = {},
): Promise<DxSupabaseFunctionInvokeResult<TResponse>> {
  try {
    const data = await invokeDxSupabaseFunction<TResponse, Body>(name, options);
    return { data, error: null };
  } catch (error) {
    return { data: null, error: toDxSupabaseFunctionInvokeError(error) };
  }
}

function toDxSupabaseFunctionInvokeError(
  error: unknown,
): DxSupabaseFunctionInvokeError {
  if (error instanceof DxSupabaseFunctionInvokeError) {
    return error;
  }
  if (error instanceof FunctionsHttpError) {
    return new DxSupabaseFunctionInvokeError(
      "http",
      error.message,
      error,
      error.context,
    );
  }
  if (error instanceof FunctionsRelayError) {
    return new DxSupabaseFunctionInvokeError("relay", error.message, error);
  }
  if (error instanceof FunctionsFetchError) {
    return new DxSupabaseFunctionInvokeError("fetch", error.message, error);
  }

  return new DxSupabaseFunctionInvokeError(
    "unknown",
    error instanceof Error ? error.message : "Supabase Edge Function invocation failed.",
    error,
  );
}

function assertDxSupabaseFunctionName(name: string): string {
  const value = name.trim();
  if (!/^[A-Za-z0-9_-]+$/.test(value)) {
    throw new Error("Supabase Edge Function name must be a simple function id.");
  }
  return value;
}
"#,
        ),
        (
            "js/supabase/auth-oauth.ts",
            r#""use client";

import type { Provider } from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseOAuthSignInOptions = {
  provider: Provider;
  callbackPath?: string;
  nextPath?: string;
  nextParam?: string;
  scopes?: string;
  queryParams?: Record<string, string>;
  skipBrowserRedirect?: boolean;
};

const DEFAULT_OAUTH_OPTIONS = {
  callbackPath: "/auth/callback",
  nextPath: "/account",
  nextParam: "next",
};

export async function signInWithDxSupabaseOAuth(
  options: DxSupabaseOAuthSignInOptions,
) {
  const supabase = createDxSupabaseBrowserClient();

  return supabase.auth.signInWithOAuth({
    provider: options.provider,
    options: {
      redirectTo: buildDxSupabaseOAuthRedirectTo(options),
      scopes: options.scopes,
      queryParams: options.queryParams,
      skipBrowserRedirect: options.skipBrowserRedirect,
    },
  });
}

export function buildDxSupabaseOAuthRedirectTo(
  options: DxSupabaseOAuthSignInOptions,
): string {
  const callbackPath = assertSupabaseLocalOAuthPath(
    options.callbackPath ?? DEFAULT_OAUTH_OPTIONS.callbackPath,
    "callbackPath",
  );
  const nextPath = assertSupabaseLocalOAuthPath(
    options.nextPath ?? DEFAULT_OAUTH_OPTIONS.nextPath,
    "nextPath",
  );
  const nextParam = assertSupabaseOAuthNextParam(
    options.nextParam ?? DEFAULT_OAUTH_OPTIONS.nextParam,
  );

  const url = new URL(callbackPath, window.location.origin);
  url.searchParams.set(nextParam, nextPath);
  return url.toString();
}

function assertSupabaseLocalOAuthPath(path: string, field: string): string {
  if (!path.startsWith("/") || path.startsWith("//")) {
    throw new Error(`Supabase OAuth ${field} must be an app-local path.`);
  }

  const url = new URL(path, "http://dx.local");
  if (url.origin !== "http://dx.local") {
    throw new Error(`Supabase OAuth ${field} must be an app-local path.`);
  }

  return `${url.pathname}${url.search}${url.hash}`;
}

function assertSupabaseOAuthNextParam(value: string): string {
  if (!/^[A-Za-z0-9_-]+$/.test(value)) {
    throw new Error("Supabase OAuth nextParam must be a simple query key.");
  }

  return value;
}
"#,
        ),
        (
            "js/supabase/auth-identities.ts",
            r#""use client";

import type {
  Provider,
  SignInWithIdTokenCredentials,
  SignInWithOAuthCredentials,
  UserIdentity,
} from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseUserIdentity = UserIdentity;
export type DxSupabaseIdentityProvider = Provider;

type DxSupabaseOAuthLinkOptions = NonNullable<
  SignInWithOAuthCredentials["options"]
>;

export type DxSupabaseOAuthIdentityLinkOptions = Omit<
  SignInWithOAuthCredentials,
  "provider" | "options"
> & {
  provider: Provider;
  callbackPath?: string;
  nextPath?: string;
  nextParam?: string;
  options?: DxSupabaseOAuthLinkOptions;
};

const DEFAULT_IDENTITY_LINK_OPTIONS = {
  callbackPath: "/auth/callback",
  nextPath: "/account",
  nextParam: "next",
};

export async function listDxSupabaseUserIdentities(): Promise<
  DxSupabaseUserIdentity[]
> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.getUserIdentities();

  if (error) {
    throw error;
  }

  return data.identities ?? [];
}

export async function getDxSupabaseIdentityByProvider(
  provider: Provider,
): Promise<DxSupabaseUserIdentity | null> {
  provider = assertDxSupabaseIdentityProvider(provider);
  const identities = await listDxSupabaseUserIdentities();
  return identities.find((identity) => identity.provider === provider) ?? null;
}

export async function linkDxSupabaseOAuthIdentity(
  options: DxSupabaseOAuthIdentityLinkOptions,
) {
  const credentials: SignInWithOAuthCredentials = {
    ...options,
    provider: assertDxSupabaseIdentityProvider(options.provider),
    options: {
      ...options.options,
      redirectTo: buildDxSupabaseIdentityRedirectTo(options),
    },
  };
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.linkIdentity(credentials);

  if (error) {
    throw error;
  }

  return data;
}

export async function linkDxSupabaseIdTokenIdentity(
  options: SignInWithIdTokenCredentials,
) {
  const credentials: SignInWithIdTokenCredentials = {
    ...options,
    provider: assertDxSupabaseIdentityProvider(options.provider),
    token: assertDxSupabaseIdentityToken(options.token),
  };
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.linkIdentity(credentials);

  if (error) {
    throw error;
  }

  return data;
}

export async function unlinkDxSupabaseIdentity(identity: UserIdentity) {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.unlinkIdentity(assertDxSupabaseUserIdentity(identity));

  if (error) {
    throw error;
  }

  const refresh = await supabase.auth.refreshSession();
  if (refresh.error) {
    throw refresh.error;
  }

  return data;
}

export async function unlinkDxSupabaseIdentityByProvider(provider: Provider) {
  provider = assertDxSupabaseIdentityProvider(provider);
  const identity = await getDxSupabaseIdentityByProvider(provider);

  if (!identity) {
    throw new Error(`Supabase identity ${provider} is not linked.`);
  }

  return unlinkDxSupabaseIdentity(identity);
}

export function buildDxSupabaseIdentityRedirectTo(
  options: Pick<
    DxSupabaseOAuthIdentityLinkOptions,
    "callbackPath" | "nextPath" | "nextParam"
  > = {},
): string {
  const callbackPath = assertSupabaseLocalIdentityPath(
    options.callbackPath ?? DEFAULT_IDENTITY_LINK_OPTIONS.callbackPath,
    "callbackPath",
  );
  const nextPath = assertSupabaseLocalIdentityPath(
    options.nextPath ?? DEFAULT_IDENTITY_LINK_OPTIONS.nextPath,
    "nextPath",
  );
  const nextParam = assertSupabaseIdentityNextParam(
    options.nextParam ?? DEFAULT_IDENTITY_LINK_OPTIONS.nextParam,
  );
  const url = new URL(callbackPath, window.location.origin);

  url.searchParams.set(nextParam, nextPath);
  return url.toString();
}

function assertDxSupabaseUserIdentity(identity: UserIdentity): UserIdentity {
  if (!identity || typeof identity !== "object") {
    throw new Error("Supabase identity is required.");
  }

  if (!identity.provider || (!identity.identity_id && !identity.id)) {
    throw new Error("Supabase identity must include a provider and identity id.");
  }

  return identity;
}

function assertDxSupabaseIdentityProvider<TProvider extends string>(
  provider: TProvider,
): TProvider {
  const value = provider.trim();

  if (!/^(custom:)?[A-Za-z0-9_-]+$/.test(value)) {
    throw new Error("Supabase identity provider must be a simple provider id.");
  }

  return value as TProvider;
}

function assertDxSupabaseIdentityToken(token: string): string {
  const value = token.trim();

  if (!value) {
    throw new Error("Supabase identity token is required.");
  }

  return value;
}

function assertSupabaseLocalIdentityPath(path: string, field: string): string {
  if (!path.startsWith("/") || path.startsWith("//")) {
    throw new Error(`Supabase identity ${field} must be an app-local path.`);
  }

  const url = new URL(path, "http://dx.local");
  if (url.origin !== "http://dx.local") {
    throw new Error(`Supabase identity ${field} must be an app-local path.`);
  }

  return `${url.pathname}${url.search}${url.hash}`;
}

function assertSupabaseIdentityNextParam(value: string): string {
  if (!/^[A-Za-z0-9_-]+$/.test(value)) {
    throw new Error("Supabase identity nextParam must be a simple query key.");
  }

  return value;
}
"#,
        ),
        (
            "js/supabase/auth-session.ts",
            r#""use client";

import type {
  AuthChangeEvent,
  Session,
  User,
} from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseAuthClaims = Record<string, unknown>;

export type DxSupabaseAuthSnapshot = {
  session: Session | null;
  user: User | null;
  claims: DxSupabaseAuthClaims | null;
  accessToken: string | null;
  expiresAt: number | null;
};

export type DxSupabaseAuthChange = {
  event: AuthChangeEvent;
  session: Session | null;
};

export type DxSupabaseAuthSubscription = {
  unsubscribe: () => void;
};

export type DxSupabaseAuthStateOptions = {
  onChange: (change: DxSupabaseAuthChange) => void;
  defer?: (callback: () => void) => void;
};

export async function getDxSupabaseBrowserSession(): Promise<Session | null> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.getSession();

  if (error) {
    throw error;
  }

  return data.session ?? null;
}

export async function getDxSupabaseTrustedUser(): Promise<User | null> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.getUser();

  if (error) {
    throw error;
  }

  return data.user ?? null;
}

export async function getDxSupabaseVerifiedClaims(): Promise<
  DxSupabaseAuthClaims | null
> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.getClaims();

  if (error) {
    throw error;
  }

  return data.claims ?? null;
}

export async function refreshDxSupabaseBrowserSession(): Promise<Session | null> {
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.refreshSession();

  if (error) {
    throw error;
  }

  return data.session ?? null;
}

export async function readDxSupabaseAuthSnapshot(): Promise<
  DxSupabaseAuthSnapshot
> {
  const session = await getDxSupabaseBrowserSession();

  if (!session) {
    return {
      session: null,
      user: null,
      claims: null,
      accessToken: null,
      expiresAt: null,
    };
  }

  const [user, claims] = await Promise.all([
    getDxSupabaseTrustedUser(),
    getDxSupabaseVerifiedClaims(),
  ]);

  return {
    session,
    user,
    claims,
    accessToken: readDxSupabaseAccessToken(session),
    expiresAt: session.expires_at ?? null,
  };
}

export function subscribeToDxSupabaseAuthState(
  options: DxSupabaseAuthStateOptions,
): DxSupabaseAuthSubscription {
  const defer = options.defer ?? deferDxSupabaseAuthChange;
  const supabase = createDxSupabaseBrowserClient();
  const { data } = supabase.auth.onAuthStateChange((event, session) => {
    defer(() => options.onChange({ event, session }));
  });

  return {
    unsubscribe: () => data.subscription.unsubscribe(),
  };
}

export function readDxSupabaseAccessToken(
  session: Session | null,
): string | null {
  return session?.access_token ?? null;
}

function deferDxSupabaseAuthChange(callback: () => void) {
  setTimeout(callback, 0);
}
"#,
        ),
        (
            "js/supabase/auth-anonymous.ts",
            r#""use client";

import type { Session, User } from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabaseAnonymousMetadata = Record<string, unknown>;

export type DxSupabaseAnonymousSignInOptions = {
  data?: DxSupabaseAnonymousMetadata;
  captchaToken?: string;
};

export type DxSupabaseAnonymousSignInResult = {
  user: User;
  session: Session | null;
  isAnonymous: boolean;
};

export type DxSupabaseAnonymousUpgradeOptions = {
  email?: string;
  phone?: string;
  password?: string;
  data?: DxSupabaseAnonymousMetadata;
  emailRedirectPath?: string;
};

export async function signInDxSupabaseAnonymously(
  options: DxSupabaseAnonymousSignInOptions = {},
): Promise<DxSupabaseAnonymousSignInResult> {
  const credentials = buildDxSupabaseAnonymousCredentials(options);
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.signInAnonymously(credentials);

  if (error) {
    throw error;
  }

  if (!data.user) {
    throw new Error("Supabase anonymous sign-in did not return a user.");
  }

  return {
    user: data.user,
    session: data.session ?? null,
    isAnonymous: readDxSupabaseAnonymousUserFlag(data.user),
  };
}

export async function upgradeDxSupabaseAnonymousUser(
  options: DxSupabaseAnonymousUpgradeOptions,
) {
  const attributes = buildDxSupabaseAnonymousUpgradeAttributes(options);
  const updateOptions = options.emailRedirectPath
    ? {
        emailRedirectTo: buildDxSupabaseAnonymousRedirectTo(
          options.emailRedirectPath,
        ),
      }
    : undefined;
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.updateUser(
    attributes,
    updateOptions,
  );

  if (error) {
    throw error;
  }

  return data;
}

export function buildDxSupabaseAnonymousCredentials(
  options: DxSupabaseAnonymousSignInOptions = {},
) {
  const authOptions: {
    data?: DxSupabaseAnonymousMetadata;
    captchaToken?: string;
  } = {};

  if (options.data) {
    authOptions.data = assertDxSupabaseAnonymousMetadata(options.data);
  }

  if (options.captchaToken) {
    authOptions.captchaToken = assertDxSupabaseAnonymousCaptchaToken(
      options.captchaToken,
    );
  }

  if (Object.keys(authOptions).length === 0) {
    return {};
  }

  return {
    options: authOptions,
  };
}

export function buildDxSupabaseAnonymousRedirectTo(path: string): string {
  const safePath = assertSupabaseLocalAnonymousPath(path);
  return new URL(safePath, window.location.origin).toString();
}

function buildDxSupabaseAnonymousUpgradeAttributes(
  options: DxSupabaseAnonymousUpgradeOptions,
) {
  const attributes: {
    email?: string;
    phone?: string;
    password?: string;
    data?: DxSupabaseAnonymousMetadata;
  } = {};

  if (options.email) {
    attributes.email = assertDxSupabaseAnonymousEmail(options.email);
  }

  if (options.phone) {
    attributes.phone = assertDxSupabaseAnonymousPhone(options.phone);
  }

  if (options.password) {
    attributes.password = assertDxSupabaseAnonymousPassword(options.password);
  }

  if (options.data) {
    attributes.data = assertDxSupabaseAnonymousMetadata(options.data);
  }

  if (Object.keys(attributes).length === 0) {
    throw new Error("At least one anonymous user upgrade field is required.");
  }

  return attributes;
}

function readDxSupabaseAnonymousUserFlag(user: User): boolean {
  return (user as User & { is_anonymous?: boolean }).is_anonymous === true;
}

function assertDxSupabaseAnonymousMetadata(
  data: DxSupabaseAnonymousMetadata,
): DxSupabaseAnonymousMetadata {
  if (!data || typeof data !== "object" || Array.isArray(data)) {
    throw new Error("Supabase anonymous metadata must be an object.");
  }

  return data;
}

function assertDxSupabaseAnonymousCaptchaToken(token: string): string {
  const value = token.trim();

  if (!value) {
    throw new Error("Supabase anonymous captcha token is required.");
  }

  return value;
}

function assertDxSupabaseAnonymousEmail(email: string): string {
  const value = email.trim();

  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
    throw new Error("Supabase anonymous upgrade email must be valid.");
  }

  return value;
}

function assertDxSupabaseAnonymousPhone(phone: string): string {
  const value = phone.trim();

  if (!/^\+?[0-9 ()-]{4,32}$/.test(value)) {
    throw new Error("Supabase anonymous upgrade phone must be valid.");
  }

  return value;
}

function assertDxSupabaseAnonymousPassword(password: string): string {
  if (password.length < 6) {
    throw new Error("Supabase anonymous upgrade password must be at least 6 characters.");
  }

  return password;
}

function assertSupabaseLocalAnonymousPath(path: string): string {
  if (!path.startsWith("/") || path.startsWith("//")) {
    throw new Error("Supabase anonymous redirect path must be an app-local path.");
  }

  const url = new URL(path, "http://dx.local");
  if (url.origin !== "http://dx.local") {
    throw new Error("Supabase anonymous redirect path must be an app-local path.");
  }

  return `${url.pathname}${url.search}${url.hash}`;
}
"#,
        ),
        (
            "js/supabase/password-recovery.ts",
            r#""use client";

import type { User } from "@supabase/supabase-js";

import { createDxSupabaseBrowserClient } from "./browser";

export type DxSupabasePasswordResetOptions = {
  email: string;
  updatePasswordPath?: string;
};

export type DxSupabasePasswordUpdateOptions = {
  password: string;
  redirectPath?: string;
};

export type DxSupabasePasswordUpdateResult = {
  user: User | null;
  redirectPath: string;
};

const DEFAULT_PASSWORD_RECOVERY_PATHS = {
  updatePasswordPath: "/auth/update-password",
  redirectPath: "/account",
};

export async function requestDxSupabasePasswordReset(
  options: DxSupabasePasswordResetOptions,
) {
  const email = assertDxSupabasePasswordRecoveryEmail(options.email);
  const supabase = createDxSupabaseBrowserClient();
  const { error } = await supabase.auth.resetPasswordForEmail(email, {
    redirectTo: buildDxSupabasePasswordResetRedirectTo(options),
  });

  if (error) {
    throw error;
  }
}

export async function updateDxSupabasePassword(
  options: DxSupabasePasswordUpdateOptions,
): Promise<DxSupabasePasswordUpdateResult> {
  const password = assertDxSupabasePassword(options.password);
  const redirectPath = assertSupabaseLocalPasswordRecoveryPath(
    options.redirectPath ?? DEFAULT_PASSWORD_RECOVERY_PATHS.redirectPath,
    "redirectPath",
  );
  const supabase = createDxSupabaseBrowserClient();
  const { data, error } = await supabase.auth.updateUser({ password });

  if (error) {
    throw error;
  }

  return {
    user: data.user ?? null,
    redirectPath,
  };
}

export function buildDxSupabasePasswordResetRedirectTo(
  options: Pick<DxSupabasePasswordResetOptions, "updatePasswordPath"> = {},
): string {
  const updatePasswordPath = assertSupabaseLocalPasswordRecoveryPath(
    options.updatePasswordPath ?? DEFAULT_PASSWORD_RECOVERY_PATHS.updatePasswordPath,
    "updatePasswordPath",
  );

  return new URL(updatePasswordPath, window.location.origin).toString();
}

function assertDxSupabasePasswordRecoveryEmail(email: string): string {
  const value = email.trim();
  if (!value || !value.includes("@")) {
    throw new Error("Supabase password reset email is required.");
  }
  return value;
}

function assertDxSupabasePassword(password: string): string {
  if (!password) {
    throw new Error("Supabase password update requires a new password.");
  }
  return password;
}

function assertSupabaseLocalPasswordRecoveryPath(
  path: string,
  field: string,
): string {
  if (!path.startsWith("/") || path.startsWith("//")) {
    throw new Error(`Supabase password recovery ${field} must be an app-local path.`);
  }

  const url = new URL(path, "http://dx.local");
  if (url.origin !== "http://dx.local") {
    throw new Error(`Supabase password recovery ${field} must be an app-local path.`);
  }

  return `${url.pathname}${url.search}${url.hash}`;
}
"#,
        ),
        (
            "js/supabase/profiles.ts",
            r#"import type { SupabaseClient, User } from "@supabase/supabase-js";

import { createDxSupabaseServerClient } from "./server";

export type DxSupabaseProfileRow = {
  id: string;
  updated_at: string | null;
  username: string | null;
  full_name: string | null;
  avatar_url: string | null;
  website: string | null;
};

export type DxSupabaseProfileInsert = {
  id: string;
  updated_at?: string | null;
  username?: string | null;
  full_name?: string | null;
  avatar_url?: string | null;
  website?: string | null;
};

export type DxSupabaseProfileUpdateRow = Partial<DxSupabaseProfileInsert>;

export type DxSupabaseProfilesDatabase = {
  public: {
    Tables: {
      profiles: {
        Row: DxSupabaseProfileRow;
        Insert: DxSupabaseProfileInsert;
        Update: DxSupabaseProfileUpdateRow;
        Relationships: [];
      };
    };
    Views: Record<string, never>;
    Functions: Record<string, never>;
    Enums: Record<string, never>;
    CompositeTypes: Record<string, never>;
  };
};

export type DxSupabaseProfile = {
  id: string;
  updatedAt: string | null;
  username: string | null;
  fullName: string | null;
  avatarUrl: string | null;
  website: string | null;
};

export type DxSupabaseProfileInput = {
  username?: string | null;
  fullName?: string | null;
  avatarUrl?: string | null;
  website?: string | null;
};

export type DxSupabaseCurrentProfile =
  | { user: User; profile: DxSupabaseProfile | null }
  | { user: null; profile: null };

const DX_SUPABASE_PROFILE_SELECT =
  "id, updated_at, username, full_name, avatar_url, website";

type DxSupabaseProfilesClient = SupabaseClient<DxSupabaseProfilesDatabase>;

export async function getDxSupabaseCurrentProfile(): Promise<DxSupabaseCurrentProfile> {
  const supabase = await createDxSupabaseServerClient<DxSupabaseProfilesDatabase>();
  const { data, error } = await supabase.auth.getUser();

  if (error) {
    throw error;
  }

  if (!data.user) {
    return { user: null, profile: null };
  }

  const profile = await readDxSupabaseProfile(supabase, data.user.id);
  return { user: data.user, profile };
}

export async function getDxSupabaseProfile(
  userId: string,
): Promise<DxSupabaseProfile | null> {
  const supabase = await createDxSupabaseServerClient<DxSupabaseProfilesDatabase>();
  return readDxSupabaseProfile(supabase, assertDxSupabaseProfileUserId(userId));
}

export async function upsertDxSupabaseProfile(
  userId: string,
  input: DxSupabaseProfileInput,
): Promise<DxSupabaseProfile> {
  const supabase = await createDxSupabaseServerClient<DxSupabaseProfilesDatabase>();
  const { data, error } = await supabase
    .from("profiles")
    .upsert(toDxSupabaseProfileUpsert(userId, input))
    .select(DX_SUPABASE_PROFILE_SELECT)
    .single();

  if (error) {
    throw error;
  }

  return toDxSupabaseProfile(data);
}

async function readDxSupabaseProfile(
  supabase: DxSupabaseProfilesClient,
  userId: string,
): Promise<DxSupabaseProfile | null> {
  const { data, error, status } = await supabase
    .from("profiles")
    .select(DX_SUPABASE_PROFILE_SELECT)
    .eq("id", userId)
    .single();

  if (error && status !== 406) {
    throw error;
  }

  return data ? toDxSupabaseProfile(data) : null;
}

function toDxSupabaseProfile(row: DxSupabaseProfileRow): DxSupabaseProfile {
  return {
    id: row.id,
    updatedAt: row.updated_at,
    username: row.username,
    fullName: row.full_name,
    avatarUrl: row.avatar_url,
    website: row.website,
  };
}

function toDxSupabaseProfileUpsert(
  userId: string,
  input: DxSupabaseProfileInput,
): DxSupabaseProfileInsert {
  const update: DxSupabaseProfileInsert = {
    id: assertDxSupabaseProfileUserId(userId),
    updated_at: new Date().toISOString(),
  };

  if ("username" in input) {
    update.username = input.username ?? null;
  }
  if ("fullName" in input) {
    update.full_name = input.fullName ?? null;
  }
  if ("avatarUrl" in input) {
    update.avatar_url = input.avatarUrl ?? null;
  }
  if ("website" in input) {
    update.website = input.website ?? null;
  }

  return update;
}

function assertDxSupabaseProfileUserId(userId: string): string {
  const value = userId.trim();
  if (!value) {
    throw new Error("Supabase profile userId is required.");
  }
  return value;
}
"#,
        ),
        (
            "js/supabase/profile-workflow.ts",
            r#"import {
  readSupabasePublicConfig,
  type DxSupabasePublicConfig,
} from "./env";
import {
  getDxSupabaseCurrentProfile,
  upsertDxSupabaseProfile,
  type DxSupabaseProfile,
  type DxSupabaseProfileInput,
} from "./profiles";

export type {
  DxSupabaseProfile,
  DxSupabaseProfileInput,
} from "./profiles";

export type DxSupabaseProfileConfigStatus =
  | {
      kind: "ready";
      config: DxSupabasePublicConfig;
      message: string;
    }
  | {
      kind: "missing-config";
      message: string;
    };

export type DxSupabaseProfileUpsertReceipt = {
  status: "ready-to-submit" | "missing-config";
  userId: string;
  operation: string;
  input: DxSupabaseProfileInput;
  boundary: string;
};

export type DxSupabaseProfileField = {
  key: Extract<keyof DxSupabaseProfileInput, "fullName" | "username" | "website">;
  label: string;
  inputType: "text" | "url";
  autoComplete: string;
};

export type DxSupabaseProfilesReadModel = {
  kind: "ready";
  table: "profiles";
  select: "id, full_name, username, website";
  operation: "supabase.from('profiles').select('id, full_name, username, website')";
  rows: readonly DxSupabaseProfile[];
  message: string;
};

export const dxSupabaseProfileApi = {
  readCurrent: getDxSupabaseCurrentProfile,
  upsert: upsertDxSupabaseProfile,
} as const;

export const dxSupabaseProfileFields = [
  {
    key: "fullName",
    label: "Full name",
    inputType: "text",
    autoComplete: "name",
  },
  {
    key: "username",
    label: "Username",
    inputType: "text",
    autoComplete: "username",
  },
  {
    key: "website",
    label: "Website",
    inputType: "url",
    autoComplete: "url",
  },
] as const satisfies readonly DxSupabaseProfileField[];

export const dxSupabaseLocalProfile: DxSupabaseProfile = {
  id: "00000000-0000-4000-8000-000000000001",
  updatedAt: null,
  username: "essencedx",
  fullName: "essencefromexistence",
  avatarUrl: null,
  website: "https://dx.local/profile",
};

export const dxSupabaseLocalProfiles: readonly DxSupabaseProfile[] = [
  dxSupabaseLocalProfile,
  {
    id: "00000000-0000-4000-8000-000000000002",
    updatedAt: null,
    username: "friday",
    fullName: "Friday",
    avatarUrl: null,
    website: "https://dx.local/friday",
  },
] as const;

export const dxSupabaseInitialProfileDraft: DxSupabaseProfileInput = {
  username: dxSupabaseLocalProfile.username,
  fullName: dxSupabaseLocalProfile.fullName,
  avatarUrl: dxSupabaseLocalProfile.avatarUrl,
  website: dxSupabaseLocalProfile.website,
};

export function readDxSupabaseProfileConfigStatus(): DxSupabaseProfileConfigStatus {
  try {
    const config = readSupabasePublicConfig();
    return {
      kind: "ready",
      config,
      message: config.isLocal
        ? "Local Supabase is configured for profile writes."
        : "Hosted Supabase public config is present for profile writes.",
    };
  } catch (error) {
    return {
      kind: "missing-config",
      message: error instanceof Error ? error.message : "Supabase config is missing.",
    };
  }
}

export function readDxSupabaseProfilesReadModel(): DxSupabaseProfilesReadModel {
  return {
    kind: "ready",
    table: "profiles",
    select: "id, full_name, username, website",
    operation: "supabase.from('profiles').select('id, full_name, username, website')",
    rows: dxSupabaseLocalProfiles,
    message:
      "Local profiles fixture is ready. Hosted Supabase queries stay behind app-owned env config.",
  };
}

export function createDxSupabaseProfilePreview(
  profile: DxSupabaseProfile,
  input: DxSupabaseProfileInput,
): DxSupabaseProfile {
  return {
    ...profile,
    username: input.username ?? null,
    fullName: input.fullName ?? null,
    avatarUrl: input.avatarUrl ?? null,
    website: input.website ?? null,
  };
}

export function updateDxSupabaseProfileDraft(
  current: DxSupabaseProfileInput,
  field: keyof DxSupabaseProfileInput,
  value: string,
): DxSupabaseProfileInput {
  return {
    ...current,
    [field]: value.trim() ? value : null,
  };
}

export function createDxSupabaseProfileUpsertReceipt(
  status: DxSupabaseProfileConfigStatus,
  profile: DxSupabaseProfile,
  input: DxSupabaseProfileInput,
): DxSupabaseProfileUpsertReceipt {
  return {
    status: status.kind === "ready" ? "ready-to-submit" : "missing-config",
    userId: profile.id,
    operation: `${dxSupabaseProfileApi.upsert.name}(userId, input)`,
    input,
    boundary:
      status.kind === "ready"
        ? "User action can submit through the app-owned Supabase project."
        : "Set NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY before live writes.",
  };
}
"#,
        ),
        (
            "js/supabase/metadata.ts",
            r#"export const dxSupabaseForgePackage = {
  packageId: "supabase/client",
  officialName: "Backend Platform Client",
  aliases: ["db/supabase", "supabase/ssr", "backend/supabase"],
  upstreamPackage: "@supabase/ssr + @supabase/supabase-js",
  upstreamVersion: "@supabase/ssr latest; @supabase/supabase-js ^2",
  sourceMirror: "G:/WWW/inspirations/supabase",
  provenance: {
    upstream: "Supabase monorepo examples and Studio API docs",
    profileSource:
      "examples/user-management/nextjs-user-management/app/account/account-form.tsx",
    databaseDocs:
      "apps/studio/components/interfaces/ProjectAPIDocs/ProjectAPIDocs.constants.ts",
  },
  upstreamPackages: [
    {
      name: "@supabase/ssr",
      version: "^0.8",
      required: true,
    },
    {
      name: "@supabase/supabase-js",
      version: "^2",
      required: true,
    },
  ],
  sourceSurface: [
    "createBrowserClient from @supabase/ssr",
    "createServerClient from @supabase/ssr",
    "SupabaseClient from @supabase/supabase-js",
    "storage.from(\"avatars\").upload, download, and getPublicUrl",
    "storage.from(bucket).createSignedUrl, createSignedUrls, createSignedUploadUrl, and uploadToSignedUrl",
    "storage.from(bucket).list, download, remove, copy, and move for object management",
    "from(table).select, eq, neq, gt, gte, lt, lte, like, ilike, is, in, contains, containedBy, or, limit, single, and maybeSingle row helpers with guarded table, column, and filter inputs",
    "rpc() for guarded Postgres function calls with get, head, and count options",
    "auth.getClaims session refresh proxy for Next proxy and middleware",
    "auth.getUser() protected-page server guard with app-local login redirects",
    "auth.signInWithPassword, auth.signUp, and auth.signOut",
    "auth.exchangeCodeForSession for OAuth and magic-link callbacks",
    "auth.verifyOtp for token-hash email confirmation and recovery links",
    "auth.signInWithOtp for passwordless Magic Link and email OTP starts",
    "auth.mfa.enroll, listFactors, challenge, verify, challengeAndVerify, unenroll, and getAuthenticatorAssuranceLevel",
    "auth.getUserIdentities, auth.linkIdentity, auth.unlinkIdentity, and auth.refreshSession for linked provider identities",
    "auth.getSession, auth.getUser, auth.getClaims, auth.refreshSession, and auth.onAuthStateChange for browser session state",
    "auth.signInAnonymously and auth.updateUser for anonymous auth upgrade flows",
    "realtime.setAuth(accessToken) for private Realtime channel authorization",
    "channel().on(\"postgres_changes\").subscribe and removeChannel for Realtime database changes",
    "channel().on(\"broadcast\"), send(), private channels, ack, self, and removeChannel for Realtime Broadcast",
    "channel().on(\"presence\"), presenceState(), track(), and untrack() for Realtime Presence",
    "functions.invoke<T>() with FunctionRegion and typed Edge Function error classes",
    "auth.signInWithOAuth for browser OAuth redirects",
    "auth.resetPasswordForEmail and auth.updateUser for password recovery",
    "profiles table helpers for auth.getUser, select, and upsert",
    "profile workflow helpers for public env readiness, local profile fixtures, typed editable field descriptors, editable drafts, profiles read-model previews, and upsert receipts",
    "profiles table RLS seed from Supabase user-management examples",
  ],
  env: [
    "NEXT_PUBLIC_SUPABASE_URL",
    "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
  ],
  requiredEnv: [
    "NEXT_PUBLIC_SUPABASE_URL",
    "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
  ],
  materializedFiles: [
    "lib/supabase/env.ts",
    "lib/supabase/browser.ts",
    "lib/supabase/avatar-storage.ts",
    "lib/supabase/signed-storage.ts",
    "lib/supabase/storage-objects.ts",
    "lib/supabase/database-rows.ts",
    "lib/supabase/rpc.ts",
    "lib/supabase/server.ts",
    "lib/supabase/auth-guard.ts",
    "lib/supabase/proxy.ts",
    "lib/supabase/auth-actions.ts",
    "lib/supabase/auth-callback.ts",
    "lib/supabase/auth-confirm.ts",
    "lib/supabase/auth-otp.ts",
    "lib/supabase/auth-mfa.ts",
    "lib/supabase/auth-identities.ts",
    "lib/supabase/auth-session.ts",
    "lib/supabase/auth-anonymous.ts",
    "lib/supabase/realtime-auth.ts",
    "lib/supabase/realtime-postgres.ts",
    "lib/supabase/realtime-broadcast.ts",
    "lib/supabase/realtime-presence.ts",
    "lib/supabase/edge-functions.ts",
    "lib/supabase/auth-oauth.ts",
    "lib/supabase/password-recovery.ts",
    "lib/supabase/profiles.ts",
    "lib/supabase/profile-workflow.ts",
    "lib/supabase/metadata.ts",
    "lib/supabase/schema.sql",
    "lib/supabase/.env.example",
  ],
  exportedFiles: [
    "lib/supabase/env.ts",
    "lib/supabase/browser.ts",
    "lib/supabase/server.ts",
    "lib/supabase/database-rows.ts",
    "lib/supabase/rpc.ts",
    "lib/supabase/auth-session.ts",
    "lib/supabase/profiles.ts",
    "lib/supabase/profile-workflow.ts",
    "lib/supabase/metadata.ts",
    "components/launch/supabase-profile-workflow.tsx",
  ],
  dashboardUsage: {
    route: "/launch",
    sourceFile: "examples/template/template-shell.tsx",
    selector: '[data-dx-section="account-data-dashboard"]',
    component: "LaunchSupabaseProfileWorkflow",
    schemaQuerySelector: '[data-dx-component="supabase-schema-query-workflow"]',
    schemaQueryWorkflow: "supabase-schema-query",
    receiptPathMarker:
      'data-dx-supabase-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"',
    interaction:
      "Editable profile draft with local fixture loading, public env readiness, safe upsert receipt, and local profiles read-model readiness.",
  },
  appOwnedBoundaries: [
    "Supabase project provisioning",
    "Auth redirect allow-list",
    "RLS policies for app tables",
    "Provider credentials and consent screens",
    "Service-role secrets and admin jobs",
  ],
  receiptPaths: [
    ".dx/forge/receipts/*-supabase-client.json",
    "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
    ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
    ".dx/forge/docs/supabase-client.md",
    "docs/packages/supabase-client.md",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
    monitoredSurfaces: [
      {
        id: "supabase-profile-workflow",
        status: "present",
        sourceFile: "examples/template/supabase-profile-workflow.tsx",
        materializedFile: "components/launch/supabase-profile-workflow.tsx",
      },
      {
        id: "supabase-schema-query-workflow",
        status: "present",
        sourceFile: "examples/template/data-status.tsx",
        materializedFile: "components/launch/data-status.tsx",
      },
    ],
    blockedBoundary:
      "Hosted Supabase reads, writes, realtime subscriptions, provider setup, and service-role operations require app-owned credentials and governed runtime proof.",
  },
  receiptIntegrity: {
    schema: "dx.forge.package.receipt_integrity",
    surface: "backend-platform-client-dashboard-workflow",
    hashAlgorithm: "sha256",
    hashScope:
      "selected Backend Platform Client dashboard files and docs; shared CLI/catalog files remain provenance-only",
    files: [
      "docs/packages/supabase-client.md",
      "core/src/ecosystem/forge_supabase.rs",
      "examples/template/supabase-profile-workflow-state.ts",
      "examples/template/supabase-profile-workflow.tsx",
      "examples/template/data-status.tsx",
    ],
    hashExclusions: [
      {
        file: "dx-www/src/cli/mod.rs",
        reason:
          "shared CLI package discovery changes for other lanes should not mark Backend Platform Client stale",
      },
      {
        file: "examples/template/package-catalog.ts",
        reason:
          "shared launch catalog changes for other lanes should not mark Backend Platform Client stale",
      },
    ],
  },
  discovery: {
    dxAdd: "dx add supabase/client --write",
    browserFactory: "createDxSupabaseBrowserClient()",
    serverFactory: "createDxSupabaseServerClient()",
    avatarUpload: "uploadDxSupabaseAvatar({ userId, file })",
    signedDownloads: "createDxSupabaseSignedDownloadUrls({ bucket: \"uploads\", paths })",
    signedUpload: "createDxSupabaseSignedUploadUrl({ bucket: \"uploads\", path })",
    storageObjects: "listDxSupabaseStorageObjects({ bucket: \"uploads\" })",
    databaseRows: "selectDxSupabaseRows({ table: \"instruments\" })",
    rowFilters: "selectDxSupabaseRows({ table: \"players\", filters: [{ column: \"age\", operator: \"gte\", value: 20 }] })",
    rpcCall: "callDxSupabaseRpc({ name: \"match_documents\", args })",
    authGuard: "requireDxSupabaseServerUser()",
    serverUser: "getDxSupabaseServerUser()",
    sessionProxy: "updateDxSupabaseSession(request)",
    signInAction: "dxSupabaseSignInWithPassword(formData)",
    callbackHandler: "handleDxSupabaseAuthCallback(request)",
    confirmHandler: "handleDxSupabaseAuthConfirm(request)",
    otpSignIn: "signInWithDxSupabaseOtp({ email })",
    mfaFactors: "listDxSupabaseMfaFactors()",
    mfaAssurance: "getDxSupabaseMfaAssuranceLevel()",
    linkedIdentities: "listDxSupabaseUserIdentities()",
    linkOAuthIdentity: "linkDxSupabaseOAuthIdentity({ provider: \"github\" })",
    unlinkIdentity: "unlinkDxSupabaseIdentityByProvider(\"github\")",
    authSnapshot: "readDxSupabaseAuthSnapshot()",
    authState: "subscribeToDxSupabaseAuthState({ onChange })",
    anonymousSignIn: "signInDxSupabaseAnonymously({ data })",
    anonymousUpgrade: "upgradeDxSupabaseAnonymousUser({ email })",
    realtimeAuth: "applyDxSupabaseRealtimeAuth()",
    privateRealtimeChannel: "buildDxSupabasePrivateRealtimeChannelConfig({ presenceKey })",
    realtimePostgres: "subscribeToDxSupabasePostgresChanges({ table: \"profiles\", onChange })",
    realtimeBroadcast: "subscribeToDxSupabaseBroadcast({ channel: \"room:launch:messages\", event: \"message_sent\", onMessage })",
    realtimePresence: "subscribeToDxSupabasePresence({ channel: \"room:launch:presence\", key: userId, initialPresence })",
    edgeFunction: "invokeDxSupabaseFunction(\"hello-world\", { body })",
    oauthSignIn: "signInWithDxSupabaseOAuth({ provider: \"github\" })",
    passwordReset: "requestDxSupabasePasswordReset({ email })",
    passwordUpdate: "updateDxSupabasePassword({ password })",
    profileHelper: "getDxSupabaseCurrentProfile()",
    profileUpsert: "upsertDxSupabaseProfile(userId, input)",
    profileWorkflowState: "readDxSupabaseProfileConfigStatus()",
    profilesReadModel: "readDxSupabaseProfilesReadModel()",
    profileWorkflowReceipt: "createDxSupabaseProfileUpsertReceipt(status, profile, input)",
    dashboardProfileWorkflow:
      "The /launch account-data-dashboard imports LaunchSupabaseProfileWorkflow for readSupabasePublicConfig(), getDxSupabaseCurrentProfile(), editable local profile drafts, and upsertDxSupabaseProfile(userId, input) receipts",
    dashboardSchemaQuery:
      'The /launch data-status dashboard exposes data-dx-dashboard-workflow="supabase-schema-query" for local profiles read-model readiness while hosted credentials stay app-owned',
  },
} as const;

export type DxSupabaseForgePackage = typeof dxSupabaseForgePackage;
"#,
        ),
        (
            "js/supabase/schema.sql",
            r#"create table if not exists public.profiles (
  id uuid references auth.users not null primary key,
  updated_at timestamp with time zone,
  username text unique,
  full_name text,
  avatar_url text,
  website text,
  constraint username_length check (char_length(username) >= 3)
);

alter table public.profiles enable row level security;

create policy "Public profiles are viewable by everyone." on public.profiles
  for select using (true);

create policy "Users can insert their own profile." on public.profiles
  for insert with check (auth.uid() = id);

create policy "Users can update own profile." on public.profiles
  for update using (auth.uid() = id);

insert into storage.buckets (id, name)
  values ('avatars', 'avatars')
  on conflict (id) do nothing;

create policy "Avatar images are publicly accessible." on storage.objects
  for select using (bucket_id = 'avatars');

create policy "Authenticated users can upload avatars." on storage.objects
  for insert to authenticated with check (bucket_id = 'avatars');

create policy "Users can update own avatar." on storage.objects
  for update using (auth.uid() = owner) with check (bucket_id = 'avatars');
"#,
        ),
        (
            "js/supabase/.env.example",
            r#"# Supabase public browser/server values required by supabase/client
NEXT_PUBLIC_SUPABASE_URL=http://127.0.0.1:54321
NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY=
"#,
        ),
        (
            "js/supabase/README.md",
            r#"# DX Forge Supabase Client

This package materializes editable Supabase SSR client source for a Next-style launch app without running package-manager lifecycle scripts or creating `node_modules`.

## Owned Surface

- `env.ts` reads and validates the public Supabase URL and publishable key.
- `browser.ts` creates a browser client with `createBrowserClient`.
- `avatar-storage.ts` uploads, downloads, and resolves profile avatar images through the `avatars` Storage bucket.
- `signed-storage.ts` creates private single and batch signed download URLs, signed upload tokens, and signed-token uploads.
- `storage-objects.ts` lists, downloads, removes, copies, and moves Storage objects with guarded bucket and path inputs.
- `database-rows.ts` wraps browser table select, insert, upsert, update, and delete calls with guarded table, column, PostgREST filter, OR expression, range, limit, and required-filter inputs.
- `rpc.ts` calls Postgres functions with guarded names and typed result/count handling.
- `server.ts` creates a cookie-aware server client with `createServerClient`.
- `auth-guard.ts` reads trusted server users with `auth.getUser()` and redirects unauthenticated requests to an app-local login path.
- `proxy.ts` refreshes Supabase SSR cookies with `auth.getClaims()` from a Next proxy or middleware entrypoint.
- `auth-actions.ts` provides small server actions for password sign in, sign up, and sign out.
- `auth-callback.ts` exchanges Supabase auth callback codes for sessions and keeps `next` redirects app-local.
- `auth-confirm.ts` verifies Supabase email `token_hash` links with `auth.verifyOtp` and keeps `next` redirects app-local.
- `auth-otp.ts` starts passwordless Magic Link and email OTP sign-ins with `auth.signInWithOtp`.
- `auth-mfa.ts` wraps Supabase Auth MFA factor listing, TOTP/phone enrollment, challenges, verification, unenrollment, and assurance-level reads.
- `auth-identities.ts` lists, links, and unlinks Supabase Auth provider identities with guarded redirect paths and a post-unlink session refresh.
- `auth-session.ts` reads browser sessions, trusted users, verified claims, refreshes sessions, and subscribes to auth-state changes with deferred callbacks.
- `auth-anonymous.ts` starts anonymous sign-ins, carries optional metadata/captcha options, and upgrades anonymous users through `auth.updateUser`.
- `realtime-auth.ts` applies an authenticated session token to Supabase Realtime with `realtime.setAuth()` and builds private-channel config.
- `realtime-postgres.ts` subscribes to Supabase Realtime Postgres Changes and returns an explicit cleanup handle.
- `realtime-broadcast.ts` sends and receives Supabase Realtime Broadcast messages on guarded channels.
- `realtime-presence.ts` tracks online user/session state with `presenceState()`, `track()`, and `untrack()`.
- `edge-functions.ts` invokes Supabase Edge Functions with typed payloads, region passthrough, and typed error classification.
- `auth-oauth.ts` starts browser OAuth redirects with real `signInWithOAuth` options and app-local callback paths.
- `password-recovery.ts` requests reset emails with `auth.resetPasswordForEmail` and updates active recovery-session passwords with `auth.updateUser`.
- `profiles.ts` reads the current user with `auth.getUser()`, loads the RLS-backed profile row, and upserts profile edits.
- `metadata.ts` gives DX CLI, Zed, and launch templates a stable discovery record.
- `schema.sql` seeds a minimal `profiles` table and `avatars` Storage bucket with Row Level Security policies.

## Template Proof

`lib/supabase/profile-workflow.ts` owns the dashboard readiness, fixture, draft merge, local `readDxSupabaseProfilesReadModel()` select preview, and upsert receipt helpers over `readSupabasePublicConfig()`, `getDxSupabaseCurrentProfile()`, and `upsertDxSupabaseProfile(userId, input)`. `components/launch/supabase-profile-workflow.tsx` consumes that package API, and `examples/template/template-shell.tsx` mounts it in `data-dx-section="account-data-dashboard"` beside a backend boundary summary, so the live dashboard can edit a local profile draft, load a safe fixture, and prepare an upsert receipt while missing public env remains honest. `examples/template/data-status.tsx` also exposes `data-dx-dashboard-workflow="supabase-schema-query"` for a safe local `profiles` read-model readiness action.

The dashboard workflow receipt classifies the slice as `REAL` for source-owned Forge package code, generated `/launch` materialization, visible account/profile and schema-query interactions, and DX Studio/Web Preview discovery. Hosted Supabase credentials, reads, writes, realtime subscriptions, RLS rollout, and governed browser QA remain app-owned partial runtime boundaries.

## DX-check Visibility

The materialized metadata and dashboard receipt expose `dx.forge.package.dx_check_visibility` with the status labels `present`, `stale`, `missing-receipt`, `blocked`, and `unsupported-surface`. `present` covers the selected profile and schema-query dashboard surfaces; `blocked` covers hosted credentials/runtime proof; `unsupported-surface` covers unselected privileged or service-role Supabase operations.

## Receipt Integrity

The dashboard workflow receipt carries `hash_algorithm: sha256` and `file_hashes` for the selected Backend Platform Client dashboard files and docs. Shared CLI and launch catalog files stay provenance-only so unrelated package-lane changes do not mark this package stale.

## App-Owned Work

Install and version `@supabase/ssr` and `@supabase/supabase-js` in the app, run the SQL in the intended Supabase project, set production redirect URLs in Supabase Auth, enable and review manual identity linking and anonymous sign-ins, configure provider consent screens, choose unlink UX, choose authorization rules, token forwarding/storage policy, local cleanup on sign-out, anonymous account retention/upgrade UX, captcha policy, MFA enrollment UX, recovery policy, phone provider/plan policy, and assurance-level enforcement, add table and bucket/object RLS policies for app-specific data paths, add `realtime.messages` RLS policies, private-channel topic policy, token-refresh resubscription policy, and channel lifetime policy for private broadcast or presence channels, and keep service-role secrets out of public env files or public-key config.

## Example

```ts
import { createDxSupabaseServerClient } from "@/lib/supabase/server";

export default async function AccountPage() {
  const supabase = await createDxSupabaseServerClient();
  const { data } = await supabase.auth.getUser();

  return <pre>{data.user?.email ?? "signed out"}</pre>;
}
```

```ts
import { requireDxSupabaseServerUser } from "@/lib/supabase/auth-guard";

const { user } = await requireDxSupabaseServerUser({ redirectTo: "/login" });
```

```ts
import { uploadDxSupabaseAvatar } from "@/lib/supabase/avatar-storage";

const { path, publicUrl } = await uploadDxSupabaseAvatar({ userId, file });
```

```ts
import {
  createDxSupabaseSignedDownloadUrls,
  createDxSupabaseSignedUploadUrl,
  uploadDxSupabaseToSignedUrl,
} from "@/lib/supabase/signed-storage";

const signedDownloads = await createDxSupabaseSignedDownloadUrls({
  bucket: "uploads",
  paths: ["private/report.pdf", "private/receipt.pdf"],
});

const upload = await createDxSupabaseSignedUploadUrl({
  bucket: "uploads",
  path: "private/report.pdf",
});
await uploadDxSupabaseToSignedUrl({ ...upload, bucket: "uploads", file });
```

```ts
import {
  copyDxSupabaseStorageObject,
  listDxSupabaseStorageObjects,
  removeDxSupabaseStorageObjects,
} from "@/lib/supabase/storage-objects";

const [firstFile] = await listDxSupabaseStorageObjects({ bucket: "uploads" });
if (firstFile) {
  await copyDxSupabaseStorageObject({
    bucket: "uploads",
    fromPath: firstFile.name,
    toPath: `archive/${firstFile.name}`,
  });
}
await removeDxSupabaseStorageObjects({
  bucket: "uploads",
  paths: ["old/report.pdf"],
});
```

```ts
import {
  insertDxSupabaseRows,
  selectSingleDxSupabaseRow,
  selectDxSupabaseRows,
  updateDxSupabaseRows,
} from "@/lib/supabase/database-rows";

const { rows: instruments } = await selectDxSupabaseRows<{
  id: number;
  name: string;
}>({
  table: "instruments",
  filters: [{ column: "status", operator: "neq", value: "archived" }],
  order: { column: "name" },
  limit: 20,
});

const launchEvent = await selectSingleDxSupabaseRow<{
  id: string;
  kind: string;
}>({
  table: "launch_events",
  filters: [{ column: "kind", value: "template_opened" }],
});

await insertDxSupabaseRows({
  table: "launch_events",
  rows: { kind: "template_opened" },
});

await updateDxSupabaseRows({
  table: "launch_events",
  values: { reviewed: true },
  filters: [{ column: "kind", value: "template_opened" }],
});
```

```ts
import { callDxSupabaseRpc } from "@/lib/supabase/rpc";

const { data, count } = await callDxSupabaseRpc<{ id: string }[]>({
  name: "match_documents",
  args: { query_embedding: embedding, match_count: 5 },
  count: "estimated",
});
```

```ts
// proxy.ts
import { type NextRequest } from "next/server";
import {
  dxSupabaseSessionProxyConfig,
  updateDxSupabaseSession,
} from "@/lib/supabase/proxy";

export async function proxy(request: NextRequest) {
  return updateDxSupabaseSession(request);
}

export const config = dxSupabaseSessionProxyConfig;
```

```ts
import { getDxSupabaseCurrentProfile } from "@/lib/supabase/profiles";

const { user, profile } = await getDxSupabaseCurrentProfile();
```

```ts
// app/auth/callback/route.ts
import { handleDxSupabaseAuthCallback } from "@/lib/supabase/auth-callback";

export const GET = handleDxSupabaseAuthCallback;
```

```ts
// app/auth/confirm/route.ts
import { handleDxSupabaseAuthConfirm } from "@/lib/supabase/auth-confirm";

export const GET = handleDxSupabaseAuthConfirm;
```

```ts
import { signInWithDxSupabaseOAuth } from "@/lib/supabase/auth-oauth";

await signInWithDxSupabaseOAuth({
  provider: "github",
  nextPath: "/account",
});
```

```ts
import {
  linkDxSupabaseOAuthIdentity,
  listDxSupabaseUserIdentities,
  unlinkDxSupabaseIdentityByProvider,
} from "@/lib/supabase/auth-identities";

const identities = await listDxSupabaseUserIdentities();

if (!identities.some((identity) => identity.provider === "github")) {
  await linkDxSupabaseOAuthIdentity({
    provider: "github",
    nextPath: "/account/security",
  });
}

await unlinkDxSupabaseIdentityByProvider("google");
```

```ts
import {
  readDxSupabaseAuthSnapshot,
  subscribeToDxSupabaseAuthState,
} from "@/lib/supabase/auth-session";

const snapshot = await readDxSupabaseAuthSnapshot();
console.log(snapshot.user?.email ?? "signed out");

const subscription = subscribeToDxSupabaseAuthState({
  onChange: ({ event, session }) => {
    console.log(event, session?.user.email);
  },
});

subscription.unsubscribe();
```

```ts
import {
  signInDxSupabaseAnonymously,
  upgradeDxSupabaseAnonymousUser,
} from "@/lib/supabase/auth-anonymous";

const anonymous = await signInDxSupabaseAnonymously({
  data: { launchId },
});

if (anonymous.isAnonymous) {
  await upgradeDxSupabaseAnonymousUser({
    email,
    emailRedirectPath: "/auth/confirm",
  });
}
```

```ts
import {
  applyDxSupabaseRealtimeAuth,
  buildDxSupabasePrivateRealtimeChannelConfig,
} from "@/lib/supabase/realtime-auth";

const { supabase } = await applyDxSupabaseRealtimeAuth();
const privateRoom = supabase.channel(
  "room:launch:messages",
  buildDxSupabasePrivateRealtimeChannelConfig({ presenceKey: userId }),
);
```

```ts
import { signInWithDxSupabaseOtp } from "@/lib/supabase/auth-otp";

await signInWithDxSupabaseOtp({
  email,
  nextPath: "/account",
  shouldCreateUser: false,
});
```

```ts
import {
  challengeAndVerifyDxSupabaseMfa,
  listDxSupabaseMfaFactors,
} from "@/lib/supabase/auth-mfa";

const factors = await listDxSupabaseMfaFactors();
const factorId = factors.totp[0]?.id;

if (factorId) {
  await challengeAndVerifyDxSupabaseMfa({ factorId, code });
}
```

```ts
import {
  requestDxSupabasePasswordReset,
  updateDxSupabasePassword,
} from "@/lib/supabase/password-recovery";

await requestDxSupabasePasswordReset({ email });
const { redirectPath } = await updateDxSupabasePassword({ password });
```

```ts
import { subscribeToDxSupabasePostgresChanges } from "@/lib/supabase/realtime-postgres";

const subscription = subscribeToDxSupabasePostgresChanges({
  table: "profiles",
  onChange: (payload) => console.log(payload.eventType),
});

await subscription.unsubscribe();
```

```ts
import { subscribeToDxSupabaseBroadcast } from "@/lib/supabase/realtime-broadcast";

const launchRoom = subscribeToDxSupabaseBroadcast<{ text: string }>({
  channel: "room:launch:messages",
  event: "message_sent",
  onMessage: ({ payload }) => console.log(payload.text),
});

await launchRoom.send({ text: "Launch ready" });
await launchRoom.unsubscribe();
```

```ts
import { subscribeToDxSupabasePresence } from "@/lib/supabase/realtime-presence";

const launchPresence = subscribeToDxSupabasePresence<{ onlineAt: string }>({
  channel: "room:launch:presence",
  key: userId,
  initialPresence: { onlineAt: new Date().toISOString() },
  onSync: (state) => console.log(Object.keys(state)),
});

await launchPresence.untrack();
await launchPresence.unsubscribe();
```

```ts
import { invokeDxSupabaseFunction } from "@/lib/supabase/edge-functions";

const data = await invokeDxSupabaseFunction<{ message: string }>(
  "hello-world",
  { body: { name: "DX" } },
);
```
"#,
        ),
    ]
}
