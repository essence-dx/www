import {
  dxWwwFrameworkCompletenessScore,
  dxWwwFrameworkCompletenessSummary,
} from "./framework-completeness";

export function templateSurfaceRegistrySummary() {
  const frameworkCompleteness = dxWwwFrameworkCompletenessSummary();

  return {
    schema: "dx.www.template_surface_registry",
    frameworkCompleteness,
    frameworkCompletenessScore: dxWwwFrameworkCompletenessScore(),
    frameworkCompletenessSchema: frameworkCompleteness.schema,
  } as const;
}
