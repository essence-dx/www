import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsNetworkIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:network" {...props} />;
}
