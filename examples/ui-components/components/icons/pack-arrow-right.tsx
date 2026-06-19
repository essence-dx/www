import { Icon, type IconProps } from "./icon";

export function PackArrowRightIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:arrow-right" {...props} />;
}
