import { Icon } from "./icon";
import type { IconProps } from "./icon";

export function ChartsPackageIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="charts:package" {...props} />;
}
