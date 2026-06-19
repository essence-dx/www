import { Icon, type IconProps } from "./icon";

export function PackLoaderIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:loader" {...props} />;
}
