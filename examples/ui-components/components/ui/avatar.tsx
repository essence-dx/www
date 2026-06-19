import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function Avatar({ className, ...props }: DxElementProps) {
  return (
    <span data-slot="avatar" className={cn("cn-avatar", className)} {...props} />
  );
}

function AvatarImage({ className, alt = "", ...props }: DxElementProps) {
  return (
    <img
      data-slot="avatar-image"
      alt={alt}
      className={cn("cn-avatar-image", className)}
      {...props}
    />
  );
}

function AvatarFallback({ className, ...props }: DxElementProps) {
  return (
    <span
      data-slot="avatar-fallback"
      className={cn("cn-avatar-fallback", className)}
      {...props}
    />
  );
}

function AvatarBadge({ className, ...props }: DxElementProps) {
  return (
    <span
      data-slot="avatar-badge"
      className={cn("cn-avatar-badge", className)}
      {...props}
    />
  );
}

function AvatarGroup({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="avatar-group"
      className={cn("cn-avatar-group", className)}
      {...props}
    />
  );
}

function AvatarGroupCount({ className, ...props }: DxElementProps) {
  return (
    <span
      data-slot="avatar-group-count"
      className={cn("cn-avatar-group-count", className)}
      {...props}
    />
  );
}

export {
  Avatar,
  AvatarImage,
  AvatarFallback,
  AvatarBadge,
  AvatarGroup,
  AvatarGroupCount,
};
