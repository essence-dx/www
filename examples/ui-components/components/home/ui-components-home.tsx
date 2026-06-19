import { Badge } from "../ui/badge";
import { buttonVariants } from "../ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { Separator } from "../ui/separator";
import { ComponentGallery } from "../gallery/primitive-gallery";
import { DependencyReplacement } from "../site/dependency-replacement";
import { RegistryMap } from "../site/registry-map";
import { uiComponentsSummary } from "../../lib/ui-components/catalog";

export function UiComponentsHome() {
  return (
    <div className="ui-page-stack" data-dx-source="shadcn-ui">
      <section className="ui-hero" aria-labelledby="ui-hero-title">
        <div className="ui-hero-copy">
          <Badge variant="outline">Source-owned registry</Badge>
          <h1 id="ui-hero-title">UI Components for DX WWW</h1>
          <p>
            A professional Forge UI component system rebuilt inside the DX
            ecosystem: editable source, DX Style tokens, DX Icon, Forge
            receipts, and no package-manager runtime path.
          </p>
          <div className="ui-hero-actions">
            <a className={buttonVariants()} href="/docs/components">
              Explore components
            </a>
            <a
              className={buttonVariants({ variant: "outline" })}
              href="/docs/components/primitives"
            >
              View primitives
            </a>
          </div>
        </div>
        <Card className="ui-hero-card">
          <CardHeader>
            <CardTitle>Registry coverage</CardTitle>
            <CardDescription>
              The shadcn surface is represented without npm runtime imports.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <dl className="ui-stat-grid">
              <div>
                <dt>Represented</dt>
                <dd>{uiComponentsSummary.represented}</dd>
              </div>
              <div>
                <dt>Source-owned</dt>
                <dd>{uiComponentsSummary.sourceOwned}</dd>
              </div>
              <div>
                <dt>Boundaries</dt>
                <dd>{uiComponentsSummary.adapterBoundaries}</dd>
              </div>
            </dl>
          </CardContent>
        </Card>
      </section>
      <Separator />
      <ComponentGallery compact />
      <RegistryMap />
      <DependencyReplacement />
    </div>
  );
}
