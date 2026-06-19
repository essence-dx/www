// DX-WWW JSX environment types
// Comprehensive HTML/SVG element type declarations for VS Code IntelliSense.
// Based on the full set of HTML/SVG attributes — no red errors in dx-www TSX projects.

type Booleanish = boolean | "true" | "false";
type NumberOrString = number | string;
type CSSProperties = Record<string, string | number>;

// ─── Base HTML Attributes ────────────────────────────────────────────────────

interface DxAriaAttributes {
  "aria-activedescendant"?: string;
  "aria-atomic"?: Booleanish;
  "aria-autocomplete"?: "none" | "inline" | "list" | "both";
  "aria-braillelabel"?: string;
  "aria-brailleroledescription"?: string;
  "aria-busy"?: Booleanish;
  "aria-checked"?: Booleanish | "mixed";
  "aria-colcount"?: number;
  "aria-colindex"?: number;
  "aria-colindextext"?: string;
  "aria-colspan"?: number;
  "aria-controls"?: string;
  "aria-current"?: Booleanish | "page" | "step" | "location" | "date" | "time";
  "aria-describedby"?: string;
  "aria-description"?: string;
  "aria-details"?: string;
  "aria-disabled"?: Booleanish;
  "aria-dropeffect"?: "none" | "copy" | "execute" | "link" | "move" | "popup";
  "aria-errormessage"?: string;
  "aria-expanded"?: Booleanish;
  "aria-flowto"?: string;
  "aria-grabbed"?: Booleanish;
  "aria-haspopup"?: Booleanish | "menu" | "listbox" | "tree" | "grid" | "dialog";
  "aria-hidden"?: Booleanish;
  "aria-invalid"?: Booleanish | "grammar" | "spelling";
  "aria-keyshortcuts"?: string;
  "aria-label"?: string;
  "aria-labelledby"?: string;
  "aria-level"?: number;
  "aria-live"?: "off" | "assertive" | "polite";
  "aria-modal"?: Booleanish;
  "aria-multiline"?: Booleanish;
  "aria-multiselectable"?: Booleanish;
  "aria-orientation"?: "horizontal" | "vertical";
  "aria-owns"?: string;
  "aria-placeholder"?: string;
  "aria-posinset"?: number;
  "aria-pressed"?: Booleanish | "mixed";
  "aria-readonly"?: Booleanish;
  "aria-relevant"?: "additions" | "additions removals" | "additions text" | "all" | "removals" | "removals additions" | "removals text" | "text" | "text additions" | "text removals";
  "aria-required"?: Booleanish;
  "aria-roledescription"?: string;
  "aria-rowcount"?: number;
  "aria-rowindex"?: number;
  "aria-rowindextext"?: string;
  "aria-rowspan"?: number;
  "aria-selected"?: Booleanish;
  "aria-setsize"?: number;
  "aria-sort"?: "none" | "ascending" | "descending" | "other";
  "aria-valuemax"?: number;
  "aria-valuemin"?: number;
  "aria-valuenow"?: number;
  "aria-valuetext"?: string;
}

type DxHtmlAttributes = DxAriaAttributes & {
  children?: any;
  // Core
  id?: string;
  class?: string;
  className?: string;
  style?: string | CSSProperties;
  title?: string;
  lang?: string;
  dir?: "ltr" | "rtl" | "auto";
  hidden?: boolean | "hidden" | "until-found";
  tabIndex?: NumberOrString;
  tabindex?: NumberOrString;
  role?: string;
  slot?: string;
  part?: string;
  exportparts?: string;
  is?: string;
  // Editing
  contentEditable?: Booleanish | "inherit" | "plaintext-only";
  contenteditable?: Booleanish | "inherit" | "plaintext-only";
  draggable?: Booleanish;
  spellCheck?: Booleanish;
  spellcheck?: Booleanish;
  translate?: "yes" | "no";
  // Popover
  popover?: string;
  popoverTarget?: string;
  popoverTargetAction?: "hide" | "show" | "toggle";
  // Misc
  inert?: boolean;
  nonce?: string;
  accessKey?: string;
  accesskey?: string;
  // Events - allow string (inline handlers) or function
  onClick?: any;
  onDblClick?: any;
  onMouseDown?: any;
  onMouseMove?: any;
  onMouseUp?: any;
  onMouseEnter?: any;
  onMouseLeave?: any;
  onMouseOver?: any;
  onMouseOut?: any;
  onKeyDown?: any;
  onKeyUp?: any;
  onKeyPress?: any;
  onChange?: any;
  onInput?: any;
  onSubmit?: any;
  onReset?: any;
  onFocus?: any;
  onBlur?: any;
  onScroll?: any;
  onWheel?: any;
  onPointerDown?: any;
  onPointerMove?: any;
  onPointerUp?: any;
  onPointerEnter?: any;
  onPointerLeave?: any;
  onPointerOver?: any;
  onPointerOut?: any;
  onPointerCancel?: any;
  onTouchStart?: any;
  onTouchMove?: any;
  onTouchEnd?: any;
  onTouchCancel?: any;
  onContextMenu?: any;
  onCopy?: any;
  onCut?: any;
  onPaste?: any;
  onDrag?: any;
  onDragEnd?: any;
  onDragEnter?: any;
  onDragExit?: any;
  onDragLeave?: any;
  onDragOver?: any;
  onDragStart?: any;
  onDrop?: any;
  onSelect?: any;
  onLoad?: any;
  onError?: any;
  onAnimationStart?: any;
  onAnimationEnd?: any;
  onAnimationIteration?: any;
  onTransitionEnd?: any;
  // Data attributes (catch-all)
  [key: `data-${string}`]: string | boolean | number | undefined;
};

// ─── Specific Element Attribute Types ────────────────────────────────────────

type DxAnchorAttributes = DxHtmlAttributes & {
  href?: string;
  target?: string;
  rel?: string;
  download?: string | boolean;
  hrefLang?: string;
  hreflang?: string;
  type?: string;
  referrerPolicy?: string;
  ping?: string;
};

type DxButtonAttributes = DxHtmlAttributes & {
  type?: "button" | "submit" | "reset";
  disabled?: boolean;
  name?: string;
  value?: string;
  form?: string;
  formAction?: string;
  formEncType?: string;
  formMethod?: string;
  formNoValidate?: boolean;
  formTarget?: string;
  autoFocus?: boolean;
  popovertarget?: string;
  popovertargetaction?: string;
};

type DxInputAttributes = DxHtmlAttributes & {
  type?: string;
  name?: string;
  value?: string | number | readonly string[];
  defaultValue?: string | number | readonly string[];
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  readonly?: boolean;
  checked?: boolean;
  defaultChecked?: boolean;
  min?: string | number;
  max?: string | number;
  step?: string | number;
  multiple?: boolean;
  accept?: string;
  autoComplete?: string;
  autocomplete?: string;
  autoFocus?: boolean;
  pattern?: string;
  minLength?: number;
  maxLength?: number;
  inputMode?: string;
  inputmode?: string;
  list?: string;
  size?: number;
  src?: string;
  alt?: string;
  width?: NumberOrString;
  height?: NumberOrString;
  capture?: "user" | "environment" | boolean;
  dirname?: string;
  form?: string;
  formAction?: string;
  formEncType?: string;
  formMethod?: string;
  formNoValidate?: boolean;
  formTarget?: string;
};

type DxTextareaAttributes = DxHtmlAttributes & {
  name?: string;
  value?: string;
  defaultValue?: string;
  placeholder?: string;
  rows?: number;
  cols?: number;
  required?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  readonly?: boolean;
  autoFocus?: boolean;
  maxLength?: number;
  minLength?: number;
  wrap?: "hard" | "soft" | "off";
  dirname?: string;
  form?: string;
};

type DxSelectAttributes = DxHtmlAttributes & {
  name?: string;
  value?: string | string[];
  multiple?: boolean;
  required?: boolean;
  disabled?: boolean;
  autoFocus?: boolean;
  size?: number;
  form?: string;
};

type DxOptionAttributes = DxHtmlAttributes & {
  value?: string;
  selected?: boolean;
  disabled?: boolean;
  label?: string;
};

type DxFormAttributes = DxHtmlAttributes & {
  action?: string;
  method?: string;
  encType?: string;
  enctype?: string;
  noValidate?: boolean;
  novalidate?: boolean;
  target?: string;
  acceptCharset?: string;
  rel?: string;
};

type DxLabelAttributes = DxHtmlAttributes & {
  htmlFor?: string;
  for?: string;
  form?: string;
};

type DxAnchorOrAreaAttributes = DxHtmlAttributes & {
  href?: string;
  target?: string;
  rel?: string;
  download?: string | boolean;
  hrefLang?: string;
  type?: string;
  referrerPolicy?: string;
  ping?: string;
};

type DxImgAttributes = DxHtmlAttributes & {
  src?: string;
  alt?: string;
  width?: NumberOrString;
  height?: NumberOrString;
  loading?: "lazy" | "eager";
  decoding?: "auto" | "async" | "sync";
  srcSet?: string;
  srcset?: string;
  sizes?: string;
  crossOrigin?: "anonymous" | "use-credentials" | "";
  referrerPolicy?: string;
  useMap?: string;
  isMap?: boolean;
  fetchpriority?: "high" | "low" | "auto";
};

type DxVideoAttributes = DxHtmlAttributes & {
  src?: string;
  autoPlay?: boolean;
  autoplay?: boolean;
  controls?: boolean;
  loop?: boolean;
  muted?: boolean;
  playsInline?: boolean;
  playsinline?: boolean;
  poster?: string;
  preload?: string;
  width?: NumberOrString;
  height?: NumberOrString;
  crossOrigin?: string;
  disablePictureInPicture?: boolean;
  disableRemotePlayback?: boolean;
};

type DxAudioAttributes = DxHtmlAttributes & {
  src?: string;
  autoPlay?: boolean;
  autoplay?: boolean;
  controls?: boolean;
  loop?: boolean;
  muted?: boolean;
  preload?: string;
  crossOrigin?: string;
  disableRemotePlayback?: boolean;
};

type DxIframeAttributes = DxHtmlAttributes & {
  src?: string;
  srcdoc?: string;
  name?: string;
  width?: NumberOrString;
  height?: NumberOrString;
  allow?: string;
  allowFullScreen?: boolean;
  allowfullscreen?: boolean;
  sandbox?: string;
  loading?: "lazy" | "eager";
  referrerPolicy?: string;
  fetchpriority?: "high" | "low" | "auto";
};

type DxScriptAttributes = DxHtmlAttributes & {
  src?: string;
  type?: string;
  async?: boolean;
  defer?: boolean;
  crossOrigin?: string;
  integrity?: string;
  nonce?: string;
  noModule?: boolean;
  referrerPolicy?: string;
};

type DxLinkAttributes = DxHtmlAttributes & {
  href?: string;
  rel?: string;
  type?: string;
  media?: string;
  crossOrigin?: string;
  integrity?: string;
  as?: string;
  fetchpriority?: "high" | "low" | "auto";
  hrefLang?: string;
  sizes?: string;
  imageSrcSet?: string;
  imageSizes?: string;
  referrerPolicy?: string;
  disabled?: boolean;
};

type DxMetaAttributes = DxHtmlAttributes & {
  name?: string;
  content?: string;
  httpEquiv?: string;
  "http-equiv"?: string;
  charSet?: string;
  charset?: string;
  property?: string;
  media?: string;
};

type DxStyleAttributes = DxHtmlAttributes & {
  media?: string;
  nonce?: string;
  scoped?: boolean;
  type?: string;
};

// ─── SVG Attribute Types ─────────────────────────────────────────────────────

type DxSvgAttributes = DxHtmlAttributes & {
  xmlns?: string;
  xmlnsXlink?: string;
  viewBox?: string;
  width?: NumberOrString;
  height?: NumberOrString;
  x?: NumberOrString;
  y?: NumberOrString;
  fill?: string;
  fillOpacity?: NumberOrString;
  fillRule?: "nonzero" | "evenodd" | "inherit";
  stroke?: string;
  strokeWidth?: NumberOrString;
  strokeLinecap?: "butt" | "round" | "square" | "inherit";
  strokeLinejoin?: "miter" | "round" | "bevel" | "inherit";
  strokeDasharray?: string | number;
  strokeDashoffset?: NumberOrString;
  strokeOpacity?: NumberOrString;
  strokeMiterlimit?: NumberOrString;
  opacity?: NumberOrString;
  transform?: string;
  clipPath?: string;
  clipRule?: string;
  mask?: string;
  filter?: string;
  overflow?: string;
  preserveAspectRatio?: string;
  color?: string;
  colorInterpolation?: string;
  colorInterpolationFilters?: string;
  cursor?: string;
  display?: string;
  dominantBaseline?: string;
  enableBackground?: string;
  fontFamily?: string;
  fontSize?: NumberOrString;
  fontSizeAdjust?: NumberOrString;
  fontStretch?: string;
  fontStyle?: string;
  fontVariant?: string;
  fontWeight?: NumberOrString;
  imageRendering?: string;
  letterSpacing?: NumberOrString;
  lightingColor?: string;
  markerEnd?: string;
  markerMid?: string;
  markerStart?: string;
  pointerEvents?: string;
  shapeRendering?: string;
  stopColor?: string;
  stopOpacity?: NumberOrString;
  textAnchor?: string;
  textDecoration?: string;
  textRendering?: string;
  unicodeBidi?: string;
  vectorEffect?: string;
  visibility?: string;
  wordSpacing?: NumberOrString;
  writingMode?: string;
};

type DxIconAttributes = DxHtmlAttributes & {
  "data-dx-icon"?: string;
  "data-icon-source"?: string;
  name?: string;
};

// ─── JSX Namespace ───────────────────────────────────────────────────────────

declare namespace JSX {
  type Element = string;
  interface ElementChildrenAttribute {
    children: {};
  }
  interface IntrinsicElements {
    // Document structure
    html: DxHtmlAttributes & { lang?: string; manifest?: string };
    head: DxHtmlAttributes;
    body: DxHtmlAttributes;
    // Metadata
    title: DxHtmlAttributes;
    meta: DxMetaAttributes;
    link: DxLinkAttributes;
    style: DxStyleAttributes;
    script: DxScriptAttributes;
    base: DxHtmlAttributes & { href?: string; target?: string };
    // Sections
    header: DxHtmlAttributes;
    footer: DxHtmlAttributes;
    main: DxHtmlAttributes;
    nav: DxHtmlAttributes;
    aside: DxHtmlAttributes;
    article: DxHtmlAttributes;
    section: DxHtmlAttributes;
    address: DxHtmlAttributes;
    hgroup: DxHtmlAttributes;
    search: DxHtmlAttributes;
    // Headings
    h1: DxHtmlAttributes;
    h2: DxHtmlAttributes;
    h3: DxHtmlAttributes;
    h4: DxHtmlAttributes;
    h5: DxHtmlAttributes;
    h6: DxHtmlAttributes;
    // Text
    p: DxHtmlAttributes;
    span: DxHtmlAttributes;
    strong: DxHtmlAttributes;
    em: DxHtmlAttributes;
    small: DxHtmlAttributes;
    mark: DxHtmlAttributes;
    del: DxHtmlAttributes & { cite?: string; dateTime?: string };
    ins: DxHtmlAttributes & { cite?: string; dateTime?: string };
    sub: DxHtmlAttributes;
    sup: DxHtmlAttributes;
    abbr: DxHtmlAttributes;
    cite: DxHtmlAttributes;
    code: DxHtmlAttributes;
    pre: DxHtmlAttributes;
    kbd: DxHtmlAttributes;
    samp: DxHtmlAttributes;
    var: DxHtmlAttributes;
    time: DxHtmlAttributes & { dateTime?: string };
    q: DxHtmlAttributes & { cite?: string };
    blockquote: DxHtmlAttributes & { cite?: string };
    bdi: DxHtmlAttributes;
    bdo: DxHtmlAttributes & { dir?: string };
    data: DxHtmlAttributes & { value?: string };
    dfn: DxHtmlAttributes;
    ruby: DxHtmlAttributes;
    rp: DxHtmlAttributes;
    rt: DxHtmlAttributes;
    s: DxHtmlAttributes;
    u: DxHtmlAttributes;
    i: DxHtmlAttributes;
    b: DxHtmlAttributes;
    // Layout
    div: DxHtmlAttributes;
    hr: DxHtmlAttributes;
    br: DxHtmlAttributes;
    wbr: DxHtmlAttributes;
    // Lists
    ul: DxHtmlAttributes;
    ol: DxHtmlAttributes & { type?: string; start?: number; reversed?: boolean };
    li: DxHtmlAttributes & { value?: number };
    dl: DxHtmlAttributes;
    dt: DxHtmlAttributes;
    dd: DxHtmlAttributes;
    // Tables
    table: DxHtmlAttributes;
    caption: DxHtmlAttributes;
    thead: DxHtmlAttributes;
    tbody: DxHtmlAttributes;
    tfoot: DxHtmlAttributes;
    tr: DxHtmlAttributes;
    th: DxHtmlAttributes & { scope?: string; colSpan?: number; rowSpan?: number; abbr?: string; headers?: string };
    td: DxHtmlAttributes & { colSpan?: number; rowSpan?: number; headers?: string };
    colgroup: DxHtmlAttributes & { span?: number };
    col: DxHtmlAttributes & { span?: number };
    // Forms
    form: DxFormAttributes;
    fieldset: DxHtmlAttributes & { disabled?: boolean; form?: string; name?: string };
    legend: DxHtmlAttributes;
    label: DxLabelAttributes;
    input: DxInputAttributes;
    button: DxButtonAttributes;
    select: DxSelectAttributes;
    option: DxOptionAttributes;
    optgroup: DxHtmlAttributes & { label?: string; disabled?: boolean };
    textarea: DxTextareaAttributes;
    output: DxHtmlAttributes & { htmlFor?: string; for?: string; form?: string; name?: string };
    progress: DxHtmlAttributes & { value?: NumberOrString; max?: NumberOrString };
    meter: DxHtmlAttributes & { value?: NumberOrString; min?: NumberOrString; max?: NumberOrString; low?: NumberOrString; high?: NumberOrString; optimum?: NumberOrString; form?: string };
    datalist: DxHtmlAttributes;
    // Media
    img: DxImgAttributes;
    figure: DxHtmlAttributes;
    figcaption: DxHtmlAttributes;
    picture: DxHtmlAttributes;
    source: DxHtmlAttributes & { src?: string; srcSet?: string; srcset?: string; media?: string; type?: string; sizes?: string; width?: NumberOrString; height?: NumberOrString };
    video: DxVideoAttributes;
    audio: DxAudioAttributes;
    track: DxHtmlAttributes & { kind?: string; src?: string; srcLang?: string; srclang?: string; label?: string; default?: boolean };
    canvas: DxHtmlAttributes & { width?: NumberOrString; height?: NumberOrString };
    iframe: DxIframeAttributes;
    object: DxHtmlAttributes & { data?: string; type?: string; width?: NumberOrString; height?: NumberOrString; form?: string; name?: string; useMap?: string };
    embed: DxHtmlAttributes & { src?: string; type?: string; width?: NumberOrString; height?: NumberOrString };
    map: DxHtmlAttributes & { name?: string };
    area: DxAnchorOrAreaAttributes & { shape?: string; coords?: string; alt?: string };
    // Interactive
    a: DxAnchorAttributes;
    details: DxHtmlAttributes & { open?: boolean };
    summary: DxHtmlAttributes;
    dialog: DxHtmlAttributes & { open?: boolean };
    menu: DxHtmlAttributes & { type?: string };
    // Scripting / Templating
    noscript: DxHtmlAttributes;
    template: DxHtmlAttributes;
    slot: DxHtmlAttributes & { name?: string };
    // SVG
    svg: DxSvgAttributes;
    path: DxSvgAttributes & { d?: string };
    circle: DxSvgAttributes & { cx?: NumberOrString; cy?: NumberOrString; r?: NumberOrString };
    rect: DxSvgAttributes & { x?: NumberOrString; y?: NumberOrString; width?: NumberOrString; height?: NumberOrString; rx?: NumberOrString; ry?: NumberOrString };
    line: DxSvgAttributes & { x1?: NumberOrString; y1?: NumberOrString; x2?: NumberOrString; y2?: NumberOrString };
    polyline: DxSvgAttributes & { points?: string };
    polygon: DxSvgAttributes & { points?: string };
    ellipse: DxSvgAttributes & { cx?: NumberOrString; cy?: NumberOrString; rx?: NumberOrString; ry?: NumberOrString };
    g: DxSvgAttributes;
    defs: DxSvgAttributes;
    use: DxSvgAttributes & { href?: string; xlinkHref?: string };
    symbol: DxSvgAttributes & { viewBox?: string; refX?: NumberOrString; refY?: NumberOrString };
    image: DxSvgAttributes & { href?: string; xlinkHref?: string; preserveAspectRatio?: string };
    text: DxSvgAttributes & { dx?: NumberOrString; dy?: NumberOrString; rotate?: string; textLength?: NumberOrString; lengthAdjust?: string };
    tspan: DxSvgAttributes & { dx?: NumberOrString; dy?: NumberOrString; rotate?: string; textLength?: NumberOrString; lengthAdjust?: string };
    textPath: DxSvgAttributes & { href?: string; startOffset?: NumberOrString; method?: string; spacing?: string; side?: string };
    clipPath: DxSvgAttributes & { clipPathUnits?: string };
    mask: DxSvgAttributes & { maskUnits?: string; maskContentUnits?: string };
    pattern: DxSvgAttributes & { patternUnits?: string; patternContentUnits?: string; patternTransform?: string };
    filter: DxSvgAttributes & { filterUnits?: string; primitiveUnits?: string };
    feBlend: DxSvgAttributes & { in?: string; in2?: string; mode?: string; result?: string };
    feColorMatrix: DxSvgAttributes & { in?: string; type?: string; values?: string; result?: string };
    feComposite: DxSvgAttributes & { in?: string; in2?: string; operator?: string; k1?: number; k2?: number; k3?: number; k4?: number; result?: string };
    feGaussianBlur: DxSvgAttributes & { in?: string; stdDeviation?: NumberOrString; edgeMode?: string; result?: string };
    feOffset: DxSvgAttributes & { in?: string; dx?: NumberOrString; dy?: NumberOrString; result?: string };
    feMerge: DxSvgAttributes;
    feMergeNode: DxSvgAttributes & { in?: string };
    feDiffuseLighting: DxSvgAttributes & { in?: string; surfaceScale?: number; diffuseConstant?: number; result?: string };
    feSpecularLighting: DxSvgAttributes & { in?: string; surfaceScale?: number; specularConstant?: number; specularExponent?: number; result?: string };
    fePointLight: DxSvgAttributes & { x?: number; y?: number; z?: number };
    feSpotLight: DxSvgAttributes & { x?: number; y?: number; z?: number; pointsAtX?: number; pointsAtY?: number; pointsAtZ?: number };
    feDistantLight: DxSvgAttributes & { azimuth?: number; elevation?: number };
    feFlood: DxSvgAttributes & { floodColor?: string; floodOpacity?: NumberOrString; result?: string };
    feTurbulence: DxSvgAttributes & { baseFrequency?: NumberOrString; numOctaves?: number; seed?: number; stitchTiles?: string; type?: string; result?: string };
    feDisplacementMap: DxSvgAttributes & { in?: string; in2?: string; scale?: number; xChannelSelector?: string; yChannelSelector?: string; result?: string };
    feImage: DxSvgAttributes & { href?: string; preserveAspectRatio?: string; result?: string };
    feComponentTransfer: DxSvgAttributes & { in?: string; result?: string };
    feFuncR: DxSvgAttributes & { type?: string; tableValues?: string; slope?: number; intercept?: number; amplitude?: number; exponent?: number; offset?: number };
    feFuncG: DxSvgAttributes & { type?: string; tableValues?: string; slope?: number; intercept?: number; amplitude?: number; exponent?: number; offset?: number };
    feFuncB: DxSvgAttributes & { type?: string; tableValues?: string; slope?: number; intercept?: number; amplitude?: number; exponent?: number; offset?: number };
    feFuncA: DxSvgAttributes & { type?: string; tableValues?: string; slope?: number; intercept?: number; amplitude?: number; exponent?: number; offset?: number };
    linearGradient: DxSvgAttributes & { x1?: NumberOrString; y1?: NumberOrString; x2?: NumberOrString; y2?: NumberOrString; gradientUnits?: string; gradientTransform?: string; spreadMethod?: string; href?: string };
    radialGradient: DxSvgAttributes & { cx?: NumberOrString; cy?: NumberOrString; r?: NumberOrString; fx?: NumberOrString; fy?: NumberOrString; gradientUnits?: string; gradientTransform?: string; spreadMethod?: string; href?: string };
    stop: DxSvgAttributes & { offset?: NumberOrString };
    animate: DxSvgAttributes & { attributeName?: string; from?: string; to?: string; dur?: string; repeatCount?: string; begin?: string };
    animateTransform: DxSvgAttributes & { attributeName?: string; type?: string; from?: string; to?: string; dur?: string; repeatCount?: string; begin?: string };
    animateMotion: DxSvgAttributes & { dur?: string; repeatCount?: string; path?: string; begin?: string };
    mpath: DxSvgAttributes & { href?: string };
    set: DxSvgAttributes & { attributeName?: string; to?: string; begin?: string; dur?: string };
    desc: DxSvgAttributes;
    title: DxHtmlAttributes;
    metadata: DxSvgAttributes;
    switch: DxSvgAttributes;
    foreignObject: DxSvgAttributes & { x?: NumberOrString; y?: NumberOrString; width?: NumberOrString; height?: NumberOrString };
    marker: DxSvgAttributes & { markerWidth?: NumberOrString; markerHeight?: NumberOrString; markerUnits?: string; orient?: string; refX?: NumberOrString; refY?: NumberOrString; viewBox?: string };
    // DX-specific custom elements
    "dx-icon": DxIconAttributes;
  }
}
