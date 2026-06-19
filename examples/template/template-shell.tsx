import { templateSurfaceRegistrySummary } from "./template-surface-registry";

export function TemplateShell() {
  const summary = templateSurfaceRegistrySummary();

  return (
    <section
      data-dx-component="dx-www-framework-completeness"
      data-dx-framework-completeness-score={summary.frameworkCompletenessScore}
      data-dx-framework-completeness-schema={summary.frameworkCompletenessSchema}
    >
      <h2>WWW framework completeness</h2>
      <p>{summary.frameworkCompletenessScore}/100</p>
    </section>
  );
}
