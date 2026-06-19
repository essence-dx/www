export type InspectorSwatch = {
  readonly label: string;
  readonly value: string;
};

export const STROKE_SWATCHES: readonly InspectorSwatch[] = [
  { label: "Ink", value: "hsl(var(--wb-swatch-ink))" },
  { label: "Graphite", value: "hsl(var(--wb-swatch-graphite))" },
  { label: "Sky", value: "hsl(var(--wb-swatch-sky))" },
  { label: "Violet", value: "hsl(var(--wb-swatch-violet))" },
  { label: "Amber", value: "hsl(var(--wb-swatch-amber))" },
  { label: "Mint", value: "hsl(var(--wb-swatch-mint))" },
  { label: "Rose", value: "hsl(var(--wb-swatch-rose))" },
] as const;

export const FILL_SWATCHES: readonly InspectorSwatch[] = [
  { label: "Transparent", value: "transparent" },
  { label: "Ink", value: "hsl(var(--wb-swatch-ink))" },
  { label: "Graphite", value: "hsl(var(--surface-raised))" },
  { label: "Sky", value: "hsl(var(--info) / 0.2)" },
  { label: "Violet", value: "hsl(var(--wb-swatch-violet) / 0.22)" },
  { label: "Amber", value: "hsl(var(--warning) / 0.22)" },
  { label: "Mint", value: "hsl(var(--success) / 0.22)" },
  { label: "Rose", value: "hsl(var(--danger) / 0.22)" },
] as const;
