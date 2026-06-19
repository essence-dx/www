import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Kbd({ className, ...props }: DxElementProps) {
  return <kbd data-slot="kbd" className={cn("cn-kbd", className)} {...props} />;
}

function KbdGroup({ className, ...props }: DxElementProps) {
  return (
    <span
      data-slot="kbd-group"
      className={cn("cn-kbd-group", className)}
      {...props}
    />
  );
}

export { Kbd, KbdGroup };
