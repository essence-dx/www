import { Icon, type IconProps } from "./icon";

export function PackWorkspaceIcon(
  props: Omit<IconProps, "name">,
) {
  return <Icon name="pack:workspace" {...props} />;
}
