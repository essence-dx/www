import { Icon, type IconProps } from "./icon";

export function PackXIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:x" {...props} />;
}
