import { Icon, type IconProps } from "./icon";

export function WhiteboardPenIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:pen" {...props} />;
}
