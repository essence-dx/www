import type {
  DxStyleDriftFixtureManifest,
  DxStyleDriftFixtureReadinessReceipt,
} from "./preview-style-package-panel-read-model.ts";

export type TemplateShellEvidenceSources = {
  readonly previewManifest: DxStyleDriftFixtureManifest;
  readonly readinessReceipt: DxStyleDriftFixtureReadinessReceipt;
};

export const TEMPLATE_SHELL_EVIDENCE_LOADER_FILE =
  "components/template-app/template-shell-evidence-loader.ts";

export function createTemplateShellEvidenceSources(
  previewManifest: DxStyleDriftFixtureManifest = {},
  readinessReceipt: DxStyleDriftFixtureReadinessReceipt = {},
): TemplateShellEvidenceSources {
  return {
    previewManifest,
    readinessReceipt,
  };
}
