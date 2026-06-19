import { Icon, type IconProps } from "./icon";

export function WhiteboardRedoIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:redo" {...props} />;
}
