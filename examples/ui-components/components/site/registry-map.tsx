import {
  registryComponents,
} from "../../lib/ui-components/catalog";
import { upstreamShadcnSource } from "../../lib/ui-components/provenance";
import { Badge } from "../ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";

export function RegistryMap() {
  return (
    <section className="ui-section" aria-labelledby="registry-map-title">
      <div className="ui-section-heading">
        <Badge variant="outline">Provenance</Badge>
        <h2 id="registry-map-title">Registry map</h2>
        <p>
          Upstream shadcn-ui source is mirrored for evidence. DX owns the
          rendered code, package receipts, and runtime boundaries.
        </p>
      </div>
      <Card>
        <CardHeader>
          <CardTitle>{upstreamShadcnSource.repository}</CardTitle>
          <CardDescription>{upstreamShadcnSource.commit}</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="ui-table" role="table" aria-label="UI Components registry map">
            <div role="row">
              <strong role="columnheader">Component</strong>
              <strong role="columnheader">Status</strong>
              <strong role="columnheader">Upstream path</strong>
            </div>
            {registryComponents.map((component) => (
              <div role="row">
                <span role="cell">{component.name}</span>
                <span role="cell">{component.status}</span>
                <span role="cell">{component.upstreamPath}</span>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </section>
  );
}
