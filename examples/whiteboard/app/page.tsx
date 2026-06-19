export default function WhiteboardPage() {
  return (
    <main
      className="wb-workbench"
      data-dx-surface="whiteboard"
      data-dx-state-runtime="source-owned"
      data-dx-renderer="canvas"
      data-whiteboard-route="direct-renderable-workbench"
    >
      <aside className="wb-toolbar" aria-label="Whiteboard activity bar">
        <div className="wb-toolbar-group wb-toolbar-actions" role="toolbar" aria-label="Board actions">
          <button aria-label="Clear board" className="wb-icon-button wb-danger-button" data-whiteboard-command="clear" title="Clear board" type="button">
            <dx-icon className="wb-icon" name="whiteboard:trash" />
          </button>
          <button aria-controls="wb-shortcuts-panel" aria-expanded="false" aria-label="Keyboard shortcuts" className="wb-icon-button" data-whiteboard-toggle="shortcuts" title="Keyboard shortcuts (?)" type="button">
            <dx-icon className="wb-icon" name="whiteboard:keyboard" />
          </button>
          <button aria-controls="wb-side-panel" aria-expanded="false" aria-label="Panel" className="wb-icon-button" data-whiteboard-toggle="side-panel" title="Panel" type="button">
            <dx-icon className="wb-icon" name="whiteboard:panel" />
          </button>
        </div>

        <div className="wb-toolbar-group" role="toolbar" aria-label="Drawing tools">
          <button aria-keyshortcuts="V" aria-label="Move tool" aria-pressed="true" className="wb-icon-button" data-active="true" data-whiteboard-tool-button="select" title="Move (V)" type="button">
            <dx-icon className="wb-icon" name="whiteboard:move" />
          </button>
          <button aria-keyshortcuts="H" aria-label="Hand tool" className="wb-icon-button" data-whiteboard-tool-button="hand" title="Hand (H)" type="button">
            <dx-icon className="wb-icon" name="whiteboard:hand" />
          </button>
          <button aria-keyshortcuts="P" aria-label="Pen tool" className="wb-icon-button" data-whiteboard-tool-button="freehand" title="Pen (P)" type="button">
            <dx-icon className="wb-icon" name="whiteboard:pen" />
          </button>
          <div className="wb-tool-popover" data-whiteboard-tool-popover="shapes">
            <button aria-expanded="false" aria-label="Choose shape tool" className="wb-icon-button" data-whiteboard-shape-menu-trigger="true" title="Shapes" type="button">
              <dx-icon className="wb-icon" data-whiteboard-current-shape-icon="true" name="whiteboard:rectangle" />
            </button>
            <div className="wb-tool-popover-panel" data-whiteboard-shape-menu="true" hidden>
              <section className="wb-tool-popover-section" aria-label="Basic shapes">
                <p>Basic</p>
                <div className="wb-tool-option-grid">
                  <button aria-keyshortcuts="R" aria-label="Rectangle tool" className="wb-tool-option" data-whiteboard-tool-button="rectangle" title="Rectangle (R)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:rectangle" />
                    <span>Rectangle</span>
                  </button>
                  <button aria-keyshortcuts="D" aria-label="Diamond tool" className="wb-tool-option" data-whiteboard-tool-button="diamond" title="Diamond (D)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:diamond" />
                    <span>Diamond</span>
                  </button>
                  <button aria-keyshortcuts="O" aria-label="Ellipse tool" className="wb-tool-option" data-whiteboard-tool-button="ellipse" title="Ellipse (O)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:ellipse" />
                    <span>Ellipse</span>
                  </button>
                </div>
              </section>
              <section className="wb-tool-popover-section" aria-label="Connectors">
                <p>Connectors</p>
                <div className="wb-tool-option-grid">
                  <button aria-keyshortcuts="L" aria-label="Line tool" className="wb-tool-option" data-whiteboard-tool-button="line" title="Line (L)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:line" />
                    <span>Line</span>
                  </button>
                  <button aria-keyshortcuts="A" aria-label="Arrow tool" className="wb-tool-option" data-whiteboard-tool-button="arrow" title="Arrow (A)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:arrow" />
                    <span>Arrow</span>
                  </button>
                </div>
              </section>
              <section className="wb-tool-popover-section" aria-label="Content">
                <p>Content</p>
                <div className="wb-tool-option-grid">
                  <button aria-keyshortcuts="T" aria-label="Text tool" className="wb-tool-option" data-whiteboard-tool-button="text" title="Text (T)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:text" />
                    <span>Text</span>
                  </button>
                  <button aria-keyshortcuts="B" aria-label="Table tool" className="wb-tool-option" data-whiteboard-tool-button="table" title="Table (B)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:table" />
                    <span>Table</span>
                  </button>
                  <button aria-keyshortcuts="M" aria-label="Math tool" className="wb-tool-option" data-whiteboard-tool-button="math" title="Math (M)" type="button">
                    <dx-icon className="wb-icon" name="whiteboard:math" />
                    <span>Math</span>
                  </button>
                </div>
              </section>
            </div>
          </div>
        </div>
        <div className="wb-toolbar-group" role="toolbar" aria-label="Assets">
          <div className="wb-tool-popover" data-whiteboard-tool-popover="icons">
            <button aria-expanded="false" aria-label="Choose DX icon" className="wb-icon-button" data-whiteboard-icon-menu-trigger="true" title="DX icons" type="button">
              <dx-icon className="wb-icon" name="whiteboard:icons" />
            </button>
            <div className="wb-tool-popover-panel wb-icon-popover-panel" data-whiteboard-icon-menu="true" hidden>
              <section className="wb-tool-popover-section" aria-label="DX icons">
                <p>DX icons</p>
                <div className="wb-icon-option-grid">
                  <button aria-label="Insert cursor icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:select" type="button"><dx-icon className="wb-icon" name="whiteboard:select" /></button>
                  <button aria-label="Insert rectangle icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:rectangle" type="button"><dx-icon className="wb-icon" name="whiteboard:rectangle" /></button>
                  <button aria-label="Insert diamond icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:diamond" type="button"><dx-icon className="wb-icon" name="whiteboard:diamond" /></button>
                  <button aria-label="Insert ellipse icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:ellipse" type="button"><dx-icon className="wb-icon" name="whiteboard:ellipse" /></button>
                  <button aria-label="Insert arrow icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:arrow" type="button"><dx-icon className="wb-icon" name="whiteboard:arrow" /></button>
                  <button aria-label="Insert line icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:line" type="button"><dx-icon className="wb-icon" name="whiteboard:line" /></button>
                  <button aria-label="Insert text icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:text" type="button"><dx-icon className="wb-icon" name="whiteboard:text" /></button>
                  <button aria-label="Insert table icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:table" type="button"><dx-icon className="wb-icon" name="whiteboard:table" /></button>
                  <button aria-label="Insert math icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:math" type="button"><dx-icon className="wb-icon" name="whiteboard:math" /></button>
                  <button aria-label="Insert keyboard icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:keyboard" type="button"><dx-icon className="wb-icon" name="whiteboard:keyboard" /></button>
                  <button aria-label="Insert image icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:image" type="button"><dx-icon className="wb-icon" name="whiteboard:image" /></button>
                  <button aria-label="Insert audio icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:audio" type="button"><dx-icon className="wb-icon" name="whiteboard:audio" /></button>
                  <button aria-label="Insert video icon" className="wb-icon-option" data-whiteboard-icon-insert="whiteboard:video" type="button"><dx-icon className="wb-icon" name="whiteboard:video" /></button>
                </div>
              </section>
            </div>
          </div>
          <button aria-label="Add image" className="wb-icon-button" data-whiteboard-media-pick="image" title="Add image" type="button">
            <dx-icon className="wb-icon" name="whiteboard:image" />
          </button>
          <button aria-label="Add audio" className="wb-icon-button" data-whiteboard-media-pick="audio" title="Add audio" type="button">
            <dx-icon className="wb-icon" name="whiteboard:audio" />
          </button>
          <button aria-label="Add video" className="wb-icon-button" data-whiteboard-media-pick="video" title="Add video" type="button">
            <dx-icon className="wb-icon" name="whiteboard:video" />
          </button>
        </div>
        <div className="wb-toolbar-group" role="toolbar" aria-label="History">
          <button aria-keyshortcuts="Control+Z" aria-label="Undo" className="wb-icon-button" data-whiteboard-command="undo" title="Undo (Ctrl+Z)" type="button">
            <dx-icon className="wb-icon" name="whiteboard:undo" />
          </button>
          <button aria-keyshortcuts="Control+Y Control+Shift+Z" aria-label="Redo" className="wb-icon-button" data-whiteboard-command="redo" title="Redo (Ctrl+Y / Ctrl+Shift+Z)" type="button">
            <dx-icon className="wb-icon" name="whiteboard:redo" />
          </button>
        </div>
        <div className="wb-toolbar-group" role="toolbar" aria-label="Zoom">
          <button aria-label="Zoom out" className="wb-icon-button" data-whiteboard-zoom="out" title="Zoom out" type="button">
            <dx-icon className="wb-icon" name="whiteboard:zoom-out" />
          </button>
          <button aria-label="Fit canvas" className="wb-icon-button" data-whiteboard-zoom="fit" title="Fit canvas" type="button">
            <dx-icon className="wb-icon" name="whiteboard:fit" />
          </button>
          <button aria-label="Zoom in" className="wb-icon-button" data-whiteboard-zoom="in" title="Zoom in" type="button">
            <dx-icon className="wb-icon" name="whiteboard:zoom-in" />
          </button>
        </div>
        <div className="wb-toolbar-group" role="toolbar" aria-label="View">
          <button aria-controls="wb-minimap-panel" aria-expanded="false" aria-label="Toggle minimap" aria-pressed="false" className="wb-icon-button" data-active="false" data-whiteboard-toggle="minimap" title="Toggle minimap" type="button">
            <dx-icon className="wb-icon" name="whiteboard:fit" />
          </button>
        </div>
      </aside>

      <section className="wb-canvas-panel" aria-label="Whiteboard canvas workbench">
        <div className="wb-canvas-scroll">
          <section
            className="whiteboard-stage"
            data-whiteboard-stage="source-owned-canvas"
            data-whiteboard-renderer="dx.whiteboard.canvas-renderer"
            data-whiteboard-input-runtime="dx.whiteboard.input-runtime"
            data-whiteboard-tool="select"
            data-whiteboard-selection=""
            data-whiteboard-grid="true"
            data-whiteboard-grid-size="24"
            data-whiteboard-snap="true"
            data-whiteboard-revision="0"
          >
            <svg
              aria-label="Whiteboard scene preview"
              className="whiteboard-svg-preview"
              data-grid="true"
              role="img"
              viewBox="0 0 1200 720"
            >
              <defs>
                <pattern id="wb-static-grid" width="24" height="24" patternUnits="userSpaceOnUse">
                  <path className="wb-grid-line" d="M 24 0 L 0 0 0 24" />
                </pattern>
              </defs>
              <rect className="wb-board-base" width="1200" height="720" />
              <rect className="wb-board-grid" fill="url(#wb-static-grid)" width="1200" height="720" />

              <rect className="wb-selection-outline" data-whiteboard-selection-outline="true" hidden x="0" y="0" width="0" height="0" rx="18" />
            </svg>
            <canvas
              aria-label="Whiteboard canvas"
              className="whiteboard-canvas"
              data-whiteboard-canvas="source-owned"
              data-whiteboard-pointer-controller="store-backed"
              data-whiteboard-stage-state="dx.whiteboard.canvas-stage"
              height="720"
              tabIndex="0"
              width="1200"
            />
            <input aria-label="Edit text object" className="wb-text-editor" data-whiteboard-text-editor="true" hidden type="text" />
            <input accept="image/*" aria-label="Choose image" className="wb-hidden-file-input" data-whiteboard-media-input="image" type="file" />
            <input accept="audio/*" aria-label="Choose audio" className="wb-hidden-file-input" data-whiteboard-media-input="audio" type="file" />
            <input accept="video/*" aria-label="Choose video" className="wb-hidden-file-input" data-whiteboard-media-input="video" type="file" />
          </section>
          <section className="wb-panel wb-minimap-panel" aria-label="Mini map" data-whiteboard-minimap-panel="true" data-whiteboard-visible="false" id="wb-minimap-panel">
            <div className="wb-panel-heading">
              <h2>Minimap</h2>
              <span className="wb-panel-badge">Live scene</span>
            </div>
            <button className="wb-minimap-button" type="button">
              <svg className="wb-minimap-svg" viewBox="0 0 240 144" aria-hidden="true">
                <rect className="wb-minimap-background" width="240" height="144" />
                <rect className="wb-minimap-element" data-selected="true" x="22" y="19" width="64" height="31" rx="4" />
                <polygon className="wb-minimap-element" points="120,24 138,37 120,50 102,37" />
                <path className="wb-minimap-element" d="M 142 37 L 185 37" />
                <ellipse className="wb-minimap-element" cx="53" cy="74" rx="19" ry="12" />
                <rect className="wb-minimap-viewport" x="8" y="8" width="224" height="128" rx="4" />
              </svg>
            </button>
          </section>
        </div>
      </section>

      <aside className="wb-side-panel" aria-label="Whiteboard panel" aria-hidden="true" data-collapsed="true" data-whiteboard-side-panel="true" id="wb-side-panel">
        <section className="wb-panel">
          <div className="wb-panel-heading">
            <h2>Panel</h2>
            <button aria-label="Close panel" className="wb-icon-button wb-panel-close" data-whiteboard-toggle="side-panel" type="button">
              <dx-icon className="wb-icon" name="whiteboard:redo" />
            </button>
          </div>
          <div className="wb-measurement-grid">
            <label className="wb-measurement-field">
              <span>X</span>
              <span className="wb-stepper">
                <button aria-label="Decrease X" data-whiteboard-field-step="x" data-whiteboard-step="-1" type="button">-</button>
                <input aria-label="X position" data-whiteboard-measure-input="x" inputMode="numeric" value="112" />
                <button aria-label="Increase X" data-whiteboard-field-step="x" data-whiteboard-step="1" type="button">+</button>
              </span>
            </label>
            <label className="wb-measurement-field">
              <span>Y</span>
              <span className="wb-stepper">
                <button aria-label="Decrease Y" data-whiteboard-field-step="y" data-whiteboard-step="-1" type="button">-</button>
                <input aria-label="Y position" data-whiteboard-measure-input="y" inputMode="numeric" value="96" />
                <button aria-label="Increase Y" data-whiteboard-field-step="y" data-whiteboard-step="1" type="button">+</button>
              </span>
            </label>
            <label className="wb-measurement-field">
              <span>W</span>
              <span className="wb-stepper">
                <button aria-label="Decrease width" data-whiteboard-field-step="width" data-whiteboard-step="-1" type="button">-</button>
                <input aria-label="Width" data-whiteboard-measure-input="width" inputMode="numeric" value="320" />
                <button aria-label="Increase width" data-whiteboard-field-step="width" data-whiteboard-step="1" type="button">+</button>
              </span>
            </label>
            <label className="wb-measurement-field">
              <span>H</span>
              <span className="wb-stepper">
                <button aria-label="Decrease height" data-whiteboard-field-step="height" data-whiteboard-step="-1" type="button">-</button>
                <input aria-label="Height" data-whiteboard-measure-input="height" inputMode="numeric" value="156" />
                <button aria-label="Increase height" data-whiteboard-field-step="height" data-whiteboard-step="1" type="button">+</button>
              </span>
            </label>
          </div>
          <div className="wb-color-control" data-whiteboard-color-control="true">
            <button aria-expanded="false" className="wb-color-trigger" data-whiteboard-color-trigger="true" type="button">
              <span className="wb-color-preview" data-whiteboard-color-preview="true" />
              <span data-whiteboard-color-label="true">Aurora</span>
            </button>
            <div className="wb-color-popover" data-whiteboard-color-popover="true" hidden>
              <div className="wb-segmented-control" role="group" aria-label="Paint type">
                <button data-active="true" data-whiteboard-paint-mode="solid" type="button">Solid</button>
                <button data-whiteboard-paint-mode="linear" type="button">Linear</button>
              </div>
              <div className="wb-color-picker-row">
                <label>
                  <span>Solid</span>
                  <input aria-label="Solid color" data-whiteboard-color-input="solid" type="color" />
                </label>
                <label>
                  <span>Start</span>
                  <input aria-label="Gradient start color" data-whiteboard-color-input="start" type="color" />
                </label>
                <label>
                  <span>End</span>
                  <input aria-label="Gradient end color" data-whiteboard-color-input="end" type="color" />
                </label>
              </div>
              <label className="wb-measurement-field">
                <span>Angle</span>
                <span className="wb-stepper">
                  <button aria-label="Decrease gradient angle" data-whiteboard-gradient-step="-15" type="button">-</button>
                  <input aria-label="Gradient angle" data-whiteboard-gradient-angle="true" inputMode="numeric" value="135" />
                  <button aria-label="Increase gradient angle" data-whiteboard-gradient-step="15" type="button">+</button>
                </span>
              </label>
              <button className="wb-primary-button" data-whiteboard-apply-custom-paint="true" type="button">
                <dx-icon className="wb-icon" name="whiteboard:select" />
                <span>Apply paint</span>
              </button>
              <div className="wb-swatch-grid" aria-label="Quick colors">
                <button aria-label="Aurora gradient" className="wb-swatch" data-active="true" data-swatch="aurora" data-whiteboard-swatch="aurora" type="button" />
                <button aria-label="Sunset gradient" className="wb-swatch" data-swatch="sunset" data-whiteboard-swatch="sunset" type="button" />
                <button aria-label="Ocean gradient" className="wb-swatch" data-swatch="ocean" data-whiteboard-swatch="ocean" type="button" />
                <button aria-label="Candy gradient" className="wb-swatch" data-swatch="candy" data-whiteboard-swatch="candy" type="button" />
                <button aria-label="Forest gradient" className="wb-swatch" data-swatch="forest" data-whiteboard-swatch="forest" type="button" />
                <button aria-label="Grape gradient" className="wb-swatch" data-swatch="grape" data-whiteboard-swatch="grape" type="button" />
                <button aria-label="Ink color" className="wb-swatch" data-swatch="ink" data-whiteboard-swatch="ink" type="button" />
                <button aria-label="Transparent fill" className="wb-swatch" data-swatch="transparent" data-whiteboard-swatch="transparent" type="button" />
              </div>
            </div>
          </div>
          <section className="wb-text-controls" aria-label="Text controls">
            <label className="wb-font-field">
              <span>Google Font</span>
              <input aria-label="Google font family" data-whiteboard-font-input="true" list="wb-google-fonts" placeholder="JetBrains Mono" value="JetBrains Mono" />
            </label>
            <datalist id="wb-google-fonts">
              <option value="JetBrains Mono" />
              <option value="Inter" />
              <option value="Roboto" />
              <option value="Open Sans" />
              <option value="Lato" />
              <option value="Montserrat" />
              <option value="Poppins" />
              <option value="Merriweather" />
              <option value="Playfair Display" />
              <option value="Oswald" />
              <option value="Source Sans 3" />
              <option value="Noto Sans" />
              <option value="Fira Code" />
            </datalist>
            <button className="wb-primary-button" data-whiteboard-apply-font="true" type="button">
              <dx-icon className="wb-icon" name="whiteboard:text" />
              <span>Apply font</span>
            </button>
          </section>
        </section>

        <section className="wb-panel wb-outline-panel">
          <div className="wb-panel-heading">
            <h2>Outline</h2>
            <span className="wb-panel-badge">6 layers</span>
          </div>
          <div className="wb-outline-list">
          </div>
        </section>

        <section className="wb-panel">
          <div className="wb-panel-heading">
            <h2>Library</h2>
          </div>
          <div className="wb-library-grid">
            <button className="wb-library-card" data-whiteboard-add="flow-card" data-whiteboard-preset="flow-card" type="button">
              <dx-icon className="wb-icon" name="whiteboard:rectangle" />
              <strong>Flow card</strong>
            </button>
            <button className="wb-library-card" data-whiteboard-add="frame" data-whiteboard-preset="frame" data-whiteboard-preset-category="template" type="button">
              <dx-icon className="wb-icon" name="whiteboard:fit" />
              <strong>Frame</strong>
            </button>
          </div>
        </section>
      </aside>
      <aside className="wb-shortcuts-panel" aria-label="Keyboard shortcuts" aria-hidden="true" data-collapsed="true" data-whiteboard-shortcuts-panel="true" id="wb-shortcuts-panel">
        <section className="wb-panel">
          <div className="wb-panel-heading">
            <h2>Shortcuts</h2>
            <button aria-label="Close shortcuts" className="wb-icon-button wb-panel-close" data-whiteboard-toggle="shortcuts" title="Close shortcuts" type="button">
              <dx-icon className="wb-icon" name="whiteboard:redo" />
            </button>
          </div>
          <div className="wb-shortcut-grid" aria-label="Keyboard shortcut list">
            <span><kbd>V</kbd><strong>Move objects</strong></span>
            <span><kbd>H</kbd><strong>Pan canvas</strong></span>
            <span><kbd>P</kbd><strong>Pen</strong></span>
            <span><kbd>R</kbd><strong>Rectangle</strong></span>
            <span><kbd>D</kbd><strong>Diamond</strong></span>
            <span><kbd>O</kbd><strong>Ellipse</strong></span>
            <span><kbd>L</kbd><strong>Line</strong></span>
            <span><kbd>A</kbd><strong>Arrow</strong></span>
            <span><kbd>T</kbd><strong>Text</strong></span>
            <span><kbd>B</kbd><strong>Table</strong></span>
            <span><kbd>M</kbd><strong>Math</strong></span>
            <span><kbd>?</kbd><strong>Shortcuts</strong></span>
            <span><kbd>Esc</kbd><strong>Close popovers</strong></span>
            <span><kbd>Del</kbd><strong>Delete selection</strong></span>
            <span><kbd>Ctrl Z</kbd><strong>Undo</strong></span>
            <span><kbd>Ctrl Y</kbd><strong>Redo</strong></span>
          </div>
        </section>
      </aside>
      <script type="module" src="/whiteboard-runtime.ts" data-whiteboard-runtime="source-owned"></script>
    </main>
  );
}
