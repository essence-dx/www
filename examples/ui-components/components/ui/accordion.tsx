import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import type { DxElementProps } from "./types";

function Accordion({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="accordion"
      className={cn("cn-accordion", className)}
      {...props}
    />
  );
}

function AccordionItem({ className, ...props }: DxElementProps) {
  return (
    <details
      data-slot="accordion-item"
      className={cn("cn-accordion-item", className)}
      {...props}
    />
  );
}

function AccordionTrigger({ children, className, ...props }: DxElementProps) {
  return (
    <summary
      data-slot="accordion-trigger"
      className={cn("cn-accordion-trigger", className)}
      {...props}
    >
      <span>{children}</span>
      <Icon name="pack:chevron-down" className="cn-slot-icon" />
    </summary>
  );
}

function AccordionContent({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="accordion-content"
      className={cn("cn-accordion-content", className)}
      {...props}
    />
  );
}

export { Accordion, AccordionItem, AccordionTrigger, AccordionContent };
