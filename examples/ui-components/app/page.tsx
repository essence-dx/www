import { UiComponentsHome } from "../components/home/ui-components-home";
import { SiteShell } from "../components/site/site-shell";

export const metadata = {
  title: "UI Components for DX WWW",
  description: "Source-owned Forge UI Components for the DX ecosystem.",
} as const;

export default function UiComponentsPage() {
  return (
    <SiteShell active="/">
      <UiComponentsHome />
    </SiteShell>
  );
}
