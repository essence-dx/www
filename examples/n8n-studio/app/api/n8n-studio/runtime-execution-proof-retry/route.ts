import {
  createRuntimeExecutionProofSchedulerRoute,
} from "../../../../server/n8n-studio/runtime-execution-proof-scheduler";

const schedulerRoute = createRuntimeExecutionProofSchedulerRoute();

export const GET = schedulerRoute.GET;

export const POST = schedulerRoute.POST;
