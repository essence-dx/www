import { cn } from "../../lib/utils";
import type { DxElementProps } from "./types";

function ChartContainer({ className, ...props }: DxElementProps) {
  return (
    <div
      data-slot="chart"
      data-adapter-boundary="dx-charts"
      className={cn("cn-chart", className)}
      {...props}
    />
  );
}

function ChartTooltip(props: DxElementProps) {
  return <div data-slot="chart-tooltip" className="cn-chart-tooltip" {...props} />;
}

function ChartTooltipContent(props: DxElementProps) {
  return <div data-slot="chart-tooltip-content" className="cn-chart-tooltip-content" {...props} />;
}

function ChartLegend(props: DxElementProps) {
  return <div data-slot="chart-legend" className="cn-chart-legend" {...props} />;
}

function ChartLegendContent(props: DxElementProps) {
  return <div data-slot="chart-legend-content" className="cn-chart-legend-content" {...props} />;
}

function ChartStyle(props: DxElementProps) {
  return <div data-slot="chart-style" hidden {...props} />;
}

export {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  ChartLegend,
  ChartLegendContent,
  ChartStyle,
};
