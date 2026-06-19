import type { AgentChartRequestSpec, AgentChartWorkflowSpec, ChartSkillSpec, ChartSpec, Datum, GraphEdgeSpec, GraphModelSpec } from "./spec";

export type AgentChartFlowConfig = {
  id: string;
  title: string;
  description: string;
  request: AgentChartRequestSpec;
  skills: ChartSkillSpec[];
  width?: number;
  height?: number;
};

export function createMcpRequestFlow(config: AgentChartFlowConfig): ChartSpec {
  const workflow: AgentChartWorkflowSpec = { request: config.request, skills: config.skills };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "flow",
    family: "MCP",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: materializeAgentFlowRows(workflow),
    marks: [
      {
        id: `${config.id}-flow`,
        type: "sankey",
        encoding: {
          source: { field: "source", type: "nominal" },
          target: { field: "target", type: "nominal" },
          size: { field: "value", type: "quantitative" },
          label: { field: "label", type: "nominal" },
        },
      },
    ],
  };
}

export function chartSkillsGraph(config: AgentChartFlowConfig): ChartSpec {
  const workflow: AgentChartWorkflowSpec = { request: config.request, skills: config.skills };
  const graph = buildSkillGraph(workflow);

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "relation",
    family: "ChartSkills",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: graph.edges.map((edge) => ({ source: edge.source, target: edge.target, relation: edge.relation ?? edge.id ?? "", value: edge.weight ?? 1 })),
    graph,
    marks: [{ id: `${config.id}-graph`, type: "graph", encoding: { source: { field: "source", type: "nominal" }, target: { field: "target", type: "nominal" } } }],
  };
}

export function materializeAgentFlowRows(workflow: AgentChartWorkflowSpec): Datum[] {
  const requestNode = `request:${workflow.request.id}`;

  return workflow.skills.flatMap((skill) => {
    const dependencyRows = (skill.dependsOn ?? []).map((dependency) => ({
      source: dependency,
      target: skill.id,
      label: "depends on",
      value: 1,
    }));

    return [
      {
        source: requestNode,
        target: skill.id,
        label: `${workflow.request.task} to ${skill.family}`,
        value: Math.max(1, skill.produces.length),
      },
      ...dependencyRows,
    ];
  });
}

export function buildSkillGraph(workflow: AgentChartWorkflowSpec): GraphModelSpec {
  const requestNodeId = `request:${workflow.request.id}`;

  return {
    nodes: [
      {
        id: requestNodeId,
        label: workflow.request.prompt,
        type: workflow.request.outputFamily ?? "MCP",
        value: Math.max(6, workflow.request.inputFields.length * 2),
      },
      ...workflow.skills.map((skill) => ({
        id: skill.id,
        label: skill.label,
        type: skill.family,
        value: Math.max(4, skill.produces.length * 2),
      })),
    ],
    edges: workflow.skills.flatMap((skill) => {
      const dependencies = skill.dependsOn ?? [];
      const requestEdges = dependencies.length === 0 ? [edgeFromSkillDependency(requestNodeId, skill.id, workflow.request.task)] : [];

      return [...requestEdges, ...dependencies.map((dependency) => edgeFromSkillDependency(dependency, skill.id, "unlocks"))];
    }),
    layout: { type: "radial" },
    behaviors: [{ type: "focus-node" }, { type: "activate-relations" }],
    plugins: [{ type: "legend" }, { type: "tooltip" }],
  };
}

export function edgeFromSkillDependency(source: string, target: string, relation: string): GraphEdgeSpec {
  return {
    id: `${source}-${target}`,
    source,
    target,
    relation,
    weight: 2,
  };
}
