import type { ReactNode } from "react";

export type ConversionRouteHeaderProps = {
  accent: string;
  logo: string;
  logoAlt: string;
  route: string;
  title: string;
  children?: ReactNode;
};

export function ConversionRouteHeader({
  accent,
  logo,
  logoAlt,
  route,
  title,
  children,
}: ConversionRouteHeaderProps) {
  return (
    <header className={`proof-header ${accent}`}>
      <div className="brand-row">
        <img src={logo} alt={logoAlt} width="32" height="32" />
        <span>DX-WWW conversion proof</span>
      </div>
      <p className="route-label">{route}</p>
      <h1>{title}</h1>
      <p className="route-summary">{children}</p>
    </header>
  );
}
