export type ClassInput =
  | string
  | number
  | false
  | null
  | undefined
  | Record<string, boolean | null | undefined>;

export function cn(...inputs: ClassInput[]): string {
  const classes: string[] = [];

  for (const input of inputs) {
    if (!input) continue;

    if (typeof input === "string" || typeof input === "number") {
      classes.push(String(input));
      continue;
    }

    for (const [className, enabled] of Object.entries(input)) {
      if (enabled) classes.push(className);
    }
  }

  return [...new Set(classes.flatMap((item) => item.split(/\s+/)).filter(Boolean))].join(" ");
}
