const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real Zod codec helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/codecs.ts", ZOD_CODECS_TS)',
    "const ZOD_CODECS_TS",
    "z.codec",
    "z.decode",
    "z.encode",
    "z.safeDecode",
    "z.safeEncode",
    "z.decodeAsync",
    "z.encodeAsync",
    "z.safeDecodeAsync",
    "z.safeEncodeAsync",
    "z.iso.datetime()",
    "z.date()",
    "dxIsoDateCodec",
    "decodeDxIsoDate",
    "encodeDxIsoDate",
    "safeDecodeDxIsoDate",
    "safeEncodeDxIsoDate",
    "safeDecodeDxIsoDateAsync",
    "safeEncodeDxIsoDateAsync",
    '"z.codec"',
    '"z.decode"',
    '"z.encode"',
    '"z.safeDecode"',
    '"z.safeEncode"',
    '"z.decodeAsync"',
    '"z.encodeAsync"',
    '"z.safeDecodeAsync"',
    '"z.safeEncodeAsync"',
    '"lib/validation/zod/codecs.ts"',
    'codecHelper: "decodeDxIsoDate(value)"',
  ]) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing ${marker}`);
  }
});

test("launch template consumes Zod codec helpers for a real boundary transform", () => {
  const status = read("examples/template/zod-validation-status.tsx");

  for (const marker of [
    '@/lib/validation/zod/codecs',
    "decodeDxIsoDate",
    "safeEncodeDxIsoDate",
    "data-dx-zod-codec-status",
    "round-trip",
    "Launch Team",
  ]) {
    assert.match(status, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing ${marker}`);
  }
});
