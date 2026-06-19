export function classes(
  ...values: Array<string | false | null | undefined>
): string {
  return values.filter(Boolean).join(" ");
}

export const dxClass = classes;
export const cn = classes;
