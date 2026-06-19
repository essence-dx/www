import { Icon, type IconProps } from "./icon";

export function WhiteboardMathIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:math" {...props} />;
}
