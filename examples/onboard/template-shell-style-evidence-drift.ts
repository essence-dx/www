import {
  dxStylePackagePanelFromPreviewAndReadiness,
  type DxStylePreviewPackagePanelWithDrift,
} from "./preview-style-package-panel-read-model.ts";
import type { TemplateShellEvidenceSources } from "./template-shell-evidence-loader.ts";

export const TEMPLATE_SHELL_STYLE_EVIDENCE_DRIFT_HELPER_FILE =
  "components/template-app/template-shell-style-evidence-drift.ts";

export function createTemplateShellStyleEvidenceDrift(
  sources: TemplateShellEvidenceSources,
  route = "/",
): DxStylePreviewPackagePanelWithDrift {
  return dxStylePackagePanelFromPreviewAndReadiness(
    sources.previewManifest,
    sources.readinessReceipt,
    route,
  );
}
