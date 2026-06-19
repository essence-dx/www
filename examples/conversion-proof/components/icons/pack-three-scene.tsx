import { Icon, type IconProps } from "./icon";

export function PackThreeSceneIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:three-scene" {...props} />;
}
