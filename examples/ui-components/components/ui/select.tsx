import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import type { DxElementProps } from "./types";

function Select({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="select"
      data-adapter-boundary="select"
      className={cn("cn-select", className)}
      {...props}
    />
  );
}

function SelectTrigger({ children, className, ...props }: DxElementProps) {
  return (
    <button
      type="button"
      data-slot="select-trigger"
      className={cn("cn-select-trigger", className)}
      {...props}
    >
      {children}
      <Icon name="pack:chevron-down" className="cn-slot-icon" />
    </button>
  );
}

function SelectValue({ className, ...props }: DxElementProps) {
  return <span data-slot="select-value" className={cn("cn-select-value", className)} {...props} />;
}

function SelectContent({ className, ...props }: DxElementProps) {
  return <div data-slot="select-content" className={cn("cn-select-content", className)} {...props} />;
}

function SelectGroup({ className, ...props }: DxElementProps) {
  return <div role="group" data-slot="select-group" className={cn("cn-select-group", className)} {...props} />;
}

function SelectItem({ className, children, selected, ...props }: DxElementProps & { selected?: boolean }) {
  return (
    <div role="option" aria-selected={selected} data-slot="select-item" className={cn("cn-select-item", className)} {...props}>
      <Icon name={selected ? "pack:check" : "pack:blank"} className="cn-slot-icon" />
      {children}
    </div>
  );
}

function SelectLabel({ className, ...props }: DxElementProps) {
  return <div data-slot="select-label" className={cn("cn-select-label", className)} {...props} />;
}

function SelectScrollDownButton({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="select-scroll-down-button" className={cn("cn-select-scroll-button", className)} {...props}><Icon name="pack:chevron-down" className="cn-slot-icon" /></button>;
}

function SelectScrollUpButton({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="select-scroll-up-button" className={cn("cn-select-scroll-button", className)} {...props}><Icon name="pack:chevron-up" className="cn-slot-icon" /></button>;
}

function SelectSeparator({ className, ...props }: DxElementProps) {
  return <div role="separator" data-slot="select-separator" className={cn("cn-select-separator", className)} {...props} />;
}

export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectScrollDownButton,
  SelectScrollUpButton,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
};
