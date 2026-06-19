import type { ReactNode } from "react";

export type SourceSurfaceTableProps = {
  eyebrow: string;
  title: string;
  children?: ReactNode;
};

export function SourceSurfaceTable({
  eyebrow,
  title,
  children,
}: SourceSurfaceTableProps) {
  return (
    <section
      className="conversion-panel source-surface"
      data-component="source-surface-table"
      data-proof="source surface"
    >
      <div>
        <p className="eyebrow">{eyebrow}</p>
        <h2>{title}</h2>
      </div>
      <table className="source-table">{children}</table>
    </section>
  );
}
