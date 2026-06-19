import type { ChartSpec, ChartTask } from "../spec";
import { g2plotSankey, g2plotSunburst, g2plotTreemap } from "./g2plot-hierarchy";
import type { G2PlotSankeyPreset, G2PlotSunburstPreset, G2PlotTreemapPreset } from "./g2plot-hierarchy";

export type AntDesignTreemapPreset = Omit<G2PlotTreemapPreset, "family" | "task"> & {
  task?: ChartTask;
};

export type AntDesignSunburstPreset = Omit<G2PlotSunburstPreset, "family" | "task"> & {
  task?: ChartTask;
};

export type AntDesignSankeyPreset = Omit<G2PlotSankeyPreset, "family" | "task"> & {
  task?: ChartTask;
};

export function antDesignTreemap(config: AntDesignTreemapPreset): ChartSpec {
  return g2plotTreemap({ ...config, task: config.task ?? "proportion", family: "AntDesignPlots" });
}

export function antDesignSunburst(config: AntDesignSunburstPreset): ChartSpec {
  return g2plotSunburst({ ...config, task: config.task ?? "proportion", family: "AntDesignPlots" });
}

export function antDesignSankey(config: AntDesignSankeyPreset): ChartSpec {
  return g2plotSankey({ ...config, task: config.task ?? "flow", family: "AntDesignPlots" });
}
