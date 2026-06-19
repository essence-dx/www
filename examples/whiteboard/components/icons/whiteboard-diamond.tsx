import { Icon, type IconProps } from "./icon";

export function WhiteboardDiamondIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:diamond" {...props} />;
}
