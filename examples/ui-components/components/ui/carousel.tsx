import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import { Button } from "./button";
import type { DxElementProps } from "./types";

export type CarouselApi = unknown;

function Carousel({
  className,
  orientation = "horizontal",
  ...props
}: DxElementProps & { orientation?: "horizontal" | "vertical" }) {
  return (
    <div
      data-slot="carousel"
      data-orientation={orientation}
      data-adapter-boundary="carousel"
      className={cn("cn-carousel", className)}
      {...props}
    />
  );
}

function CarouselContent({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="carousel-content"
      className={cn("cn-carousel-content", className)}
      {...props}
    />
  );
}

function CarouselItem({ className, ...props }: DxElementProps) {
  return (
    <div
      role="group"
      aria-roledescription="slide"
      data-slot="carousel-item"
      className={cn("cn-carousel-item", className)}
      {...props}
    />
  );
}

function CarouselPrevious({ className, ...props }: DxElementProps) {
  return (
    <Button
      type="button"
      aria-label="Previous slide"
      data-slot="carousel-previous"
      size="icon"
      variant="outline"
      className={cn("cn-carousel-button", className)}
      {...props}
    >
      <Icon name="pack:arrow-left" className="cn-slot-icon" />
    </Button>
  );
}

function CarouselNext({ className, ...props }: DxElementProps) {
  return (
    <Button
      type="button"
      aria-label="Next slide"
      data-slot="carousel-next"
      size="icon"
      variant="outline"
      className={cn("cn-carousel-button", className)}
      {...props}
    >
      <Icon name="pack:arrow-right" className="cn-slot-icon" />
    </Button>
  );
}

export { Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext };
