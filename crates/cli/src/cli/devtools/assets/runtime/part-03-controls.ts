      "data-dx-devtools-panel": edge,
      "data-open": isOpen ? "true" : "false",
      "aria-hidden": isOpen ? "false" : "true",
      hidden: !isOpen,
      inert: !isOpen,
      "aria-label": `DX Devtools ${title}`
    }, [
      el("header", { className: "dx-devtools-panel-header" }, [
        el("div", {}, [
          el("strong", { text: title }),
          el("span", { text: panelSubtitle(edge) })
        ]),
        el("button", {
          type: "button",
          "data-dx-devtools-close": edge,
          "aria-label": `Close ${title} panel`,
          onPointerdown: (event) => {
            event.preventDefault();
            event.stopPropagation();
          },
          onClick: (event) => {
            event.preventDefault();
            event.stopPropagation();
            closePanel(edge);
          },
          text: `Close ${title}`
        })
      ]),
      panelBody(edge)
    ]);
  }

  function panelSubtitle(edge) {
    if (edge === "left") return "Style, layout, spacing, responsive, source";
    if (edge === "top") return "Route, status, diagnostics";
    if (edge === "bottom") return "Margin, border, padding, content";
    return "Preview insert and selection state";
  }

  function panelBody(edge) {
    if (edge === "top") return routePanel();
    if (edge === "left") return inspectorPanel();
    if (edge === "bottom") return boxPanel();
    return previewPanel();
  }

  function routePanel() {
    const route = STATE.route || {};
    const diagnostics = STATE.diagnostics || {};
    const selected = route.selected || {};
    return el("div", { className: "dx-devtools-panel-body" }, [
      metricGrid([
        ["Route", pathOnly()],
        ["Matched", route.matched ? "yes" : "preview-only"],
        ["Source", selected.source_path || "preview-only / not writable"],
        ["Diagnostics", diagnostics.issue_count ?? 0],
        ["Hot reload", STATE.session?.endpoints?.hot_reload_version || "/_dx/hot-reload/version"],
        ["Devtools", STATE.session?.enabled === false ? "disabled" : "enabled"]
      ]),
      el("section", { className: "dx-devtools-list" }, [
        el("strong", { text: "Routes" }),
        ...(route.routes || []).slice(0, 8).map((item) =>
          el("button", {
            type: "button",
            onClick: () => {
              window.history.pushState(null, "", item.route);
              refreshProtocol();
            },
            text: `${item.route} - ${item.source_path}`
          })
        )
      ]),
      el("section", { className: "dx-devtools-list" }, [
        el("strong", { text: "Diagnostics" }),
        ...((diagnostics.issues || []).slice(0, 5).map((issue) =>
          el("span", { text: issue.message || issue.code || JSON.stringify(issue).slice(0, 120) })
        )),
        ...(STATE.runtimeIssues.slice(-4).map((issue) => el("span", { text: issue })))
      ])
    ]);
  }

  function previewPanel() {
    const snapshot = STATE.selectedSnapshot;
    const source = snapshot?.source || {};
    return el("div", { className: "dx-devtools-panel-body" }, [
      el("div", { className: "dx-devtools-actions" }, [
        el("button", {
          type: "button",
          className: "dx-devtools-primary-action",
          "aria-pressed": STATE.inspectMode ? "true" : "false",
          onClick: () => setInspectMode(!STATE.inspectMode),
          text: STATE.inspectMode ? "Inspecting..." : "Inspect element"
        }),
        el("button", { type: "button", onClick: () => openPanel("left"), text: "Inspector" }),
        el("button", { type: "button", onClick: () => openPanel("bottom"), text: "Box model" })
      ]),
      metricGrid([
        ["Element", snapshot?.selectedElement?.selector || "none selected"],
        ["Source", source.relativePath || "preview-only / not writable"],
        ["Writable", source.writable ? "exact source target" : "preview-only / not writable"],
        ["Input value", snapshot?.selectedElement?.state?.value],
        ["Checked", snapshot?.selectedElement?.state?.checked],
        ["Selected index", snapshot?.selectedElement?.state?.selectedIndex],
        ["Breakpoint", breakpointLabel()]
      ]),
      el("section", { className: "dx-devtools-list" }, [
        el("strong", { text: "Closest parent chain" }),
        ...((snapshot?.closestParentChain || []).map((item) =>
          el("button", {
            type: "button",
            "data-dx-devtools-parent-chain-item": item.depth,
            "data-dx-devtools-parent-chain-selector": item.selector,
            onClick: () => selectParentChainItem(item),
            text: `${item.depth}: ${item.selector} ${item.sourceTarget?.relativePath || ""}`
          })
        ))
      ]),
      el("section", { className: "dx-devtools-list" }, [
        el("strong", { text: "Selected attributes" }),
        ...Object.entries(snapshot?.selectedElement?.attributes || {}).map(([name, value]) =>
          el("span", { text: `${name}="${value}"` })
        ),
        ...(snapshot ? [] : [el("span", { text: "Press Inspect element, then click any app element." })])
      ])
    ]);
  }

  function inspectorPanel() {
    const tabButtons = TABS.map((tab) => el("button", {
      type: "button",
      "data-dx-devtools-tab": tab,
      "data-dx-devtools-focus": `inspector-tab-${tab}`,
      id: `dx-devtools-tab-${tab}`,
      role: "tab",
      "aria-selected": STATE.activeTab === tab ? "true" : "false",
      "aria-controls": `dx-devtools-tabpanel-${tab}`,
      tabindex: STATE.activeTab === tab ? "0" : "-1",
      onClick: () => {
        STATE.activeTab = tab;
        renderAll({ focusToken: `inspector-tab-${tab}` });
      },
      onKeydown: (event) => onTabKeydown(event, TABS, STATE.activeTab, (next) => {
        STATE.activeTab = next;
        renderAll({ focusToken: `inspector-tab-${next}` });
      }),
      text: tab
    }));
    return el("div", { className: "dx-devtools-panel-body" }, [
      el("div", { className: "dx-devtools-tabs", role: "tablist" }, tabButtons),
      inspectorTabBody()
    ]);
  }

  function inspectorTabBody() {
    const snapshot = STATE.selectedSnapshot;
    if (STATE.activeTab === "source") {
      const resolution = STATE.sourceResult?.resolution || {};
      const frame = resolution.code_frame?.lines || [];
      return el("section", { className: "dx-devtools-section", id: "dx-devtools-tabpanel-source", role: "tabpanel", "aria-labelledby": "dx-devtools-tab-source" }, [
        metricGrid([
          ["Source", resolution.source_path || snapshot?.source?.relativePath || "preview-only / not writable"],
          ["Writable", resolution.writable ? "yes" : "preview-only / not writable"],
          ["Style", resolution.style_writable ? "source known" : "preview-only / not writable"],
          ["Message", resolution.message || "Select an element"]
        ]),
        el("pre", { className: "dx-devtools-code", text: frame.map((line) => `${line.highlight ? ">" : " "} ${line.line}: ${line.text}`).join("\n") || "No source frame available." })
      ]);
    }
    if (STATE.activeTab === "layout" || STATE.activeTab === "spacing" || STATE.activeTab === "responsive") {
      const model = snapshot?.boxModel || boxModelPayload(null);
      const groups = snapshot?.computedCss?.groups || computedPayload(null).groups;
      const groupNames = STATE.activeTab === "spacing"
        ? ["spacing", "border"]
        : STATE.activeTab === "responsive"
          ? ["box", "layout", "paint"]
          : ["box", "layout", "text"];
      return el("section", { className: "dx-devtools-section", id: `dx-devtools-tabpanel-${STATE.activeTab}`, role: "tabpanel", "aria-labelledby": `dx-devtools-tab-${STATE.activeTab}` }, [
        metricGrid([
          ["Display", snapshot?.computedCss?.properties?.display || "none"],
          ["Position", snapshot?.computedCss?.properties?.position || "none"],
          ["Content", boxSize(model.contentBox || model.content)],
          ["Border box", boxSize(model.borderBox)],
          ["Margin", edgeSummary(model.margin)],
          ["Padding", edgeSummary(model.padding)],
          ["Breakpoint", `${breakpointLabel()} (${window.innerWidth}px)`]
        ]),
        ...groupNames.map((name) => cssGroupReadout(name, groups[name] || {}))
      ]);
    }
    return styleEditorPanel(snapshot);
  }

  function cssGroupReadout(name, values) {
    const entries = Array.isArray(values)
      ? values.map((item) => [item.name, item.value])
      : Object.entries(values || {});
    return el("section", { className: "dx-devtools-css-group" }, [
      el("strong", { text: name }),
      el("div", { className: "dx-devtools-css-grid" }, entries.map(([property, value]) =>
        el("button", {
          type: "button",
          "data-dx-devtools-computed-css-property": property,
          "data-dx-devtools-computed-css-value": value || "",
          onClick: () => setStyleDraft(property, value || ""),
          title: `${property}: ${value || "unset"}`,
          text: `${property}: ${value || "unset"}`
        })
      ))
    ]);
  }

  function catalogPropertyOptions(current) {
    const fromCatalog = (STATE.cssData?.properties || []).map((entry) => entry.name);
    return [...new Set([...fromCatalog, ...STYLE_PROPERTY_OPTIONS, ...Object.keys(current || {}).sort()])].sort();
  }

  function catalogValueOptions(property) {
    const mdnEntry = STATE.cssDataByProperty.get(property);
    return [...new Set([
      ...(STYLE_VALUE_OPTIONS[property] || []),
      ...((mdnEntry?.valueHints || []).slice(0, 64))
    ])].filter(Boolean);
  }

  function cssCatalogSummary() {
    const counts = STATE.cssData?.counts;
    if (!counts) return "MDN CSS data loading";
    return `${counts.properties} properties / ${counts.selectors} selectors / ${counts.propertyValueSyntaxes} value syntaxes`;
  }

  function cssCatalogPanel(property) {
    const entry = STATE.cssDataByProperty.get(property);
    const counts = STATE.cssData?.counts || {};
    return el("section", { className: "dx-devtools-css-group" }, [
      el("strong", { text: "MDN CSS catalog" }),
      metricGrid([
        ["Source", STATE.cssData?.source ? `${STATE.cssData.source.package}@${STATE.cssData.source.version}` : "loading"],
        ["Commit", STATE.cssData?.source?.commit?.slice(0, 12) || "loading"],
        ["Properties", counts.properties],
        ["Selectors", counts.selectors],
        ["Value syntaxes", counts.propertyValueSyntaxes],
        ["Current syntax", entry?.syntax || "preview-only / not writable"],
        ["Initial", Array.isArray(entry?.initial) ? entry.initial.join(", ") : entry?.initial],
        ["MDN", entry?.mdnUrl || "preview-only / not writable"]
      ])
    ]);
  }

  function isColorProperty(property) {
    return COLOR_PROPERTIES.has(property) || property.includes("color");
  }

  function setStyleProperty(property, current) {
    const value = current[property] || "";
    setStyleDraft(property, value || (isColorProperty(property) ? "#ededed" : ""));
  }

  function styleEditorPanel(snapshot) {
    const current = snapshot?.computedCss?.properties || {};
    const allCss = snapshot?.computedCss?.all || computedPayload(null).all;
    const propertyOptions = catalogPropertyOptions(current);
    const property = STATE.styleDraft.property || "color";
    const computedValue = current[property] || "";
    const draft = {
      property,
      value: STATE.styleDraft.value || computedValue || (isColorProperty(property) ? current.color || "#ededed" : "")
    };
    const valueOptions = catalogValueOptions(draft.property);
    const showColorPopover = isColorProperty(draft.property) || draft.property === "background-image";
    return el("section", { className: "dx-devtools-section", id: "dx-devtools-tabpanel-style", role: "tabpanel", "aria-labelledby": "dx-devtools-tab-style" }, [
      el("div", { className: "dx-devtools-control-grid" }, [
        customSelectControl("Property", "data-dx-devtools-style-property", draft.property, propertyOptions, "propertyPopoverOpen", (value) => {
          setStyleProperty(value || "color", current);
        }),
        customValueControl("Value", "data-dx-devtools-style-value", draft.value, valueOptions, showColorPopover, (value) => {
          STATE.styleDraft.value = value || "currentColor";
        })
      ]),
      showColorPopover ? colorPopoverControl(draft.property, draft.value) : null,
      el("div", { className: "dx-devtools-actions" }, [
        el("button", { type: "button", className: "dx-devtools-primary-action", "data-dx-devtools-style-preview": "true", onClick: previewStyle, text: "Preview" }),
        el("button", { type: "button", "data-dx-devtools-style-apply": "true", onClick: applyStyle, text: "Apply safely" }),
        el("button", { type: "button", "data-dx-devtools-style-undo": "true", onClick: undoStyle, text: "Undo source" })
      ]),
      metricGrid([
        ["CSS catalog", cssCatalogSummary()],
        ["Preview", "live DOM only"],
        ["Apply", snapshot?.selectedElement?.sourceTarget?.range ? "exact source range" : "preview-only / not writable"],
        ["Result", STATE.styleResult?.operation?.reason || STATE.styleResult?.resolution?.message || STATE.styleResult?.reason || STATE.styleResult?.message || "none"]
      ]),
      cssCatalogPanel(draft.property),
      cssGroupReadout("computed paint", snapshot?.computedCss?.groups?.paint || computedPayload(null).groups.paint),
      cssGroupReadout("all computed css", allCss)
    ].filter(Boolean));
  }

  function editableControlValue(node) {
    if (!node) return "";
    const raw = "value" in node && node.value != null ? node.value : node.textContent;
    return String(raw || "").replace(/\u00a0/g, " ");
  }

  function activeStyleDraftEditor() {
    const active = document.activeElement;
    return Boolean(active?.matches?.("[data-dx-devtools-style-value], [data-dx-devtools-color-value]"));
  }

  function commitEditableText(event, onInput) {
    const value = editableControlValue(event.currentTarget);
    onInput(value);
  }

  function onPlainTextPaste(event) {
    const textValue = event.clipboardData?.getData("text/plain");
    if (textValue == null) return;
    event.preventDefault();
    const selection = window.getSelection?.();
    if (!selection || selection.rangeCount === 0) return;
    selection.deleteFromDocument();
    const range = selection.getRangeAt(0);
    range.insertNode(document.createTextNode(textValue));
    range.collapse(false);
    selection.removeAllRanges();
    selection.addRange(range);
    const inputEvent = typeof InputEvent === "function"
      ? new InputEvent("input", { bubbles: true, inputType: "insertText", data: textValue })
      : new Event("input", { bubbles: true });
    event.currentTarget.dispatchEvent(inputEvent);
  }

  function cssNumberParts(value) {
    const match = String(value || "").trim().match(/^(-?(?:\d+|\d*\.\d+))([a-z%]*)$/i);
    if (!match) return null;
    return { number: Number.parseFloat(match[1]), unit: match[2] || "", decimals: (match[1].split(".")[1] || "").length };
  }

  function nudgeCssValue(value, direction) {
    const parts = cssNumberParts(value);
    if (!parts) return String(value || "");
    const step = parts.unit === "" || parts.unit === "px" || parts.unit === "%" ? 1 : 0.1;
    const decimals = Math.max(parts.decimals, step < 1 ? 1 : 0);
    const next = Number((parts.number + direction * step).toFixed(Math.min(3, decimals)));
    return `${next}${parts.unit}`;
  }

  function currentEditableValue(marker, fallback) {
    const node = root?.querySelector?.(`[${marker}]`);
    return editableControlValue(node).trim() || fallback || "";
  }

  function customTextControl(label, marker, value, onInput) {
    return el("label", { className: "dx-devtools-control" }, [
      el("span", { text: label }),
      el("span", { className: "dx-devtools-input-shell" }, [
        el("span", {
          [marker]: "true",
          "data-dx-devtools-focus": marker,
          role: "textbox",
          "aria-label": label,
          "aria-multiline": "false",
          contenteditable: "true",
          tabindex: "0",
          spellcheck: "false",
          text: value,
          onInput: (event) => commitEditableText(event, onInput),
          onBlur: (event) => commitEditableText(event, onInput),
          onPaste: onPlainTextPaste,
          onKeydown: onSingleLineTextboxKeydown
        })
      ])
    ]);
  }

  function customPopoverSearch(label, marker, openKey, options, value, onPick) {
    const searchMarker = `${marker}-search`;
    const query = STATE.popoverQuery[openKey] || "";
    return el("label", { className: "dx-devtools-popover-search" }, [
      el("span", { text: label }),
      el("span", { className: "dx-devtools-input-shell" }, [
        el("span", {
          "data-dx-devtools-popover-search": openKey,
          "data-dx-devtools-focus": searchMarker,
          role: "textbox",
          "aria-label": label,
          "aria-multiline": "false",
          contenteditable: "plaintext-only",
          spellcheck: "false",
          text: query,
          onInput: (event) => {
            setPopoverQuery(openKey, options, event.target.textContent || "");
            renderAll({ focusToken: searchMarker });
          },
          onKeydown: (event) => onPopoverSearchKeydown(event, openKey, options, value, onPick)
        })
      ])
    ]);
  }

  function customSelectControl(label, marker, value, options, openKey, onPick) {
    const open = Boolean(STATE[openKey]);
    const listId = `${marker.replace(/[^a-zA-Z0-9_-]/g, "-")}-list`;
    const visibleOptions = filteredPopoverOptions(openKey, options);
    const activeIndex = activeOptionIndex(openKey, visibleOptions, value);
    const activeDescendant = open && visibleOptions.length ? selectOptionId(listId, activeIndex) : "";
    const commitPick = (option) => {
      delete STATE.activePopoverIndex[openKey];
      delete STATE.popoverQuery[openKey];
      onPick(option);
    };
    return el("div", { className: "dx-devtools-control dx-devtools-select" }, [
      el("span", { text: label }),
      el("button", {
        type: "button",
        className: "dx-devtools-select-trigger",
        [marker]: "true",
        "data-dx-devtools-focus": marker,
        role: "combobox",
        "aria-label": label,
        "aria-haspopup": "listbox",
        "aria-controls": listId,
        "aria-activedescendant": activeDescendant,
        "aria-expanded": open ? "true" : "false",
        onClick: () => {
          closePopovers(open ? "" : openKey);
          if (!open) setActiveOptionIndex(openKey, visibleOptions, activeIndex);
          STATE[openKey] = !open;
          renderAll();
        },
        onKeydown: (event) => onSelectKeydown(event, openKey, visibleOptions, value, commitPick),
        text: value
      }),
      open ? el("div", { className: "dx-devtools-popover" }, [
        customPopoverSearch(`Search ${label.toLowerCase()}`, marker, openKey, options, value, commitPick),
        visibleOptions.length
          ? el("div", { className: "dx-devtools-option-list", id: listId, role: "listbox" }, visibleOptions.map((option, index) =>
            el("button", {
              id: selectOptionId(listId, index),
              type: "button",
              role: "option",
              "aria-selected": option === value ? "true" : "false",
              "data-dx-devtools-active": index === activeIndex ? "true" : "false",
              "data-dx-devtools-focus": `${marker}-option-${index}`,
              tabIndex: "-1",
              onClick: () => commitPick(option),
              text: option
            })
          ))
          : el("p", { className: "dx-devtools-empty", text: "No matching CSS options" })
      ]) : null
    ].filter(Boolean));
  }

  function customValueControl(label, marker, value, options, showColorPopover, onInput) {
    const open = STATE.valuePopoverOpen && options.length > 0;
    const optionsId = `${marker.replace(/[^a-zA-Z0-9_-]/g, "-")}-options`;
    const colorDialogId = "dx-devtools-color-popover";
    const visibleOptions = filteredPopoverOptions("valuePopoverOpen", options);
    const activeIndex = activeOptionIndex("valuePopoverOpen", visibleOptions, value);
    const activeDescendant = open && visibleOptions.length ? selectOptionId(optionsId, activeIndex) : "";
    const canNudgeValue = Boolean(cssNumberParts(value));
    const pickValueOption = (option) => {
      delete STATE.popoverQuery.valuePopoverOpen;
      setStyleDraft(STATE.styleDraft.property, option);
    };
    const nudgeValue = (direction) => {
      const current = currentEditableValue(marker, value);
      const next = nudgeCssValue(current, direction);
      if (next === current) return;
      onInput(next);
      setStyleDraft(STATE.styleDraft.property, next, { closePopovers: false });
    };
    return el("div", { className: "dx-devtools-control dx-devtools-value-control" }, [
      el("span", { text: label }),
      el("span", { className: "dx-devtools-input-row" }, [
        el("span", { className: "dx-devtools-input-shell" }, [
          el("span", {
            [marker]: "true",
            "data-dx-devtools-focus": marker,
            role: "textbox",
            "aria-label": label,
            "aria-multiline": "false",
            contenteditable: "true",
            tabindex: "0",
            spellcheck: "false",
            text: value,
            onInput: (event) => commitEditableText(event, onInput),
            onBlur: (event) => commitEditableText(event, onInput),
            onPaste: onPlainTextPaste,
            onKeydown: onSingleLineTextboxKeydown
          })
        ]),
        el("span", { className: "dx-devtools-stepper", role: "group", "aria-label": `${label} numeric steppers` }, [
          el("button", {
            type: "button",
            className: "dx-devtools-stepper-button",
            "data-dx-devtools-value-decrement": "true",
            "aria-label": `Decrease ${label}`,
            "aria-disabled": canNudgeValue ? "false" : "true",
            disabled: !canNudgeValue,
            onClick: () => nudgeValue(-1),
            text: "-"
          }),
          el("button", {
            type: "button",
            className: "dx-devtools-stepper-button",
            "data-dx-devtools-value-increment": "true",
            "aria-label": `Increase ${label}`,
            "aria-disabled": canNudgeValue ? "false" : "true",
            disabled: !canNudgeValue,
            onClick: () => nudgeValue(1),
            text: "+"
          })
        ]),
        options.length ? el("button", {
          type: "button",
