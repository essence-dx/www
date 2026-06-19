import { Icon, type IconProps } from "./icon";

export function WhiteboardHandIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:hand" {...props} />;
}
