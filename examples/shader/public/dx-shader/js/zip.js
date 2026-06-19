/* Minimal store-only (no compression) ZIP writer for batch PNG export. */

var ZipWriter = (function () {

  var CRC_TABLE = (function () {
    var t = new Uint32Array(256);
    for (var n = 0; n < 256; n++) {
      var c = n;
      for (var k = 0; k < 8; k++) c = (c & 1) ? (0xEDB88320 ^ (c >>> 1)) : (c >>> 1);
      t[n] = c >>> 0;
    }
    return t;
  })();
  var TEXT_ENCODER = new TextEncoder();

  function dosDateTime() {
    var d = new Date();
    var time = (d.getHours() << 11) | (d.getMinutes() << 5) | (d.getSeconds() >> 1);
    var date = ((d.getFullYear() - 1980) << 9) | ((d.getMonth() + 1) << 5) | d.getDate();
    return { time: time, date: date };
  }

  function createZipState() {
    return { parts: [], central: [], offset: 0, dt: dosDateTime() };
  }

  function clearZipState(state) {
    state.parts.length = 0;
    state.central.length = 0;
    state.offset = 0;
  }

  async function crc32Async(buf, isCancelled) {
    var c = 0xFFFFFFFF;
    for (var i = 0; i < buf.length; i++) {
      if ((i & 262143) === 0) {
        if (isCancelled && isCancelled()) return null;
        if (i > 0) await waitForBrowser();
      }
      c = CRC_TABLE[(c ^ buf[i]) & 0xFF] ^ (c >>> 8);
    }
    if (isCancelled && isCancelled()) return null;
    return (c ^ 0xFFFFFFFF) >>> 0;
  }

  function appendEntry(state, e, knownCrc) {
    var nameBytes = TEXT_ENCODER.encode(e.name);
    var crc = knownCrc;
    var local = new Uint8Array(30 + nameBytes.length);
    var v = new DataView(local.buffer);
    v.setUint32(0, 0x04034b50, true);
    v.setUint16(4, 20, true);
    v.setUint16(6, 0x0800, true);
    v.setUint16(8, 0, true);
    v.setUint16(10, state.dt.time, true);
    v.setUint16(12, state.dt.date, true);
    v.setUint32(14, crc, true);
    v.setUint32(18, e.data.length, true);
    v.setUint32(22, e.data.length, true);
    v.setUint16(26, nameBytes.length, true);
    v.setUint16(28, 0, true);
    local.set(nameBytes, 30);
    state.parts.push(local, e.data);

    var cd = new Uint8Array(46 + nameBytes.length);
    var c = new DataView(cd.buffer);
    c.setUint32(0, 0x02014b50, true);
    c.setUint16(4, 20, true);
    c.setUint16(6, 20, true);
    c.setUint16(8, 0x0800, true);
    c.setUint16(10, 0, true);
    c.setUint16(12, state.dt.time, true);
    c.setUint16(14, state.dt.date, true);
    c.setUint32(16, crc, true);
    c.setUint32(20, e.data.length, true);
    c.setUint32(24, e.data.length, true);
    c.setUint16(28, nameBytes.length, true);
    c.setUint32(42, state.offset, true);
    cd.set(nameBytes, 46);
    state.central.push(cd);

    state.offset += local.length + e.data.length;
  }

  function finalizeZip(state, entryCount) {
    var cdSize = 0;
    state.central.forEach(function (c) { cdSize += c.length; });

    var end = new Uint8Array(22);
    var ev = new DataView(end.buffer);
    ev.setUint32(0, 0x06054b50, true);
    ev.setUint16(8, entryCount, true);
    ev.setUint16(10, entryCount, true);
    ev.setUint32(12, cdSize, true);
    ev.setUint32(16, state.offset, true);

    return new Blob(state.parts.concat(state.central, [end]), { type: "application/zip" });
  }

  function waitForBrowser() {
    if (window.scheduler && typeof window.scheduler.yield === "function") {
      return window.scheduler.yield();
    }
    if (typeof window.requestIdleCallback === "function") {
      return new Promise(function (resolve) {
        window.requestIdleCallback(function () { resolve(); }, { timeout: 80 });
      });
    }
    return new Promise(function (resolve) { setTimeout(resolve, 0); });
  }

  async function buildAsync(entries, onProgress, isCancelled) {
    var state = createZipState();
    for (var i = 0; i < entries.length; i++) {
      if (isCancelled && isCancelled()) {
        clearZipState(state);
        return null;
      }
      var crc = await crc32Async(entries[i].data, isCancelled);
      if (crc === null) {
        clearZipState(state);
        return null;
      }
      appendEntry(state, entries[i], crc);
      entries[i].data = null;
      if (onProgress) onProgress((i + 1) / entries.length);
      await waitForBrowser();
    }
    if (isCancelled && isCancelled()) {
      clearZipState(state);
      return null;
    }
    try {
      return finalizeZip(state, entries.length);
    } finally {
      clearZipState(state);
    }
  }

  return { buildAsync: buildAsync };
})();
