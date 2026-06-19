import type { ReactNode } from "react";

export type ProofCardProps = {
  eyebrow: string;
  title: string;
  children?: ReactNode;
};

export function ProofCard({ eyebrow, title, children }: ProofCardProps) {
  return (
    <article className="rounded-lg border border-neutral-200 bg-white p-4 shadow-sm">
      <p className="text-xs font-semibold uppercase tracking-wide text-neutral-500">
        {eyebrow}
      </p>
      <h2 className="mt-2 text-lg font-semibold text-neutral-950">{title}</h2>
      <div className="mt-3 text-sm text-neutral-700">{children}</div>
    </article>
  );
}
