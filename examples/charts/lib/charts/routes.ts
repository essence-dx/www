export type ChartRouteId = "overview" | "gallery" | "examples" | "docs" | "theme" | "ecosystem" | "playground";

export type ChartRouteIcon = "activity" | "bar" | "layers" | "book" | "palette" | "network" | "terminal";

export interface ChartRoute {
  id: ChartRouteId;
  href: string;
  label: string;
  icon: ChartRouteIcon;
}

export const chartRoutes: ChartRoute[] = [
  { id: "overview", href: "/", label: "Overview", icon: "activity" },
  { id: "gallery", href: "/charts", label: "Gallery", icon: "bar" },
  { id: "examples", href: "/examples", label: "Examples", icon: "layers" },
  { id: "docs", href: "/docs", label: "Docs", icon: "book" },
  { id: "theme", href: "/theme", label: "Theme", icon: "palette" },
  { id: "ecosystem", href: "/ecosystem", label: "Ecosystem", icon: "network" },
  { id: "playground", href: "/playground", label: "Playground", icon: "terminal" },
];

export const chartRuntimePath = "/chart-runtime.js";
export const chartFaviconPath = "/favicon.svg";
export const chartTouchIconPath = "/icon.svg";
export const chartHomeHref = "/";
export const chartGalleryHref = "/charts";
export const chartDocsHref = "/docs";

export function taskHref(taskId: string): string {
  return `/charts#task-${taskId}`;
}

export function taskAnchor(taskId: string): string {
  return `#task-${taskId}`;
}
