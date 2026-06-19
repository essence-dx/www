          className: "dx-devtools-mini-button",
          role: "combobox",
          "aria-label": `${label} options`,
          "aria-haspopup": "listbox",
          "aria-controls": optionsId,
          "aria-activedescendant": activeDescendant,
          "aria-expanded": open ? "true" : "false",
          "data-dx-devtools-focus": `${marker}-options`,
          onClick: () => {
            closePopovers(open ? "" : "valuePopoverOpen");
            if (!open) setActiveOptionIndex("valuePopoverOpen", visibleOptions, activeIndex);
            STATE.valuePopoverOpen = !STATE.valuePopoverOpen;
            renderAll();
          },
          onKeydown: (event) => onSelectKeydown(event, "valuePopoverOpen", visibleOptions, value, pickValueOption),
          text: "Options"
        }) : null,
        showColorPopover ? el("button", {
          type: "button",
          className: "dx-devtools-color-trigger",
          style: `--dx-devtools-swatch:${value || "#ededed"}`,
          "aria-label": "Open color picker",
          "aria-haspopup": "dialog",
          "aria-controls": colorDialogId,
          "aria-expanded": STATE.colorPopoverOpen ? "true" : "false",
          "data-dx-devtools-focus": `${marker}-color`,
          onClick: () => {
            closePopovers(STATE.colorPopoverOpen ? "" : "colorPopoverOpen");
            STATE.colorPopoverOpen = !STATE.colorPopoverOpen;
            renderAll();
          }
        }) : null
      ].filter(Boolean)),
      open ? el("div", { className: "dx-devtools-popover" }, [
        customPopoverSearch("Search values", `${marker}-options`, "valuePopoverOpen", options, value, pickValueOption),
        visibleOptions.length
          ? el("div", { className: "dx-devtools-option-list", id: optionsId, role: "listbox" }, visibleOptions.map((option, index) =>
            el("button", {
              id: selectOptionId(optionsId, index),
              type: "button",
              role: "option",
              "aria-selected": option === value ? "true" : "false",
              "data-dx-devtools-active": index === activeIndex ? "true" : "false",
              "data-dx-devtools-focus": `${marker}-options-option-${index}`,
              tabIndex: "-1",
              onClick: () => pickValueOption(option),
              text: option
            })
          ))
          : el("p", { className: "dx-devtools-empty", text: "No matching CSS values" })
      ]) : null
    ].filter(Boolean));
  }

  function clampNumber(value, min, max) {
    const next = Number.parseFloat(value);
    if (!Number.isFinite(next)) return min;
    return Math.max(min, Math.min(max, next));
  }

  function setColorPicker(key, value, min, max) {
    STATE.colorPicker[key] = min == null ? String(value || "") : clampNumber(value, min, max);
    if (key === "hue" || key === "saturation" || key === "lightness") syncRgbFromHsl();
  }

  function pointerPercent(event, node) {
    const rect = node.getBoundingClientRect();
    return pointerPercentFromRect(event, rect);
  }

  function pointerPercentFromRect(event, rect) {
    if (!rect.width || !rect.height) return { x: 0, y: 0 };
    return {
      x: Math.max(0, Math.min(100, ((event.clientX - rect.left) / rect.width) * 100)),
      y: Math.max(0, Math.min(100, ((event.clientY - rect.top) / rect.height) * 100))
    };
  }

  function scheduleRenderAll() {
    if (renderFrame) return;
    renderFrame = requestAnimationFrame(() => {
      renderFrame = 0;
      renderAll();
    });
  }

  function pickerReadout(key, value, suffix = "") {
    const node = root?.querySelector?.(`[data-dx-devtools-picker-${key}]`);
    if (node) node.textContent = `${Math.round(value)}${suffix}`;
  }

  function applySliderVisual(slider, key, min, max, suffix = "") {
    const value = clampNumber(STATE.colorPicker[key], min, max);
    const fill = ((value - min) / Math.max(1, max - min)) * 100;
    slider.style.setProperty("--dx-devtools-slider-fill", `${fill}%`);
    slider.setAttribute("aria-valuenow", String(Math.round(value)));
    slider.setAttribute("aria-valuetext", `${Math.round(value)}${suffix}`);
    pickerReadout(key, value, suffix);
  }

  function adjustColorPicker(key, delta, min, max) {
    const next = clampNumber(STATE.colorPicker[key], min, max) + delta;
    setColorPicker(key, next, min, max);
    renderAll();
  }

  function onSliderKeydown(event, key, min, max) {
    const range = Math.max(1, max - min);
    const normalStep = Math.max(1, Math.round(range / 100));
    const pageStep = Math.max(5, Math.round(range / 10));
    const step = event.shiftKey || event.key === "PageUp" || event.key === "PageDown" ? pageStep : normalStep;
    if (event.key === "ArrowRight" || event.key === "ArrowUp") {
      event.preventDefault();
      adjustColorPicker(key, step, min, max);
    } else if (event.key === "ArrowLeft" || event.key === "ArrowDown") {
      event.preventDefault();
      adjustColorPicker(key, -step, min, max);
    } else if (event.key === "PageUp") {
      event.preventDefault();
      adjustColorPicker(key, pageStep, min, max);
    } else if (event.key === "PageDown") {
      event.preventDefault();
      adjustColorPicker(key, -pageStep, min, max);
    } else if (event.key === "Home") {
      event.preventDefault();
      setColorPicker(key, min, min, max);
      renderAll();
    } else if (event.key === "End") {
      event.preventDefault();
      setColorPicker(key, max, min, max);
      renderAll();
    }
  }

  function onPositionKeydown(event, xKey, yKey) {
    const step = event.shiftKey || event.key === "PageUp" || event.key === "PageDown" ? 10 : 1;
    let nextX = clampNumber(STATE.colorPicker[xKey], 0, 100);
    let nextY = clampNumber(STATE.colorPicker[yKey], 0, 100);
    if (event.key === "ArrowRight") nextX += step;
    else if (event.key === "ArrowLeft") nextX -= step;
    else if (event.key === "ArrowDown" || event.key === "PageDown") nextY += step;
    else if (event.key === "ArrowUp" || event.key === "PageUp") nextY -= step;
    else if (event.key === "Home") {
      nextX = 0;
      nextY = 0;
    } else if (event.key === "End") {
      nextX = 100;
      nextY = 100;
    } else {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    setColorPicker(xKey, nextX, 0, 100);
    setColorPicker(yKey, nextY, 0, 100);
    renderAll();
  }

  function startTrackDrag(event, key, min, max) {
    event.preventDefault();
    event.stopPropagation();
    const slider = event.currentTarget;
    const rect = slider.getBoundingClientRect();
    const suffix = slider.dataset.dxDevtoolsSliderSuffix || "";
    slider.setPointerCapture?.(event.pointerId);
    const update = (nextEvent) => {
      const percent = pointerPercentFromRect(nextEvent, rect).x;
      setColorPicker(key, min + ((max - min) * percent / 100), min, max);
      applySliderVisual(slider, key, min, max, suffix);
    };
    const stop = () => {
      window.removeEventListener("pointermove", update, true);
      window.removeEventListener("pointerup", stop, true);
      window.removeEventListener("pointercancel", stop, true);
      renderAll();
    };
    update(event);
    window.addEventListener("pointermove", update, true);
    window.addEventListener("pointerup", stop, true);
    window.addEventListener("pointercancel", stop, true);
  }

  function startAngleDialDrag(event) {
    event.preventDefault();
    event.stopPropagation();
    const dial = event.currentTarget;
    const rect = dial.getBoundingClientRect();
    dial.setPointerCapture?.(event.pointerId);
    const update = (nextEvent) => {
      const x = nextEvent.clientX - (rect.left + rect.width / 2);
      const y = nextEvent.clientY - (rect.top + rect.height / 2);
      const degrees = (Math.atan2(y, x) * 180 / Math.PI + 450) % 360;
      setColorPicker("linearAngle", degrees, 0, 360);
      dial.style.setProperty("--dx-devtools-angle", `${Math.round(degrees)}deg`);
      dial.setAttribute("aria-valuenow", String(Math.round(degrees)));
      dial.setAttribute("aria-valuetext", `${Math.round(degrees)} degrees`);
      const label = dial.querySelector("span");
      if (label) label.textContent = `${Math.round(degrees)}deg`;
      pickerReadout("linearAngle", degrees, "deg");
    };
    const stop = () => {
      window.removeEventListener("pointermove", update, true);
      window.removeEventListener("pointerup", stop, true);
      window.removeEventListener("pointercancel", stop, true);
      renderAll();
    };
    update(event);
    window.addEventListener("pointermove", update, true);
    window.addEventListener("pointerup", stop, true);
    window.addEventListener("pointercancel", stop, true);
  }

  function startPositionPadDrag(event, xKey, yKey) {
    event.preventDefault();
    event.stopPropagation();
    const handle = event.currentTarget;
    const pad = handle.closest(".dx-devtools-mesh-stage") || handle.closest("[data-dx-devtools-position-pad]") || handle;
    const rect = pad.getBoundingClientRect();
    handle.setPointerCapture?.(event.pointerId);
    const update = (nextEvent) => {
      const percent = pointerPercentFromRect(nextEvent, rect);
      setColorPicker(xKey, percent.x, 0, 100);
      setColorPicker(yKey, percent.y, 0, 100);
      handle.style.setProperty("--dx-devtools-pad-x", `${Math.round(percent.x)}%`);
      handle.style.setProperty("--dx-devtools-pad-y", `${Math.round(percent.y)}%`);
      handle.setAttribute("aria-valuenow", String(Math.round(percent.x)));
      handle.setAttribute("aria-valuetext", `X ${Math.round(percent.x)}%, Y ${Math.round(percent.y)}%`);
      if (pad.classList.contains("dx-devtools-mesh-stage")) {
        pad.style.setProperty("--dx-devtools-swatch", meshGradientValue());
      }
      pickerReadout(xKey, percent.x, "%");
      pickerReadout(yKey, percent.y, "%");
    };
    const stop = () => {
      window.removeEventListener("pointermove", update, true);
      window.removeEventListener("pointerup", stop, true);
      window.removeEventListener("pointercancel", stop, true);
      renderAll();
    };
    update(event);
    window.addEventListener("pointermove", update, true);
    window.addEventListener("pointerup", stop, true);
    window.addEventListener("pointercancel", stop, true);
  }

  function pickerValueControl(label, key, min, max, suffix = "") {
    return customTextControl(label, `data-dx-devtools-picker-${key}`, `${STATE.colorPicker[key]}${suffix}`, (next) => {
      setColorPicker(key, String(next).replace(suffix, ""), min, max);
    });
  }

  function sliderControl(label, key, min, max, suffix = "") {
    const value = clampNumber(STATE.colorPicker[key], min, max);
    const fill = ((value - min) / Math.max(1, max - min)) * 100;
    return el("div", { className: "dx-devtools-channel-control" }, [
      pickerValueControl(label, key, min, max, suffix),
      el("button", {
        type: "button",
        className: "dx-devtools-slider",
        role: "slider",
        "aria-label": label,
        "aria-orientation": "horizontal",
        "aria-valuemin": min,
        "aria-valuemax": max,
        "aria-valuenow": Math.round(value),
        "aria-valuetext": `${Math.round(value)}${suffix}`,
        "data-dx-devtools-focus": `slider-${key}`,
        "data-dx-devtools-slider-suffix": suffix,
        style: `--dx-devtools-slider-fill:${fill}%`,
        onPointerdown: (event) => startTrackDrag(event, key, min, max),
        onKeydown: (event) => onSliderKeydown(event, key, min, max)
      }, [
        el("span", { className: "dx-devtools-slider-track" }),
        el("span", { className: "dx-devtools-slider-thumb" })
      ])
    ]);
  }

  function pickerColorControl(label, key) {
    return customTextControl(label, `data-dx-devtools-picker-${key}`, STATE.colorPicker[key], (next) => {
      setColorPicker(key, next);
    });
  }

  function rgbaPickerValue() {
    const { red, green, blue, alpha } = STATE.colorPicker;
    return `rgb(${Math.round(red)} ${Math.round(green)} ${Math.round(blue)} / ${Math.round(alpha)}%)`;
  }

  function hslPickerValue() {
    const { hue, saturation, lightness, alpha } = STATE.colorPicker;
    return `hsl(${Math.round(hue)} ${Math.round(saturation)}% ${Math.round(lightness)}% / ${Math.round(alpha)}%)`;
  }

  function linearGradientValue() {
    const { linearAngle, stopA, stopB, stopC, stopAOffset, stopBOffset, stopCOffset } = STATE.colorPicker;
    return `linear-gradient(${Math.round(linearAngle)}deg, ${stopA} ${Math.round(stopAOffset)}%, ${stopB} ${Math.round(stopBOffset)}%, ${stopC} ${Math.round(stopCOffset)}%)`;
  }

  function radialGradientValue() {
    const { radialShape, radialX, radialY, stopA, stopB, stopC, stopAOffset, stopBOffset, stopCOffset } = STATE.colorPicker;
    return `radial-gradient(${radialShape} at ${Math.round(radialX)}% ${Math.round(radialY)}%, ${stopA} ${Math.round(stopAOffset)}%, ${stopB} ${Math.round(stopBOffset)}%, ${stopC} ${Math.round(stopCOffset)}%)`;
  }

  function meshGradientValue() {
    const { meshX1, meshY1, meshX2, meshY2, meshX3, meshY3, meshSize1, meshSize2, meshSize3, stopA, stopB, stopC } = STATE.colorPicker;
    return `radial-gradient(circle at ${Math.round(meshX1)}% ${Math.round(meshY1)}%, ${stopA} 0 15%, transparent ${Math.round(meshSize1)}%), radial-gradient(circle at ${Math.round(meshX2)}% ${Math.round(meshY2)}%, ${stopB} 0 15%, transparent ${Math.round(meshSize2)}%), radial-gradient(circle at ${Math.round(meshX3)}% ${Math.round(meshY3)}%, ${stopC} 0 18%, transparent ${Math.round(meshSize3)}%)`;
  }

  function pickerModeTabs() {
    const modes = ["solid", "linear", "radial", "mesh"];
    return el("div", { className: "dx-devtools-picker-modes", role: "tablist", "aria-label": "Color picker mode" }, modes.map((mode) =>
      el("button", {
        type: "button",
        id: `dx-devtools-color-tab-${mode}`,
        "data-dx-devtools-focus": `color-tab-${mode}`,
        role: "tab",
        "aria-selected": STATE.colorMode === mode ? "true" : "false",
        "aria-controls": `dx-devtools-color-panel-${mode}`,
        tabindex: STATE.colorMode === mode ? "0" : "-1",
        onClick: () => {
          STATE.colorMode = mode;
          renderAll({ focusToken: `color-tab-${mode}` });
        },
        onKeydown: (event) => onTabKeydown(event, modes, STATE.colorMode, (next) => {
          STATE.colorMode = next;
          renderAll({ focusToken: `color-tab-${next}` });
        }),
        text: mode
      })
    ));
  }

  function pickerApplyButton(label, property, valueFactory) {
    return el("button", {
      type: "button",
      className: "dx-devtools-primary-action",
      onClick: () => setStyleDraft(property, valueFactory()),
      text: label
    });
  }

  function angleDialControl() {
    return el("button", {
      type: "button",
      className: "dx-devtools-angle-dial",
      role: "slider",
      "aria-label": "Linear gradient angle",
      "aria-orientation": "horizontal",
      "aria-valuemin": "0",
      "aria-valuemax": "360",
      "aria-valuenow": Math.round(STATE.colorPicker.linearAngle),
      "aria-valuetext": `${Math.round(STATE.colorPicker.linearAngle)} degrees`,
      "data-dx-devtools-focus": "linear-angle-dial",
      style: `--dx-devtools-angle:${Math.round(STATE.colorPicker.linearAngle)}deg`,
      onPointerdown: startAngleDialDrag,
      onKeydown: (event) => onSliderKeydown(event, "linearAngle", 0, 360)
    }, [
      el("span", { text: `${Math.round(STATE.colorPicker.linearAngle)}deg` })
    ]);
  }

  function positionPadControl(label, xKey, yKey) {
    const x = clampNumber(STATE.colorPicker[xKey], 0, 100);
    const y = clampNumber(STATE.colorPicker[yKey], 0, 100);
    return el("section", { className: "dx-devtools-picker-card" }, [
      el("strong", { text: label }),
      el("button", {
        type: "button",
        className: "dx-devtools-position-pad",
        "data-dx-devtools-position-pad": "true",
        "data-dx-devtools-focus": `position-${xKey}-${yKey}`,
        role: "slider",
        "aria-label": label,
        "aria-orientation": "horizontal",
        "aria-valuemin": "0",
        "aria-valuemax": "100",
        "aria-valuenow": Math.round(x),
        "aria-valuetext": `${label}: X ${Math.round(x)}%, Y ${Math.round(y)}%`,
        style: `--dx-devtools-pad-x:${x}%;--dx-devtools-pad-y:${y}%`,
        onPointerdown: (event) => startPositionPadDrag(event, xKey, yKey),
        onKeydown: (event) => onPositionKeydown(event, xKey, yKey)
      }, [
        el("span", { className: "dx-devtools-position-dot" })
      ])
    ]);
  }

  function meshStageControl() {
    const layers = [
      ["meshX1", "meshY1", "stopA", "Layer 1"],
      ["meshX2", "meshY2", "stopB", "Layer 2"],
      ["meshX3", "meshY3", "stopC", "Layer 3"]
    ];
    return el("section", { className: "dx-devtools-picker-card" }, [
      el("strong", { text: "Mesh positions" }),
      el("div", { className: "dx-devtools-mesh-stage", style: `--dx-devtools-swatch:${meshGradientValue()}` }, layers.map(([xKey, yKey, colorKey, label]) =>
        el("button", {
          type: "button",
          className: "dx-devtools-mesh-layer",
          "data-dx-devtools-position-pad": "true",
          "data-dx-devtools-focus": `mesh-${xKey}-${yKey}`,
          role: "slider",
          "aria-label": `${label} position`,
          "aria-orientation": "horizontal",
          "aria-valuemin": "0",
          "aria-valuemax": "100",
          "aria-valuenow": Math.round(clampNumber(STATE.colorPicker[xKey], 0, 100)),
          "aria-valuetext": `${label}: X ${Math.round(clampNumber(STATE.colorPicker[xKey], 0, 100))}%, Y ${Math.round(clampNumber(STATE.colorPicker[yKey], 0, 100))}%`,
          style: `--dx-devtools-pad-x:${clampNumber(STATE.colorPicker[xKey], 0, 100)}%;--dx-devtools-pad-y:${clampNumber(STATE.colorPicker[yKey], 0, 100)}%;--dx-devtools-swatch:${STATE.colorPicker[colorKey]}`,
          onPointerdown: (event) => startPositionPadDrag(event, xKey, yKey),
          onKeydown: (event) => onPositionKeydown(event, xKey, yKey)
        })
      ))
    ]);
  }

  function hslToRgb(hue, saturation, lightness) {
    const h = (((hue % 360) + 360) % 360) / 360;
    const s = Math.max(0, Math.min(1, saturation / 100));
    const l = Math.max(0, Math.min(1, lightness / 100));
    if (s === 0) {
      const channel = Math.round(l * 255);
      return [channel, channel, channel];
    }
    const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
    const p = 2 * l - q;
    const channel = (offset) => {
      let t = h + offset;
      if (t < 0) t += 1;
      if (t > 1) t -= 1;
      if (t < 1 / 6) return p + (q - p) * 6 * t;
      if (t < 1 / 2) return q;
      if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
      return p;
    };
    return [channel(1 / 3), channel(0), channel(-1 / 3)].map((value) => Math.round(value * 255));
  }

  function syncRgbFromHsl() {
    const [red, green, blue] = hslToRgb(
      clampNumber(STATE.colorPicker.hue, 0, 360),
      clampNumber(STATE.colorPicker.saturation, 0, 100),
      clampNumber(STATE.colorPicker.lightness, 0, 100)
    );
    STATE.colorPicker.red = red;
    STATE.colorPicker.green = green;
    STATE.colorPicker.blue = blue;
  }

  function spectrumStyle() {
    const x = clampNumber(STATE.colorPicker.saturation, 0, 100);
    const y = 100 - clampNumber(STATE.colorPicker.lightness, 0, 100);
    return [
      `--dx-devtools-spectrum-hue:${Math.round(clampNumber(STATE.colorPicker.hue, 0, 360))}`,
      `--dx-devtools-pad-x:${Math.round(x)}%`,
      `--dx-devtools-pad-y:${Math.round(y)}%`,
      `--dx-devtools-swatch:${hslPickerValue()}`
    ].join(";");
  }

  function setSpectrumPoint(x, y, targetProperty) {
    setColorPicker("saturation", x, 0, 100);
    setColorPicker("lightness", 100 - y, 0, 100);
    STATE.styleDraft = { property: targetProperty, value: hslPickerValue() };
  }

  function applySpectrumVisual(surface, x, y, targetProperty) {
    setSpectrumPoint(x, y, targetProperty);
    surface.style.setProperty("--dx-devtools-pad-x", `${Math.round(x)}%`);
    surface.style.setProperty("--dx-devtools-pad-y", `${Math.round(y)}%`);
    surface.style.setProperty("--dx-devtools-swatch", hslPickerValue());
    surface.setAttribute("aria-valuenow", String(Math.round(x)));
    surface.setAttribute("aria-valuetext", `Saturation ${Math.round(x)}%, lightness ${Math.round(100 - y)}%`);
    const readout = surface.querySelector("[data-dx-devtools-spectrum-readout]");
    if (readout) readout.textContent = hslPickerValue();
    pickerReadout("saturation", x, "%");
    pickerReadout("lightness", 100 - y, "%");
  }

  function startSpectrumDrag(event) {
    event.preventDefault();
    event.stopPropagation();
    const surface = event.currentTarget;
    const targetProperty = surface.dataset.dxDevtoolsSpectrumTarget || STATE.styleDraft.property || "color";
    const rect = surface.getBoundingClientRect();
    surface.setPointerCapture?.(event.pointerId);
    const update = (nextEvent) => {
      const percent = pointerPercentFromRect(nextEvent, rect);
      applySpectrumVisual(surface, percent.x, percent.y, targetProperty);
    };
    const stop = () => {
      window.removeEventListener("pointermove", update, true);
      window.removeEventListener("pointerup", stop, true);
      window.removeEventListener("pointercancel", stop, true);
      renderAll();
    };
    update(event);
    window.addEventListener("pointermove", update, true);
    window.addEventListener("pointerup", stop, true);
    window.addEventListener("pointercancel", stop, true);
  }

  function onSpectrumKeydown(event) {
    const surface = event.currentTarget;
    const targetProperty = surface.dataset.dxDevtoolsSpectrumTarget || STATE.styleDraft.property || "color";
    const step = event.shiftKey || event.key === "PageUp" || event.key === "PageDown" ? 10 : 1;
    let x = clampNumber(STATE.colorPicker.saturation, 0, 100);
    let y = 100 - clampNumber(STATE.colorPicker.lightness, 0, 100);
    if (event.key === "ArrowRight") x += step;
    else if (event.key === "ArrowLeft") x -= step;
    else if (event.key === "ArrowDown" || event.key === "PageDown") y += step;
    else if (event.key === "ArrowUp" || event.key === "PageUp") y -= step;
    else if (event.key === "Home") {
      x = 0;
      y = 100;
    } else if (event.key === "End") {
      x = 100;
      y = 0;
    } else {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    applySpectrumVisual(surface, Math.max(0, Math.min(100, x)), Math.max(0, Math.min(100, y)), targetProperty);
    renderAll();
  }

  function colorSpectrumControl(targetProperty) {
    const saturation = clampNumber(STATE.colorPicker.saturation, 0, 100);
    const lightness = clampNumber(STATE.colorPicker.lightness, 0, 100);
    return el("button", {
      type: "button",
      className: "dx-devtools-spectrum",
      role: "slider",
      "aria-label": "Saturation and lightness color surface",
      "aria-orientation": "horizontal",
      "aria-valuemin": "0",
      "aria-valuemax": "100",
      "aria-valuenow": Math.round(saturation),
      "aria-valuetext": `Saturation ${Math.round(saturation)}%, lightness ${Math.round(lightness)}%`,
      "data-dx-devtools-spectrum": "true",
      "data-dx-devtools-spectrum-target": targetProperty,
      "data-dx-devtools-focus": "solid-color-spectrum",
      style: spectrumStyle(),
      onPointerdown: startSpectrumDrag,
      onKeydown: onSpectrumKeydown
    }, [
      el("span", { className: "dx-devtools-spectrum-thumb" }),
      el("span", { className: "dx-devtools-spectrum-readout", "data-dx-devtools-spectrum-readout": "true", text: hslPickerValue() })
    ]);
  }

  function solidPicker(property) {
    const targetProperty = property === "background-image" ? "background-color" : property;
    return el("div", { className: "dx-devtools-picker-form" }, [
      el("section", { className: "dx-devtools-picker-card dx-devtools-spectrum-card" }, [
        el("strong", { text: "Color surface" }),
        colorSpectrumControl(targetProperty),
        pickerApplyButton("Use picked color", targetProperty, hslPickerValue)
      ]),
      el("section", { className: "dx-devtools-picker-card" }, [
