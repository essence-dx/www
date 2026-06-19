const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');

const root = __dirname;
const port = 3000;

function send(res, status, body, type) {
  res.writeHead(status, {
    'content-type': type,
    'cache-control': 'no-store',
  });
  res.end(body);
}

function readIfFile(file) {
  try {
    const stat = fs.statSync(file);
    return stat.isFile() ? fs.readFileSync(file) : null;
  } catch {
    return null;
  }
}

function contentType(file) {
  if (file.endsWith('.css')) return 'text/css; charset=utf-8';
  if (file.endsWith('.js')) return 'text/javascript; charset=utf-8';
  if (file.endsWith('.svg')) return 'image/svg+xml';
  if (file.endsWith('.json')) return 'application/json; charset=utf-8';
  return 'text/html; charset=utf-8';
}

const routeFiles = new Map([
  ['/', 'pages/index.html'],
  ['/login', 'pages/login.html'],
  ['/logout', 'pages/logout.html'],
  ['/dashboard', 'pages/dashboard.html'],
  ['/automations', 'pages/automations.html'],
  ['/ui', 'pages/ui.html'],
  ['/database', 'pages/database.html'],
  ['/backend', 'pages/backend.html'],
]);

const server = http.createServer((req, res) => {
  const url = new URL(req.url, `http://${req.headers.host || 'localhost:3000'}`);
  if (url.pathname === '/api/auth/session') {
    send(res, 200, JSON.stringify({ ok: true, session: null, source: 'static-preview' }), 'application/json; charset=utf-8');
    return;
  }

  const routeFile = routeFiles.get(url.pathname);
  const relativePath = routeFile || decodeURIComponent(url.pathname.replace(/^\/+/, ''));
  const file = path.join(root, relativePath);
  if (!file.startsWith(root)) {
    send(res, 403, 'Forbidden', 'text/plain; charset=utf-8');
    return;
  }
  const body = readIfFile(file);
  if (body) {
    send(res, 200, body, contentType(file));
    return;
  }
  send(res, 404, 'Not found', 'text/plain; charset=utf-8');
});

server.listen(port, '127.0.0.1', () => {
  fs.writeFileSync(path.join(root, 'server.pid'), String(process.pid));
  console.log(`DX template preview running at http://localhost:${port}`);
});
