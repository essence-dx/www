import * as React from "react";

export type SlotProps = React.HTMLAttributes<HTMLElement> & {
  children?: React.ReactNode;
};

const SlotRoot = React.forwardRef<HTMLElement, SlotProps>(
  ({ children, ...props }, ref) => {
    if (React.isValidElement(children)) {
      return React.cloneElement(children, {
        ...props,
        ref,
        className: [props.className, (children.props as { className?: string }).className]
          .filter(Boolean)
          .join(" "),
      } as React.HTMLAttributes<HTMLElement>);
    }

    return null;
  },
);

SlotRoot.displayName = "Slot.Root";

export const Slot = {
  Root: SlotRoot,
};
