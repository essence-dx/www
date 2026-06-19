import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import {
  connectorRouteForElement,
  connectorRouteMetadata,
  type WhiteboardConnectorRoute,
} from "../../lib/whiteboard/connector-routes";
import type { WhiteboardElement } from "../../lib/whiteboard/model";

type ConnectorElement = Extract<WhiteboardElement, { type: "line" | "arrow" }>;

export type ConnectorInfoProps = {
  readonly element: ConnectorElement;
};

export function ConnectorInfo({ element }: ConnectorInfoProps) {
  const route = connectorRouteForElement(element);

  return (
    <div
      className="wb-connector-stack"
      data-whiteboard-connector-route={route}
      data-whiteboard-connector-route-controls="command-backed"
      data-whiteboard-connector-route-storage="metadata.connectorRoute"
    >
      <dl
        className="wb-connector-info"
        data-whiteboard-connector-info="model-backed"
        data-whiteboard-connector-rerouting="scene-reducer-live"
        data-whiteboard-connector-type={element.type}
        data-whiteboard-end-anchor={element.endBinding?.anchor ?? "auto"}
        data-whiteboard-end-bound={element.endBinding ? "true" : "false"}
        data-whiteboard-start-anchor={element.startBinding?.anchor ?? "auto"}
        data-whiteboard-start-bound={element.startBinding ? "true" : "false"}
      >
        <dt>Start</dt>
        <dd data-whiteboard-start-binding={element.startBinding?.elementId ?? "unbound"}>
          {element.startBinding?.elementId ?? "Unbound"}
        </dd>
        <dt>End</dt>
        <dd data-whiteboard-end-binding={element.endBinding?.elementId ?? "unbound"}>
          {element.endBinding?.elementId ?? "Unbound"}
        </dd>
        <dt>Anchors</dt>
        <dd>
          {element.startBinding?.anchor ?? "auto"} / {element.endBinding?.anchor ?? "auto"}
        </dd>
        <dt>Route</dt>
        <dd data-whiteboard-connector-route-value={route}>{route}</dd>
        <dt>Arrows</dt>
        <dd
          data-whiteboard-start-arrow={element.startArrow ?? "none"}
          data-whiteboard-end-arrow={element.endArrow ?? "none"}
        >
          {element.startArrow ?? "none"} / {element.endArrow ?? "none"}
        </dd>
      </dl>
      <div className="wb-connector-route-controls" role="toolbar" aria-label="Connector route">
        {(["straight", "orthogonal"] as const).map((mode) => (
          <button
            aria-pressed={route === mode}
            className="wb-toggle-button"
            data-active={route === mode}
            data-whiteboard-command="element.update"
            data-whiteboard-connector-route={mode}
            data-whiteboard-connector-route-option={mode}
            key={mode}
            onClick={() => setConnectorRoute(element, mode)}
            type="button"
          >
            <span>{mode}</span>
          </button>
        ))}
      </div>
    </div>
  );
}

function setConnectorRoute(
  element: ConnectorElement,
  route: WhiteboardConnectorRoute,
) {
  return whiteboardActions.dispatch({
    type: "element.update",
    id: element.id,
    patch: {
      metadata: {
        ...(element.metadata ?? {}),
        ...connectorRouteMetadata(route),
      },
    },
  });
}
