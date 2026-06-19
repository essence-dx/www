import { Icon, type IconProps } from "./icon";

export function PackChevronRightIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:chevron-right" {...props} />;
}
