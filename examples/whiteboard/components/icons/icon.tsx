export type IconProps = {
  name: string;
  title?: string;
  className?: string;
  "aria-hidden"?: string | boolean;
  [attribute: string]: unknown;
};

export function Icon({ name, title, className, ...props }: IconProps) {
  return (
    <dx-icon
      aria-hidden={title ? undefined : true}
      aria-label={title}
      className={className}
      name={name}
      role={title ? "img" : undefined}
      {...props}
    />
  );
}
