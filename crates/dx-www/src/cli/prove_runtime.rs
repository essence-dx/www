/// Browser-side runtime fixture used by `dx prove vertical --write`.
///
/// This is intentionally a proof harness, not the final production runtime. It
/// proves the canonical `DXPK` envelope can be fetched, decoded, and applied to
/// the fallback DOM without pulling in npm, wasm, or framework runtime code.
pub(super) const DXPK_RUNTIME_FIXTURE_JS: &str = r#"const DXPK_HEADER_LEN = 52;
const DXPK_SECTION_HEADER_LEN = 40;
const decoder = new TextDecoder();

const packetKinds = {
  1: "Route",
  2: "TemplateDictionary",
  3: "InstanceBatch",
  4: "PatchStream",
  5: "Style",
  6: "Manifest",
};

const sectionKinds = {
  1: "FallbackHtmlRef",
  2: "TemplateSlots",
  3: "ColumnarSlots",
  4: "SemanticCodec",
  5: "PatchOps",
  6: "StyleGraph",
  7: "SourceManifest",
};

const sectionEncodings = {
  1: "Json",
  2: "HtipV2",
  3: "DeliveryLab",
  4: "CanonicalBinary",
};

function hex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function readVarint(bytes, cursor) {
  let value = 0;
  let shift = 0;
  let offset = cursor.offset;
  while (offset < bytes.length) {
    const byte = bytes[offset];
    offset += 1;
    value += (byte & 0x7f) * 2 ** shift;
    if ((byte & 0x80) === 0) {
      cursor.offset = offset;
      return value;
    }
    shift += 7;
  }
  throw new Error("Truncated DXVT varint");
}

function readBytes(bytes, cursor) {
  const length = readVarint(bytes, cursor);
  const start = cursor.offset;
  const end = start + length;
  if (end > bytes.length) {
    throw new Error("Truncated DXVT bytes");
  }
  cursor.offset = end;
  return bytes.slice(start, end);
}

function decodeTemplateSlots(bytes) {
  const magic = decoder.decode(bytes.slice(0, 5));
  if (magic !== "DXVT1") {
    throw new Error(`Invalid DXVT magic: ${magic}`);
  }
  const cursor = { offset: 5 };
  const templates = [];
  const templateCount = readVarint(bytes, cursor);
  for (let index = 0; index < templateCount; index += 1) {
    const id = readVarint(bytes, cursor);
    const html = decoder.decode(readBytes(bytes, cursor));
    const slots = [];
    const slotCount = readVarint(bytes, cursor);
    for (let slotIndex = 0; slotIndex < slotCount; slotIndex += 1) {
      const slotId = readVarint(bytes, cursor);
      const slotType = bytes[cursor.offset];
      cursor.offset += 1;
      const path = decoder.decode(readBytes(bytes, cursor));
      slots.push({ id: slotId, slotType, path });
    }
    templates.push({ id, html, slots });
  }
  return templates;
}

function decodeDxPacket(buffer) {
  const view = new DataView(buffer);
  const bytes = new Uint8Array(buffer);
  let offset = 0;
  const readU8 = () => view.getUint8(offset++);
  const readU16 = () => {
    const value = view.getUint16(offset, true);
    offset += 2;
    return value;
  };
  const readU32 = () => {
    const value = view.getUint32(offset, true);
    offset += 4;
    return value;
  };
  const readHash = () => {
    const hash = bytes.slice(offset, offset + 32);
    offset += 32;
    return hex(hash);
  };

  const magic = decoder.decode(bytes.slice(0, 4));
  offset += 4;
  if (magic !== "DXPK") {
    throw new Error(`Invalid DXPK magic: ${magic}`);
  }

  const version = readU16();
  const kind = packetKinds[readU8()] || "Unknown";
  readU8();
  const flags = readU32();
  const headerLength = readU16();
  if (headerLength !== DXPK_HEADER_LEN) {
    throw new Error(`Unsupported DXPK header length: ${headerLength}`);
  }
  const sectionCount = readU16();
  const payloadLength = readU32();
  const payloadHash = readHash();
  const sections = [];

  for (let index = 0; index < sectionCount; index += 1) {
    const sectionKind = sectionKinds[readU8()] || "Unknown";
    const encoding = sectionEncodings[readU8()] || "Unknown";
    readU16();
    const length = readU32();
    const contentHash = readHash();
    const start = offset;
    const end = start + length;
    if (end > bytes.length) {
      throw new Error(`Truncated DXPK section: ${sectionKind}`);
    }
    sections.push({
      kind: sectionKind,
      encoding,
      length,
      contentHash,
      bytes: bytes.slice(start, end),
      headerBytes: DXPK_SECTION_HEADER_LEN,
    });
    offset = end;
  }

  if (offset !== bytes.length) {
    throw new Error(`DXPK trailing bytes: ${bytes.length - offset}`);
  }

  const actualPayloadLength = sections.reduce((total, section) => total + section.length, 0);
  if (actualPayloadLength !== payloadLength) {
    throw new Error(`DXPK payload length mismatch: ${actualPayloadLength} !== ${payloadLength}`);
  }

  return {
    format: "dxp-v1",
    header: { version, kind, flags, sectionCount, payloadLength, payloadHash },
    sections,
  };
}

function jsonSection(packet, kind) {
  const section = packet.sections.find((candidate) => candidate.kind === kind);
  if (!section || section.encoding !== "Json") {
    return null;
  }
  return JSON.parse(decoder.decode(section.bytes));
}

function templateSection(packet) {
  const section = packet.sections.find((candidate) => candidate.kind === "TemplateSlots");
  if (!section || section.encoding !== "CanonicalBinary") {
    return [];
  }
  return decodeTemplateSlots(section.bytes);
}

function applyPacketToDom(packet, packetUrl) {
  const fallback = jsonSection(packet, "FallbackHtmlRef");
  const manifest = jsonSection(packet, "SourceManifest");
  const templates = templateSection(packet);
  const root = document.documentElement;
  root.dataset.dxPacketStatus = "applied";
  root.dataset.dxPacketFormat = packet.format;
  root.dataset.dxPacketKind = packet.header.kind;
  root.dataset.dxPacketSections = String(packet.sections.length);
  root.dataset.dxPacketPayloadBytes = String(packet.header.payloadLength);
  root.dataset.dxPacketRoute = (manifest && manifest.route) || (fallback && fallback.route) || "";
  root.dataset.dxPacketTemplateCount = String(templates.length);

  const marker = document.createElement("meta");
  marker.id = "dx-packet-proof";
  marker.name = "dx-packet-proof";
  marker.content = JSON.stringify({
    packetUrl,
    format: packet.format,
    kind: packet.header.kind,
    sections: packet.sections.length,
    route: root.dataset.dxPacketRoute,
    templates: templates.length,
    fallback,
    manifest,
  });
  document.head.appendChild(marker);

  window.__DX_PACKET_PROOF__ = {
    applied: true,
    packetUrl,
    packet,
    fallback,
    manifest,
    templates,
  };
  return window.__DX_PACKET_PROOF__;
}

async function applyDxPacket() {
  const script = document.currentScript;
  const packetUrl = (script && script.dataset.dxPacket) || "index.dxp";
  try {
    const response = await fetch(packetUrl, { cache: "no-cache" });
    if (!response.ok) {
      throw new Error(`DXPK fetch failed: ${response.status}`);
    }
    const packet = decodeDxPacket(await response.arrayBuffer());
    return applyPacketToDom(packet, packetUrl);
  } catch (error) {
    document.documentElement.dataset.dxPacketStatus = "error";
    document.documentElement.dataset.dxPacketError = error instanceof Error ? error.message : String(error);
    console.error(error);
    return { applied: false, error: document.documentElement.dataset.dxPacketError };
  }
}

window.__DX_PACKET_APPLIED__ = applyDxPacket();
"#;

pub(super) fn inject_dxpk_runtime_fixture(
    html: &str,
    packet_file_name: &str,
    runtime_file_name: &str,
) -> String {
    let packet_file_name = escape_attr(packet_file_name);
    let runtime_file_name = escape_attr(runtime_file_name);
    let preload = format!(
        r#"<link rel="preload" href="{packet_file_name}" as="fetch" type="application/octet-stream" crossorigin>"#
    );
    let script = format!(
        r#"<script src="{runtime_file_name}" data-dx-packet="{packet_file_name}"></script>"#
    );
    let html = insert_before_case_insensitive(html, "</head>", &preload)
        .unwrap_or_else(|| format!("{preload}{html}"));
    insert_before_case_insensitive(&html, "</body>", &script)
        .unwrap_or_else(|| format!("{html}{script}"))
}

fn insert_before_case_insensitive(source: &str, needle: &str, insert: &str) -> Option<String> {
    let index = source
        .to_ascii_lowercase()
        .find(&needle.to_ascii_lowercase())?;
    let mut output = String::with_capacity(source.len() + insert.len());
    output.push_str(&source[..index]);
    output.push_str(insert);
    output.push_str(&source[index..]);
    Some(output)
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injects_preload_and_runtime_before_document_closers() {
        let html =
            "<!doctype html><html><head><title>x</title></head><body><main></main></body></html>";
        let injected = inject_dxpk_runtime_fixture(html, "index.dxp", "index.dxp.js");

        assert!(injected.contains(r#"<link rel="preload" href="index.dxp""#));
        assert!(
            injected.contains(
                r#"<script src="index.dxp.js" data-dx-packet="index.dxp"></script></body>"#
            )
        );
        assert!(injected.find("<link").expect("preload") < injected.find("</head>").expect("head"));
        assert!(
            injected.find("<script").expect("script") < injected.find("</body>").expect("body")
        );
    }
}
