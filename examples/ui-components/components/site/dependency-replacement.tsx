import { dependencyReplacements } from "../../lib/ui-components/provenance";
import { Badge } from "../ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";

export function DependencyReplacement() {
  return (
    <section className="ui-section" aria-labelledby="dependency-map-title">
      <div className="ui-section-heading">
        <Badge variant="secondary">DX ecosystem</Badge>
        <h2 id="dependency-map-title">Package replacement map</h2>
        <p>
          The website does not load upstream npm packages. Each dependency
          becomes a DX source package, an implemented primitive, or an honest
          adapter boundary.
        </p>
      </div>
      <div className="ui-component-list">
        {dependencyReplacements.map((dependency) => (
          <Card>
            <CardHeader>
              <CardTitle>{dependency.upstreamPackage}</CardTitle>
              <CardDescription>{dependency.status}</CardDescription>
            </CardHeader>
            <CardContent>
              <p>{dependency.dxReplacement}</p>
            </CardContent>
          </Card>
        ))}
      </div>
    </section>
  );
}
