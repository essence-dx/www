import { Icon, type IconProps } from "./icon";

export function StatusCheckIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="status:check" {...props} />;
}
