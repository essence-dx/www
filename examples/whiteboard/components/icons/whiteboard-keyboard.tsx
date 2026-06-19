import { Icon, type IconProps } from "./icon";

export function WhiteboardKeyboardIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:keyboard" {...props} />;
}
