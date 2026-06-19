import { cn } from "../../lib/utils";
import { Icon } from "../icons/icon";
import type { DxElementProps } from "./types";

function Breadcrumb({ className, ...props }: DxElementProps) {
  return (
    <nav
      aria-label="breadcrumb"
      data-slot="breadcrumb"
      className={cn("cn-breadcrumb", className)}
      {...props}
    />
  );
}

function BreadcrumbList({ className, ...props }: DxElementProps) {
  return (
    <ol
      data-slot="breadcrumb-list"
      className={cn("cn-breadcrumb-list", className)}
      {...props}
    />
  );
}

function BreadcrumbItem({ className, ...props }: DxElementProps) {
  return (
    <li
      data-slot="breadcrumb-item"
      className={cn("cn-breadcrumb-item", className)}
      {...props}
    />
  );
}

function BreadcrumbLink({ className, ...props }: DxElementProps) {
  return (
    <a
      data-slot="breadcrumb-link"
      className={cn("cn-breadcrumb-link", className)}
      {...props}
    />
  );
}

function BreadcrumbPage({ className, ...props }: DxElementProps) {
  return (
    <span
      role="link"
      aria-disabled="true"
      aria-current="page"
      data-slot="breadcrumb-page"
      className={cn("cn-breadcrumb-page", className)}
      {...props}
    />
  );
}

function BreadcrumbSeparator({ children, className, ...props }: DxElementProps) {
  return (
    <li
      role="presentation"
      aria-hidden="true"
      data-slot="breadcrumb-separator"
      className={cn("cn-breadcrumb-separator", className)}
      {...props}
    >
      {children || <Icon name="pack:chevron-right" className="cn-slot-icon" />}
    </li>
  );
}

function BreadcrumbEllipsis({ className, ...props }: DxElementProps) {
  return (
    <span
      role="presentation"
      aria-hidden="true"
      data-slot="breadcrumb-ellipsis"
      className={cn("cn-breadcrumb-ellipsis", className)}
      {...props}
    >
      <Icon name="pack:more-horizontal" className="cn-slot-icon" />
    </span>
  );
}

export {
  Breadcrumb,
  BreadcrumbList,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbPage,
  BreadcrumbSeparator,
  BreadcrumbEllipsis,
};
