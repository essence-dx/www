import { Icon, type IconProps } from "./icon";

export function PackPlayIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:play" {...props} />;
}
