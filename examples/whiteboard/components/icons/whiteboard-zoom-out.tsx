import { Icon, type IconProps } from "./icon";

export function WhiteboardZoomOutIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:zoom-out" {...props} />;
}
