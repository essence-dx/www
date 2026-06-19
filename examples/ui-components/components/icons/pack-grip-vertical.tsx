import { Icon, type IconProps } from "./icon";

export function PackGripVerticalIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:grip-vertical" {...props} />;
}
