import { Icon, type IconProps } from "./icon";

export function WhiteboardTextIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:text" {...props} />;
}
