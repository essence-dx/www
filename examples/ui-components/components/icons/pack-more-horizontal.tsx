import { Icon, type IconProps } from "./icon";

export function PackMoreHorizontalIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:more-horizontal" {...props} />;
}
