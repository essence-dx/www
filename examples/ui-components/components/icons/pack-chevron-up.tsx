import { Icon, type IconProps } from "./icon";

export function PackChevronUpIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:chevron-up" {...props} />;
}
