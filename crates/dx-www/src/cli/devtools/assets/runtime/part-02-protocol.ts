      },
      closestParentChain: chain,
      computedCss: computedPayload(target),
      boxModel: boxModelPayload(target),
      breakpointLabel: breakpointLabel(),
      source: sourceTarget && sourceTarget.range
        ? { status: "writable", writable: true, previewOnly: false, reason: "exact-source-range-present", relativePath: sourceTarget.relativePath }
        : { status: "preview-only", writable: false, previewOnly: true, reason: "preview-only / not writable", relativePath: sourceTarget ? sourceTarget.relativePath : null }
    };
    const property = STATE.styleDraft.property || "color";
    const realValue = STATE.selectedSnapshot.computedCss.properties[property];
    if (realValue && !activeStyleDraftEditor()) STATE.styleDraft = { property, value: realValue };
    const sourceLookupKey = sourceTarget?.relativePath || "";
    if (!sourceLookupKey) {
      lastSourceLookupKey = "";
      STATE.sourceResult = { resolution: { message: "preview-only / not writable" } };
    } else if (sourceLookupKey !== lastSourceLookupKey) {
      lastSourceLookupKey = sourceLookupKey;
      loadSourceMap();
    }
    readinessRecordVisualInspect();
    updateBoxOverlay();
    scheduleRenderAll();
  }

  function selectParentChainItem(item) {
    if (!item) return;
    let target = null;
    const depth = Number(item.depth);
    if (Number.isInteger(depth) && depth >= 0 && STATE.selectedElement) {
      target = STATE.selectedElement;
      for (let index = 0; target && index < depth; index += 1) {
        target = target.parentElement;
      }
    }
    try {
      if (!target && item.selector) target = document.querySelector(item.selector);
    } catch (error) {
      STATE.runtimeIssues.push(`parent chain selector failed: ${error.message}`);
    }
    if (!target && item.id) target = document.getElementById(item.id);
    if (!target || root.contains(target) || target === document.documentElement || target === document.body) {
      STATE.runtimeIssues.push(`parent chain target unavailable: ${item.selector || item.id || "unknown"}`);
      renderAll();
      return;
    }
    inspectedTarget = target;
    captureElement(target);
    openPanel("left", { force: true });
  }

  function readinessVisualState() {
    return STATE.readinessVisualEdit;
  }

  function readinessStyleRecord(property, snapshot = STATE.selectedSnapshot) {
    const value =
      snapshot?.computedCss?.properties?.[property] ??
      (STATE.selectedElement && property ? window.getComputedStyle(STATE.selectedElement).getPropertyValue(property) : "");
    return { property, value: value || "" };
  }

  function readinessElementFingerprint(snapshot = STATE.selectedSnapshot) {
    const selected = snapshot?.selectedElement;
    if (!selected) return "";
    const sourceTarget = selected.sourceTarget || {};
    const range = sourceTarget.range || {};
    return [
      selected.tagName,
      selected.stableId,
      selected.selector,
      sourceTarget.relativePath,
      range.startByte,
      range.endByte
    ].filter((part) => part != null && part !== "").join("|");
  }

  function readinessSourceTargetIsSafe(sourceTarget) {
    return Boolean(
      sourceTarget &&
      sourceTarget.relativePath &&
      sourceTarget.range &&
      Number.isFinite(Number(sourceTarget.range.startByte)) &&
      Number.isFinite(Number(sourceTarget.range.endByte)) &&
      Number(sourceTarget.range.endByte) >= Number(sourceTarget.range.startByte) &&
      Number(sourceTarget.range.endByte) > 0
    );
  }

  function readinessResetVisualProof(nextFingerprint) {
    const proof = readinessVisualState();
    proof.phases = new Set();
    proof.inspectedElementPresent = false;
    proof.cascadeInspected = false;
    proof.previewProtocolOk = false;
    proof.applyProtocolOk = false;
    proof.undoProtocolOk = false;
    proof.previewSourceMutated = false;
    proof.applySourceMutated = false;
    proof.undoSourceRestored = false;
    proof.safeLocalSourceTargetKnown = false;
    proof.applyReceiptWritten = false;
    proof.undoReceiptWritten = false;
    proof.receiptDurability = null;
    proof.inspectedSelector = null;
    proof.inspectedElementFingerprint = nextFingerprint || null;
    proof.sourceTarget = null;
    proof.styleProperty = null;
    proof.styleValue = null;
    proof.computedStyleBefore = null;
    proof.computedStyleAfterPreview = null;
    proof.computedStyleAfterUndo = null;
    proof.inlineStyleBefore = null;
  }

  function readinessRecordVisualInspect() {
    const snapshot = STATE.selectedSnapshot;
    const fingerprint = readinessElementFingerprint(snapshot);
    if (!snapshot || !fingerprint) {
      readinessPublishVisualEditReplay();
      return;
    }
    const proof = readinessVisualState();
    if (proof.inspectedElementFingerprint && proof.inspectedElementFingerprint !== fingerprint) {
      readinessResetVisualProof(fingerprint);
    }
    const sourceTarget = snapshot.selectedElement?.sourceTarget || null;
    proof.phases.add("inspect");
    proof.phases.add("cascade");
    proof.inspectedElementPresent = true;
    proof.cascadeInspected = (snapshot.closestParentChain || []).length > 0;
    proof.inspectedSelector = snapshot.selectedElement?.selector || null;
    proof.inspectedElementFingerprint = fingerprint;
    if (readinessSourceTargetIsSafe(sourceTarget) || !readinessSourceTargetIsSafe(proof.sourceTarget)) {
      proof.sourceTarget = sourceTarget;
      proof.safeLocalSourceTargetKnown = readinessSourceTargetIsSafe(sourceTarget);
    }
    if (!proof.styleProperty) proof.styleProperty = STATE.styleDraft.property || "color";
    if (!proof.styleValue) proof.styleValue = STATE.styleDraft.value || "";
    if (!proof.computedStyleBefore && proof.styleProperty) {
      proof.computedStyleBefore = readinessStyleRecord(proof.styleProperty, snapshot);
    }
    readinessPublishVisualEditReplay();
  }

  function readinessBeginStyleOperation(request) {
    readinessRecordVisualInspect();
    const proof = readinessVisualState();
    proof.styleProperty = request.property;
    proof.styleValue = request.value;
    proof.sourceTarget = request.sourceTarget || null;
    proof.safeLocalSourceTargetKnown = readinessSourceTargetIsSafe(request.sourceTarget);
    proof.computedStyleBefore = readinessStyleRecord(request.property, STATE.selectedSnapshot);
    proof.inlineStyleBefore = STATE.selectedElement
      ? {
          property: request.property,
          present: STATE.selectedElement.style.getPropertyValue(request.property) !== "",
          value: STATE.selectedElement.style.getPropertyValue(request.property),
          priority: STATE.selectedElement.style.getPropertyPriority(request.property)
        }
      : null;
    readinessPublishVisualEditReplay();
  }

  function readinessReplayFixture() {
    const fixture = STATE.session?.readiness_visual_edit_replay_fixture;
    if (!fixture || fixture.ready !== true || !readinessSourceTargetIsSafe(fixture.sourceTarget)) {
      return null;
    }
    return fixture;
  }

  async function readinessEnsureReplayFixture() {
    if (!readinessReplayFixture()) {
      await refreshProtocol();
    }
    return readinessReplayFixture();
  }

  function readinessReplayTarget(selector) {
    const candidates = Array.from(document.querySelectorAll(selector || "[data-dx-component], [data-dx-route], main, section"));
    return candidates.find((candidate) =>
      candidate &&
      candidate !== document.body &&
      candidate !== document.documentElement &&
      !root.contains(candidate)
    ) || null;
  }

  function readinessStyleRequestFromFixture(operation, fixture) {
    const request = styleRequest(operation);
    const property = fixture.property || READINESS_VISUAL_EDIT_REPLAY_MISSING;
    const value = fixture.value || "";
    return {
      ...request,
      operation,
      property,
      value,
      sourceTarget: fixture.sourceTarget,
      computedCss: {
        ...(request.computedCss || {}),
        properties: {
          ...(request.computedCss?.properties || {}),
          [property]: request.computedCss?.properties?.[property] || fixture.sourceTarget?.range?.expectedText || ""
        }
      },
      boxModel: request.boxModel || boxModelPayload(STATE.selectedElement)
    };
  }

  async function readinessRunVisualEditReplay() {
    const fixture = await readinessEnsureReplayFixture();
    if (!fixture) {
      readinessPublishVisualEditReplay();
      return {
        ok: false,
        reason: "missing-readiness-visual-edit-replay-fixture",
        replay: window.__DX_READINESS_VISUAL_EDIT_REPLAY__
      };
    }
    const target = readinessReplayTarget(fixture.target_selector);
    if (!target) {
      readinessPublishVisualEditReplay();
      return {
        ok: false,
        reason: "missing-readiness-visual-edit-replay-target",
        replay: window.__DX_READINESS_VISUAL_EDIT_REPLAY__
      };
    }

    captureElement(target);
    STATE.styleDraft = { property: fixture.property, value: fixture.value };
    const previewRequest = readinessStyleRequestFromFixture("style-preview", fixture);
    readinessBeginStyleOperation(previewRequest);
    STATE.selectedElement.style.setProperty(previewRequest.property, previewRequest.value);
    captureElement(STATE.selectedElement);
    const preview = await requestJson(ENDPOINTS.stylePreview, {
      method: "POST",
      body: JSON.stringify(previewRequest)
    });
    STATE.styleResult = preview.body;
    readinessRecordPreview(previewRequest, preview);

    const applyRequest = readinessStyleRequestFromFixture("style-apply", fixture);
    const apply = await requestJson(ENDPOINTS.styleApply, {
      method: "POST",
      body: JSON.stringify(applyRequest)
    });
    STATE.styleResult = apply.body;
    readinessRecordApply(applyRequest, apply);

    const undo = await requestJson(ENDPOINTS.styleUndo, {
      method: "POST"
    });
    STATE.styleResult = undo.body;
    if (undo.body?.undone === true) {
      readinessRestoreInlinePreview();
      captureElement(STATE.selectedElement);
    }
    readinessRecordUndo(undo);
    renderAll();
    return {
      ok: readinessVisualEditReady(),
      replay: window.__DX_READINESS_VISUAL_EDIT_REPLAY__,
      preview: preview.body,
      apply: apply.body,
      undo: undo.body
    };
  }

  function readinessReceiptWritten(result) {
    return result?.body?.receipt_candidate?.receipt_written === true || result?.body?.receipt_write?.written === true;
  }

  function readinessReceiptDurability(result) {
    return result?.body?.receipt_candidate?.receipt_durability || null;
  }

  function readinessRecordPreview(request, result) {
    const proof = readinessVisualState();
    proof.phases.add("preview");
    proof.previewProtocolOk = result?.ok === true && result?.body?.schema === "dx.devtools.style_preview";
    proof.previewSourceMutated =
      result?.body?.mutates_source === true ||
      result?.body?.writes_source === true ||
      result?.body?.source_mutated === true;
    proof.computedStyleAfterPreview = readinessStyleRecord(request.property, STATE.selectedSnapshot);
    readinessPublishVisualEditReplay();
  }

  function readinessRecordApply(request, result) {
    const proof = readinessVisualState();
    proof.phases.add("apply");
    proof.applyProtocolOk = result?.ok === true && result?.body?.schema === "dx.devtools.style_apply";
    proof.applySourceMutated =
      result?.body?.applied === true ||
      result?.body?.operation?.mutated === true ||
      result?.body?.receipt_candidate?.source_mutated === true;
    proof.safeLocalSourceTargetKnown =
      proof.safeLocalSourceTargetKnown ||
      result?.body?.receipt_candidate?.safe_local_source_target_known === true ||
      readinessSourceTargetIsSafe(request.sourceTarget);
    proof.applyReceiptWritten = readinessReceiptWritten(result);
    proof.receiptDurability = readinessReceiptDurability(result) || proof.receiptDurability;
    if (proof.applyReceiptWritten) proof.phases.add("receipt");
    readinessPublishVisualEditReplay();
  }

  function readinessRestoreInlinePreview() {
    const proof = readinessVisualState();
    const previous = proof.inlineStyleBefore;
    if (!STATE.selectedElement || !previous?.property) return;
    if (previous.present) {
      STATE.selectedElement.style.setProperty(previous.property, previous.value, previous.priority || "");
    } else {
      STATE.selectedElement.style.removeProperty(previous.property);
    }
  }

  function readinessRecordUndo(result) {
    const proof = readinessVisualState();
    proof.phases.add("undo");
    proof.undoProtocolOk = result?.ok === true && result?.body?.schema === "dx.devtools.style_undo";
    proof.undoSourceRestored = result?.body?.undone === true && result?.body?.operation?.mutated === true;
    proof.undoReceiptWritten = readinessReceiptWritten(result);
    proof.receiptDurability = readinessReceiptDurability(result) || proof.receiptDurability;
    if (proof.undoReceiptWritten) proof.phases.add("receipt");
    if (proof.styleProperty) proof.computedStyleAfterUndo = readinessStyleRecord(proof.styleProperty, STATE.selectedSnapshot);
    readinessPublishVisualEditReplay();
  }

  function readinessVisualEditReady() {
    const proof = readinessVisualState();
    return (
      proof.inspectedElementPresent === true &&
      proof.cascadeInspected === true &&
      proof.previewProtocolOk === true &&
      proof.applyProtocolOk === true &&
      proof.undoProtocolOk === true &&
      proof.previewSourceMutated === false &&
      proof.applySourceMutated === true &&
      proof.undoSourceRestored === true &&
      proof.safeLocalSourceTargetKnown === true &&
      proof.applyReceiptWritten === true &&
      proof.undoReceiptWritten === true &&
      proof.receiptDurability === "json-sr-machine-written" &&
      Boolean(proof.inspectedSelector) &&
      Boolean(proof.inspectedElementFingerprint) &&
      Boolean(proof.styleProperty) &&
      Boolean(proof.styleValue) &&
      Boolean(proof.computedStyleBefore?.value) &&
      Boolean(proof.computedStyleAfterPreview?.value) &&
      Boolean(proof.computedStyleAfterUndo?.value) &&
      readinessSourceTargetIsSafe(proof.sourceTarget) &&
      READINESS_VISUAL_EDIT_PHASES.every((phase) => proof.phases.has(phase))
    );
  }

  function readinessPublishVisualEditReplay() {
    const proof = readinessVisualState();
    const replayCurrent = readinessVisualEditReady();
    window.__DX_READINESS_VISUAL_EDIT_REPLAY__ = {
      devtools_global_present: window.__DX_DEVTOOLS_ACTIVE__ === true,
      browser_workbench_replay: replayCurrent ? READINESS_VISUAL_EDIT_REPLAY_CURRENT : READINESS_VISUAL_EDIT_REPLAY_MISSING,
      workbench_phases: READINESS_VISUAL_EDIT_PHASES.filter((phase) => proof.phases.has(phase)),
      inspected_element_present: proof.inspectedElementPresent === true,
      cascade_inspected: proof.cascadeInspected === true,
      preview_source_mutated: proof.previewSourceMutated === true,
      apply_source_mutated: proof.applySourceMutated === true,
      undo_source_restored: proof.undoSourceRestored === true,
      safe_local_source_target_known: proof.safeLocalSourceTargetKnown === true,
      apply_receipt_written: proof.applyReceiptWritten === true,
      undo_receipt_written: proof.undoReceiptWritten === true,
      receipt_durability: proof.receiptDurability,
      inspected_selector: proof.inspectedSelector,
      inspected_element_fingerprint: proof.inspectedElementFingerprint,
      style_property: proof.styleProperty,
      style_value: proof.styleValue,
      computed_style_before: proof.computedStyleBefore,
      computed_style_after_preview: proof.computedStyleAfterPreview,
      computed_style_after_undo: proof.computedStyleAfterUndo,
      source_target: proof.sourceTarget,
      release_ready: false,
      fastest_world_claim: false
    };
  }

  async function loadSourceMap() {
    const snapshot = STATE.selectedSnapshot;
    const sourcePath = snapshot?.selectedElement?.sourceTarget?.relativePath;
    if (!sourcePath) {
      STATE.sourceResult = { resolution: { message: "preview-only / not writable" } };
      renderAll();
      return;
    }
    const result = await requestJson(`${ENDPOINTS.sourceMap}?source_path=${encodeURIComponent(sourcePath)}`);
    STATE.sourceResult = result.body;
    renderAll();
  }

  function styleRequest(operation) {
    const propertyNode = root.querySelector("[data-dx-devtools-style-property]");
    const valueNode = root.querySelector("[data-dx-devtools-style-value]");
    const property = editableControlValue(propertyNode).trim() || STATE.styleDraft.property || "color";
    const value = editableControlValue(valueNode).trim() || STATE.styleDraft.value || "currentColor";
    STATE.styleDraft = { property, value };
    const snapshot = STATE.selectedSnapshot;
    return {
      schema: "dx.visual_edit.style_operation",
      format: 1,
      operation,
      property,
      value,
      breakpointLabel: breakpointLabel(),
      viewportWidth: window.innerWidth,
      sourceTarget: snapshot?.selectedElement?.sourceTarget || null,
      computedCss: snapshot?.computedCss || computedPayload(STATE.selectedElement),
      boxModel: styleOperationBoxModelPayload(snapshot?.boxModel || boxModelPayload(STATE.selectedElement))
    };
  }

  async function previewStyle() {
    const request = styleRequest("style-preview");
    readinessBeginStyleOperation(request);
    if (STATE.selectedElement) {
      STATE.selectedElement.style.setProperty(request.property, request.value);
      captureElement(STATE.selectedElement);
    }
    const result = await requestJson(ENDPOINTS.stylePreview, {
      method: "POST",
      body: JSON.stringify(request)
    });
    STATE.styleResult = result.body;
    readinessRecordPreview(request, result);
    renderAll();
  }

  async function applyStyle() {
    const request = styleRequest("style-apply");
    if (!request.sourceTarget || !request.sourceTarget.range) {
      STATE.styleResult = {
        schema: "dx.devtools.style_apply",
        status: "preview-only",
        previewOnly: true,
        writable: false,
        mutated: false,
        reason: "preview-only / not writable"
      };
      readinessPublishVisualEditReplay();
      renderAll();
      return;
    }
    const result = await requestJson(ENDPOINTS.styleApply, {
      method: "POST",
      body: JSON.stringify(request)
    });
    STATE.styleResult = result.body;
    readinessRecordApply(request, result);
    renderAll();
  }

  async function undoStyle() {
    const result = await requestJson(ENDPOINTS.styleUndo, {
      method: "POST"
    });
    STATE.styleResult = result.body;
    if (result.body?.undone === true) {
      readinessRestoreInlinePreview();
      if (STATE.selectedElement) captureElement(STATE.selectedElement);
    }
    readinessRecordUndo(result);
    renderAll();
  }

  function setStyleDraft(property, value, options = {}) {
    STATE.styleDraft = { property, value };
    if (options.closePopovers !== false) {
      closePopovers();
    }
    renderAll();
  }

  function closePopovers(except = "") {
    for (const key of ["propertyPopoverOpen", "valuePopoverOpen", "colorPopoverOpen", "radialShapePopoverOpen"]) {
      if (key !== except) {
        STATE[key] = false;
        delete STATE.popoverQuery[key];
      }
    }
    for (const key of Object.keys(STATE.activePopoverIndex)) {
      if (key !== except) delete STATE.activePopoverIndex[key];
    }
  }

  function hasOpenPopover() {
    return ["propertyPopoverOpen", "valuePopoverOpen", "colorPopoverOpen", "radialShapePopoverOpen"].some((key) => STATE[key]);
  }

  function captureFocusToken() {
    const active = document.activeElement;
    if (!active || !root?.contains?.(active)) return "";
    return active.getAttribute("data-dx-devtools-focus") || "";
  }

  function restoreFocusToken(token) {
    if (!token || !root) return;
    const fallbackToken = token.includes("-options-option-")
      ? token.replace(/-option-\d+$/, "")
      : token.includes("-option-")
        ? token.replace(/-option-\d+$/, "")
        : token;
    const target =
      root.querySelector(`[data-dx-devtools-focus="${cssEscape(token)}"]`) ||
      root.querySelector(`[data-dx-devtools-focus="${cssEscape(fallbackToken)}"]`);
    target?.focus?.({ preventScroll: true });
    if (target?.isContentEditable) {
      const range = document.createRange();
      range.selectNodeContents(target);
      range.collapse(false);
      const selection = window.getSelection();
      selection?.removeAllRanges();
      selection?.addRange(range);
    }
  }

  function selectOptionId(listId, index) {
    return `${listId}-option-${Math.max(0, index)}`;
  }

  function optionIndex(options, value) {
    const currentIndex = options.indexOf(value);
    return currentIndex >= 0 ? currentIndex : 0;
  }

  function activeOptionIndex(openKey, options, value) {
    const stored = STATE.activePopoverIndex[openKey];
    if (Number.isInteger(stored)) return Math.max(0, Math.min(options.length - 1, stored));
    return optionIndex(options, value);
  }

  function setActiveOptionIndex(openKey, options, index) {
    if (!options.length) return;
    STATE.activePopoverIndex[openKey] = (index + options.length) % options.length;
  }

  function normalizeOptionQuery(value) {
    return String(value || "").trim().toLowerCase();
  }

  function filteredPopoverOptions(openKey, options) {
    const query = normalizeOptionQuery(STATE.popoverQuery[openKey]);
    if (!query) return options;
    return options.filter((option) => option.toLowerCase().includes(query));
  }

  function setPopoverQuery(openKey, options, value) {
    STATE.popoverQuery[openKey] = value;
    setActiveOptionIndex(openKey, filteredPopoverOptions(openKey, options), 0);
  }

  function onSelectKeydown(event, openKey, options, value, onPick) {
    if (!options.length) return;
    const currentIndex = activeOptionIndex(openKey, options, value);
    const pickAt = (index) => {
      const next = options[(index + options.length) % options.length];
      onPick(next);
    };
    const moveTo = (index) => {
      setActiveOptionIndex(openKey, options, index);
      closePopovers(openKey);
      STATE[openKey] = true;
      renderAll();
    };
    if (event.key === "Escape") {
      if (STATE[openKey]) {
        event.preventDefault();
        delete STATE.activePopoverIndex[openKey];
        STATE[openKey] = false;
        renderAll();
      }
      return;
    }
    if (event.key === "Enter" || event.key === " " || event.key === "Space") {
      event.preventDefault();
      if (!STATE[openKey]) {
        closePopovers(openKey);
        setActiveOptionIndex(openKey, options, currentIndex);
        STATE[openKey] = true;
        renderAll();
      } else {
        pickAt(currentIndex);
      }
      return;
    }
    if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      if (!STATE[openKey]) {
        closePopovers(openKey);
        setActiveOptionIndex(openKey, options, currentIndex);
        STATE[openKey] = true;
        renderAll();
      } else {
        moveTo(currentIndex + 1);
      }
      return;
    }
    if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      if (!STATE[openKey]) {
        closePopovers(openKey);
        setActiveOptionIndex(openKey, options, currentIndex);
        STATE[openKey] = true;
        renderAll();
      } else {
        moveTo(currentIndex - 1);
      }
      return;
    }
    if (event.key === "Home") {
      event.preventDefault();
      moveTo(0);
      return;
    }
    if (event.key === "End") {
      event.preventDefault();
      moveTo(options.length - 1);
    }
  }

  function onPopoverSearchKeydown(event, openKey, options, value, onPick) {
    const visibleOptions = filteredPopoverOptions(openKey, options);
    if (!visibleOptions.length) {
      if (event.key === "Escape") {
        event.preventDefault();
        delete STATE.popoverQuery[openKey];
        STATE[openKey] = false;
        renderAll();
      }
      if (event.key === "Enter") {
        event.preventDefault();
        event.stopPropagation();
      }
      return;
    }
    if (event.key === "ArrowDown" || event.key === "ArrowUp" || event.key === "Enter" || event.key === "Home" || event.key === "End" || event.key === "Escape") {
      onSelectKeydown(event, openKey, visibleOptions, value, onPick);
    }
  }

  function onSingleLineTextboxKeydown(event) {
    if (event.key === "Enter") {
      event.preventDefault();
      event.stopPropagation();
      return;
    }
    if (event.key === "Escape") {
      event.preventDefault();
      event.currentTarget?.blur?.();
    }
  }

  function onTabKeydown(event, tabs, current, onPick) {
    const currentIndex = Math.max(0, tabs.indexOf(current));
    let nextIndex = -1;
    if (event.key === "ArrowRight" || event.key === "ArrowDown") nextIndex = (currentIndex + 1) % tabs.length;
    else if (event.key === "ArrowLeft" || event.key === "ArrowUp") nextIndex = (currentIndex - 1 + tabs.length) % tabs.length;
    else if (event.key === "Home") nextIndex = 0;
    else if (event.key === "End") nextIndex = tabs.length - 1;
    if (nextIndex < 0) return;
    event.preventDefault();
    onPick(tabs[nextIndex]);
  }

  function focusFirstMenuItem() {
    const item = menu?.querySelector?.('[role="menuitem"]');
    item?.focus?.({ preventScroll: true });
  }

  function onMenuKeydown(event) {
    const items = Array.from(menu?.querySelectorAll?.('[role="menuitem"]') || []);
    if (!items.length) return;
    const currentIndex = Math.max(0, items.indexOf(document.activeElement));
    let nextIndex = -1;
    if (event.key === "Escape") {
      event.preventDefault();
      event.stopPropagation();
      STATE.menuOpen = false;
      renderAll({ focusToken: "puck" });
      puck?.focus?.({ preventScroll: true });
      return;
    }
    if (event.key === "ArrowDown") nextIndex = (currentIndex + 1) % items.length;
    else if (event.key === "ArrowUp") nextIndex = (currentIndex - 1 + items.length) % items.length;
    else if (event.key === "Home") nextIndex = 0;
    else if (event.key === "End") nextIndex = items.length - 1;
    else if (event.key === "Enter" || event.key === " " || event.key === "Space") {
      event.preventDefault();
      document.activeElement?.click?.();
      return;
    }
    if (nextIndex >= 0) {
      event.preventDefault();
      for (const [index, item] of items.entries()) {
        item.tabIndex = index === nextIndex ? 0 : -1;
      }
      items[nextIndex]?.focus?.({ preventScroll: true });
    }
  }

  function edgePx(values, side) {
    const edge = values?.[side];
    if (typeof edge === "number") return edge;
    return edge?.px || 0;
  }

  function edgeValue(values, side) {
    const edge = values?.[side];
    if (typeof edge === "number") return `${edge}px`;
    return edge?.value || `${edge?.px || 0}px`;
  }

  function edgeSummary(values) {
    return ["top", "right", "bottom", "left"].map((side) => edgeValue(values, side)).join(" / ");
  }

  function boxSize(box) {
    return `${Math.round(box?.width || 0)} x ${Math.round(box?.height || 0)}`;
  }

  function updateBoxOverlay() {
    if (!overlay || !STATE.selectedElement || !STATE.selectedElement.isConnected) {
      if (overlay) overlay.hidden = true;
      return;
    }
    overlay.hidden = false;
    const model = boxModelPayload(STATE.selectedElement);
    const rect = model.borderBox;
    overlay.style.left = `${Math.max(0, rect.x)}px`;
    overlay.style.top = `${Math.max(0, rect.y)}px`;
    overlay.style.width = `${rect.width}px`;
    overlay.style.height = `${rect.height}px`;
  }

  function panelInCloseCooldown(edge) {
    return STATE.lastClosedEdge === edge && performance.now() < STATE.edgeCooldownUntil;
  }

  function openPanel(edge, options) {
    const force = Boolean(options?.force);
    if (!force && panelInCloseCooldown(edge)) return;
    if (STATE.lastClosedEdge === edge) {
      STATE.lastClosedEdge = null;
      STATE.edgeCooldownUntil = 0;
    }
    STATE.openPanels.add(edge);
    renderAll();
  }

  function closePanel(edge) {
    STATE.lastClosedEdge = edge;
    STATE.edgeCooldownUntil = performance.now() + 700;
    STATE.openPanels.delete(edge);
    STATE.menuOpen = false;
    renderAll();
  }

  function panel(edge) {
    const title = { left: "Inspector", top: "Route", right: "Preview", bottom: "Box model" }[edge];
    const isOpen = STATE.openPanels.has(edge);
    return el("aside", {
      className: `dx-devtools-panel dx-devtools-panel-${edge}`,
