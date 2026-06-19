import { Popover, PopoverContent, PopoverTrigger } from "./popover";
import type { DxElementProps } from "./types";

function HoverCard(props: DxElementProps) {
  return <Popover data-slot="hover-card" data-adapter-boundary="hover-card" {...props} />;
}

function HoverCardTrigger(props: DxElementProps) {
  return <PopoverTrigger data-slot="hover-card-trigger" {...props} />;
}

function HoverCardContent(props: DxElementProps) {
  return <PopoverContent data-slot="hover-card-content" {...props} />;
}

export { HoverCard, HoverCardTrigger, HoverCardContent };
