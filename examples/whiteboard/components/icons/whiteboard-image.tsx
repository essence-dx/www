import { Icon, type IconProps } from "./icon";

export function WhiteboardImageIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:image" {...props} />;
}
