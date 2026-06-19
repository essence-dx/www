import { Icon, type IconProps } from "./icon";

export function WhiteboardEllipseIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:ellipse" {...props} />;
}
