import { Icon, type IconProps } from "./icon";

export function WhiteboardVideoIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:video" {...props} />;
}
