import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "./dialog";
import type { DxElementProps } from "./types";

function Sheet(props: DxElementProps) {
  return <Dialog data-slot="sheet" data-adapter-boundary="sheet" {...props} />;
}

function SheetTrigger(props: DxElementProps) {
  return <DialogTrigger data-slot="sheet-trigger" {...props} />;
}

function SheetClose(props: DxElementProps) {
  return <DialogClose data-slot="sheet-close" {...props} />;
}

function SheetContent({ side = "right", ...props }: DxElementProps & { side?: "top" | "right" | "bottom" | "left" }) {
  return <DialogContent data-slot="sheet-content" data-side={side} className="cn-sheet-content" {...props} />;
}

function SheetHeader(props: DxElementProps) {
  return <DialogHeader data-slot="sheet-header" {...props} />;
}

function SheetFooter(props: DxElementProps) {
  return <DialogFooter data-slot="sheet-footer" {...props} />;
}

function SheetTitle(props: DxElementProps) {
  return <DialogTitle data-slot="sheet-title" {...props} />;
}

function SheetDescription(props: DxElementProps) {
  return <DialogDescription data-slot="sheet-description" {...props} />;
}

export {
  Sheet,
  SheetTrigger,
  SheetClose,
  SheetContent,
  SheetHeader,
  SheetFooter,
  SheetTitle,
  SheetDescription,
};
