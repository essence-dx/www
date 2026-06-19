import { Icon, type IconProps } from "./icon";

export function WhiteboardSelectIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:select" {...props} />;
}
