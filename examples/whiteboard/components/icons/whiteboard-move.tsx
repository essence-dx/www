import { Icon, type IconProps } from "./icon";

export function WhiteboardMoveIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:move" {...props} />;
}
