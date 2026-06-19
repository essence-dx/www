import { upstreamShadcnSource } from "./provenance";

export type ComponentCatalogItem = {
  name: string;
  packageId: string;
  href: string;
  upstreamPath: string;
  status: "source-owned" | "adapter-boundary";
  summary: string;
};

type ComponentCatalogRow = [
  name: string,
  slug: string,
  status: ComponentCatalogItem["status"],
  summary: string,
];

const registryRows: ComponentCatalogRow[] = [
  ["Accordion", "accordion", "source-owned", "Native details-based disclosure with shadcn-compatible parts."],
  ["Alert Dialog", "alert-dialog", "adapter-boundary", "Dialog semantics are represented while full focus trapping becomes a DX runtime package."],
  ["Alert", "alert", "source-owned", "Status messaging with destructive/default variants."],
  ["Aspect Ratio", "aspect-ratio", "source-owned", "Ratio container for stable media frames."],
  ["Avatar", "avatar", "source-owned", "Image, fallback, badge, and grouped avatar primitives."],
  ["Badge", "badge", "source-owned", "Compact status labels with DX Style variants."],
  ["Breadcrumb", "breadcrumb", "source-owned", "Semantic navigation trail with DX Icon separators."],
  ["Button Group", "button-group", "source-owned", "Grouped actions without class-variance or merge packages."],
  ["Button", "button", "source-owned", "Token-backed actions with source-owned variants."],
  ["Calendar", "calendar", "adapter-boundary", "Calendar shell is present while date-grid logic moves to a DX calendar package."],
  ["Card", "card", "source-owned", "Composable panel, header, content, action, and footer primitives."],
  ["Carousel", "carousel", "adapter-boundary", "Carousel parts exist without importing Embla; motion/state belongs to DX Carousel."],
  ["Chart", "chart", "adapter-boundary", "Chart slots exist without Recharts; visualization belongs to DX Charts."],
  ["Checkbox", "checkbox", "source-owned", "Native checkbox input styled through DX Style."],
  ["Collapsible", "collapsible", "source-owned", "Native details/summary disclosure for simple collapse surfaces."],
  ["Combobox", "combobox", "adapter-boundary", "Command-like parts are represented while keyboard collection logic stays DX-owned."],
  ["Command", "command", "adapter-boundary", "Command palette shell without cmdk imports."],
  ["Context Menu", "context-menu", "adapter-boundary", "Menu slots exist while pointer, roving focus, and portal behavior stay DX-owned."],
  ["Dialog", "dialog", "adapter-boundary", "Overlay/content/title slots with honest focus-trap boundary."],
  ["Direction", "direction", "source-owned", "Direction provider and hook-shaped utility for LTR/RTL surfaces."],
  ["Drawer", "drawer", "adapter-boundary", "Drawer maps to the DX dialog boundary without Vaul."],
  ["Dropdown Menu", "dropdown-menu", "adapter-boundary", "Dropdown slots exist without Radix menu imports."],
  ["Empty", "empty", "source-owned", "Empty-state layout primitives."],
  ["Field", "field", "source-owned", "Accessible form layout, labels, descriptions, and errors."],
  ["Form", "form", "adapter-boundary", "Form slots exist while react-hook-form integration is a future DX adapter."],
  ["Hover Card", "hover-card", "adapter-boundary", "Hover-card slots exist without Radix hover-card runtime."],
  ["Input Group", "input-group", "source-owned", "Input addons, buttons, text, input, and textarea composition."],
  ["Input OTP", "input-otp", "adapter-boundary", "OTP parts exist without the input-otp package."],
  ["Input", "input", "source-owned", "Text input primitive with no package runtime import."],
  ["Item", "item", "source-owned", "List row, media, content, actions, and separators."],
  ["Kbd", "kbd", "source-owned", "Keyboard shortcut primitives."],
  ["Label", "label", "source-owned", "Label primitive for form and control surfaces."],
  ["Menubar", "menubar", "adapter-boundary", "Menubar slots exist while roving keyboard logic remains DX-owned."],
  ["Native Select", "native-select", "source-owned", "Native select, optgroup, and option primitives."],
  ["Navigation Menu", "navigation-menu", "adapter-boundary", "Navigation menu slots exist without Radix navigation runtime."],
  ["Pagination", "pagination", "source-owned", "Pagination links and directional controls."],
  ["Popover", "popover", "adapter-boundary", "Popover slots exist while positioning/portal behavior stays DX-owned."],
  ["Progress", "progress", "source-owned", "Progressbar semantics with DX Style indicator."],
  ["Radio Group", "radio-group", "source-owned", "Native radio-group primitives."],
  ["Resizable", "resizable", "adapter-boundary", "Resizable panel slots exist without react-resizable-panels."],
  ["Scroll Area", "scroll-area", "source-owned", "Source-owned scroll container and scrollbar marker."],
  ["Select", "select", "adapter-boundary", "Custom select slots exist while full collection behavior stays DX-owned."],
  ["Separator", "separator", "source-owned", "Semantic and decorative visual separation."],
  ["Sheet", "sheet", "adapter-boundary", "Sheet maps to the DX dialog boundary."],
  ["Sidebar", "sidebar", "adapter-boundary", "Sidebar slots and hook-shaped API exist while responsive state wiring stays DX-owned."],
  ["Skeleton", "skeleton", "source-owned", "Loading placeholder primitive."],
  ["Slider", "slider", "source-owned", "Native range input styled as a slider."],
  ["Sonner", "sonner", "adapter-boundary", "Toaster surface exists without Sonner runtime import."],
  ["Spinner", "spinner", "source-owned", "DX Icon spinner primitive."],
  ["Switch", "switch", "source-owned", "Native switch input."],
  ["Table", "table", "source-owned", "Semantic table parts with responsive container."],
  ["Tabs", "tabs", "adapter-boundary", "Tab slots exist while active-state orchestration stays DX-owned."],
  ["Textarea", "textarea", "source-owned", "Long-form input primitive."],
  ["Toggle Group", "toggle-group", "source-owned", "Grouped toggle buttons."],
  ["Toggle", "toggle", "source-owned", "Pressed-state button primitive."],
  ["Tooltip", "tooltip", "adapter-boundary", "Tooltip slots exist while positioning/timing behavior stays DX-owned."],
];

export const registryComponents: ComponentCatalogItem[] = registryRows.map(
  ([name, slug, status, summary]) => ({
    name,
    packageId: `shadcn/ui/${slug}`,
    href: `/docs/components/primitives#${slug}`,
    upstreamPath: `apps/v4/registry/new-york-v4/ui/${slug}.tsx`,
    status,
    summary,
  }),
);

export const implementedComponents = registryComponents.filter(
  (component) => component.status === "source-owned",
);

export const adapterBoundaryComponents = registryComponents.filter(
  (component) => component.status === "adapter-boundary",
);

export const siteNavigation = [
  { label: "Components", href: "/docs/components" },
  { label: "Primitives", href: "/docs/components/primitives" },
  { label: "Registry", href: "/registry" },
] as const;

export const uiComponentsSummary = {
  represented: registryComponents.length,
  sourceOwned: implementedComponents.length,
  adapterBoundaries: adapterBoundaryComponents.length,
  upstreamRegistryComponents: upstreamShadcnSource.registryComponentCount,
  packageManager: "none",
} as const;
