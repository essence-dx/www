import { Icon, type IconProps } from "./icon";

export function WhiteboardZoomInIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:zoom-in" {...props} />;
}
