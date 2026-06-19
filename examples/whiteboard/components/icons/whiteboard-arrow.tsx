import { Icon, type IconProps } from "./icon";

export function WhiteboardArrowIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:arrow" {...props} />;
}
