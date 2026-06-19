import { Icon, type IconProps } from "./icon";

export function PackUiComponentsIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:ui-components" {...props} />;
}
