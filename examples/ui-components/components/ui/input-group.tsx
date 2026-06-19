import { cn } from "../../lib/utils";
import { Button, type ButtonProps } from "./button";
import { Input } from "./input";
import { Textarea } from "./textarea";
import type { DxElementProps, DxInputProps, DxTextareaProps } from "./types";

function InputGroup({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="input-group"
      className={cn("cn-input-group", className)}
      {...props}
    />
  );
}

function InputGroupAddon({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="input-group-addon"
      className={cn("cn-input-group-addon", className)}
      {...props}
    />
  );
}

function InputGroupButton({ className, ...props }: ButtonProps) {
  return (
    <Button
      data-slot="input-group-button"
      className={cn("cn-input-group-button", className)}
      {...props}
    />
  );
}

function InputGroupText({ className, ...props }: DxElementProps) {
  return (
    <span
      data-slot="input-group-text"
      className={cn("cn-input-group-text", className)}
      {...props}
    />
  );
}

function InputGroupInput({ className, ...props }: DxInputProps) {
  return (
    <Input
      data-slot="input-group-input"
      className={cn("cn-input-group-input", className)}
      {...props}
    />
  );
}

function InputGroupTextarea({ className, ...props }: DxTextareaProps) {
  return (
    <Textarea
      data-slot="input-group-textarea"
      className={cn("cn-input-group-textarea", className)}
      {...props}
    />
  );
}

export {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupText,
  InputGroupInput,
  InputGroupTextarea,
};
