import { Button } from "./button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger } from "./dialog";
import type { ButtonProps } from "./button";
import type { DxElementProps } from "./types";

function AlertDialog(props: DxElementProps) {
  return <div data-slot="alert-dialog" data-adapter-boundary="alert-dialog" {...props} />;
}

function AlertDialogTrigger(props: ButtonProps) {
  return <DialogTrigger data-slot="alert-dialog-trigger" {...props} />;
}

function AlertDialogPortal(props: DxElementProps) {
  return <DialogPortal data-slot="alert-dialog-portal" {...props} />;
}

function AlertDialogOverlay(props: DxElementProps) {
  return <DialogOverlay data-slot="alert-dialog-overlay" {...props} />;
}

function AlertDialogContent(props: DxElementProps) {
  return <DialogContent data-slot="alert-dialog-content" role="alertdialog" {...props} />;
}

function AlertDialogHeader(props: DxElementProps) {
  return <DialogHeader data-slot="alert-dialog-header" {...props} />;
}

function AlertDialogFooter(props: DxElementProps) {
  return <DialogFooter data-slot="alert-dialog-footer" {...props} />;
}

function AlertDialogTitle(props: DxElementProps) {
  return <DialogTitle data-slot="alert-dialog-title" {...props} />;
}

function AlertDialogDescription(props: DxElementProps) {
  return <DialogDescription data-slot="alert-dialog-description" {...props} />;
}

function AlertDialogMedia(props: DxElementProps) {
  return <div data-slot="alert-dialog-media" className="cn-dialog-media" {...props} />;
}

function AlertDialogAction(props: ButtonProps) {
  return <Button data-slot="alert-dialog-action" {...props} />;
}

function AlertDialogCancel(props: ButtonProps) {
  return <Button data-slot="alert-dialog-cancel" variant="outline" {...props} />;
}

export {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogMedia,
  AlertDialogOverlay,
  AlertDialogPortal,
  AlertDialogTitle,
  AlertDialogTrigger,
};
