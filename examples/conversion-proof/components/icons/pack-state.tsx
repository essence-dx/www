import { Icon, type IconProps } from "./icon";

export function PackStateIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:state" {...props} />;
}
