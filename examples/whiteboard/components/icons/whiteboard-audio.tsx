import { Icon, type IconProps } from "./icon";

export function WhiteboardAudioIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="whiteboard:audio" {...props} />;
}
