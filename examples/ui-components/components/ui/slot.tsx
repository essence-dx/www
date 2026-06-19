import type { DxElementProps } from "./types";

export type SlotProps = DxElementProps & {
  children?: any;
};

function SlotRoot({ children, ...props }: SlotProps) {
  return (
    <span data-as-child-boundary="pending-www-slot" {...props}>
      {children}
    </span>
  );
}

export const Slot = {
  Root: SlotRoot,
};
