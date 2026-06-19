import { ComponentsIndex } from "../../../components/docs/components-index";
import { SiteShell } from "../../../components/site/site-shell";

export const metadata = {
  title: "Components - UI Components for DX WWW",
  description: "Source-owned component docs for the DX WWW UI Components lane.",
} as const;

export default function ComponentsPage() {
  return (
    <SiteShell active="/docs/components">
      <div data-dx-route="/docs/components">
        <ComponentsIndex />
      </div>
    </SiteShell>
  );
}
