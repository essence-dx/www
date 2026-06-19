import { Icon, type IconProps } from "./icon";

export function PackArrowLeftIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:arrow-left" {...props} />;
}
