import { Icon, type IconProps } from "./icon";

export function WhiteboardUndoIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:undo" {...props} />;
}
