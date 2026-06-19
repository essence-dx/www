import {
  adapterBoundaryComponents,
  implementedComponents,
  uiComponentsSummary,
} from "../../lib/ui-components/catalog";
import { Badge } from "../ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";

export function ComponentsIndex() {
  return (
    <section className="ui-section" aria-labelledby="components-index-title">
      <div className="ui-section-heading">
        <Badge variant="outline">Docs</Badge>
        <h1 id="components-index-title">Components</h1>
        <p>
          DX WWW now represents all {uiComponentsSummary.represented} components
          from the {uiComponentsSummary.upstreamRegistryComponents} component
          upstream registry. Source-owned primitives render now; adapter
          boundaries name the behavior engines DX will own instead of importing
          npm runtime packages.
        </p>
      </div>
      <Card>
        <CardHeader>
          <CardTitle>Registry coverage</CardTitle>
          <CardDescription>
            {uiComponentsSummary.sourceOwned} source-owned components and{" "}
            {uiComponentsSummary.adapterBoundaries} adapter boundaries.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <p>
            Every registry file has a WWW component file under{" "}
            <code>components/ui</code>. The hard runtime pieces are explicit,
            not hidden behind fake package compatibility.
          </p>
        </CardContent>
      </Card>
      <div className="ui-component-list">
        {implementedComponents.map((component) => (
          <Card>
            <CardHeader>
              <CardTitle>{component.name}</CardTitle>
              <CardDescription>{component.packageId}</CardDescription>
            </CardHeader>
            <CardContent>
              <p>{component.summary}</p>
              <a className="ui-inline-link" href={component.href}>
                View component
              </a>
            </CardContent>
          </Card>
        ))}
      </div>
      <div id="adapter-boundaries" className="ui-component-list">
        {adapterBoundaryComponents.map((component) => (
          <Card>
            <CardHeader>
              <CardTitle>{component.name}</CardTitle>
              <CardDescription>{component.status}</CardDescription>
            </CardHeader>
            <CardContent>
              <p>{component.summary}</p>
            </CardContent>
          </Card>
        ))}
      </div>
    </section>
  );
}
