import { Icon, type IconProps } from "./icon";

export function PackPanelLeftIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:panel-left" {...props} />;
}
