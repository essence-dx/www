import { iconAttrs, type DxIconProps } from "../../lib/icons";

export function SearchIcon({
  size = 24,
  title,
  className,
  strokeWidth = 2,
  ...props
}: DxIconProps) {
  const titleId = title ? "dx-search-icon-title" : undefined;

  return (
    <svg
      {...iconAttrs({ size, className, strokeWidth, titleId })}
      {...props}
    >
      {title ? <title id={titleId}>{title}</title> : null}
      <circle cx="11" cy="11" r="8" />
      <path d="m21 21-4.3-4.3" />
    </svg>
  );
}
