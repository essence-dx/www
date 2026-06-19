import { cn } from "../../lib/utils";
import { CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandSeparator } from "./command";
import type { DxElementProps } from "./types";

function Combobox({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="combobox"
      data-adapter-boundary="combobox"
      className={cn("cn-combobox", className)}
      {...props}
    />
  );
}

function ComboboxInput(props: DxElementProps) {
  return <CommandInput data-slot="combobox-input" {...props} />;
}

function ComboboxContent({ className, ...props }: DxElementProps) {
  return <div data-slot="combobox-content" className={cn("cn-combobox-content", className)} {...props} />;
}

function ComboboxList(props: DxElementProps) {
  return <CommandList data-slot="combobox-list" {...props} />;
}

function ComboboxItem(props: DxElementProps) {
  return <CommandItem data-slot="combobox-item" {...props} />;
}

function ComboboxGroup(props: DxElementProps) {
  return <CommandGroup data-slot="combobox-group" {...props} />;
}

function ComboboxLabel({ className, ...props }: DxElementProps) {
  return <div data-slot="combobox-label" className={cn("cn-combobox-label", className)} {...props} />;
}

function ComboboxCollection({ className, ...props }: DxElementProps) {
  return <div data-slot="combobox-collection" className={cn("cn-combobox-collection", className)} {...props} />;
}

function ComboboxEmpty(props: DxElementProps) {
  return <CommandEmpty data-slot="combobox-empty" {...props} />;
}

function ComboboxSeparator(props: DxElementProps) {
  return <CommandSeparator data-slot="combobox-separator" {...props} />;
}

function ComboboxChips({ className, ...props }: DxElementProps) {
  return <div data-slot="combobox-chips" className={cn("cn-combobox-chips", className)} {...props} />;
}

function ComboboxChip({ className, ...props }: DxElementProps) {
  return <span data-slot="combobox-chip" className={cn("cn-combobox-chip", className)} {...props} />;
}

function ComboboxChipsInput(props: DxElementProps) {
  return <ComboboxInput data-slot="combobox-chips-input" {...props} />;
}

function ComboboxTrigger({ className, ...props }: DxElementProps) {
  return <button type="button" data-slot="combobox-trigger" className={cn("cn-combobox-trigger", className)} {...props} />;
}

function ComboboxValue({ className, ...props }: DxElementProps) {
  return <span data-slot="combobox-value" className={cn("cn-combobox-value", className)} {...props} />;
}

function useComboboxAnchor() {
  return { ref: undefined };
}

export {
  Combobox,
  ComboboxInput,
  ComboboxContent,
  ComboboxList,
  ComboboxItem,
  ComboboxGroup,
  ComboboxLabel,
  ComboboxCollection,
  ComboboxEmpty,
  ComboboxSeparator,
  ComboboxChips,
  ComboboxChip,
  ComboboxChipsInput,
  ComboboxTrigger,
  ComboboxValue,
  useComboboxAnchor,
};
