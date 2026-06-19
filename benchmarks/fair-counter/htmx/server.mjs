import { createReadStream, readFileSync } from "node:fs";
import { createServer } from "node:http";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const publicDir = join(here, "public");
const htmxPath = join(here, "node_modules", "htmx.org", "dist", "htmx.min.js");
let count = 0;

function send(res, status, body, type = "text/plain; charset=utf-8") {
  const buffer = Buffer.from(body);
  res.writeHead(status, {
    "content-type": type,
    "content-length": buffer.length,
  });
  res.end(buffer);
}

export function createHtmxServer() {
  return createServer((req, res) => {
    if (req.url === "/" || req.url === "/index.html") {
      const body = readFileSync(join(publicDir, "index.html"));
      res.writeHead(200, {
        "content-type": "text/html; charset=utf-8",
        "content-length": body.length,
      });
      res.end(body);
      return;
    }

    if (req.url === "/htmx.min.js") {
      res.writeHead(200, {
        "content-type": "application/javascript; charset=utf-8",
      });
      createReadStream(htmxPath).pipe(res);
      return;
    }

    if (req.method === "POST" && req.url === "/counter/increment") {
      count += 1;
      send(res, 200, String(count));
      return;
    }

    if (req.method === "POST" && req.url === "/counter/decrement") {
      count -= 1;
      send(res, 200, String(count));
      return;
    }

    if (req.method === "POST" && req.url === "/counter/reset") {
      count = 0;
      send(res, 200, "0");
      return;
    }

    send(res, 404, "Not found");
  });
}

if (import.meta.url === `file://${process.argv[1].replace(/\\/g, "/")}`) {
  const port = Number(process.env.PORT || 8104);
  createHtmxServer().listen(port, "127.0.0.1", () => {
    console.log(`HTMX fair counter listening on http://127.0.0.1:${port}`);
  });
}
