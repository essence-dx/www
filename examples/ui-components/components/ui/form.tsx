import { cn } from "../../lib/utils";
import { Label } from "./label";
import { Slot } from "./slot";
import type { DxElementProps } from "./types";

function useFormField() {
  return {
    id: undefined,
    name: undefined,
    formItemId: undefined,
    formDescriptionId: undefined,
    formMessageId: undefined,
    error: undefined,
  };
}

function Form({ className, ...props }: DxElementProps) {
  return <form data-slot="form" className={cn("cn-form", className)} {...props} />;
}

function FormItem({ className, ...props }: DxElementProps) {
  return <div data-slot="form-item" className={cn("cn-form-item", className)} {...props} />;
}

function FormLabel({ className, ...props }: DxElementProps) {
  return <Label data-slot="form-label" className={cn("cn-form-label", className)} {...props} />;
}

function FormControl(props: DxElementProps) {
  return <Slot.Root data-slot="form-control" {...props} />;
}

function FormDescription({ className, ...props }: DxElementProps) {
  return <p data-slot="form-description" className={cn("cn-form-description", className)} {...props} />;
}

function FormMessage({ className, ...props }: DxElementProps) {
  return <p role="alert" data-slot="form-message" className={cn("cn-form-message", className)} {...props} />;
}

function FormField({ children, render, ...props }: DxElementProps & { render?: (context: unknown) => any }) {
  const content = render ? render({ field: {}, fieldState: {}, formState: {} }) : children;

  return (
    <div data-slot="form-field" {...props}>
      {content}
    </div>
  );
}

export {
  useFormField,
  Form,
  FormItem,
  FormLabel,
  FormControl,
  FormDescription,
  FormMessage,
  FormField,
};
