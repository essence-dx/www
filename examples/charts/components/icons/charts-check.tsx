import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsCheckIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:check" {...props} />;
}
