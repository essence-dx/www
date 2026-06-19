import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger } from "./dialog";
import type { DxElementProps } from "./types";

function Drawer(props: DxElementProps) {
  return <Dialog data-slot="drawer" data-adapter-boundary="drawer" {...props} />;
}

function DrawerPortal(props: DxElementProps) {
  return <DialogPortal data-slot="drawer-portal" {...props} />;
}

function DrawerOverlay(props: DxElementProps) {
  return <DialogOverlay data-slot="drawer-overlay" {...props} />;
}

function DrawerTrigger(props: DxElementProps) {
  return <DialogTrigger data-slot="drawer-trigger" {...props} />;
}

function DrawerClose(props: DxElementProps) {
  return <DialogClose data-slot="drawer-close" {...props} />;
}

function DrawerContent(props: DxElementProps) {
  return <DialogContent data-slot="drawer-content" className="cn-drawer-content" {...props} />;
}

function DrawerHeader(props: DxElementProps) {
  return <DialogHeader data-slot="drawer-header" {...props} />;
}

function DrawerFooter(props: DxElementProps) {
  return <DialogFooter data-slot="drawer-footer" {...props} />;
}

function DrawerTitle(props: DxElementProps) {
  return <DialogTitle data-slot="drawer-title" {...props} />;
}

function DrawerDescription(props: DxElementProps) {
  return <DialogDescription data-slot="drawer-description" {...props} />;
}

export {
  Drawer,
  DrawerPortal,
  DrawerOverlay,
  DrawerTrigger,
  DrawerClose,
  DrawerContent,
  DrawerHeader,
  DrawerFooter,
  DrawerTitle,
  DrawerDescription,
};
