import { Icon, type IconProps } from "./icon";

export function PackChevronDownIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:chevron-down" {...props} />;
}
