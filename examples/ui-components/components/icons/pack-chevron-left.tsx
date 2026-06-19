import { Icon, type IconProps } from "./icon";

export function PackChevronLeftIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:chevron-left" {...props} />;
}
