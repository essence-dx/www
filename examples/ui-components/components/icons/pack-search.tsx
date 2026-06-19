import { Icon, type IconProps } from "./icon";

export function PackSearchIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:search" {...props} />;
}
