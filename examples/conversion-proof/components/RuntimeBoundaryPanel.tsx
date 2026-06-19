import type { ReactNode } from "react";

export type RuntimeBoundaryPanelProps = {
  primitiveTitle: string;
  primitiveSummary: string;
  boundaryTitle: string;
  children?: ReactNode;
};

export function RuntimeBoundaryPanel({
  primitiveTitle,
  primitiveSummary,
  boundaryTitle,
  children,
}: RuntimeBoundaryPanelProps) {
  return (
    <section
      className="runtime-boundary"
      data-component="runtime-boundary-panel"
      data-proof="runtime boundary"
    >
      <div>
        <p className="eyebrow">Forge primitives</p>
        <h2>{primitiveTitle}</h2>
        <p>{primitiveSummary}</p>
      </div>
      <div>
        <p className="eyebrow">Launch shim</p>
        <h2>{boundaryTitle}</h2>
        <p>{children}</p>
      </div>
    </section>
  );
}
