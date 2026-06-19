export function startCase(value: string): string {
  return value
    .replace(/[-_]+/g, " ")
    .replace(/([a-z])([A-Z])/g, "$1 $2")
    .replace(/\s+/g, " ")
    .trim()
    .replace(/(^|\s)\S/g, (part) => part.toUpperCase());
}

export default {
  startCase,
};
