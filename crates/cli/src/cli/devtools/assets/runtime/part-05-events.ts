        el("strong", { text: "RGB / alpha" }),
        el("div", { className: "dx-devtools-picker-grid" }, [
          sliderControl("R", "red", 0, 255),
          sliderControl("G", "green", 0, 255),
          sliderControl("B", "blue", 0, 255),
          sliderControl("Alpha", "alpha", 0, 100, "%")
        ]),
        pickerApplyButton("Use RGB", targetProperty, rgbaPickerValue)
      ]),
      el("section", { className: "dx-devtools-picker-card" }, [
        el("strong", { text: "HSL / alpha" }),
        el("div", { className: "dx-devtools-picker-grid" }, [
          sliderControl("Hue", "hue", 0, 360),
          sliderControl("Sat", "saturation", 0, 100, "%"),
          sliderControl("Light", "lightness", 0, 100, "%"),
          sliderControl("Alpha", "alpha", 0, 100, "%")
        ]),
        pickerApplyButton("Use HSL", targetProperty, hslPickerValue)
      ]),
      swatchList("Quick solid colors", targetProperty, SOLID_PRESETS)
    ]);
  }

  function linearPicker() {
    return el("div", { className: "dx-devtools-picker-form" }, [
      el("section", { className: "dx-devtools-picker-card dx-devtools-angle-card" }, [
        el("strong", { text: "Angle" }),
        angleDialControl(),
        sliderControl("Angle", "linearAngle", 0, 360, "deg")
      ]),
      el("div", { className: "dx-devtools-picker-grid" }, [
        pickerColorControl("Stop 1", "stopA"),
        pickerColorControl("Stop 2", "stopB"),
        pickerColorControl("Stop 3", "stopC"),
        sliderControl("Pos 1", "stopAOffset", 0, 100, "%"),
        sliderControl("Pos 2", "stopBOffset", 0, 100, "%"),
        sliderControl("Pos 3", "stopCOffset", 0, 100, "%")
      ]),
      stopTargetControls(),
      colorStopSwatchList("Quick stop colors", SOLID_PRESETS),
      pickerApplyButton("Use linear gradient", "background-image", linearGradientValue)
    ]);
  }

  function radialPicker() {
    return el("div", { className: "dx-devtools-picker-form" }, [
      positionPadControl("Gradient center", "radialX", "radialY"),
      el("div", { className: "dx-devtools-picker-grid" }, [
        customSelectControl("Shape", "data-dx-devtools-radial-shape", STATE.colorPicker.radialShape, ["circle", "ellipse"], "radialShapePopoverOpen", (value) => {
          STATE.colorPicker.radialShape = value;
          STATE.radialShapePopoverOpen = false;
          renderAll();
        }),
        sliderControl("X", "radialX", 0, 100, "%"),
        sliderControl("Y", "radialY", 0, 100, "%"),
        pickerColorControl("Stop 1", "stopA"),
        pickerColorControl("Stop 2", "stopB"),
        pickerColorControl("Stop 3", "stopC"),
        sliderControl("Pos 1", "stopAOffset", 0, 100, "%"),
        sliderControl("Pos 2", "stopBOffset", 0, 100, "%"),
        sliderControl("Pos 3", "stopCOffset", 0, 100, "%")
      ]),
      stopTargetControls(),
      colorStopSwatchList("Quick stop colors", SOLID_PRESETS),
      pickerApplyButton("Use radial gradient", "background-image", radialGradientValue)
    ]);
  }

  function meshPicker() {
    return el("div", { className: "dx-devtools-picker-form" }, [
      meshStageControl(),
      el("div", { className: "dx-devtools-picker-grid" }, [
        sliderControl("X1", "meshX1", 0, 100, "%"),
        sliderControl("Y1", "meshY1", 0, 100, "%"),
        pickerColorControl("Color 1", "stopA"),
        sliderControl("X2", "meshX2", 0, 100, "%"),
        sliderControl("Y2", "meshY2", 0, 100, "%"),
        pickerColorControl("Color 2", "stopB"),
        sliderControl("X3", "meshX3", 0, 100, "%"),
        sliderControl("Y3", "meshY3", 0, 100, "%"),
        pickerColorControl("Color 3", "stopC"),
        sliderControl("Size 1", "meshSize1", 10, 100, "%"),
        sliderControl("Size 2", "meshSize2", 10, 100, "%"),
        sliderControl("Size 3", "meshSize3", 10, 100, "%")
      ]),
      stopTargetControls(),
      colorStopSwatchList("Quick mesh colors", SOLID_PRESETS),
      pickerApplyButton("Use mesh gradient", "background-image", meshGradientValue)
    ]);
  }

  function stopTargetControls() {
    const stops = [
      ["stopA", "Stop 1"],
      ["stopB", "Stop 2"],
      ["stopC", "Stop 3"]
    ];
    return el("section", { className: "dx-devtools-picker-card" }, [
      el("strong", { text: "Active color stop" }),
      el("div", { className: "dx-devtools-stop-targets", role: "group", "aria-label": "Gradient color stop target" }, stops.map(([key, label]) =>
        el("button", {
          type: "button",
          className: "dx-devtools-stop-target",
          "aria-pressed": STATE.activeColorStop === key ? "true" : "false",
          style: `--dx-devtools-swatch:${STATE.colorPicker[key]}`,
          onClick: () => {
            STATE.activeColorStop = key;
            renderAll();
          },
          text: label
        })
      ))
    ]);
  }

  function colorStopSwatchList(title, presets) {
    return el("div", { className: "dx-devtools-swatch-grid", "aria-label": title }, presets.map(([name, value]) =>
      el("button", {
        type: "button",
        className: "dx-devtools-swatch",
        style: `--dx-devtools-swatch:${value}`,
        onClick: () => {
          setColorPicker(STATE.activeColorStop, value);
          renderAll();
        },
        title: `${name} -> ${STATE.activeColorStop}: ${value}`,
        "aria-label": `${name} for active gradient stop`
      })
    ));
  }

  function colorPopoverControl(property, value) {
    if (!STATE.colorPopoverOpen) {
      return el("section", { className: "dx-devtools-picker dx-devtools-color-summary" }, [
        el("strong", { text: "Color picker" }),
        el("button", {
          type: "button",
          className: "dx-devtools-current-color",
          style: `--dx-devtools-swatch:${value || "#ededed"}`,
          "aria-label": "Open color picker",
          "aria-haspopup": "dialog",
          "aria-controls": "dx-devtools-color-popover",
          "aria-expanded": "false",
          onClick: () => {
            STATE.colorPopoverOpen = true;
            renderAll();
          },
          text: value || "current value"
        })
      ]);
    }
    const body = {
      solid: solidPicker(property),
      linear: linearPicker(),
      radial: radialPicker(),
      mesh: meshPicker()
    }[STATE.colorMode] || solidPicker(property);
    return el("section", { id: "dx-devtools-color-popover", className: "dx-devtools-picker dx-devtools-color-popover", role: "dialog", "aria-label": "Color picker" }, [
      el("strong", { text: "Color picker" }),
      el("div", { className: "dx-devtools-popover dx-devtools-popover-open" }, [
        customTextControl("Any CSS color", "data-dx-devtools-color-value", value || "", (next) => {
          STATE.styleDraft.value = next || "";
        }),
        pickerModeTabs(),
        el("div", {
          id: `dx-devtools-color-panel-${STATE.colorMode}`,
          role: "tabpanel",
          "aria-labelledby": `dx-devtools-color-tab-${STATE.colorMode}`
        }, [body]),
        swatchList("Quick gradients", "background-image", GRADIENT_PRESETS)
      ])
    ]);
  }

  function swatchList(title, property, presets) {
    return el("div", { className: "dx-devtools-swatch-grid", "aria-label": title }, presets.map(([name, value]) =>
      el("button", {
        type: "button",
        className: "dx-devtools-swatch",
        style: `--dx-devtools-swatch:${value}`,
        onClick: () => setStyleDraft(property, value),
        title: `${name}: ${value}`,
        "aria-label": `${name}: ${value}`
      })
    ));
  }

  function boxPanel() {
    const model = STATE.selectedSnapshot?.boxModel || boxModelPayload(null);
    return el("div", { className: "dx-devtools-panel-body" }, [
      metricGrid([
        ["Selector", STATE.selectedSnapshot?.selectedElement?.selector || "none"],
        ["Margin box", boxSize(model.marginBox)],
        ["Border box", boxSize(model.borderBox)],
        ["Padding box", boxSize(model.paddingBox)],
        ["Content box", boxSize(model.contentBox || model.content)],
        ["Margin", edgeSummary(model.margin)],
        ["Border", edgeSummary(model.border)],
        ["Padding", edgeSummary(model.padding)],
        ["Breakpoint", `${breakpointLabel()} (${window.innerWidth}px)`],
        ["Overlay", STATE.selectedElement ? "active" : "none"]
      ])
    ]);
  }

  function metricGrid(rows) {
    return el("dl", { className: "dx-devtools-metrics" }, rows.flatMap(([name, value]) => [
      el("dt", { text: name }),
      el("dd", { text: text(value) })
    ]));
  }

  let puckMotionFrame = 0;
  let puckPositionInitialized = false;
  let puckReleaseVelocity = { x: 0, y: 0 };

  function cancelPuckMotion() {
    if (!puckMotionFrame) return;
    cancelAnimationFrame(puckMotionFrame);
    puckMotionFrame = 0;
    puck?.classList?.remove("is-settling");
  }

  function puckBounds(size) {
    const margin = 10;
    const width = size?.width || 44;
    const height = size?.height || 44;
    return {
      minX: margin,
      minY: margin,
      maxX: Math.max(margin, window.innerWidth - width - margin),
      maxY: Math.max(margin, window.innerHeight - height - margin)
    };
  }

  function constrainedPuckPosition(position, size) {
    const bounds = puckBounds(size);
    return {
      x: Math.max(bounds.minX, Math.min(bounds.maxX, position.x)),
      y: Math.max(bounds.minY, Math.min(bounds.maxY, position.y))
    };
  }

  function nearestPuckCorner(position, size) {
    const bounds = puckBounds(size);
    const corners = [
      { x: bounds.minX, y: bounds.minY },
      { x: bounds.maxX, y: bounds.minY },
      { x: bounds.minX, y: bounds.maxY },
      { x: bounds.maxX, y: bounds.maxY }
    ];
    return corners.reduce((nearest, corner) => {
      const nearestDistance = ((nearest.x - position.x) ** 2) + ((nearest.y - position.y) ** 2);
      const cornerDistance = ((corner.x - position.x) ** 2) + ((corner.y - position.y) ** 2);
      return cornerDistance < nearestDistance ? corner : nearest;
    }, corners[0]);
  }

  function settlePuckToCorner(size, releaseVelocity) {
    const target = nearestPuckCorner(STATE.puck, size);
    let velocityX = (releaseVelocity?.x || 0) * 8;
    let velocityY = (releaseVelocity?.y || 0) * 8;
    const startedAt = performance.now();
    let lastFrameAt = startedAt;
    const stiffness = 0.048;
    const damping = 0.86;
    cancelPuckMotion();
    puck?.classList?.add("is-settling");
    const step = (now) => {
      const delta = Math.min(32, Math.max(8, now - lastFrameAt)) / 16.67;
      lastFrameAt = now;
      const remainingX = target.x - STATE.puck.x;
      const remainingY = target.y - STATE.puck.y;
      velocityX = (velocityX + remainingX * stiffness * delta) * damping;
      velocityY = (velocityY + remainingY * stiffness * delta) * damping;
      const bounds = puckBounds(size);
      let nextX = STATE.puck.x + velocityX * delta;
      let nextY = STATE.puck.y + velocityY * delta;
      if (nextX < bounds.minX || nextX > bounds.maxX) {
        nextX = Math.max(bounds.minX, Math.min(bounds.maxX, nextX));
        velocityX *= -0.18;
      }
      if (nextY < bounds.minY || nextY > bounds.maxY) {
        nextY = Math.max(bounds.minY, Math.min(bounds.maxY, nextY));
        velocityY *= -0.18;
      }
      STATE.puck = { x: nextX, y: nextY };
      renderAll({ restoreFocus: false });
      const settled = Math.hypot(remainingX, remainingY) < 0.6 && Math.hypot(velocityX, velocityY) < 0.6;
      if (settled || now - startedAt > 1500) {
        STATE.puck = constrainedPuckPosition(target, size);
        puckMotionFrame = 0;
        puck?.classList?.remove("is-settling");
        renderAll({ restoreFocus: false });
        return;
      }
      puckMotionFrame = requestAnimationFrame(step);
    };
    puckMotionFrame = requestAnimationFrame(step);
  }

  function renderAll(options) {
    options = options || {};
    if (!root) return;
    const focusToken = options.focusToken ?? captureFocusToken();
    root.dataset.openPanels = STATE.openPanels.size ? Array.from(STATE.openPanels).join(" ") : "none";
    const panelLayer = root.querySelector("[data-dx-devtools-panels]");
    panelLayer.replaceChildren(...EDGES.map(panel));
    if (!puckPositionInitialized) {
      const bounds = puckBounds(puck?.getBoundingClientRect?.());
      STATE.puck = { x: bounds.maxX, y: bounds.minY };
      puckPositionInitialized = true;
    }
    STATE.puck = constrainedPuckPosition(STATE.puck, puck?.getBoundingClientRect?.());
    puck.style.left = `${STATE.puck.x}px`;
    puck.style.top = `${STATE.puck.y}px`;
    puck.classList.toggle("is-dragging", STATE.dragging);
    puck.classList.toggle("is-panel-open", STATE.openPanels.size > 0);
    menu.style.left = `${Math.max(8, Math.min(window.innerWidth - 208, STATE.puck.x))}px`;
    menu.style.top = `${Math.max(8, Math.min(window.innerHeight - 260, STATE.puck.y + 52))}px`;
    menu.hidden = !STATE.menuOpen;
    menu.classList.toggle("is-open", STATE.menuOpen);
    puck.setAttribute("aria-expanded", STATE.menuOpen ? "true" : "false");
    updateBoxOverlay();
    if (options.restoreFocus !== false) restoreFocusToken(focusToken);
    if (STATE.menuOpen && focusToken === "puck") focusFirstMenuItem();
  }

  function setInspectMode(next) {
    STATE.inspectMode = next;
    document.documentElement.classList.toggle("dx-devtools-inspecting", next);
    renderAll();
  }

  function onDocumentPointerMove(event) {
    if (!STATE.inspectMode) return;
    const target = event.target;
    if (!target || root.contains(target)) return;
    if (target === inspectedTarget) return;
    inspectedTarget = target;
    captureElement(target);
  }

  function onDocumentClick(event) {
    if (!STATE.inspectMode) return;
    const target = event.target;
    if (!target || root.contains(target)) return;
    event.preventDefault();
    event.stopPropagation();
    inspectedTarget = target;
    captureElement(target);
    setInspectMode(false);
  }

  function buildRoot() {
    root = document.getElementById(ROOT_ID);
    if (root) return;
    root = el("section", {
      id: ROOT_ID,
      className: "dx-devtools-root",
      "data-dx-devtools-root": "true",
      "data-open-panels": "left"
    });
    const panelLayer = el("div", { "data-dx-devtools-panels": "true" });
    puck = el("button", {
      type: "button",
      className: "dx-devtools-puck",
      "data-dx-devtools-puck": "true",
      "data-dx-devtools-focus": "puck",
      "aria-label": "DX Devtools menu",
      "aria-controls": "dx-devtools-puck-menu",
      "aria-expanded": STATE.menuOpen ? "true" : "false",
      "aria-haspopup": "menu",
      onPointerdown: startPuckDrag,
      onClick: () => {
        if (STATE.dragMoved) {
          STATE.dragMoved = false;
          return;
        }
        STATE.menuOpen = !STATE.menuOpen;
        renderAll({ focusToken: STATE.menuOpen ? "puck" : "puck" });
        if (!STATE.menuOpen) puck?.focus?.({ preventScroll: true });
      }
    }, [el("span", { text: "DX" })]);
    const menuAction = (action) => () => {
      STATE.menuOpen = false;
      action();
    };
    menu = el("div", { id: "dx-devtools-puck-menu", className: "dx-devtools-puck-menu", role: "menu", hidden: true, onKeydown: onMenuKeydown }, [
      el("button", { type: "button", role: "menuitem", tabindex: "0", "data-dx-devtools-focus": "menu-inspect", onClick: () => { STATE.menuOpen = false; openPanel("left"); setInspectMode(true); }, text: "Inspect element" }),
      el("button", { type: "button", role: "menuitem", tabindex: "-1", "data-dx-devtools-focus": "menu-route", onClick: menuAction(() => openPanel("top")), text: "Route dock" }),
      el("button", { type: "button", role: "menuitem", tabindex: "-1", "data-dx-devtools-focus": "menu-style", onClick: menuAction(() => openPanel("left")), text: "Style dock" }),
      el("button", { type: "button", role: "menuitem", tabindex: "-1", "data-dx-devtools-focus": "menu-preview", onClick: menuAction(() => openPanel("right")), text: "Preview insert" }),
      el("button", { type: "button", role: "menuitem", tabindex: "-1", "data-dx-devtools-focus": "menu-box", onClick: menuAction(() => openPanel("bottom")), text: "Box model" }),
      el("button", { type: "button", role: "menuitem", tabindex: "-1", "data-dx-devtools-focus": "menu-error", onClick: menuAction(() => window.__DX_SHOW_ERROR__?.({ message: "DX Devtools opened error overlay." })), text: "Error overlay" }),
      el("button", { type: "button", role: "menuitem", tabindex: "-1", "data-dx-devtools-focus": "menu-hide", onClick: () => { STATE.openPanels.clear(); STATE.menuOpen = false; renderAll({ focusToken: "puck" }); }, text: "Hide panels" })
    ]);
    overlay = el("div", { className: "dx-devtools-box-overlay", "data-dx-devtools-overlay": "box-model", hidden: true });
    root.append(panelLayer, puck, menu, overlay);
    document.body.append(root);
    readinessPublishVisualEditReplay();
    root.addEventListener("click", (event) => {
      const closeButton = event.target?.closest?.("[data-dx-devtools-close]");
      if (!closeButton) return;
      event.preventDefault();
      event.stopPropagation();
      closePanel(closeButton.dataset.dxDevtoolsClose);
    }, true);
    renderAll();
  }

  function startPuckDrag(event) {
    cancelPuckMotion();
    STATE.dragging = true;
    STATE.dragMoved = false;
    const rect = puck.getBoundingClientRect();
    const offsetX = event.clientX - rect.left;
    const offsetY = event.clientY - rect.top;
    let lastSample = { x: event.clientX, y: event.clientY, t: performance.now() };
    puckReleaseVelocity = { x: 0, y: 0 };
    puck.setPointerCapture?.(event.pointerId);
    const move = (moveEvent) => {
      STATE.dragMoved = true;
      const now = performance.now();
      const delta = Math.max(8, now - lastSample.t);
      puckReleaseVelocity = {
        x: (moveEvent.clientX - lastSample.x) / delta,
        y: (moveEvent.clientY - lastSample.y) / delta
      };
      lastSample = { x: moveEvent.clientX, y: moveEvent.clientY, t: now };
      STATE.puck = constrainedPuckPosition({
        x: moveEvent.clientX - offsetX,
        y: moveEvent.clientY - offsetY
      }, rect);
      renderAll();
    };
    const end = () => {
      const shouldSettle = STATE.dragMoved;
      STATE.dragging = false;
      window.removeEventListener("pointermove", move, true);
      window.removeEventListener("pointerup", end, true);
      window.removeEventListener("pointercancel", end, true);
      window.setTimeout(() => { STATE.dragMoved = false; }, 120);
      renderAll();
    };
    window.addEventListener("pointermove", move, true);
    window.addEventListener("pointerup", end, true);
    window.addEventListener("pointercancel", end, true);
    renderAll();
  }

  window.addEventListener("resize", () => {
    if (STATE.selectedElement) captureElement(STATE.selectedElement);
    renderAll();
  }, { signal: listenerSignal });
  window.addEventListener("error", (event) => {
    STATE.runtimeIssues.push(event.message || "runtime error");
    if (STATE.runtimeIssues.length > 20) STATE.runtimeIssues.splice(0, STATE.runtimeIssues.length - 20);
    renderAll();
  }, { signal: listenerSignal });
  window.addEventListener("unhandledrejection", (event) => {
    STATE.runtimeIssues.push(event.reason?.message || String(event.reason || "unhandled rejection"));
    if (STATE.runtimeIssues.length > 20) STATE.runtimeIssues.splice(0, STATE.runtimeIssues.length - 20);
    renderAll();
  }, { signal: listenerSignal });
  window.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      const shouldHandleEscape = STATE.menuOpen || hasOpenPopover() || STATE.inspectMode;
      if (shouldHandleEscape) event.preventDefault();
      if (STATE.menuOpen) {
        STATE.menuOpen = false;
        closePopovers();
        renderAll({ focusToken: "puck" });
        puck?.focus?.({ preventScroll: true });
        return;
      }
      closePopovers();
      if (STATE.inspectMode) {
        setInspectMode(false);
      } else {
        renderAll();
      }
      return;
    }
  }, { signal: listenerSignal });
  document.addEventListener("pointermove", onDocumentPointerMove, { capture: true, signal: listenerSignal });
  document.addEventListener("click", onDocumentClick, { capture: true, signal: listenerSignal });

  buildRoot();
  window.__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__ = readinessRunVisualEditReplay;
  refreshProtocol().catch((error) => {
    STATE.runtimeIssues.push(`devtools protocol unavailable: ${error.message}`);
    if (STATE.runtimeIssues.length > 20) STATE.runtimeIssues.splice(0, STATE.runtimeIssues.length - 20);
    renderAll();
  });
  window.__DX_DEVTOOLS_CLEANUP__ = () => {
    controller.abort();
    if (renderFrame) cancelAnimationFrame(renderFrame);
    cancelPuckMotion();
    root?.remove?.();
    document.documentElement.classList.remove("dx-devtools-inspecting");
    window.__DX_DEVTOOLS_ACTIVE__ = false;
    delete window.__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__;
    readinessPublishVisualEditReplay();
  };
  window.dispatchEvent(new CustomEvent("dx-devtools:ready", {
    detail: { protocol: "dx.devtools.runtime", source: "dx-www-rust-dev-server" }
  }));
})();
