import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsPaletteIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:palette" {...props} />;
}
