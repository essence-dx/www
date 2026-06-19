const BLOCK_LEN = 64;
const CHUNK_LEN = 1024;
const CHUNK_START = 1;
const CHUNK_END = 2;
const PARENT = 4;
const ROOT = 8;
const IV = [
  0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
  0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];
const MSG_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

function blake3Hex(bytes) {
  const chunkCount = Math.max(1, Math.ceil(bytes.length / CHUNK_LEN));
  const stack = [];
  for (let chunkIndex = 0; chunkIndex < chunkCount - 1; chunkIndex += 1) {
    addChunkCv(stack, chunkOutput(bytes.subarray(
      chunkIndex * CHUNK_LEN,
      (chunkIndex + 1) * CHUNK_LEN,
    ), chunkIndex), chunkIndex + 1);
  }

  let output = chunkOutput(bytes.subarray((chunkCount - 1) * CHUNK_LEN), chunkCount - 1);
  while (stack.length > 0) {
    output = parentOutput(stack.pop(), outputChainingValue(output));
  }
  return outputHex(output);
}

function addChunkCv(stack, output, totalChunks) {
  let cv = outputChainingValue(output);
  while ((totalChunks & 1) === 0) {
    cv = outputChainingValue(parentOutput(stack.pop(), cv));
    totalChunks >>= 1;
  }
  stack.push(cv);
}

function chunkOutput(bytes, chunkCounter) {
  let cv = IV;
  let blocksCompressed = 0;
  for (let offset = 0; offset <= bytes.length; offset += BLOCK_LEN) {
    const block = bytes.subarray(offset, Math.min(offset + BLOCK_LEN, bytes.length));
    const isLast = offset + BLOCK_LEN >= bytes.length;
    const flags =
      (blocksCompressed === 0 ? CHUNK_START : 0) |
      (isLast ? CHUNK_END : 0);
    const output = { cv, blockWords: wordsFromBlock(block), counter: chunkCounter, blockLen: block.length, flags };
    if (isLast) return output;
    cv = outputChainingValue(output);
    blocksCompressed += 1;
  }
  throw new Error("unreachable BLAKE3 chunk output state");
}

function parentOutput(leftCv, rightCv) {
  return { cv: IV, blockWords: leftCv.concat(rightCv), counter: 0, blockLen: BLOCK_LEN, flags: PARENT };
}

function outputChainingValue(output) {
  return compress(output).slice(0, 8);
}

function outputHex(output) {
  const bytes = Buffer.alloc(32);
  compress({ ...output, flags: output.flags | ROOT })
    .slice(0, 8)
    .forEach((word, index) => bytes.writeUInt32LE(word, index * 4));
  return bytes.toString("hex");
}

function compress(output) {
  const counterLow = output.counter >>> 0;
  const counterHigh = Math.floor(output.counter / 0x100000000) >>> 0;
  const v = output.cv.concat(
    IV.slice(0, 4),
    [counterLow, counterHigh, output.blockLen, output.flags],
  );
  let m = output.blockWords.slice();
  for (let round = 0; round < 7; round += 1) {
    roundFn(v, m);
    m = MSG_PERMUTATION.map((index) => m[index]);
  }
  return v.slice(0, 8).map((word, index) => (word ^ v[index + 8]) >>> 0)
    .concat(v.slice(8, 16).map((word, index) => (word ^ output.cv[index]) >>> 0));
}

function roundFn(v, m) {
  g(v, 0, 4, 8, 12, m[0], m[1]);
  g(v, 1, 5, 9, 13, m[2], m[3]);
  g(v, 2, 6, 10, 14, m[4], m[5]);
  g(v, 3, 7, 11, 15, m[6], m[7]);
  g(v, 0, 5, 10, 15, m[8], m[9]);
  g(v, 1, 6, 11, 12, m[10], m[11]);
  g(v, 2, 7, 8, 13, m[12], m[13]);
  g(v, 3, 4, 9, 14, m[14], m[15]);
}

function g(v, a, b, c, d, mx, my) {
  v[a] = (v[a] + v[b] + mx) >>> 0;
  v[d] = rotr(v[d] ^ v[a], 16);
  v[c] = (v[c] + v[d]) >>> 0;
  v[b] = rotr(v[b] ^ v[c], 12);
  v[a] = (v[a] + v[b] + my) >>> 0;
  v[d] = rotr(v[d] ^ v[a], 8);
  v[c] = (v[c] + v[d]) >>> 0;
  v[b] = rotr(v[b] ^ v[c], 7);
}

function wordsFromBlock(bytes) {
  const block = Buffer.alloc(BLOCK_LEN);
  bytes.copy(block);
  return Array.from({ length: 16 }, (_, index) => block.readUInt32LE(index * 4));
}

function rotr(value, shift) {
  return ((value >>> shift) | (value << (32 - shift))) >>> 0;
}

module.exports = {
  blake3Hex,
};
