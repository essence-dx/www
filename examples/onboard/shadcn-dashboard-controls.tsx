"use client";

import * as React from "react";

import { shallow } from "@/lib/forge/state/zustand";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemTitle,
} from "@/components/ui/item";
import { Separator } from "@/components/ui/separator";
import { Textarea } from "@/components/ui/textarea";

import {
  createShadcnLaunchDashboardReceipt,
  shadcnDashboardDensityOptions,
  shadcnDashboardQueueOptions,
  shadcnLaunchDashboardMetadata,
  type ShadcnDashboardQueue,
  type ShadcnDashboardReceipt,
} from "./shadcn-dashboard-controls-contract";
import { useLaunchDashboardSettings } from "./state-zustand-dashboard";

type DashboardQueue = ShadcnDashboardQueue;

export function LaunchShadcnDashboardControls() {
  const queueButtonRefs = React.useRef<
    Partial<Record<ShadcnDashboardQueue, HTMLButtonElement | null>>
  >({});
  const {
    commandHints,
    density,
    queue,
    rehydrateState,
    save,
    savedAt,
    setDensity,
    setFocus,
    setRehydrateState,
    toggleCommandHints,
  } = useLaunchDashboardSettings(
    (state) => ({
      commandHints: state.commandHints,
      density: state.density,
      queue: state.focus,
      rehydrateState: state.rehydrateState,
      save: state.save,
      savedAt: state.savedAt,
      setDensity: state.setDensity,
      setFocus: state.setFocus,
      setRehydrateState: state.setRehydrateState,
      toggleCommandHints: state.toggleCommandHints,
    }),
    shallow,
  );
  const [filter, setFilter] = React.useState("launch blockers");
  const [note, setNote] = React.useState(
    "Keep session, payment, data, and Studio controls together.",
  );
  const [receipt, setReceipt] = React.useState<ShadcnDashboardReceipt>({
    state: "idle",
  });
  const [lastFocusedTarget, setLastFocusedTarget] = React.useState<string>(
    "none",
  );
  const activeDensity =
    shadcnDashboardDensityOptions.find((option) => option.id === density) ??
    shadcnDashboardDensityOptions[0];
  const activeQueue =
    shadcnDashboardQueueOptions.find((option) => option.id === queue) ??
    shadcnDashboardQueueOptions[0];

  function selectQueue(nextQueue: DashboardQueue, shouldFocus = false) {
    setFocus(nextQueue);
    setReceipt({ state: "idle" });
    if (shouldFocus) {
      window.requestAnimationFrame(() => {
        queueButtonRefs.current[nextQueue]?.focus();
      });
    }
  }

  function handleQueueKeyDown(
    event: React.KeyboardEvent<HTMLButtonElement>,
  ) {
    const currentIndex = Math.max(
      shadcnDashboardQueueOptions.findIndex((option) => option.id === queue),
      0,
    );
    let nextIndex = currentIndex;

    switch (event.key) {
      case "ArrowRight":
      case "ArrowDown":
        nextIndex = (currentIndex + 1) % shadcnDashboardQueueOptions.length;
        break;
      case "ArrowLeft":
      case "ArrowUp":
        nextIndex =
          (currentIndex - 1 + shadcnDashboardQueueOptions.length) %
          shadcnDashboardQueueOptions.length;
        break;
      case "Home":
        nextIndex = 0;
        break;
      case "End":
        nextIndex = shadcnDashboardQueueOptions.length - 1;
        break;
      default:
        return;
    }

    event.preventDefault();
    selectQueue(shadcnDashboardQueueOptions[nextIndex].id, true);
  }

  function focusSelectedDashboardTarget() {
    const target = document.getElementById(activeQueue.controlsId);
    const card = target?.closest<HTMLElement>('[data-dx-dashboard-card]');
    const focusTarget = card ?? target;

    setReceipt({ state: "idle" });
    setLastFocusedTarget(activeQueue.controlsId);

    if (!focusTarget) return;

    document
      .querySelectorAll("[data-dx-shadcn-dashboard-target-focused]")
      .forEach((node) => {
        node.setAttribute("data-dx-shadcn-dashboard-target-focused", "false");
      });
    focusTarget.setAttribute("tabindex", "-1");
    focusTarget.setAttribute(
      "data-dx-shadcn-dashboard-target-focused",
      "true",
    );
    focusTarget.setAttribute(
      "data-dx-shadcn-dashboard-focus-source",
      "shadcn-dashboard-controls",
    );
    focusTarget.focus({ preventScroll: true });
    focusTarget.scrollIntoView({ block: "nearest", behavior: "smooth" });
  }

  function previewReceipt() {
    save();
    setReceipt(createShadcnLaunchDashboardReceipt({ density, queue, filter }));
  }

  function rehydrateDashboardSettings() {
    if (rehydrateState === "loading") return;

    setReceipt({ state: "idle" });
    setRehydrateState("loading");
    void useLaunchDashboardSettings.persist
      .rehydrate()
      .finally(() => setRehydrateState("idle"));
  }

  return (
    <Card
      data-dx-component="shadcn-dashboard-controls"
      data-dx-dashboard-workflow="operator-controls"
      data-dx-package="state/zustand,shadcn/ui/button"
      data-dx-shadcn-dashboard-density={density}
      data-dx-shadcn-dashboard-queue={queue}
      data-dx-shadcn-dashboard-controls-target={activeQueue.controlsId}
      data-dx-shadcn-dashboard-focus-target={activeQueue.controlsId}
      data-dx-shadcn-dashboard-receipt={
        receipt.state === "ready" ? receipt.receiptId : "idle"
      }
      data-dx-node-modules="forbidden"
      data-dx-source="examples/template/shadcn-dashboard-controls.tsx"
      data-dx-zustand-command-hints={commandHints ? "enabled" : "disabled"}
      data-dx-zustand-dashboard-density={density}
      data-dx-zustand-dashboard-focus={queue}
      data-dx-zustand-rehydrate-state={rehydrateState}
      data-dx-zustand-dashboard-saved-at={savedAt}
      data-dx-zustand-persist-key="dx-template-dashboard-settings"
      data-dx-zustand-store="launch-dashboard-settings"
      data-slot="card"
    >
      <CardHeader data-slot="card-header">
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div className="grid gap-1">
            <CardDescription>
              <dx-icon name="pack:settings" aria-hidden="true" /> Dashboard
              controls
            </CardDescription>
            <CardTitle>Operator controls powered by shadcn/ui</CardTitle>
          </div>
          <Badge data-slot="badge" variant="secondary">
            {shadcnLaunchDashboardMetadata.packageId}
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="grid gap-4" data-slot="card-content">
        <div className="flex flex-wrap gap-2" aria-label="Dashboard density">
          {shadcnDashboardDensityOptions.map((option) => (
            <Button
              key={option.id}
              aria-controls="shadcn-dashboard-receipt-preview"
              aria-pressed={density === option.id}
              data-dx-package="shadcn/ui/button"
              data-dx-shadcn-dashboard-action="set-density"
              data-dx-shadcn-dashboard-density-option={option.id}
              data-dx-shadcn-dashboard-selected={
                density === option.id ? "true" : "false"
              }
              data-dx-zustand-action="set-dashboard-density"
              data-dx-zustand-dashboard-density-option={option.id}
              data-size="sm"
              data-slot="button"
              data-variant={density === option.id ? "secondary" : "outline"}
              onClick={() => {
                setDensity(option.id);
                setReceipt({ state: "idle" });
              }}
              size="sm"
              type="button"
              variant={density === option.id ? "secondary" : "outline"}
            >
              {option.label}
            </Button>
          ))}
        </div>

        <ItemGroup
          data-dx-package="shadcn/ui/item"
          data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"
          data-slot="item-group"
        >
          {shadcnDashboardQueueOptions.map((option) => (
            <Item
              key={option.id}
              data-dx-package={option.packageId}
              data-dx-shadcn-dashboard-action="select-queue"
              data-dx-shadcn-dashboard-queue-option={option.id}
              data-dx-shadcn-dashboard-selected={
                queue === option.id ? "true" : "false"
              }
              data-slot="item"
              size="sm"
              variant={queue === option.id ? "default" : "outline"}
            >
              <ItemContent>
                <ItemTitle>{option.label}</ItemTitle>
                <ItemDescription>{option.target}</ItemDescription>
              </ItemContent>
              <ItemActions>
                <Button
                  aria-controls={option.controlsId}
                  aria-current={queue === option.id ? "true" : undefined}
                  aria-pressed={queue === option.id}
                  data-dx-package="shadcn/ui/button"
                  data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"
                  data-dx-zustand-action="select-dashboard-focus"
                  data-dx-zustand-dashboard-focus-option={option.id}
                  data-size="sm"
                  data-slot="button"
                  onKeyDown={handleQueueKeyDown}
                  onClick={() => {
                    selectQueue(option.id);
                  }}
                  ref={(node) => {
                    queueButtonRefs.current[option.id] = node;
                  }}
                  size="sm"
                  tabIndex={queue === option.id ? 0 : -1}
                  type="button"
                  variant={queue === option.id ? "secondary" : "outline"}
                >
                  Focus
                </Button>
              </ItemActions>
            </Item>
          ))}
        </ItemGroup>

        <Separator data-dx-package="shadcn/ui/separator" data-slot="separator" />

        <FieldGroup data-dx-package="shadcn/ui/field" data-slot="field-group">
          <Field data-slot="field">
            <FieldLabel htmlFor="shadcn-dashboard-filter">
              Control filter
            </FieldLabel>
            <Input
              data-dx-package="shadcn/ui/input"
              data-dx-shadcn-dashboard-input="filter"
              data-slot="input"
              id="shadcn-dashboard-filter"
              onChange={(event) => {
                setFilter(event.target.value);
                setReceipt({ state: "idle" });
              }}
              value={filter}
            />
            <FieldDescription>
              {commandHints
                ? activeDensity.description
                : "Command hints are hidden by the persisted dashboard state."}
            </FieldDescription>
          </Field>
          <Field data-slot="field">
            <FieldLabel htmlFor="shadcn-dashboard-note">
              Operator note
            </FieldLabel>
            <Textarea
              data-dx-package="shadcn/ui/textarea"
              data-dx-shadcn-dashboard-input="note"
              data-slot="textarea"
              id="shadcn-dashboard-note"
              onChange={(event) => setNote(event.target.value)}
              rows={3}
              value={note}
            />
            <FieldDescription>
              Stored by dx-template-dashboard-settings; durable storage remains
              app-owned.
            </FieldDescription>
          </Field>
        </FieldGroup>

        <div className="flex flex-wrap items-center gap-2">
          <Button
            aria-controls="shadcn-dashboard-receipt-preview"
            data-dx-package="shadcn/ui/button"
            data-dx-shadcn-dashboard-action="preview-dashboard-receipt"
            data-size="default"
            data-slot="button"
            data-variant="default"
            onClick={previewReceipt}
            type="button"
          >
            Preview dashboard receipt
          </Button>
          <Button
            aria-controls={activeQueue.controlsId}
            data-dx-package="shadcn/ui/button"
            data-dx-shadcn-dashboard-action="focus-target-card"
            data-dx-shadcn-dashboard-focus-target={activeQueue.controlsId}
            data-size="default"
            data-slot="button"
            data-variant="outline"
            onClick={focusSelectedDashboardTarget}
            type="button"
            variant="outline"
          >
            Focus selected card
          </Button>
          <Button
            aria-pressed={commandHints}
            data-dx-package="shadcn/ui/button"
            data-dx-zustand-action="toggle-command-hints"
            data-size="default"
            data-slot="button"
            data-variant="outline"
            onClick={() => {
              toggleCommandHints();
              setReceipt({ state: "idle" });
            }}
            type="button"
            variant="outline"
          >
            {commandHints ? "Hide hints" : "Show hints"}
          </Button>
          <Button
            data-dx-package="shadcn/ui/button"
            data-dx-zustand-action="rehydrate-dashboard-settings"
            data-dx-zustand-rehydrate-state={rehydrateState}
            data-size="default"
            data-slot="button"
            data-variant="outline"
            disabled={rehydrateState === "loading"}
            onClick={rehydrateDashboardSettings}
            type="button"
            variant="outline"
          >
            {rehydrateState === "loading" ? "Rehydrating view" : "Rehydrate view"}
          </Button>
          <Badge data-slot="badge" variant="outline">
            {activeQueue.target}
          </Badge>
        </div>

        <p
          aria-live="polite"
          className="rounded-md border bg-muted p-3 text-sm text-muted-foreground"
          data-dx-shadcn-dashboard-receipt={
            receipt.state === "ready" ? receipt.receiptId : "idle"
          }
          data-dx-shadcn-dashboard-state={receipt.state}
          data-dx-shadcn-dashboard-target-focused={lastFocusedTarget}
          data-dx-zustand-dashboard-saved-at={savedAt}
          id="shadcn-dashboard-receipt-preview"
        >
          {receipt.state === "ready"
            ? `${receipt.receiptId}: ${receipt.filter} controls are stored by dx-template-dashboard-settings; durable dashboard save remains app-owned.`
            : lastFocusedTarget !== "none"
              ? `${lastFocusedTarget} is selected for dashboard review.`
              : "Choose a dashboard queue and preview a local shadcn/ui receipt."}
        </p>
      </CardContent>
    </Card>
  );
}
