import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function extractFunction(source, signature) {
  const start = source.indexOf(signature);
  assert.notEqual(start, -1, `missing function signature ${signature}`);
  const nextFunction = source.indexOf("\nfn ", start + signature.length);
  return source.slice(start, nextFunction === -1 ? source.length : nextFunction);
}

test("generic JSON receipt machine alias writes typed v2 cache before v1 document fallback", () => {
  const writer = read("core/src/ecosystem/json_receipt_machine.rs");
  const reader = writer;
  const cliWriter = read("dx-www/src/cli/serializer_artifacts.rs");
  const ecosystem = read("core/src/ecosystem/mod.rs");

  assert.match(writer, /JSON_RECEIPT_MACHINE_SCHEMA_V2:\s*&str\s*=\s*"dx\.www\.json_receipt"/);
  assert.match(reader, /JSON_RECEIPT_MACHINE_SCHEMA_V2:\s*&str\s*=\s*"dx\.www\.json_receipt"/);
  assert.match(writer, /JSON_RECEIPT_MACHINE_SCHEMA_VERSION:\s*u32\s*=\s*2/);
  assert.match(reader, /JSON_RECEIPT_MACHINE_SCHEMA_VERSION:\s*u32\s*=\s*2/);
  assert.match(writer, /struct JsonReceiptMachineCache/);
  assert.match(reader, /struct JsonReceiptMachineCache/);
  assert.match(writer, /enum JsonReceiptMachineJsonValue/);
  assert.match(reader, /enum JsonReceiptMachineJsonValue/);
  assert.match(writer, /write_typed_machine_cache\(/);
  assert.match(reader, /open_typed_machine_cache::<JsonReceiptMachineCache>/);
  assert.match(reader, /access_typed_machine_cache::<JsonReceiptMachineCache>/);
  assert.match(writer, /MachineCacheKind::Receipt/);
  assert.match(reader, /MachineCacheKind::Receipt/);
  assert.match(writer, /MachineCacheCodec::None/);
  assert.match(writer, /mod tests/);
  assert.match(writer, /json_receipt_typed_machine_round_trips_nested_report/);
  assert.match(writer, /json_receipt_typed_machine_rejects_stale_source/);
  assert.match(writer, /json_receipt_typed_machine_rejects_mismatched_source_path/);
  assert.match(writer, /json_receipt_machine_rejects_cache_name_and_path_guards/);
  assert.match(writer, /json_receipt_machine_preserves_legacy_v1_document_fallback/);

  const typedReaderStart = reader.indexOf(
    "fn read_json_receipt_typed_machine_alias",
  );
  const legacyReaderStart = reader.indexOf(
    "fn read_json_receipt_document_machine_alias",
  );
  assert.notEqual(typedReaderStart, -1);
  assert.notEqual(legacyReaderStart, -1);
  assert.ok(
    typedReaderStart < legacyReaderStart,
    "reader must attempt typed v2 cache before legacy v1 document alias",
  );

  const publicReader = extractFunction(
    reader,
    "pub(crate) fn read_json_receipt_machine_alias",
  );
  assert.match(publicReader, /read_json_receipt_typed_machine_alias\(/);
  assert.match(publicReader, /read_json_receipt_document_machine_alias\(/);
  assert.ok(
    publicReader.indexOf("read_json_receipt_typed_machine_alias(") <
      publicReader.indexOf("read_json_receipt_document_machine_alias("),
    "public reader must preserve v2 -> v1 -> source fallback order",
  );

  const typedWriter = extractFunction(writer, "pub fn write_json_receipt_machine_alias");
  assert.doesNotMatch(typedWriter, /serde_json::to_string\(report\)/);
  assert.doesNotMatch(typedWriter, /json_receipt_context_key\("report_json"\)/);
  assert.match(typedWriter, /JsonReceiptMachineCache::from_report\(receipt_relative_path,\s*report\)/);
  assert.match(
    cliWriter,
    /dx_compiler::ecosystem::write_json_receipt_machine_alias\(/,
  );
  assert.doesNotMatch(cliWriter, /serde_json::to_string\(report\)/);
  assert.doesNotMatch(cliWriter, /json_receipt_context_key\("report_json"\)/);
  assert.match(
    ecosystem,
    /pub use json_receipt_machine::write_json_receipt_machine_alias;/,
  );

  const typedReader = extractFunction(reader, "fn read_json_receipt_typed_machine_alias");
  const archiveReader = extractFunction(reader, "fn json_receipt_from_archive");
  assert.doesNotMatch(typedReader, /serde_json::from_str/);
  assert.match(archiveReader, /cache\.source_path\s*==\s*source_relative_path/);
  assert.match(archiveReader, /cache\.report\.into_json_value\(\)/);

  const legacyReader = extractFunction(reader, "fn read_json_receipt_document_machine_alias");
  assert.match(legacyReader, /JSON_RECEIPT_MACHINE_SCHEMA_V1/);
  assert.match(legacyReader, /serde_json::from_str/);
});
