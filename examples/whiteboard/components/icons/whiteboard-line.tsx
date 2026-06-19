import { Icon, type IconProps } from "./icon";

export function WhiteboardLineIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:line" {...props} />;
}
