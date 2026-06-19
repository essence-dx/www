import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsBarIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:bar" {...props} />;
}
