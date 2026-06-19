"use client";

import * as React from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { summarizeDxLaunchPackageCatalog } from "@/lib/validation/zod/catalog";
import {
  decodeDxIsoDate,
  safeEncodeDxIsoDate,
} from "@/lib/validation/zod/codecs";
import { parseDxLaunchSearchParams } from "@/lib/validation/zod/coerce";
import {
  encodeDxStringBool,
  parseDxLaunchEnvFlags,
} from "@/lib/validation/zod/env";
import { safeParseDxLaunchSignupForDisplay } from "@/lib/validation/zod/errors";
import { createDxLaunchAssetFileProbe } from "@/lib/validation/zod/files";
import { dxToJsonSchema } from "@/lib/validation/zod/json-schema";
import { safeParseDxLaunchExternalPackage } from "@/lib/validation/zod/json-schema-import";
import { safeParseDxLaunchSignupSubmission } from "@/lib/validation/zod/objects";
import { validateDxInput } from "@/lib/validation/zod/parse";
import {
  parseDxLaunchRoutePath,
  safeParseDxForgeReceiptPath,
} from "@/lib/validation/zod/patterns";
import {
  dxLaunchSignupSchemaWithMetadata,
  readDxGlobalSchemaMetadata,
  readDxLaunchSchemaMetadata,
} from "@/lib/validation/zod/registry";
import {
  formatDxLaunchApprovalIssues,
  safeParseDxLaunchApprovalGate,
} from "@/lib/validation/zod/refinements";
import { dxLaunchSignupSchema } from "@/lib/validation/zod/schemas";
import { parseDxLaunchScoreInput } from "@/lib/validation/zod/transforms";
import { launchPackageCatalog } from "./package-catalog";

const launchSignupSample = {
  email: "team@example.com",
  name: "Launch Team",
  intent: "builder",
};

const invalidZodValidationInput = {
  email: "not-an-email",
  name: "D",
  intent: "builder",
};

const launchCheckedAt = "2026-05-21T00:00:00.000Z";
const launchEnvFlagInput = {
  DX_ENABLE_RUNTIME_PREVIEW: "disabled",
  DX_REQUIRE_SOURCE_RECEIPTS: "enabled",
};
const launchReceiptPath = ".dx/forge/template-readiness/launch-route.json";
const launchSearchParamInput = {
  query: " dx launch ",
  page: "2",
  pageSize: "12",
  checkedAt: launchCheckedAt,
};

type JsonSchemaSummary = {
  properties?: Record<string, unknown>;
  required?: unknown[];
};

export function LaunchZodValidationStatus() {
  const [validationInput, setValidationInput] = React.useState(invalidZodValidationInput);
  const validationResult = safeParseDxLaunchSignupForDisplay(validationInput);
  const validationState = validationResult.success ? "valid" : "invalid";
  const validationMessage = validationResult.success
    ? `Accepted ${validationResult.data.email} as ${validationResult.data.intent}`
    : validationResult.displayError.message;
  const result = validateDxInput(
    dxLaunchSignupSchemaWithMetadata,
    launchSignupSample,
  );
  const schema = dxToJsonSchema(
    dxLaunchSignupSchemaWithMetadata,
  ) as JsonSchemaSummary;
  const baseSchema = dxToJsonSchema(dxLaunchSignupSchema) as JsonSchemaSummary;
  const propertyCount = Object.keys(
    schema.properties ?? baseSchema.properties ?? {},
  ).length;
  const requiredCount =
    schema.required?.length ?? baseSchema.required?.length ?? 0;
  const checkedAt = decodeDxIsoDate(launchCheckedAt);
  const encodedCheckedAt = safeEncodeDxIsoDate(checkedAt);
  const registryMetadata = readDxLaunchSchemaMetadata(
    dxLaunchSignupSchemaWithMetadata,
  );
  const globalMetadata = readDxGlobalSchemaMetadata(
    dxLaunchSignupSchemaWithMetadata,
  );
  const envFlags = parseDxLaunchEnvFlags(launchEnvFlagInput);
  const runtimePreviewFlag = encodeDxStringBool(
    envFlags.runtimePreviewEnabled,
  );
  const launchRoutePath = parseDxLaunchRoutePath("/");
  const receiptPath = safeParseDxForgeReceiptPath(launchReceiptPath);
  const assetFileProbe = createDxLaunchAssetFileProbe();
  const normalizedScore = parseDxLaunchScoreInput("92");
  const catalogSummary = summarizeDxLaunchPackageCatalog(launchPackageCatalog);
  const approvalGateInput = {
    approvalId: "dx-www-template",
    packageCount: catalogSummary.packageCount,
    sourceReceipts: catalogSummary.packageCount,
    runtimeApproved: false,
    blockers: [],
  };
  const approvalGate = safeParseDxLaunchApprovalGate(approvalGateInput);
  const approvalIssues = formatDxLaunchApprovalIssues(approvalGateInput);
  const displayValidation = safeParseDxLaunchSignupForDisplay({
    email: "bad",
    name: "D",
  });
  const coercedSearchParams = parseDxLaunchSearchParams(
    launchSearchParamInput,
  );
  const submissionValidation = safeParseDxLaunchSignupSubmission({
    email: launchSignupSample.email,
    name: launchSignupSample.name,
    intent: launchSignupSample.intent,
    acceptedTerms: true,
    source: "launch",
  });
  const externalPackageValidation = safeParseDxLaunchExternalPackage({
    packageId: "validation/zod",
    command: "dx add validation-schemas --write",
    requiredEnv: [],
  });

  return (
    <div
      className="grid gap-3 text-sm text-muted-foreground"
      data-dx-component="zod-validation-readiness"
      data-dx-package="validation/zod"
      data-dx-zod-codec-status={
        encodedCheckedAt.success ? "round-trip" : "blocked"
      }
      data-dx-zod-coerce-page={coercedSearchParams.page}
      data-dx-zod-catalog-packages={catalogSummary.packageCount}
      data-dx-zod-validation-result={validationMessage}
      data-dx-zod-validation-state={validationState}
      data-dx-zod-env-status={
        envFlags.sourceReceiptsRequired
          ? "source-receipts-required"
          : "source-receipts-optional"
      }
      data-dx-zod-error-policy={
        displayValidation.success ? "display-valid" : "display-error"
      }
      data-dx-zod-file-status={
        assetFileProbe.success ? "asset-file" : assetFileProbe.reason
      }
      data-dx-zod-json-schema-loader-status={
        externalPackageValidation.success
          ? "external-contract-valid"
          : "external-contract-blocked"
      }
      data-dx-zod-object-status={
        submissionValidation.success ? "submission-ready" : "submission-blocked"
      }
      data-dx-zod-pattern-status={
        receiptPath.success ? "route-pattern" : "blocked"
      }
      data-dx-zod-refinement-status={
        approvalGate.success ? "approval-gate-valid" : "approval-gate-blocked"
      }
      data-dx-zod-registry-status={
        registryMetadata && globalMetadata ? "registered" : "missing"
      }
      data-dx-zod-status={result.success ? "valid" : "invalid"}
      data-dx-zod-transform-score={normalizedScore}
    >
      <p>
        {globalMetadata?.title ??
          registryMetadata?.title ??
          "Zod signup schema"}{" "}
        {result.success ? "validated" : "blocked"} with {propertyCount} fields,{" "}
        {requiredCount} required outputs, and{" "}
        {encodedCheckedAt.success ? encodedCheckedAt.data : "blocked codec"}{" "}
        codec output. Runtime preview flag: {runtimePreviewFlag}. Route{" "}
        {launchRoutePath}. Asset upload{" "}
        {assetFileProbe.success ? "guarded" : "awaiting File runtime"}. Score{" "}
        {normalizedScore}. Catalog roles {catalogSummary.roles.length},
        packages {catalogSummary.packageCount}. Approval gate{" "}
        {approvalGate.success
          ? "valid"
          : approvalIssues[0]?.message ?? "blocked"}
        . Error policy{" "}
        {displayValidation.success
          ? "clear"
          : displayValidation.displayError.message}
        . Search page {coercedSearchParams.page} of size{" "}
        {coercedSearchParams.pageSize}. Submission{" "}
        {submissionValidation.success ? "ready" : "blocked"}. External package
        contract {externalPackageValidation.success ? "valid" : "blocked"}.
      </p>

      <div
        className="grid gap-3 rounded-md border border-border bg-card p-3"
        data-dx-zod-validation-email={validationInput.email}
      >
        <div className="flex flex-wrap items-center justify-between gap-2">
          <span className="font-medium text-foreground">
            Local validation workflow
          </span>
          <span data-dx-zod-validation-state={validationState}>{validationState}</span>
        </div>

        <div className="grid gap-3 md:grid-cols-[1fr_1fr_12rem]">
          <label className="grid gap-1 text-xs">
            <span>Email</span>
            <Input
              aria-invalid={validationState === "invalid"}
              data-dx-zod-validation-input="email"
              onChange={(event) =>
                setValidationInput((current) => ({
                  ...current,
                  email: event.currentTarget.value,
                }))
              }
              value={validationInput.email}
            />
          </label>
          <label className="grid gap-1 text-xs">
            <span>Name</span>
            <Input
              aria-invalid={validationState === "invalid"}
              data-dx-zod-validation-input="name"
              onChange={(event) =>
                setValidationInput((current) => ({
                  ...current,
                  name: event.currentTarget.value,
                }))
              }
              value={validationInput.name}
            />
          </label>
          <label className="grid gap-1 text-xs">
            <span>Intent</span>
            <select
              className="h-9 rounded-md border border-border bg-background px-3 text-sm text-foreground"
              data-dx-zod-validation-input="intent"
              onChange={(event) =>
                setValidationInput((current) => ({
                  ...current,
                  intent: event.currentTarget.value,
                }))
              }
              value={validationInput.intent}
            >
              <option value="builder">Builder</option>
              <option value="designer">Designer</option>
              <option value="operator">Operator</option>
            </select>
          </label>
        </div>

        <p
          data-dx-zod-validation-error={
            validationState === "invalid" ? validationMessage : ""
          }
          data-dx-zod-validation-result={validationMessage}
        >
          {validationMessage}
        </p>
        <div className="flex flex-wrap gap-2">
          <Button
            type="button"
            variant="outline"
            data-dx-zod-validation-action="load-invalid"
            onClick={() => setValidationInput(invalidZodValidationInput)}
          >
            Show error
          </Button>
          <Button
            type="button"
            variant="secondary"
            data-dx-zod-validation-action="load-valid"
            onClick={() => setValidationInput(launchSignupSample)}
          >
            Validate sample
          </Button>
        </div>
      </div>
    </div>
  );
}
