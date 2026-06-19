import { Icon, type IconProps } from "./icon";

export function WhiteboardIconsIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:icons" {...props} />;
}
