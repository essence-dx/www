pub(super) const ZOD_VALIDATION_VERSION: &str = "4.4.3-dx.13";

pub(super) fn zod_validation_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/validation/zod/schemas.ts", ZOD_SCHEMAS_TS),
        ("js/validation/zod/objects.ts", ZOD_OBJECTS_TS),
        ("js/validation/zod/parse.ts", ZOD_PARSE_TS),
        ("js/validation/zod/errors.ts", ZOD_ERRORS_TS),
        ("js/validation/zod/json-schema.ts", ZOD_JSON_SCHEMA_TS),
        (
            "js/validation/zod/json-schema-import.ts",
            ZOD_JSON_SCHEMA_IMPORT_TS,
        ),
        ("js/validation/zod/codecs.ts", ZOD_CODECS_TS),
        ("js/validation/zod/coerce.ts", ZOD_COERCE_TS),
        ("js/validation/zod/env.ts", ZOD_ENV_TS),
        ("js/validation/zod/files.ts", ZOD_FILES_TS),
        ("js/validation/zod/transforms.ts", ZOD_TRANSFORMS_TS),
        ("js/validation/zod/catalog.ts", ZOD_CATALOG_TS),
        (
            "js/validation/zod/dashboard-settings.ts",
            ZOD_DASHBOARD_SETTINGS_TS,
        ),
        ("js/validation/zod/refinements.ts", ZOD_REFINEMENTS_TS),
        ("js/validation/zod/patterns.ts", ZOD_PATTERNS_TS),
        ("js/validation/zod/registry.ts", ZOD_REGISTRY_TS),
        ("js/validation/zod/example.ts", ZOD_EXAMPLE_TS),
        ("js/validation/zod/metadata.ts", ZOD_METADATA_TS),
        ("js/validation/zod/README.md", ZOD_README_MD),
    ]
}

const ZOD_SCHEMAS_TS: &str = r#"import { z } from "zod";

export const dxLaunchEnvironmentSchema = z.object({
  NODE_ENV: z.enum(["development", "test", "production"]).default("development"),
  NEXT_PUBLIC_APP_URL: z.url().optional(),
  DX_WWW_TEMPLATE: z.string().min(1).default("dx-www"),
});

export const dxLaunchUserSchema = z.object({
  id: z.string().min(1),
  email: z.email(),
  name: z.string().trim().min(2).max(80),
  role: z.enum(["admin", "member"]).default("member"),
});

export const dxLaunchStatusSchema = z.discriminatedUnion("status", [
  z.object({
    status: z.literal("ready"),
    score: z.number().int().min(0).max(100),
    checkedAt: z.iso.datetime(),
    summary: z.string().min(1),
  }),
  z.object({
    status: z.literal("blocked"),
    reason: z.string().min(1),
    nextAction: z.string().min(1),
  }),
]);

export const dxLaunchSignupSchema = z.object({
  email: z.email(),
  name: z.string().trim().min(2).max(80),
  intent: z.enum(["builder", "designer", "operator"]).default("builder"),
});

export type DxLaunchEnvironment = z.infer<typeof dxLaunchEnvironmentSchema>;
export type DxLaunchUser = z.infer<typeof dxLaunchUserSchema>;
export type DxLaunchStatus = z.infer<typeof dxLaunchStatusSchema>;
export type DxLaunchSignup = z.infer<typeof dxLaunchSignupSchema>;
"#;

const ZOD_OBJECTS_TS: &str = r#"import { z } from "zod";

import { dxLaunchSignupSchema } from "./schemas";

export const dxLaunchSignupDraftSchema = dxLaunchSignupSchema.partial();

export const dxLaunchSignupSubmissionSchema = dxLaunchSignupDraftSchema
  .required({
    email: true,
    name: true,
  })
  .safeExtend({
    acceptedTerms: z.literal(true),
    source: z.enum(["hero", "launch", "waitlist"]).default("launch"),
  });

export const dxLaunchSignupContactSchema = dxLaunchSignupSubmissionSchema.pick({
  email: true,
  name: true,
  intent: true,
});

export const dxLaunchSignupPublicProfileSchema =
  dxLaunchSignupSubmissionSchema.omit({
    acceptedTerms: true,
  });

export type DxLaunchSignupDraft = z.output<typeof dxLaunchSignupDraftSchema>;
export type DxLaunchSignupSubmissionInput = z.input<
  typeof dxLaunchSignupSubmissionSchema
>;
export type DxLaunchSignupSubmission = z.output<
  typeof dxLaunchSignupSubmissionSchema
>;
export type DxLaunchSignupPublicProfile = z.output<
  typeof dxLaunchSignupPublicProfileSchema
>;

export function parseDxLaunchSignupDraft(
  value: unknown,
): DxLaunchSignupDraft {
  return dxLaunchSignupDraftSchema.parse(value);
}

export function safeParseDxLaunchSignupSubmission(value: unknown) {
  return dxLaunchSignupSubmissionSchema.safeParse(value);
}

export function toDxLaunchSignupPublicProfile(
  value: unknown,
): DxLaunchSignupPublicProfile {
  return dxLaunchSignupPublicProfileSchema.parse(
    dxLaunchSignupSubmissionSchema.parse(value),
  );
}
"#;

const ZOD_PARSE_TS: &str = r#"import { z } from "zod";

export type DxValidationSuccess<TSchema extends z.ZodType> = {
  success: true;
  data: z.output<TSchema>;
  issues: [];
};

export type DxValidationFailure = {
  success: false;
  data: null;
  issues: z.core.$ZodIssue[];
  fieldErrors: ReturnType<typeof z.flattenError>;
  tree: ReturnType<typeof z.treeifyError>;
  message: string;
};

export type DxValidationResult<TSchema extends z.ZodType> =
  | DxValidationSuccess<TSchema>
  | DxValidationFailure;

export function parseDxInput<TSchema extends z.ZodType>(
  schema: TSchema,
  value: unknown,
): z.output<TSchema> {
  return schema.parse(value);
}

export function validateDxInput<TSchema extends z.ZodType>(
  schema: TSchema,
  value: unknown,
): DxValidationResult<TSchema> {
  const result = schema.safeParse(value);

  if (result.success) {
    return {
      success: true,
      data: result.data,
      issues: [],
    };
  }

  return validationFailure(result.error);
}

export async function validateDxInputAsync<TSchema extends z.ZodType>(
  schema: TSchema,
  value: unknown,
): Promise<DxValidationResult<TSchema>> {
  const result = await schema.safeParseAsync(value);

  if (result.success) {
    return {
      success: true,
      data: result.data,
      issues: [],
    };
  }

  return validationFailure(result.error);
}

export function validationFailure(error: z.ZodError): DxValidationFailure {
  return {
    success: false,
    data: null,
    issues: error.issues,
    fieldErrors: z.flattenError(error),
    tree: z.treeifyError(error),
    message: z.prettifyError(error),
  };
}
"#;

const ZOD_ERRORS_TS: &str = r#"import { z } from "zod";

import { dxLaunchSignupSchema } from "./schemas";

export const dxLaunchErrorMap: z.ZodErrorMap = (issue) => {
  if (issue.code === "invalid_type") {
    return "DX launch received an invalid value";
  }

  if (issue.code === "too_small" && issue.path[0] === "name") {
    return "Launch name needs at least two characters";
  }

  return undefined;
};

export type DxZodDisplayError = {
  fieldErrors: ReturnType<typeof z.flattenError>;
  tree: ReturnType<typeof z.treeifyError>;
  message: string;
};

export function configureDxZodEnglishLocale() {
  z.config(z.locales.en());
  return z.config();
}

export function formatDxZodErrorForDisplay(error: z.ZodError): DxZodDisplayError {
  return {
    fieldErrors: z.flattenError(error),
    tree: z.treeifyError(error),
    message: z.prettifyError(error),
  };
}

export function safeParseDxLaunchSignupForDisplay(value: unknown) {
  const result = dxLaunchSignupSchema.safeParse(value, { error: dxLaunchErrorMap });

  if (result.success) {
    return {
      success: true,
      data: result.data,
      displayError: null,
    } as const;
  }

  return {
    success: false,
    data: null,
    displayError: formatDxZodErrorForDisplay(result.error),
  } as const;
}
"#;

const ZOD_JSON_SCHEMA_TS: &str = r#"import { z } from "zod";

import {
  dxLaunchEnvironmentSchema,
  dxLaunchSignupSchema,
  dxLaunchStatusSchema,
  dxLaunchUserSchema,
} from "./schemas";

export type DxJsonSchemaOptions = z.core.ToJSONSchemaParams;

export function dxToJsonSchema<TSchema extends z.ZodType>(
  schema: TSchema,
  options: DxJsonSchemaOptions = {},
) {
  return z.toJSONSchema(schema, {
    target: "draft-2020-12",
    io: "output",
    ...options,
  });
}

export const dxLaunchJsonSchemas = {
  environment: dxToJsonSchema(dxLaunchEnvironmentSchema),
  user: dxToJsonSchema(dxLaunchUserSchema),
  status: dxToJsonSchema(dxLaunchStatusSchema),
  signup: dxToJsonSchema(dxLaunchSignupSchema),
} as const;
"#;

const ZOD_JSON_SCHEMA_IMPORT_TS: &str = r#"import { z } from "zod";

export const dxLaunchExternalPackageJsonSchema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  required: ["packageId", "command"],
  additionalProperties: false,
  properties: {
    packageId: {
      type: "string",
      minLength: 1,
    },
    command: {
      type: "string",
      pattern: "^dx add .+ --write$",
    },
    requiredEnv: {
      type: "array",
      items: {
        type: "string",
      },
      default: [],
    },
  },
} as const satisfies Parameters<typeof z.fromJSONSchema>[0];

export const dxLaunchExternalPackageSchema = z.fromJSONSchema(
  dxLaunchExternalPackageJsonSchema,
  {
    defaultTarget: "draft-2020-12",
  },
);

export type DxLaunchExternalPackage = z.output<
  typeof dxLaunchExternalPackageSchema
>;

export type DxLaunchJsonSchemaInput = Parameters<typeof z.fromJSONSchema>[0];

export function parseDxLaunchExternalPackage(
  value: unknown,
): DxLaunchExternalPackage {
  return dxLaunchExternalPackageSchema.parse(value);
}

export function safeParseDxLaunchExternalPackage(value: unknown) {
  return dxLaunchExternalPackageSchema.safeParse(value);
}

export function importDxLaunchJsonSchema(schema: DxLaunchJsonSchemaInput) {
  return z.fromJSONSchema(schema, {
    defaultTarget: "draft-2020-12",
  });
}
"#;

const ZOD_CODECS_TS: &str = r#"import { z } from "zod";

export const dxIsoDateCodec = z.codec(z.iso.datetime(), z.date(), {
  decode: (value) => new Date(value),
  encode: (value) => value.toISOString(),
});

export type DxIsoDateInput = z.input<typeof dxIsoDateCodec>;
export type DxIsoDate = z.output<typeof dxIsoDateCodec>;

export function decodeDxIsoDate(value: DxIsoDateInput): DxIsoDate {
  return z.decode(dxIsoDateCodec, value);
}

export function encodeDxIsoDate(value: DxIsoDate): DxIsoDateInput {
  return z.encode(dxIsoDateCodec, value);
}

export function safeDecodeDxIsoDate(value: DxIsoDateInput) {
  return z.safeDecode(dxIsoDateCodec, value);
}

export function safeEncodeDxIsoDate(value: DxIsoDate) {
  return z.safeEncode(dxIsoDateCodec, value);
}

export async function decodeDxIsoDateAsync(value: DxIsoDateInput): Promise<DxIsoDate> {
  return z.decodeAsync(dxIsoDateCodec, value);
}

export async function encodeDxIsoDateAsync(value: DxIsoDate): Promise<DxIsoDateInput> {
  return z.encodeAsync(dxIsoDateCodec, value);
}

export async function safeDecodeDxIsoDateAsync(value: DxIsoDateInput) {
  return z.safeDecodeAsync(dxIsoDateCodec, value);
}

export async function safeEncodeDxIsoDateAsync(value: DxIsoDate) {
  return z.safeEncodeAsync(dxIsoDateCodec, value);
}
"#;

const ZOD_COERCE_TS: &str = r#"import { z } from "zod";

export const dxLaunchRawCoercionSchemas = {
  string: z.coerce.string(),
  number: z.coerce.number(),
  boolean: z.coerce.boolean(),
  bigint: z.coerce.bigint(),
  date: z.coerce.date(),
} as const;

export const dxLaunchSearchParamsSchema = z.object({
  query: z.coerce.string().trim().default(""),
  page: z.coerce.number().int().min(1).default(1),
  pageSize: z.coerce.number().int().min(1).max(50).default(12),
  checkedAt: z.coerce.date().optional(),
});

export type DxLaunchSearchParamsInput = z.input<
  typeof dxLaunchSearchParamsSchema
>;
export type DxLaunchSearchParams = z.output<typeof dxLaunchSearchParamsSchema>;

export function parseDxLaunchSearchParams(
  value: DxLaunchSearchParamsInput,
): DxLaunchSearchParams {
  return dxLaunchSearchParamsSchema.parse(value);
}

export function safeParseDxLaunchSearchParams(value: unknown) {
  return dxLaunchSearchParamsSchema.safeParse(value);
}

export function readDxLaunchSearchParamsFromUrl(
  searchParams: URLSearchParams,
): DxLaunchSearchParams {
  return parseDxLaunchSearchParams({
    query: searchParams.get("query") ?? undefined,
    page: searchParams.get("page") ?? undefined,
    pageSize: searchParams.get("pageSize") ?? undefined,
    checkedAt: searchParams.get("checkedAt") ?? undefined,
  });
}
"#;

const ZOD_ENV_TS: &str = r#"import { z } from "zod";

export const dxStringBool = z.stringbool({
  truthy: ["true", "1", "yes", "on", "y", "enabled"],
  falsy: ["false", "0", "no", "off", "n", "disabled"],
});

export const dxLaunchEnvFlagsSchema = z.object({
  DX_ENABLE_RUNTIME_PREVIEW: dxStringBool.optional(),
  DX_REQUIRE_SOURCE_RECEIPTS: dxStringBool.optional(),
});

export type DxLaunchEnvFlagsInput = z.input<typeof dxLaunchEnvFlagsSchema>;

export type DxLaunchEnvFlags = {
  runtimePreviewEnabled: boolean;
  sourceReceiptsRequired: boolean;
};

function normalizeDxLaunchEnvFlags(
  value: z.output<typeof dxLaunchEnvFlagsSchema>,
): DxLaunchEnvFlags {
  return {
    runtimePreviewEnabled: value.DX_ENABLE_RUNTIME_PREVIEW ?? false,
    sourceReceiptsRequired: value.DX_REQUIRE_SOURCE_RECEIPTS ?? true,
  };
}

export function parseDxLaunchEnvFlags(
  input: DxLaunchEnvFlagsInput = {},
): DxLaunchEnvFlags {
  return normalizeDxLaunchEnvFlags(dxLaunchEnvFlagsSchema.parse(input));
}

export function safeParseDxLaunchEnvFlags(input: unknown) {
  const result = dxLaunchEnvFlagsSchema.safeParse(input);

  if (!result.success) {
    return result;
  }

  return {
    success: true,
    data: normalizeDxLaunchEnvFlags(result.data),
  } as const;
}

export function encodeDxStringBool(value: boolean): string {
  return z.encode(dxStringBool, value);
}
"#;

const ZOD_FILES_TS: &str = r#"import { z } from "zod";

export const dxLaunchAssetMimeTypes: ["image/png", "image/jpeg", "image/webp"] = [
  "image/png",
  "image/jpeg",
  "image/webp",
];

export const dxLaunchAssetFileSchema = z
  .file()
  .min(1, "Launch assets cannot be empty")
  .max(2 * 1024 * 1024, "Launch assets must be 2 MB or smaller")
  .mime(dxLaunchAssetMimeTypes, "Launch assets must be PNG, JPEG, or WebP");

export type DxLaunchAssetFile = z.infer<typeof dxLaunchAssetFileSchema>;

export type DxLaunchAssetFileProbe =
  | {
      success: true;
      file: DxLaunchAssetFile;
    }
  | {
      success: false;
      reason: "missing-file-runtime" | "invalid-file";
      error?: z.ZodError;
    };

export const dxLaunchAssetFileJsonSchema = z.toJSONSchema(
  dxLaunchAssetFileSchema,
  {
    target: "draft-2020-12",
    io: "output",
  },
);

export function parseDxLaunchAssetFile(file: unknown): DxLaunchAssetFile {
  return dxLaunchAssetFileSchema.parse(file);
}

export function safeParseDxLaunchAssetFile(file: unknown) {
  return dxLaunchAssetFileSchema.safeParse(file);
}

export function createDxLaunchAssetFileProbe(): DxLaunchAssetFileProbe {
  if (typeof File !== "function") {
    return {
      success: false,
      reason: "missing-file-runtime",
    };
  }

  const result = safeParseDxLaunchAssetFile(
    new File(["dx-launch"], "launch-preview.png", {
      type: "image/png",
    }),
  );

  if (!result.success) {
    return {
      success: false,
      reason: "invalid-file",
      error: result.error,
    };
  }

  return {
    success: true,
    file: result.data,
  };
}
"#;

const ZOD_TRANSFORMS_TS: &str = r#"import { z } from "zod";

function normalizeDxLaunchScoreInput(value: unknown) {
  if (value === null || value === undefined) {
    return undefined;
  }

  if (typeof value === "number") {
    return String(value);
  }

  if (typeof value === "string") {
    return value.trim();
  }

  return value;
}

const dxLaunchScoreTextSchema = z
  .string()
  .transform((value) => Number(value))
  .pipe(z.number().int().min(0).max(100));

export const dxLaunchScoreInputSchema = z.preprocess(
  normalizeDxLaunchScoreInput,
  dxLaunchScoreTextSchema.prefault("0").catch(0),
);

export type DxLaunchScoreInput = z.input<typeof dxLaunchScoreInputSchema>;
export type DxLaunchScore = z.output<typeof dxLaunchScoreInputSchema>;

export function parseDxLaunchScoreInput(value: DxLaunchScoreInput): DxLaunchScore {
  return dxLaunchScoreInputSchema.parse(value);
}

export function safeParseDxLaunchScoreInput(value: unknown) {
  return dxLaunchScoreInputSchema.safeParse(value);
}
"#;

const ZOD_CATALOG_TS: &str = r#"import { z } from "zod";

export const dxLaunchPackageRoleSchema = z.enum([
  "ui-primitive",
  "selected-asset",
  "icons",
  "auth",
  "animation",
  "forms",
  "i18n",
  "server-state",
  "validation",
  "payments",
  "launch-state",
  "ai",
  "api",
  "docs",
  "content-rendering",
  "backend-client",
  "database",
  "realtime-data",
  "wasm",
  "scene",
  "migration",
]);

export const dxLaunchPackageIdSchema = z
  .string()
  .min(1)
  .regex(/^@?[a-z0-9][a-z0-9@./-]*$/);

export const dxLaunchEnvNameSchema = z
  .string()
  .regex(/^[A-Z][A-Z0-9_]*$/);

export const dxLaunchPackageCommandSchema = z.templateLiteral([
  "dx add ",
  z.string().min(1),
  " --write",
]);

export const dxLaunchPackageDxCheckStatusSchema = z.enum([
  "present",
  "stale",
  "missing receipt",
  "blocked",
  "unsupported surface",
]);

export const dxLaunchPackageDxCheckVisibilitySchema = z
  .strictObject({
    schema: z.literal("dx.forge.package.dx_check_visibility"),
    receiptPath: z.string().min(1),
    currentStatus: dxLaunchPackageDxCheckStatusSchema,
    statuses: z.array(dxLaunchPackageDxCheckStatusSchema).min(1).readonly(),
    monitoredSurfaces: z.array(z.string().min(1)).min(1).readonly(),
  })
  .readonly();

export const dxLaunchPackageCatalogItemSchema = z
  .strictObject({
    packageId: dxLaunchPackageIdSchema,
    officialName: z.string().min(1).optional(),
    aliases: z.array(dxLaunchPackageIdSchema).readonly().optional(),
    upstreamPackage: dxLaunchPackageIdSchema.optional(),
    upstreamVersion: z.string().min(1).optional(),
    role: dxLaunchPackageRoleSchema,
    command: dxLaunchPackageCommandSchema,
    env: z.array(dxLaunchEnvNameSchema).readonly(),
    exportedFiles: z.array(z.string().min(1)).readonly().optional(),
    provenance: z.string().min(1).optional(),
    receiptPaths: z.array(z.string().min(1)).readonly().optional(),
    dxCheckVisibility: dxLaunchPackageDxCheckVisibilitySchema.optional(),
    sourceMirror: z.string().min(1).optional(),
    appOwnedBoundaries: z.array(z.string().min(1)).min(1).readonly(),
  })
  .readonly();

export const dxLaunchPackageCatalogSchema = z
  .array(dxLaunchPackageCatalogItemSchema)
  .min(1)
  .readonly();

export const dxLaunchPackageRoleBucketSchema = z
  .partialRecord(
    dxLaunchPackageRoleSchema,
    z.array(dxLaunchPackageIdSchema).readonly(),
  )
  .readonly();

export const dxLaunchPackageDiscoverySchema = z
  .strictObject({
    packages: z.record(
      dxLaunchPackageIdSchema,
      dxLaunchPackageCatalogItemSchema,
    ),
    roles: dxLaunchPackageRoleBucketSchema,
    metadata: z
      .object({
        source: z.string().min(1),
      })
      .catchall(z.string().min(1))
      .readonly()
      .optional(),
  })
  .readonly();

export type DxLaunchPackageRole = z.infer<typeof dxLaunchPackageRoleSchema>;
export type DxLaunchPackageCatalogItem = z.infer<
  typeof dxLaunchPackageCatalogItemSchema
>;
export type DxLaunchPackageCatalog = z.infer<
  typeof dxLaunchPackageCatalogSchema
>;
export type DxLaunchPackageDiscovery = z.infer<
  typeof dxLaunchPackageDiscoverySchema
>;

export function parseDxLaunchPackageCatalog(
  value: unknown,
): DxLaunchPackageCatalog {
  return dxLaunchPackageCatalogSchema.parse(value);
}

export function safeParseDxLaunchPackageCatalog(value: unknown) {
  return dxLaunchPackageCatalogSchema.safeParse(value);
}

export function summarizeDxLaunchPackageCatalog(value: unknown) {
  const catalog = parseDxLaunchPackageCatalog(value);

  return {
    packageCount: catalog.length,
    roles: [...new Set(catalog.map((item) => item.role))].sort(),
    requiredEnv: [...new Set(catalog.flatMap((item) => item.env))].sort(),
  };
}

export function indexDxLaunchPackageCatalog(
  value: unknown,
): DxLaunchPackageDiscovery {
  const catalog = parseDxLaunchPackageCatalog(value);
  const packages: Record<string, DxLaunchPackageCatalogItem> = {};
  const roles: Partial<Record<DxLaunchPackageRole, string[]>> = {};

  for (const item of catalog) {
    packages[item.packageId] = item;
    roles[item.role] = [...(roles[item.role] ?? []), item.packageId];
  }

  return dxLaunchPackageDiscoverySchema.parse({
    packages,
    roles,
    metadata: {
      source: "dx-www launch package catalog",
      generatedBy: "validation/zod",
    },
  });
}
"#;

const ZOD_DASHBOARD_SETTINGS_TS: &str = r#"import { z } from "zod";

export const dxDashboardThemeSchema = z.enum(["system", "light", "dark"]);
export const dxDashboardLocaleSchema = z.enum(["en", "bn", "hi"]);
export const dxDashboardPreviewModeSchema = z.enum(["stable", "preview"]);

export const dxDashboardSettingsFormSchema = z.strictObject({
  workspaceName: z.string().trim().min(2).max(48),
  contactEmail: z.email(),
  defaultLocale: dxDashboardLocaleSchema.default("en"),
  theme: dxDashboardThemeSchema.default("system"),
  previewMode: dxDashboardPreviewModeSchema.default("stable"),
  packageReceiptsRequired: z.boolean().default(true),
  launchScoreTarget: z.coerce.number().int().min(70).max(100).default(90),
});

export const dxDashboardSettingsSchema = dxDashboardSettingsFormSchema
  .safeExtend({
    updatedAt: z.iso.datetime().optional(),
  })
  .readonly()
  .meta({
    id: "dx.dashboard.settings",
    title: "DX dashboard settings",
    description:
      "Validated launch-dashboard settings for source-owned DX-WWW templates.",
  });

export type DxDashboardSettingsFormInput = z.input<
  typeof dxDashboardSettingsFormSchema
>;
export type DxDashboardSettings = z.output<typeof dxDashboardSettingsSchema>;

export type DxDashboardSettingsIssue = {
  path: string;
  code: string;
  message: string;
};

export const dxDashboardSettingsExample = {
  workspaceName: "DX Launch",
  contactEmail: "launch@example.com",
  defaultLocale: "en",
  theme: "system",
  previewMode: "preview",
  packageReceiptsRequired: true,
  launchScoreTarget: 94,
} satisfies DxDashboardSettingsFormInput;

export const dxDashboardSettingsInvalidExample = {
  workspaceName: "D",
  contactEmail: "not-an-email",
  defaultLocale: "en",
  theme: "system",
  previewMode: "preview",
  packageReceiptsRequired: true,
  launchScoreTarget: 42,
} satisfies DxDashboardSettingsFormInput;

export function parseDxDashboardSettings(
  value: unknown,
): DxDashboardSettings {
  return dxDashboardSettingsSchema.parse(value);
}

export function safeParseDxDashboardSettingsForm(value: unknown) {
  return dxDashboardSettingsFormSchema.safeParse(value);
}

export function readDxDashboardSettingsFormData(
  formData: FormData,
): DxDashboardSettingsFormInput {
  const readString = (key: string) => {
    const value = formData.get(key);
    return typeof value === "string" ? value : undefined;
  };

  return {
    workspaceName: readString("workspaceName"),
    contactEmail: readString("contactEmail"),
    defaultLocale: readString("defaultLocale"),
    theme: readString("theme"),
    previewMode: readString("previewMode"),
    packageReceiptsRequired:
      formData.get("packageReceiptsRequired") === "on" ||
      formData.get("packageReceiptsRequired") === "true",
    launchScoreTarget: readString("launchScoreTarget"),
  };
}

export function formatDxDashboardSettingsIssues(
  value: unknown,
): DxDashboardSettingsIssue[] {
  const result = dxDashboardSettingsFormSchema.safeParse(value);

  if (result.success) {
    return [];
  }

  return result.error.issues.map((issue) => ({
    path: issue.path.join(".") || "settings",
    code: issue.code,
    message: issue.message,
  }));
}

export function createDxDashboardSettingsReceipt(value: unknown) {
  const result = dxDashboardSettingsSchema.safeParse(value);

  if (result.success) {
    return {
      status: "accepted",
      data: result.data,
      issues: [],
      fieldErrors: null,
    } as const;
  }

  return {
    status: "blocked",
    data: null,
    issues: formatDxDashboardSettingsIssues(value),
    fieldErrors: z.flattenError(result.error),
  } as const;
}
"#;

const ZOD_REFINEMENTS_TS: &str = r#"import { z } from "zod";

export const dxLaunchApprovalIdSchema = z
  .string()
  .min(1)
  .check(
    z.refine((value) => value.startsWith("dx-"), {
      error: "Launch approval ids must start with dx-",
    }),
  );

export const dxLaunchApprovalGateSchema = z
  .strictObject({
    approvalId: dxLaunchApprovalIdSchema,
    packageCount: z.number().int().min(1),
    sourceReceipts: z.number().int().min(0),
    runtimeApproved: z.boolean(),
    riskAcceptedBy: z.email().optional(),
    blockers: z.array(z.string().min(1)).default([]),
  })
  .refine((value) => value.sourceReceipts >= value.packageCount, {
    error: "Every launch package needs a source receipt",
    path: ["sourceReceipts"],
    when(payload) {
      if (payload.value === undefined || payload.value === null) {
        return false;
      }

      return payload.issues.every((issue) => {
        const field = issue.path?.[0];
        return field !== "sourceReceipts" && field !== "packageCount";
      });
    },
  })
  .superRefine((value, ctx) => {
    if (value.riskAcceptedBy && !value.runtimeApproved) {
      ctx.addIssue({
        code: "custom",
        path: ["riskAcceptedBy"],
        message: "Risk acceptance requires explicit runtime approval",
      });
    }

    if (value.runtimeApproved && value.blockers.length > 0) {
      ctx.addIssue({
        code: "custom",
        path: ["blockers"],
        message: "Approved launches cannot keep unresolved blockers",
      });
    }
  });

export type DxLaunchApprovalGateInput = z.input<
  typeof dxLaunchApprovalGateSchema
>;
export type DxLaunchApprovalGate = z.output<typeof dxLaunchApprovalGateSchema>;

export function parseDxLaunchApprovalGate(
  value: DxLaunchApprovalGateInput,
): DxLaunchApprovalGate {
  return dxLaunchApprovalGateSchema.parse(value);
}

export function safeParseDxLaunchApprovalGate(value: unknown) {
  return dxLaunchApprovalGateSchema.safeParse(value);
}

export function formatDxLaunchApprovalIssues(value: unknown) {
  const result = safeParseDxLaunchApprovalGate(value);

  if (result.success) {
    return [];
  }

  return result.error.issues.map((issue) => ({
    path: issue.path.join("."),
    message: issue.message,
  }));
}
"#;

const ZOD_PATTERNS_TS: &str = r#"import { z } from "zod";

export const dxLaunchRoutePathSchema = z.templateLiteral([
  "/",
  z.enum(["launch", "automations", "docs"]),
]);

export const dxForgeReceiptPathSchema = z.templateLiteral([
  ".dx/forge/",
  z.enum(["template-readiness", "runtime", "release"]),
  "/",
  z.string().regex(/^[a-z0-9][a-z0-9-]*$/),
  ".",
  z.enum(["json", "md"]),
]);

export type DxLaunchRoutePath = z.infer<typeof dxLaunchRoutePathSchema>;
export type DxForgeReceiptPath = z.infer<typeof dxForgeReceiptPathSchema>;

export function parseDxLaunchRoutePath(value: unknown): DxLaunchRoutePath {
  return dxLaunchRoutePathSchema.parse(value);
}

export function safeParseDxLaunchRoutePath(value: unknown) {
  return dxLaunchRoutePathSchema.safeParse(value);
}

export function parseDxForgeReceiptPath(value: unknown): DxForgeReceiptPath {
  return dxForgeReceiptPathSchema.parse(value);
}

export function safeParseDxForgeReceiptPath(value: unknown) {
  return dxForgeReceiptPathSchema.safeParse(value);
}
"#;

const ZOD_REGISTRY_TS: &str = r#"import { z } from "zod";

import {
  dxLaunchEnvironmentSchema,
  dxLaunchSignupSchema,
  dxLaunchStatusSchema,
} from "./schemas";

export type DxLaunchSchemaRole = "environment" | "signup" | "status";

export type DxLaunchSchemaMetadata = z.core.GlobalMeta & {
  id: string;
  title: string;
  description: string;
  role: DxLaunchSchemaRole;
  appOwnedBoundary: string;
  examples: readonly unknown[];
};

const environmentMetadata = {
  id: "dx.launch.environment",
  title: "DX launch environment",
  description: "Runtime environment values required by the DX launch template.",
  role: "environment",
  appOwnedBoundary: "Deployment targets, secrets, and production URL policy stay app-owned.",
  examples: [
    {
      NODE_ENV: "production",
      NEXT_PUBLIC_APP_URL: "https://dx.example.com",
      DX_WWW_TEMPLATE: "dx-www",
    },
  ],
} as const satisfies DxLaunchSchemaMetadata;

const signupMetadata = {
  id: "dx.launch.signup",
  title: "DX launch signup",
  description: "Lead-capture payload accepted by the launch template.",
  role: "signup",
  appOwnedBoundary: "Accepted intent taxonomy, rate limits, and authorization stay app-owned.",
  examples: [
    {
      email: "team@example.com",
      name: "Launch Team",
      intent: "builder",
    },
  ],
} as const satisfies DxLaunchSchemaMetadata;

const statusMetadata = {
  id: "dx.launch.status",
  title: "DX launch status",
  description: "Launch-readiness status payload for source-owned handoff surfaces.",
  role: "status",
  appOwnedBoundary: "Score policy, blocker taxonomy, and release approval stay app-owned.",
  examples: [
    {
      status: "ready",
      score: 92,
      checkedAt: "2026-05-21T00:00:00.000Z",
      summary: "Source-owned package slice is ready for template usage.",
    },
  ],
} as const satisfies DxLaunchSchemaMetadata;

export const dxLaunchSchemaRegistry = z.registry<DxLaunchSchemaMetadata>();

export const dxLaunchEnvironmentSchemaWithMetadata = dxLaunchEnvironmentSchema
  .describe(environmentMetadata.description)
  .meta(environmentMetadata)
  .register(dxLaunchSchemaRegistry, environmentMetadata);

export const dxLaunchSignupSchemaWithMetadata = dxLaunchSignupSchema
  .describe(signupMetadata.description)
  .meta(signupMetadata)
  .register(dxLaunchSchemaRegistry, signupMetadata);

export const dxLaunchStatusSchemaWithMetadata = dxLaunchStatusSchema
  .describe(statusMetadata.description)
  .meta(statusMetadata)
  .register(dxLaunchSchemaRegistry, statusMetadata);

export const dxLaunchSchemaEntries = [
  {
    key: "environment",
    schema: dxLaunchEnvironmentSchemaWithMetadata,
    metadata: environmentMetadata,
  },
  {
    key: "signup",
    schema: dxLaunchSignupSchemaWithMetadata,
    metadata: signupMetadata,
  },
  {
    key: "status",
    schema: dxLaunchStatusSchemaWithMetadata,
    metadata: statusMetadata,
  },
] as const;

export function readDxLaunchSchemaMetadata(schema: z.ZodType) {
  return dxLaunchSchemaRegistry.get(schema);
}

export function readDxGlobalSchemaMetadata(schema: z.ZodType) {
  return z.globalRegistry.get(schema);
}

export function listDxLaunchSchemaMetadata() {
  return dxLaunchSchemaEntries.map((entry) => entry.metadata);
}
"#;

const ZOD_EXAMPLE_TS: &str = r#"import { summarizeDxLaunchPackageCatalog } from "./catalog";
import { decodeDxIsoDate, encodeDxIsoDate } from "./codecs";
import { parseDxLaunchSearchParams } from "./coerce";
import {
  createDxDashboardSettingsReceipt,
  dxDashboardSettingsExample,
  safeParseDxDashboardSettingsForm,
} from "./dashboard-settings";
import { parseDxLaunchEnvFlags } from "./env";
import { safeParseDxLaunchSignupForDisplay } from "./errors";
import { createDxLaunchAssetFileProbe } from "./files";
import { dxToJsonSchema } from "./json-schema";
import { safeParseDxLaunchExternalPackage } from "./json-schema-import";
import { safeParseDxLaunchSignupSubmission } from "./objects";
import { validateDxInput } from "./parse";
import {
  parseDxLaunchRoutePath,
  safeParseDxForgeReceiptPath,
} from "./patterns";
import { safeParseDxLaunchApprovalGate } from "./refinements";
import {
  dxLaunchSignupSchemaWithMetadata,
  readDxLaunchSchemaMetadata,
} from "./registry";
import {
  dxLaunchSignupSchema,
  dxLaunchStatusSchema,
  type DxLaunchSignup,
  type DxLaunchStatus,
} from "./schemas";
import { parseDxLaunchScoreInput } from "./transforms";

export function validateLaunchSignup(input: unknown) {
  return validateDxInput(dxLaunchSignupSchema, input);
}

export function parseLaunchStatus(input: unknown): DxLaunchStatus {
  return dxLaunchStatusSchema.parse(input);
}

export function decodeLaunchCheckedAt(input: string): Date {
  return decodeDxIsoDate(input);
}

export function encodeLaunchCheckedAt(input: Date): string {
  return encodeDxIsoDate(input);
}

export function createLaunchSignupFromForm(formData: FormData): DxLaunchSignup {
  return dxLaunchSignupSchema.parse({
    email: formData.get("email"),
    name: formData.get("name"),
    intent: formData.get("intent") ?? undefined,
  });
}

export const launchSignupMetadata = readDxLaunchSchemaMetadata(
  dxLaunchSignupSchemaWithMetadata,
);

export const launchEnvFlags = parseDxLaunchEnvFlags({
  DX_ENABLE_RUNTIME_PREVIEW: "disabled",
  DX_REQUIRE_SOURCE_RECEIPTS: "enabled",
});

export const launchRoutePath = parseDxLaunchRoutePath("/");

export const launchReceiptPath = safeParseDxForgeReceiptPath(
  ".dx/forge/template-readiness/launch-route.json",
);

export const launchAssetFileProbe = createDxLaunchAssetFileProbe();

export const launchReadinessScore = parseDxLaunchScoreInput("92");

export const launchSearchParams = parseDxLaunchSearchParams({
  query: " dx launch ",
  page: "2",
  pageSize: "12",
  checkedAt: "2026-05-21T00:00:00.000Z",
});

export const launchPackageCatalogSummary = summarizeDxLaunchPackageCatalog([
  {
    packageId: "validation/zod",
    officialName: "Validation & Schemas",
    upstreamPackage: "zod",
    upstreamVersion: "4.4.3",
    role: "validation",
    command: "dx add validation-schemas --write",
    env: [],
    dxCheckVisibility: {
      schema: "dx.forge.package.dx_check_visibility",
      receiptPath:
        "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
      currentStatus: "present",
      statuses: [
        "present",
        "stale",
        "missing receipt",
        "blocked",
        "unsupported surface",
      ],
      monitoredSurfaces: [
        "dashboard-settings-validation",
        "launch-package-catalog",
        "starter-dashboard-settings-validator",
      ],
    },
    appOwnedBoundaries: ["Accepted schema design stays app-owned."],
  },
]);

export const launchApprovalGate = safeParseDxLaunchApprovalGate({
  approvalId: "dx-www-template",
  packageCount: launchPackageCatalogSummary.packageCount,
  sourceReceipts: launchPackageCatalogSummary.packageCount,
  runtimeApproved: false,
  blockers: [],
});

export const launchSignupDisplayValidation = safeParseDxLaunchSignupForDisplay({
  email: "not-an-email",
  name: "D",
});

export const launchSignupSubmissionValidation =
  safeParseDxLaunchSignupSubmission({
    email: "team@example.com",
    name: "Launch Team",
    intent: "builder",
    acceptedTerms: true,
    source: "launch",
  });

export const launchExternalPackageValidation =
  safeParseDxLaunchExternalPackage({
    packageId: "validation/zod",
    command: "dx add validation-schemas --write",
    requiredEnv: [],
  });

export const dashboardSettingsValidation =
  safeParseDxDashboardSettingsForm(dxDashboardSettingsExample);

export const dashboardSettingsReceipt =
  createDxDashboardSettingsReceipt(dxDashboardSettingsExample);

export const launchStatusJsonSchema = dxToJsonSchema(dxLaunchStatusSchema);
"#;

const ZOD_METADATA_TS: &str = r#"export const dxZodForgePackage = {
  packageId: "validation/zod",
  officialDxPackageName: "Validation & Schemas",
  upstreamPackage: "zod",
  upstreamVersion: "4.4.3",
  forgeVersion: "4.4.3-dx.13",
  aliases: ["zod", "zod/v4", "schema/zod", "validation/zod/v4"],
  sourceMirror: "G:/WWW/inspirations/zod",
  provenance:
    "Inspected the local Zod 4.4.3 source mirror, package metadata, v4 classic exports, schema parse methods, metadata APIs, and error formatters before curating this source-owned dashboard validation slice.",
  receiptPaths: [
    ".dx/forge/docs/validation-zod.md",
    ".dx/forge/receipts/*-validation-zod.json",
    ".dx/forge/docs/launch-companions/validation-status.md",
    "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
    "examples/dashboard/README.md#zod-settings-validation",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
    currentStatus: "present",
    statuses: [
      "present",
      "stale",
      "missing receipt",
      "blocked",
      "unsupported surface",
    ],
    monitoredSurfaces: [
      "dashboard-settings-validation",
      "launch-package-catalog",
      "starter-dashboard-settings-validator",
    ],
    blockedReason:
      "Runtime proof requires explicit approval before browser or build verification.",
    unsupportedSurfacePolicy:
      "Only selected Validation & Schemas dashboard, catalog, JSON Schema, codec, coerce, file, metadata, refinement, transform, parse, and error surfaces are materialized.",
  },
  requiredEnv: [],
  appOwnedBoundaries: [
    "Accepted schema design",
    "Form submit destination",
    "Persistence",
    "Authorization",
    "Locale/error copy",
    "External schema trust policy",
  ],
  publicApi: [
    "z.object",
    "z.string",
    "z.email",
    "z.enum",
    "z.discriminatedUnion",
    "z.infer",
    "schema.safeExtend",
    "object.pick",
    "object.omit",
    "object.partial",
    "object.required",
    "schema.parse",
    "schema.safeParse",
    "schema.safeParseAsync",
    "z.flattenError",
    "z.treeifyError",
    "z.prettifyError",
    "z.config",
    "z.locales",
    "schema.safeParse(errorMap)",
    "z.toJSONSchema",
    "z.fromJSONSchema",
    "z.codec",
    "z.decode",
    "z.encode",
    "z.safeDecode",
    "z.safeEncode",
    "z.decodeAsync",
    "z.encodeAsync",
    "z.safeDecodeAsync",
    "z.safeEncodeAsync",
    "z.coerce.string",
    "z.coerce.number",
    "z.coerce.boolean",
    "z.coerce.bigint",
    "z.coerce.date",
    "z.input",
    "z.registry",
    "z.globalRegistry",
    "schema.register",
    "schema.meta",
    "schema.describe",
    "z.stringbool",
    "z.templateLiteral",
    "z.file",
    "file.min",
    "file.max",
    "file.mime",
    "z.preprocess",
    "schema.transform",
    "schema.pipe",
    "schema.prefault",
    "schema.catch",
    "z.strictObject",
    "z.record",
    "z.partialRecord",
    "schema.readonly",
    "object.catchall",
    "schema.refine",
    "schema.superRefine",
    "schema.check",
    "z.refine",
    "ctx.addIssue",
    "dxDashboardSettingsSchema",
    "safeParseDxDashboardSettingsForm",
    "createDxDashboardSettingsReceipt",
  ],
  materializedFiles: [
    "lib/validation/zod/schemas.ts",
    "lib/validation/zod/objects.ts",
    "lib/validation/zod/parse.ts",
    "lib/validation/zod/errors.ts",
    "lib/validation/zod/json-schema.ts",
    "lib/validation/zod/json-schema-import.ts",
    "lib/validation/zod/codecs.ts",
    "lib/validation/zod/coerce.ts",
    "lib/validation/zod/env.ts",
    "lib/validation/zod/files.ts",
    "lib/validation/zod/transforms.ts",
    "lib/validation/zod/catalog.ts",
    "lib/validation/zod/dashboard-settings.ts",
    "lib/validation/zod/refinements.ts",
    "lib/validation/zod/patterns.ts",
    "lib/validation/zod/registry.ts",
    "lib/validation/zod/example.ts",
    "lib/validation/zod/metadata.ts",
    "lib/validation/zod/README.md",
  ],
  requiredDependencies: [
    {
      name: "zod",
      version: "^4.4.3",
      required: true,
    },
  ],
  discovery: {
    dxAdd: "dx add validation-schemas --write",
    canonicalPackage: "validation/zod",
    dashboardUsage: {
      route: "examples/dashboard/src/pages/Settings.tsx",
      component: "examples/dashboard/src/components/ZodSettingsValidator.tsx",
      localApi: "examples/dashboard/src/lib/forge/validation/zod/dashboard-settings.ts",
      markers: [
        "data-dx-package=\"validation/zod\"",
        "data-dx-component=\"dashboard-zod-settings-validator\"",
        "data-dx-dashboard-workflow=\"settings-validation\"",
      ],
    },
    launchRuntimeUsage: {
      route: "tools/launch/runtime-template/pages/index.html",
      script: "tools/launch/runtime-template/assets/launch-runtime.ts",
      component: "launch-settings-validation-summary",
      statusTarget: "mission-settings-status",
      dashboardControls: "mission-control",
      dashboardFieldset: "editable-settings",
      workflow: "settings-validation",
      form: "dashboard-settings",
      schema: "dxDashboardSettingsSchema",
      publicApi: "safeParseDxDashboardSettingsForm",
      receiptApi: "createDxDashboardSettingsReceipt",
      receiptTarget: "mission-settings-receipt-json",
      fieldErrorsApi: "z.flattenError",
      runtimeBoundary: "runtime-safe-form",
      sourceOwnedApi: "lib/validation/zod/dashboard-settings.ts",
      fieldErrorsTarget: "form-field-errors",
      settingsSummaryTarget: "form-settings-summary",
      markers: [
        "data-dx-package=\"validation/zod\"",
        "data-dx-form-package=\"forms/react-hook-form\"",
        "data-dx-dashboard-card=\"settings\"",
        "data-dx-dashboard-workflow=\"settings-validation\"",
        "data-dx-product-surface=\"account-settings\"",
        "data-dx-zod-dashboard-controls=\"mission-control\"",
        "data-dx-zod-dashboard-fieldset=\"editable-settings\"",
        "data-dx-zod-dashboard-field=\"workspaceName\"",
        "data-dx-zod-dashboard-field=\"contactEmail\"",
        "data-dx-zod-dashboard-field=\"launchScoreTarget\"",
        "data-dx-zod-dashboard-field=\"defaultLocale\"",
        "data-dx-zod-dashboard-field=\"theme\"",
        "data-dx-zod-dashboard-field=\"previewMode\"",
        "data-dx-zod-dashboard-field=\"packageReceiptsRequired\"",
        "data-dx-zod-dashboard-action=\"load-invalid-settings\"",
        "data-dx-zod-dashboard-action=\"load-valid-settings\"",
        "data-dx-zod-dashboard-receipt=\"idle\"",
        "data-dx-zod-dashboard-receipt-api=\"createDxDashboardSettingsReceipt\"",
        "data-dx-zod-dashboard-receipt-json=\"idle\"",
        "data-dx-zod-form=\"dashboard-settings\"",
        "data-dx-zod-schema=\"dxDashboardSettingsSchema\"",
        "data-dx-zod-field-errors-api=\"z.flattenError\"",
        "data-dx-zod-settings-summary=\"idle\"",
        "data-dx-zod-settings-field=\"workspaceName\"",
        "data-dx-zod-settings-field=\"contactEmail\"",
        "data-dx-zod-settings-field=\"launchScoreTarget\"",
      ],
    },
    schemaEntry: "dxLaunchStatusSchema",
    objectHelper: "safeParseDxLaunchSignupSubmission(value)",
    safeParseHelper: "validateDxInput(schema, value)",
    jsonSchemaHelper: "dxToJsonSchema(schema)",
    jsonSchemaImportHelper: "safeParseDxLaunchExternalPackage(value)",
    codecHelper: "decodeDxIsoDate(value)",
    coerceHelper: "parseDxLaunchSearchParams(value)",
    registryHelper: "readDxLaunchSchemaMetadata(schema)",
    envFlagHelper: "parseDxLaunchEnvFlags(env)",
    patternHelper: "parseDxLaunchRoutePath(value)",
    fileHelper: "safeParseDxLaunchAssetFile(file)",
    transformHelper: "parseDxLaunchScoreInput(value)",
    catalogHelper: "parseDxLaunchPackageCatalog(value)",
    dashboardSettingsHelper: "safeParseDxDashboardSettingsForm(value)",
    refinementHelper: "safeParseDxLaunchApprovalGate(value)",
    errorPolicyHelper: "safeParseDxLaunchSignupForDisplay(value)",
  },
} as const;

export type DxZodForgePackageMetadata = typeof dxZodForgePackage;
"#;

const ZOD_README_MD: &str = r#"# DX Forge Validation & Schemas Slice

This package materializes a small source-owned Validation & Schemas adapter around the real `zod` 4.4 public API. It does not reimplement Zod, vendor Zod internals, or hide validation behind fake runtime checks.

## Owned Files

- `schemas.ts` defines launch-ready environment, user, status, and signup schemas with `z.object`, `z.enum`, `z.email`, `z.discriminatedUnion`, and `z.infer`.
- `objects.ts` exposes `.partial()`, `.required()`, `.safeExtend()`, `.pick()`, and `.omit()` helpers for draft, submitted, and public launch signup shapes.
- `parse.ts` wraps `parse`, `safeParse`, `safeParseAsync`, `flattenError`, `treeifyError`, and `prettifyError` into typed helpers.
- `errors.ts` exposes `z.config`, `z.locales.en()`, typed `z.ZodErrorMap`, per-parse error maps, and display-safe error formatting helpers.
- `json-schema.ts` exposes `z.toJSONSchema` with a default draft 2020-12 output target.
- `json-schema-import.ts` exposes experimental `z.fromJSONSchema` helpers for trusted external package contracts.
- `codecs.ts` exposes a real `z.codec` ISO datetime to `Date` transform with encode/decode, safe, and async helpers.
- `coerce.ts` exposes `z.coerce.string()`, `z.coerce.number()`, `z.coerce.boolean()`, `z.coerce.bigint()`, `z.coerce.date()`, and `z.input` types for URL/search-param normalization.
- `env.ts` exposes `z.stringbool` helpers for env-style launch feature flags.
- `files.ts` exposes `z.file()` upload validation with size, MIME, JSON Schema, and a guarded runtime probe.
- `transforms.ts` exposes `z.preprocess`, `.transform()`, `.pipe()`, `.prefault()`, and `.catch()` score normalization for form-like launch inputs.
- `catalog.ts` exposes `z.strictObject`, `z.record`, `z.partialRecord`, `.catchall()`, and `.readonly()` helpers for launch package catalog validation.
- `dashboard-settings.ts` exposes a launch-dashboard settings schema, safe form parser, `z.flattenError` field-error maps, parsed settings output, and receipt helper for visible starter-dashboard settings workflows.
- `refinements.ts` exposes `.refine()`, `.superRefine()`, `.check(z.refine())`, and `ctx.addIssue()` helpers for cross-field launch approval gates.
- `patterns.ts` exposes `z.templateLiteral` route and receipt path guards for www-template links and Forge receipts.
- `registry.ts` exposes a typed `z.registry` plus global `.meta()`/`.describe()` metadata for DX CLI and Zed schema discovery.
- `example.ts` shows a tiny www-template form/status usage path.
- `metadata.ts` gives DX CLI, Zed, and launch templates a stable discovery record.

## Required App Dependency

Install or provide `zod` in the host app. Forge owns these adapter files and receipts; Zod remains the validation engine. Applications still own accepted schemas, external JSON Schema trust policy, `z.fromJSONSchema` experimental API upgrade timing, form draft policy, submit acceptance policy, metadata labels, input normalization policy, boolean coercion policy, locale selection, validation copy, global-config timing, catalog item ownership, package approval policy, launch approval authority, upload destinations, MIME policy, file retention, route naming, receipt naming, timezone/local-date policy, authorization, and persistence rules.

## Launch Dashboard Usage

- Root mission control summary: `data-dx-component="launch-settings-validation-summary"`.
- Runtime package marker: `data-dx-package="validation/zod"`.
- Runtime form package marker: `data-dx-form-package="forms/react-hook-form"`.
- Runtime dashboard card: `data-dx-dashboard-card="settings"`.
- Runtime workflow marker: `data-dx-dashboard-workflow="settings-validation"`.
- Runtime dashboard controls marker: `data-dx-zod-dashboard-controls="mission-control"`.
- Runtime editable settings fieldset: `data-dx-zod-dashboard-fieldset="editable-settings"`.
- Runtime editable fields: `workspaceName`, `contactEmail`, `defaultLocale`, `theme`, `previewMode`, `launchScoreTarget`, and `packageReceiptsRequired`.
- Runtime dashboard actions: `data-dx-zod-dashboard-action="load-invalid-settings"` and `data-dx-zod-dashboard-action="load-valid-settings"`.
- Runtime dashboard receipt marker: `data-dx-zod-dashboard-receipt="idle"`.
- Runtime dashboard receipt JSON target: `id="mission-settings-receipt-json"` with `data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"`.
- Runtime form marker: `data-dx-zod-form="dashboard-settings"`.
- Runtime schema marker: `data-dx-zod-schema="dxDashboardSettingsSchema"`.
- Runtime public API marker: `data-dx-zod-public-api="safeParseDxDashboardSettingsForm"`.
- Runtime field-error API marker: `data-dx-zod-field-errors-api="z.flattenError"`.
- Runtime settings summary marker: `data-dx-zod-settings-summary="idle"`.
- Runtime form boundary marker: `data-dx-rhf-boundary="runtime-safe-form"`.
- Runtime fields: `workspaceName`, `contactEmail`, `defaultLocale`, `theme`, `previewMode`, `launchScoreTarget`, and `packageReceiptsRequired`.
- Source files: `tools/launch/runtime-template/pages/index.html` and `tools/launch/runtime-template/assets/launch-runtime.ts`.

## dx-check Visibility

Validation & Schemas exposes `dx.forge.package.dx_check_visibility` metadata with `present`, `stale`, `missing receipt`, `blocked`, and `unsupported surface` states. The monitored surfaces are `dashboard-settings-validation`, `launch-package-catalog`, and `starter-dashboard-settings-validator`. The selected dashboard validation surface is currently `present`; `blocked` is reserved for missing governed runtime approval or app-owned authorization/persistence, and `unsupported surface` marks Zod APIs that were not selected for this source-owned slice.

## Template Usage

```ts
import { validateDxInput } from "@/lib/validation/zod/parse";
import { summarizeDxLaunchPackageCatalog } from "@/lib/validation/zod/catalog";
import { decodeDxIsoDate, encodeDxIsoDate } from "@/lib/validation/zod/codecs";
import { parseDxLaunchSearchParams } from "@/lib/validation/zod/coerce";
import {
  createDxDashboardSettingsReceipt,
  dxDashboardSettingsExample,
  safeParseDxDashboardSettingsForm,
} from "@/lib/validation/zod/dashboard-settings";
import { parseDxLaunchEnvFlags } from "@/lib/validation/zod/env";
import {
  configureDxZodEnglishLocale,
  safeParseDxLaunchSignupForDisplay,
} from "@/lib/validation/zod/errors";
import { createDxLaunchAssetFileProbe } from "@/lib/validation/zod/files";
import { safeParseDxLaunchExternalPackage } from "@/lib/validation/zod/json-schema-import";
import { safeParseDxLaunchSignupSubmission } from "@/lib/validation/zod/objects";
import { parseDxLaunchRoutePath } from "@/lib/validation/zod/patterns";
import {
  dxLaunchSignupSchemaWithMetadata,
  readDxLaunchSchemaMetadata,
} from "@/lib/validation/zod/registry";
import { safeParseDxLaunchApprovalGate } from "@/lib/validation/zod/refinements";
import { dxLaunchSignupSchema } from "@/lib/validation/zod/schemas";
import { parseDxLaunchScoreInput } from "@/lib/validation/zod/transforms";

export async function signupAction(formData: FormData) {
  const result = validateDxInput(dxLaunchSignupSchema, {
    email: formData.get("email"),
    name: formData.get("name"),
    intent: formData.get("intent") ?? undefined,
  });

  if (!result.success) {
    return { ok: false, errors: result.fieldErrors };
  }

  return { ok: true, data: result.data };
}

const checkedAt = decodeDxIsoDate("2026-05-21T00:00:00.000Z");
const wireCheckedAt = encodeDxIsoDate(checkedAt);
const searchParams = parseDxLaunchSearchParams({ page: "2", pageSize: "12" });
const submission = safeParseDxLaunchSignupSubmission({
  email: "team@example.com",
  name: "Launch Team",
  acceptedTerms: true,
});
const externalPackage = safeParseDxLaunchExternalPackage({
  packageId: "validation/zod",
  command: "dx add validation-schemas --write",
  requiredEnv: [],
});
configureDxZodEnglishLocale();
const displayValidation = safeParseDxLaunchSignupForDisplay({ email: "bad", name: "D" });
const signupMetadata = readDxLaunchSchemaMetadata(dxLaunchSignupSchemaWithMetadata);
const flags = parseDxLaunchEnvFlags({ DX_REQUIRE_SOURCE_RECEIPTS: "enabled" });
const launchRoute = parseDxLaunchRoutePath("/");
const assetProbe = createDxLaunchAssetFileProbe();
const score = parseDxLaunchScoreInput("92");
const dashboardSettings = safeParseDxDashboardSettingsForm(dxDashboardSettingsExample);
const dashboardReceipt = createDxDashboardSettingsReceipt(dxDashboardSettingsExample);
const catalogSummary = summarizeDxLaunchPackageCatalog([
  {
    packageId: "validation/zod",
    officialName: "Validation & Schemas",
    upstreamPackage: "zod",
    upstreamVersion: "4.4.3",
    role: "validation",
    command: "dx add validation-schemas --write",
    env: [],
    dxCheckVisibility: {
      schema: "dx.forge.package.dx_check_visibility",
      receiptPath:
        "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
      currentStatus: "present",
      statuses: [
        "present",
        "stale",
        "missing receipt",
        "blocked",
        "unsupported surface",
      ],
      monitoredSurfaces: [
        "dashboard-settings-validation",
        "launch-package-catalog",
        "starter-dashboard-settings-validator",
      ],
    },
    appOwnedBoundaries: ["Accepted schema design stays app-owned."],
  },
]);
const approvalGate = safeParseDxLaunchApprovalGate({
  approvalId: "dx-www-template",
  packageCount: catalogSummary.packageCount,
  sourceReceipts: catalogSummary.packageCount,
  runtimeApproved: false,
  blockers: [],
});
```
"#;
