import { Icon, type IconProps } from "./icon";

export function WhiteboardRectangleIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:rectangle" {...props} />;
}
