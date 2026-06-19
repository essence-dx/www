/* Dependency-free animated GIF encoder.
   - global 256-color palette via median cut over sampled frame pixels
   - optional 4x4 Bayer ordered dithering (kills gradient banding)
   - standard GIF LZW (jsgif-compatible code-size handling)
   - NETSCAPE2.0 extension for infinite looping */

var GIFEnc = (function () {

  /* ---------- byte writer ---------- */
  function ByteWriter() {
    this.chunks = [];
    this.cur = new Uint8Array(1 << 16);
    this.len = 0;
  }
  ByteWriter.prototype.byte = function (b) {
    if (this.len === this.cur.length) { this.chunks.push(this.cur); this.cur = new Uint8Array(1 << 16); this.len = 0; }
    this.cur[this.len++] = b & 0xff;
  };
  ByteWriter.prototype.bytes = function (arr) {
    for (var i = 0; i < arr.length; i++) this.byte(arr[i]);
  };
  ByteWriter.prototype.short = function (v) { this.byte(v & 0xff); this.byte((v >> 8) & 0xff); };
  ByteWriter.prototype.string = function (s) { for (var i = 0; i < s.length; i++) this.byte(s.charCodeAt(i)); };
  ByteWriter.prototype.result = function () {
    var total = this.chunks.length * (1 << 16) + this.len;
    var out = new Uint8Array(total);
    var o = 0;
    for (var i = 0; i < this.chunks.length; i++) { out.set(this.chunks[i], o); o += this.chunks[i].length; }
    out.set(this.cur.subarray(0, this.len), o);
    return out;
  };

  /* ---------- median cut quantization ---------- */
  function sampleChannel(sample, channel) {
    if (channel === 0) return (sample >> 16) & 255;
    if (channel === 1) return (sample >> 8) & 255;
    return sample & 255;
  }

  async function buildPalette(samples, sampleCount, maxColors, isCancelled) {
    var boxes = [samples.subarray(0, sampleCount)];
    var splitCount = 0;
    while (boxes.length < maxColors) {
      if (isCancelled && isCancelled()) return null;
      var bi = -1, bRange = -1, bCh = 0;
      for (var i = 0; i < boxes.length; i++) {
        var box = boxes[i];
        if (box.length < 2) continue;
        var mins = [255, 255, 255], maxs = [0, 0, 0];
        for (var j = 0; j < box.length; j++) {
          var sample = box[j];
          for (var c = 0; c < 3; c++) {
            var value = sampleChannel(sample, c);
            if (value < mins[c]) mins[c] = value;
            if (value > maxs[c]) maxs[c] = value;
          }
        }
        for (var c2 = 0; c2 < 3; c2++) {
          var r = maxs[c2] - mins[c2];
          if (r > bRange) { bRange = r; bi = i; bCh = c2; }
        }
      }
      if (bi < 0 || bRange === 0) break;
      var target = boxes[bi];
      target.sort(function (a, b) { return sampleChannel(a, bCh) - sampleChannel(b, bCh); });
      var mid = target.length >> 1;
      boxes.splice(bi, 1, target.slice(0, mid), target.slice(mid));
      splitCount++;
      if ((splitCount & 7) === 0) {
        if (isCancelled && isCancelled()) return null;
        await microtask();
      }
    }
    var palette = [];
    for (var k = 0; k < boxes.length; k++) {
      if (isCancelled && isCancelled()) return null;
      var bx = boxes[k];
      if (!bx.length) continue;
      var rs = 0, gs = 0, bs = 0;
      for (var m = 0; m < bx.length; m++) {
        var packed = bx[m];
        rs += sampleChannel(packed, 0);
        gs += sampleChannel(packed, 1);
        bs += sampleChannel(packed, 2);
      }
      palette.push([Math.round(rs / bx.length), Math.round(gs / bx.length), Math.round(bs / bx.length)]);
      if ((k & 15) === 15) await microtask();
    }
    while (palette.length < maxColors) palette.push([0, 0, 0]);
    return palette;
  }

  function makeNearest(palette) {
    var cache = new Int16Array(32768).fill(-1);
    return function (r, g, b) {
      var key = ((r >> 3) << 10) | ((g >> 3) << 5) | (b >> 3);
      var hit = cache[key];
      if (hit >= 0) return hit;
      var best = 0, bd = 1e9;
      for (var i = 0; i < palette.length; i++) {
        var dr = palette[i][0] - r, dg = palette[i][1] - g, db = palette[i][2] - b;
        var d = dr * dr + dg * dg + db * db;
        if (d < bd) { bd = d; best = i; }
      }
      cache[key] = best;
      return best;
    };
  }

  var BAYER = [
    0, 8, 2, 10,
    12, 4, 14, 6,
    3, 11, 1, 9,
    15, 7, 13, 5
  ];

  function clearFrameBuffers(frames, start) {
    for (var i = start || 0; i < frames.length; i++) frames[i] = null;
  }

  /* ---------- LZW (GIF variant) ---------- */
  async function lzwEncode(minCodeSize, indices, out, isCancelled) {
    var clearCode = 1 << minCodeSize;
    var eoiCode = clearCode + 1;
    var nextCode = eoiCode + 1;
    var curBits = minCodeSize + 1;
    var maxCode = (1 << curBits) - 1;
    var dict = new Map();

    var bitAcc = 0, bitCnt = 0;
    var sub = new Uint8Array(255), subLen = 0;

    function flushSub() {
      if (subLen > 0) { out.byte(subLen); for (var i = 0; i < subLen; i++) out.byte(sub[i]); subLen = 0; }
    }
    function pushByte(b) {
      sub[subLen++] = b;
      if (subLen === 255) flushSub();
    }
    function emit(code) {
      bitAcc |= code << bitCnt;
      bitCnt += curBits;
      while (bitCnt >= 8) { pushByte(bitAcc & 0xff); bitAcc >>= 8; bitCnt -= 8; }
      if (nextCode > maxCode && curBits < 12) {
        curBits++;
        maxCode = (1 << curBits) - 1;
      }
    }

    out.byte(minCodeSize);
    emit(clearCode);

    var prev = indices[0];
    for (var i = 1; i < indices.length; i++) {
      if ((i & 8191) === 0) {
        if (isCancelled && isCancelled()) return false;
        await microtask();
      }
      var k = indices[i];
      var key = (prev << 8) | k;
      var hit = dict.get(key);
      if (hit !== undefined) { prev = hit; continue; }
      emit(prev);
      if (nextCode < 4096) {
        dict.set(key, nextCode++);
      } else {
        emit(clearCode);
        dict.clear();
        nextCode = eoiCode + 1;
        curBits = minCodeSize + 1;
        maxCode = (1 << curBits) - 1;
      }
      prev = k;
    }
    emit(prev);
    emit(eoiCode);
    if (bitCnt > 0) pushByte(bitAcc & 0xff);
    flushSub();
    out.byte(0); /* block terminator */
    return !(isCancelled && isCancelled());
  }

  /* ---------- main encode ---------- */
  /* frames: array of Uint8Array RGBA (top-down), all width*height*4 */
  async function encode(opts) {
    var frames = opts.frames, w = opts.width, h = opts.height;
    var fps = opts.fps || 25;
    var dither = opts.dither !== false;
    var onProgress = opts.onProgress || function () {};
    var isCancelled = opts.isCancelled || function () { return false; };
    var completed = false;

    try {
      /* sample pixels across all frames for the global palette */
      var targetSamples = 42000;
      var totalPx = frames.length * w * h;
      var samples = new Uint32Array(Math.min(targetSamples, Math.max(1, totalPx)));
      var stride = Math.max(1, Math.floor(totalPx / targetSamples));
      var sampleCount = 0;
      for (var f = 0; f < frames.length; f++) {
        if (isCancelled()) return null;
        if (sampleCount >= samples.length) break;
        var d = frames[f];
        for (var p = (f * 7919) % stride; p < w * h; p += stride) {
          if (sampleCount >= samples.length) break;
          samples[sampleCount++] = (d[p * 4] << 16) | (d[p * 4 + 1] << 8) | d[p * 4 + 2];
          if ((sampleCount & 4095) === 0) {
            if (isCancelled()) return null;
            await microtask();
          }
        }
      }
      if (isCancelled()) return null;
      onProgress(0.02, "building palette");
      await microtask();
      if (isCancelled()) return null;
      var palette = await buildPalette(samples, sampleCount, 256, isCancelled);
      samples = null;
      if (!palette) return null;
      var nearest = makeNearest(palette);

      var out = new ByteWriter();
      out.string("GIF89a");
      out.short(w); out.short(h);
      out.byte(0xF7); /* global table, 256 colors, 8-bit res */
      out.byte(0);    /* bg index */
      out.byte(0);    /* aspect */
      for (var c = 0; c < 256; c++) {
        var col = palette[c] || [0, 0, 0];
        out.byte(col[0]); out.byte(col[1]); out.byte(col[2]);
      }

      /* NETSCAPE loop extension: 0 = forever. Omitted entirely -> play once. */
      if (opts.loop !== false) {
        out.byte(0x21); out.byte(0xFF); out.byte(11);
        out.string("NETSCAPE2.0");
        out.byte(3); out.byte(1); out.short(0); out.byte(0);
      }

      var delay = Math.max(2, Math.round(100 / fps));
      var indices = new Uint8Array(w * h);
      var rowsPerYield = Math.max(1, Math.min(32, Math.floor(16384 / w)));

      for (var fi = 0; fi < frames.length; fi++) {
        if (isCancelled()) return null;
        var data = frames[fi];
        try {
          var di = 0;
          for (var y = 0; y < h; y++) {
            for (var x = 0; x < w; x++) {
              var o = (y * w + x) * 4;
              var r = data[o], g = data[o + 1], b = data[o + 2];
              if (dither) {
                var dth = (BAYER[(y & 3) * 4 + (x & 3)] / 16 - 0.5) * 14;
                r = Math.max(0, Math.min(255, r + dth));
                g = Math.max(0, Math.min(255, g + dth));
                b = Math.max(0, Math.min(255, b + dth));
              }
              indices[di++] = nearest(r | 0, g | 0, b | 0);
            }
            if (((y + 1) % rowsPerYield) === 0) {
              if (isCancelled()) return null;
              await microtask();
            }
          }
          if (isCancelled()) return null;

          /* graphic control extension */
          out.byte(0x21); out.byte(0xF9); out.byte(4);
          out.byte(0x04); /* disposal: do not dispose */
          out.short(delay);
          out.byte(0); out.byte(0);

          /* image descriptor */
          out.byte(0x2C);
          out.short(0); out.short(0); out.short(w); out.short(h);
          out.byte(0); /* no local table */

          if (!await lzwEncode(8, indices, out, isCancelled)) return null;

          onProgress(0.05 + 0.95 * (fi + 1) / frames.length, "frame " + (fi + 1) + "/" + frames.length);
          await microtask();
        } finally {
          frames[fi] = null;
          data = null;
        }
      }

      out.byte(0x3B); /* trailer */
      completed = true;
      return out.result();
    } finally {
      if (!completed) clearFrameBuffers(frames, 0);
    }
  }

  function microtask() {
    return new Promise(function (res) { setTimeout(res, 0); });
  }

  return { encode: encode };
})();
