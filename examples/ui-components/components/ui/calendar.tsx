import { cn } from "../../lib/utils";
import { Button, type ButtonProps } from "./button";
import type { DxElementProps } from "./types";

function Calendar({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="calendar"
      data-adapter-boundary="calendar"
      className={cn("cn-calendar", className)}
      {...props}
    />
  );
}

function CalendarDayButton({ className, selected, ...props }: ButtonProps & { selected?: boolean }) {
  return (
    <Button
      data-slot="calendar-day-button"
      data-selected={selected}
      variant={selected ? "default" : "ghost"}
      size="icon-sm"
      className={cn("cn-calendar-day-button", className)}
      {...props}
    />
  );
}

export { Calendar, CalendarDayButton };
