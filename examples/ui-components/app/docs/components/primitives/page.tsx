import { ComponentGallery } from "../../../../components/gallery/primitive-gallery";
import { SiteShell } from "../../../../components/site/site-shell";

export const metadata = {
  title: "Primitives - UI Components for DX WWW",
  description: "The first source-owned UI primitives in the DX WWW component system.",
} as const;

export default function PrimitivesPage() {
  return (
    <SiteShell active="/docs/components/primitives">
      <div data-dx-route="/docs/components/primitives">
        <ComponentGallery />
      </div>
    </SiteShell>
  );
}
