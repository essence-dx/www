import { Icon } from "../icons/icon";
import { Badge } from "../ui/badge";
import { buttonVariants } from "../ui/button";
import { siteNavigation } from "../../lib/ui-components/catalog";

type SiteHeaderProps = {
  active?: string;
};

export function SiteHeader({ active = "/" }: SiteHeaderProps) {
  return (
    <header className="ui-site-header">
      <a className="ui-brand" href="/" aria-label="UI Components home">
        <img src="/logo.svg" alt="" className="ui-brand-mark" />
        <span>
          <strong>UI Components</strong>
          <small>DX WWW</small>
        </span>
      </a>
      <nav className="ui-nav" aria-label="UI Components">
        {siteNavigation.map((item) => (
          <a
            className="ui-nav-link"
            data-active={active === item.href}
            href={item.href}
          >
            {item.label}
          </a>
        ))}
      </nav>
      <div className="ui-header-actions">
        <Badge variant="outline">No npm</Badge>
        <a className={buttonVariants({ variant: "secondary", size: "sm" })} href="/registry">
          <Icon name="pack:check" className="ui-button-icon" />
          Receipts
        </a>
      </div>
    </header>
  );
}
