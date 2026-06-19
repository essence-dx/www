export type DxIconProps = {
  size?: number | string;
  title?: string;
  className?: string;
  strokeWidth?: number | string;
  role?: string;
  "aria-hidden"?: boolean | "true" | "false";
  [attribute: string]: string | number | boolean | undefined;
};

export function iconAttrs({
  size = 24,
  className,
  strokeWidth = 2,
  titleId,
}: {
  size?: number | string;
  className?: string;
  strokeWidth?: number | string;
  titleId?: string;
}) {
  return {
    xmlns: "http://www.w3.org/2000/svg",
    width: size,
    height: size,
    viewBox: "0 0 24 24",
    fill: "none",
    stroke: "currentColor",
    strokeWidth,
    strokeLinecap: "round",
    strokeLinejoin: "round",
    className,
    role: titleId ? "img" : undefined,
    "aria-hidden": titleId ? undefined : true,
    "aria-labelledby": titleId,
  };
}
