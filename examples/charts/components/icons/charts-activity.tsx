import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsActivityIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:activity" {...props} />;
}
