/// Source-owned native DOM event names that DX WWW can lower from React-style
/// `onX` TSX attributes into browser `addEventListener` names.
const NATIVE_DOM_EVENT_NAMES: &[&str] = &[
    "abort",
    "abortpayment",
    "activate",
    "active",
    "addsourcebuffer",
    "addstream",
    "addtrack",
    "afterprint",
    "afterscriptexecute",
    "animationcancel",
    "animationend",
    "animationiteration",
    "animationstart",
    "appinstalled",
    "audioend",
    "audioprocess",
    "audiostart",
    "auxclick",
    "backgroundfetchabort",
    "backgroundfetchclick",
    "backgroundfetchfail",
    "backgroundfetchsuccess",
    "beforeinput",
    "beforeinstallprompt",
    "beforematch",
    "beforeprint",
    "beforescriptexecute",
    "beforetoggle",
    "beforeunload",
    "beforexrselect",
    "beginevent",
    "blocked",
    "blur",
    "bounce",
    "boundary",
    "bufferedamountlow",
    "bufferedchange",
    "cancel",
    "canmakepayment",
    "canplay",
    "canplaythrough",
    "capturehandlechange",
    "change",
    "characterboundsupdate",
    "characteristicvaluechanged",
    "chargingchange",
    "chargingtimechange",
    "click",
    "clipboardchange",
    "close",
    "closing",
    "command",
    "complete",
    "compositionend",
    "compositionstart",
    "compositionupdate",
    "connect",
    "connecting",
    "connectionavailable",
    "connectionstatechange",
    "contentdelete",
    "contentvisibilityautostatechange",
    "contextlost",
    "contextmenu",
    "contextoverflow",
    "contextrestored",
    "controllerchange",
    "cookiechange",
    "copy",
    "cuechange",
    "currententrychange",
    "currentscreenchange",
    "cut",
    "dataavailable",
    "datachannel",
    "dblclick",
    "dequeue",
    "devicechange",
    "devicemotion",
    "deviceorientation",
    "deviceorientationabsolute",
    "dischargingtimechange",
    "disconnect",
    "dispose",
    "domactivate",
    "domcontentloaded",
    "dommousescroll",
    "downloadprogress",
    "drag",
    "dragend",
    "dragenter",
    "dragexit",
    "dragleave",
    "dragover",
    "dragstart",
    "drop",
    "durationchange",
    "emptied",
    "encrypted",
    "end",
    "ended",
    "endevent",
    "endstreaming",
    "enter",
    "enterpictureinpicture",
    "error",
    "exit",
    "fetch",
    "finish",
    "focus",
    "focusin",
    "focusout",
    "formdata",
    "freeze",
    "fullscreenchange",
    "fullscreenerror",
    "gamepadconnected",
    "gamepaddisconnected",
    "gatheringstatechange",
    "gattserverdisconnected",
    "geometrychange",
    "gesturechange",
    "gestureend",
    "gesturestart",
    "gotpointercapture",
    "hashchange",
    "icecandidate",
    "icecandidateerror",
    "iceconnectionstatechange",
    "icegatheringstatechange",
    "inactive",
    "input",
    "inputreport",
    "inputsourceschange",
    "install",
    "interest",
    "invalid",
    "keydown",
    "keypress",
    "keystatuseschange",
    "keyup",
    "languagechange",
    "leavepictureinpicture",
    "levelchange",
    "load",
    "loadeddata",
    "loadedmetadata",
    "loadend",
    "loading",
    "loadingdone",
    "loadingerror",
    "loadstart",
    "location",
    "loseinterest",
    "lostpointercapture",
    "managedconfigurationchange",
    "mark",
    "merchantvalidation",
    "message",
    "messageerror",
    "midimessage",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "mousewheel",
    "mozmousepixelscroll",
    "mute",
    "navigate",
    "navigateerror",
    "navigatesuccess",
    "negotiationneeded",
    "nomatch",
    "notificationclick",
    "notificationclose",
    "offline",
    "online",
    "open",
    "orientationchange",
    "pagehide",
    "pagereveal",
    "pageshow",
    "pageswap",
    "paste",
    "pause",
    "payerdetailchange",
    "paymentmethodchange",
    "paymentrequest",
    "periodicsync",
    "play",
    "playing",
    "pointercancel",
    "pointerdown",
    "pointerenter",
    "pointerleave",
    "pointerlockchange",
    "pointerlockerror",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerrawupdate",
    "pointerup",
    "popstate",
    "prerenderingchange",
    "prioritychange",
    "processorerror",
    "progress",
    "promptaction",
    "promptdismiss",
    "push",
    "pushsubscriptionchange",
    "ratechange",
    "reading",
    "readingerror",
    "readystatechange",
    "redraw",
    "reflectionchange",
    "rejectionhandled",
    "release",
    "remove",
    "removesourcebuffer",
    "removestream",
    "removetrack",
    "repeatevent",
    "reset",
    "resize",
    "resourcetimingbufferfull",
    "result",
    "resume",
    "rtctransform",
    "screenschange",
    "scroll",
    "scrollend",
    "scrollsnapchange",
    "scrollsnapchanging",
    "search",
    "securitypolicyviolation",
    "seeked",
    "seeking",
    "select",
    "selectedcandidatepairchange",
    "selectend",
    "selectionchange",
    "selectstart",
    "shippingaddresschange",
    "shippingoptionchange",
    "show",
    "signalingstatechange",
    "sinkchange",
    "slotchange",
    "soundend",
    "soundstart",
    "sourceclose",
    "sourceended",
    "sourceopen",
    "speechend",
    "speechstart",
    "squeeze",
    "squeezeend",
    "squeezestart",
    "stalled",
    "start",
    "startstreaming",
    "statechange",
    "stop",
    "storage",
    "submit",
    "success",
    "suspend",
    "sync",
    "terminate",
    "textformatupdate",
    "textupdate",
    "timeout",
    "timeupdate",
    "toggle",
    "tonechange",
    "touchcancel",
    "touchend",
    "touchmove",
    "touchstart",
    "track",
    "transitioncancel",
    "transitionend",
    "transitionrun",
    "transitionstart",
    "typechange",
    "uncapturederror",
    "unhandledrejection",
    "unload",
    "unmute",
    "update",
    "updateend",
    "updatefound",
    "updatestart",
    "upgradeneeded",
    "validationstatuschange",
    "versionchange",
    "visibilitychange",
    "visibilitymaskchange",
    "voiceschanged",
    "volumechange",
    "vrdisplayactivate",
    "vrdisplayconnect",
    "vrdisplaydeactivate",
    "vrdisplaydisconnect",
    "vrdisplaypresentchange",
    "waiting",
    "waitingforkey",
    "webglcontextcreationerror",
    "webglcontextlost",
    "webglcontextrestored",
    "webkitmouseforcechanged",
    "webkitmouseforcedown",
    "webkitmouseforceup",
    "webkitmouseforcewillbegin",
    "wheel",
    "zoomlevelchange",
];

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct NativeDomEventCatalogIntegrity {
    pub source_of_truth: &'static str,
    pub catalog_count: usize,
    pub sorted_unique: bool,
    pub duplicate_events: Vec<&'static str>,
    pub catalog_hash: String,
}

pub fn native_dom_event_names() -> &'static [&'static str] {
    NATIVE_DOM_EVENT_NAMES
}

pub fn native_dom_event_catalog_integrity() -> NativeDomEventCatalogIntegrity {
    let duplicate_events = duplicate_native_dom_event_names();
    NativeDomEventCatalogIntegrity {
        source_of_truth: "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES",
        catalog_count: NATIVE_DOM_EVENT_NAMES.len(),
        sorted_unique: duplicate_events.is_empty() && native_dom_event_names_are_sorted(),
        duplicate_events,
        catalog_hash: native_dom_event_catalog_hash(),
    }
}

pub fn react_style_event_attribute_to_dom_event(attribute_name: &str) -> Option<String> {
    let mapped = match attribute_name {
        "onDoubleClick" => Some("dblclick"),
        _ => None,
    };
    if let Some(event_name) = mapped {
        return Some(event_name.to_string());
    }

    let rest = attribute_name.strip_prefix("on")?;
    let mut characters = rest.chars();
    let first = characters.next()?;
    if !first.is_ascii_uppercase() {
        return None;
    }
    let mut event_name = first.to_ascii_lowercase().to_string();
    event_name.extend(characters.flat_map(char::to_lowercase));
    native_dom_event_names()
        .contains(&event_name.as_str())
        .then_some(event_name)
}

fn duplicate_native_dom_event_names() -> Vec<&'static str> {
    let mut duplicate_events = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for event_name in NATIVE_DOM_EVENT_NAMES {
        if !seen.insert(*event_name) {
            duplicate_events.push(*event_name);
        }
    }
    duplicate_events
}

fn native_dom_event_names_are_sorted() -> bool {
    NATIVE_DOM_EVENT_NAMES
        .windows(2)
        .all(|window| window[0] < window[1])
}

fn native_dom_event_catalog_hash() -> String {
    let mut hasher = blake3::Hasher::new();
    for event_name in NATIVE_DOM_EVENT_NAMES {
        hasher.update(event_name.as_bytes());
        hasher.update(b"\0");
    }
    format!("blake3:{}", hasher.finalize().to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_dom_event_catalog_integrity_reports_sorted_unique_hash() {
        let integrity = native_dom_event_catalog_integrity();

        assert_eq!(
            integrity.source_of_truth,
            "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES"
        );
        assert_eq!(integrity.catalog_count, NATIVE_DOM_EVENT_NAMES.len());
        assert!(integrity.sorted_unique);
        assert!(integrity.duplicate_events.is_empty());
        assert!(integrity.catalog_hash.starts_with("blake3:"));
    }

    #[test]
    fn react_style_event_attributes_map_from_source_catalog() {
        for (attribute, dom_event) in [
            ("onClick", "click"),
            ("onInput", "input"),
            ("onPointerMove", "pointermove"),
        ] {
            assert!(native_dom_event_names().contains(&dom_event));
            assert_eq!(
                react_style_event_attribute_to_dom_event(attribute).as_deref(),
                Some(dom_event)
            );
        }
    }

    #[test]
    fn unsupported_react_style_events_are_not_silently_lowered() {
        let unsupported: [(&str, Option<&str>); 4] = [
            ("onMagicGesture", None),
            ("onOnce_per", None),
            ("onclick", None),
            ("on", None),
        ];

        for (attribute, expected) in unsupported {
            assert_eq!(
                react_style_event_attribute_to_dom_event(attribute).as_deref(),
                expected
            );
        }
    }

    #[test]
    fn native_dom_event_catalog_excludes_non_browser_pseudo_events() {
        assert!(!native_dom_event_names().contains(&"once_per"));
    }
}
