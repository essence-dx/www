import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsLayersIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:layers" {...props} />;
}
