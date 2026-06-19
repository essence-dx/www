import { Icon, type IconProps } from "./icon";

export function WhiteboardTrashIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:trash" {...props} />;
}
