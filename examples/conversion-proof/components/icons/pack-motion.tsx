import { Icon, type IconProps } from "./icon";

export function PackMotionIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:motion" {...props} />;
}
