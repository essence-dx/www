import { SiteShell } from "../../components/site/site-shell";
import { DependencyReplacement } from "../../components/site/dependency-replacement";
import { RegistryMap } from "../../components/site/registry-map";

export const metadata = {
  title: "Registry - UI Components for DX WWW",
  description: "Provenance, package replacements, and runtime boundaries for UI Components.",
} as const;

export default function RegistryPage() {
  return (
    <SiteShell active="/registry">
      <div data-dx-route="/registry">
        <RegistryMap />
        <DependencyReplacement />
      </div>
    </SiteShell>
  );
}
