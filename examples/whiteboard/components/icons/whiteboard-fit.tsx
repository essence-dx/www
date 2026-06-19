import { Icon, type IconProps } from "./icon";

export function WhiteboardFitIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:fit" {...props} />;
}
