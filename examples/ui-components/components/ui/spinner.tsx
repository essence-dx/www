import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import type { DxElementProps } from "./types";

function Spinner({ className, ...props }: DxElementProps) {
  return (
    <Icon
      name="pack:loader"
      data-slot="spinner"
      className={cn("cn-spinner", className)}
      {...props}
    />
  );
}

export { Spinner };
