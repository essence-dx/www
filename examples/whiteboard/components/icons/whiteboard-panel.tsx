import { Icon, type IconProps } from "./icon";

export function WhiteboardPanelIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:panel" {...props} />;
}
