import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsSparklineIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:sparkline" {...props} />;
}
