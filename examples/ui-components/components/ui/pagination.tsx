import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import { buttonVariants } from "./button";
import type { DxElementProps } from "./types";

function Pagination({ className, ...props }: DxElementProps) {
  return (
    <nav
      role="navigation"
      aria-label="pagination"
      data-slot="pagination"
      className={cn("cn-pagination", className)}
      {...props}
    />
  );
}

function PaginationContent({ className, ...props }: DxElementProps) {
  return (
    <ul
      data-slot="pagination-content"
      className={cn("cn-pagination-content", className)}
      {...props}
    />
  );
}

function PaginationItem({ className, ...props }: DxElementProps) {
  return (
    <li
      data-slot="pagination-item"
      className={cn("cn-pagination-item", className)}
      {...props}
    />
  );
}

function PaginationLink({
  className,
  isActive = false,
  size = "icon",
  ...props
}: DxElementProps & { isActive?: boolean; size?: "default" | "sm" | "lg" | "icon" }) {
  return (
    <a
      aria-current={isActive ? "page" : undefined}
      data-slot="pagination-link"
      data-active={isActive}
      className={cn(
        buttonVariants({ variant: isActive ? "outline" : "ghost", size }),
        className,
      )}
      {...props}
    />
  );
}

function PaginationPrevious({ className, ...props }: DxElementProps) {
  return (
    <PaginationLink
      aria-label="Go to previous page"
      size="default"
      className={cn("cn-pagination-previous", className)}
      {...props}
    >
      <Icon name="pack:chevron-left" className="cn-slot-icon" />
      <span>Previous</span>
    </PaginationLink>
  );
}

function PaginationNext({ className, ...props }: DxElementProps) {
  return (
    <PaginationLink
      aria-label="Go to next page"
      size="default"
      className={cn("cn-pagination-next", className)}
      {...props}
    >
      <span>Next</span>
      <Icon name="pack:chevron-right" className="cn-slot-icon" />
    </PaginationLink>
  );
}

function PaginationEllipsis({ className, ...props }: DxElementProps) {
  return (
    <span
      aria-hidden="true"
      data-slot="pagination-ellipsis"
      className={cn("cn-pagination-ellipsis", className)}
      {...props}
    >
      <Icon name="pack:more-horizontal" className="cn-slot-icon" />
    </span>
  );
}

export {
  Pagination,
  PaginationContent,
  PaginationLink,
  PaginationItem,
  PaginationPrevious,
  PaginationNext,
  PaginationEllipsis,
};
