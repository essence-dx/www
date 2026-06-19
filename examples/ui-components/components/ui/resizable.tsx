import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import type { DxElementProps } from "./types";

function ResizablePanelGroup({
  className,
  direction = "horizontal",
  ...props
}: DxElementProps & { direction?: "horizontal" | "vertical" }) {
  return (
    <div
      data-slot="resizable-panel-group"
      data-direction={direction}
      data-adapter-boundary="resizable"
      className={cn("cn-resizable-panel-group", className)}
      {...props}
    />
  );
}

function ResizablePanel({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="resizable-panel"
      className={cn("cn-resizable-panel", className)}
      {...props}
    />
  );
}

function ResizableHandle({ className, withHandle = false, ...props }: DxElementProps & { withHandle?: boolean }) {
  return (
    <div
      role="separator"
      data-slot="resizable-handle"
      data-handle={withHandle}
      className={cn("cn-resizable-handle", className)}
      {...props}
    >
      {withHandle ? <Icon name="pack:grip-vertical" className="cn-slot-icon" /> : null}
    </div>
  );
}

export { ResizableHandle, ResizablePanel, ResizablePanelGroup };
