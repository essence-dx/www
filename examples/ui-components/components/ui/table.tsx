import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Table({ className, ...props }: DxElementProps) {
  return (
    <div data-slot="records-shell" className={cn("ui-data-table-container", className)}>
      <table data-slot="records" className="ui-data-table" {...props} />
    </div>
  );
}

function TableHeader({ className, ...props }: DxElementProps) {
  return <thead data-slot="records-header" className={className} {...props} />;
}

function TableBody({ className, ...props }: DxElementProps) {
  return <tbody data-slot="records-body" className={className} {...props} />;
}

function TableFooter({ className, ...props }: DxElementProps) {
  return <tfoot data-slot="records-footer" className={className} {...props} />;
}

function TableRow({ className, ...props }: DxElementProps) {
  return (
    <tr data-slot="records-row" className={cn("ui-data-table-row", className)} {...props} />
  );
}

function TableHead({ className, ...props }: DxElementProps) {
  return (
    <th data-slot="records-heading-cell" className={cn("ui-data-table-head", className)} {...props} />
  );
}

function TableCell({ className, ...props }: DxElementProps) {
  return (
    <td data-slot="records-cell" className={cn("ui-data-table-cell", className)} {...props} />
  );
}

function TableCaption({ className, ...props }: DxElementProps) {
  return (
    <caption
      data-slot="records-caption"
      className={cn("ui-data-table-caption", className)}
      {...props}
    />
  );
}

export {
  Table,
  TableHeader,
  TableBody,
  TableFooter,
  TableHead,
  TableRow,
  TableCell,
  TableCaption,
};
