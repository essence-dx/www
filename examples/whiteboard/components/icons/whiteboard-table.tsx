import { Icon, type IconProps } from "./icon";

export function WhiteboardTableIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:table" {...props} />;
}
