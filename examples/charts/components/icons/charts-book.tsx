import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsBookIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:book" {...props} />;
}
