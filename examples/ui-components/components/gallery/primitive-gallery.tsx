import { implementedComponents } from "../../lib/ui-components/catalog";
import { Badge } from "../ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { PrimitivePreview } from "./primitive-preview";

type ComponentGalleryProps = {
  compact?: boolean;
};

export function ComponentGallery({ compact = false }: ComponentGalleryProps) {
  return (
    <section className="ui-section" aria-labelledby="primitive-gallery-title">
      <div className="ui-section-heading">
        <Badge variant="secondary">Source-owned surface</Badge>
        <h2 id="primitive-gallery-title">Renderable components</h2>
        <p>
          These are real project files under <code>components/ui</code>,
          rebuilt as WWW-native TSX with DX Style and DX Icon.
        </p>
      </div>
      <div className="ui-component-list" data-compact={compact}>
        {implementedComponents.map((component) => (
          <Card>
            <CardHeader>
              <CardTitle>{component.name}</CardTitle>
              <CardDescription>{component.packageId}</CardDescription>
            </CardHeader>
            <CardContent>
              <p>{component.summary}</p>
              <a className="ui-inline-link" href={component.href}>
                Open primitive
              </a>
            </CardContent>
          </Card>
        ))}
      </div>
      <PrimitivePreview />
    </section>
  );
}
