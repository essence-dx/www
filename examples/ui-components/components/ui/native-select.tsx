import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function NativeSelect({ className, ...props }: DxElementProps) {
  return (
    <select
      data-slot="native-select"
      className={cn("cn-native-select", className)}
      {...props}
    />
  );
}

function NativeSelectOptGroup({ className, ...props }: DxElementProps) {
  return (
    <optgroup
      data-slot="native-select-optgroup"
      className={cn("cn-native-select-optgroup", className)}
      {...props}
    />
  );
}

function NativeSelectOption({ className, ...props }: DxElementProps) {
  return (
    <option
      data-slot="native-select-option"
      className={cn("cn-native-select-option", className)}
      {...props}
    />
  );
}

export { NativeSelect, NativeSelectOptGroup, NativeSelectOption };
