import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsTargetIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:target" {...props} />;
}
