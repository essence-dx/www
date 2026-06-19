import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsTerminalIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:terminal" {...props} />;
}
