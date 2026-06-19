import { Icon } from "@/components/icons/icon";

export function IconLaunchStatus() {
  return (
    <span className="inline-flex items-center gap-2">
      <Icon name="pack:check" />
      Launch checks ready
    </span>
  );
}
