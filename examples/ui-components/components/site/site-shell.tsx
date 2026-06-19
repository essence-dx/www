import { SiteHeader } from "./site-header";

type SiteShellProps = {
  active?: string;
  children: any;
};

export function SiteShell({ active, children }: SiteShellProps) {
  return (
    <main className="ui-site-shell" data-dx-project="ui-components">
      <SiteHeader active={active} />
      {children}
    </main>
  );
}
