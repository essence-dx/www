import { Icon, type IconProps } from "./icon";

export function PackCheckIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:check" {...props} />;
}
