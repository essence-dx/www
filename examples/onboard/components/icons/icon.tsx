import * as React from "react";

export type IconProps = {
  name: string;
  title?: string;
} & React.HTMLAttributes<HTMLSpanElement>;

export function Icon({ name, title, className, ...props }: IconProps) {
  return (
    <span
      aria-hidden={title ? undefined : true}
      aria-label={title}
      className={className}
      data-dx-icon={name}
      data-icon-source="dx-icons"
      role={title ? "img" : undefined}
      {...props}
    />
  );
}
