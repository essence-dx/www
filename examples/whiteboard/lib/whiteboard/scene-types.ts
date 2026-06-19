import type { WhiteboardSelectionMode } from "./model";

export interface SceneChangeOptions {
  readonly now?: string;
}

export interface InsertElementsOptions extends SceneChangeOptions {
  readonly index?: number;
  readonly select?: boolean;
}

export interface SelectElementsOptions extends SceneChangeOptions {
  readonly mode?: WhiteboardSelectionMode;
}
