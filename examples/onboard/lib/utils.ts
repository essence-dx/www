export type ClassValue =
  | string
  | number
  | null
  | false
  | undefined
  | ClassValue[]
  | Record<string, boolean | null | undefined>;

function collectClassNames(input: ClassValue, output: string[]): void {
  if (!input) return;
  if (typeof input === "string" || typeof input === "number") {
    output.push(String(input));
    return;
  }
  if (Array.isArray(input)) {
    for (const item of input) collectClassNames(item, output);
    return;
  }
  for (const [className, enabled] of Object.entries(input)) {
    if (enabled) output.push(className);
  }
}

export function cn(...inputs: ClassValue[]): string {
  const classNames: string[] = [];
  for (const input of inputs) collectClassNames(input, classNames);
  return classNames.join(" ");
}
