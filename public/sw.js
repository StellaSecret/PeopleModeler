// People Modeler — service worker (offline-first shell)
//
// Deployed as /PeopleModeler/sw.js (repo-root public/ is uploaded verbatim by
// the deploy workflow). Since every build ships hashed assets
// (assets/peoplemodeler-app-<hash>.js / *_bg-<hash>.wasm), the precache list is
// derived at install time from spa.html instead of being hard-coded.

const CACHE = 'pm-shell-v1';

const SHELL = ['./', 'spa.html', '404.html', 'landing.html', 'icon.svg', 'manifest.json'];

async function addIfOk(cache, url) {
  try {
    const res = await fetch(url, { cache: 'reload' });
    if (res.ok) {
      await cache.put(url, res);
    }
  } catch {
    // ignore — offline install, or a missing optional asset
  }
}

async function precacheAssets(cache, base) {
  // Derive the hashed wasm/js bundle names from the app shell markup.
  try {
    const res = await fetch(base + 'spa.html', { cache: 'reload' });
    if (!res.ok) return;
    const html = await res.text();
    const names = [...html.matchAll(/["'](\/PeopleModeler\/assets\/[^"']+\.(?:js|wasm))["']/g)]
      .map((m) => m[1]);
    for (const n of names) {
      await addIfOk(cache, n);
    }
  } catch {
    // ignore — registry will retry on next online load
  }
}

self.addEventListener('install', (event) => {
  event.waitUntil(
    (async () => {
      const cache = await caches.open(CACHE);
      const base = self.registration.scope;
      for (const url of SHELL) {
        await addIfOk(cache, new URL(url, base).href);
      }
      await precacheAssets(cache, base);
    })().then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    (async () => {
      const keys = await caches.keys();
      await Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k)));
      await self.clients.claim();
    })()
  );
});

self.addEventListener('fetch', (event) => {
  const req = event.request;
  const scopeOrigin = new URL(self.registration.scope).origin;
  if (req.method !== 'GET' || new URL(req.url).origin !== scopeOrigin) {
    return; // never touch cross-origin (Google APIs, fonts) or non-GET
  }

  const url = new URL(req.url);

  // Navigation → network-first, fall back to this URL's cached copy, then to
  // the cached SPA shell, so the app still boots offline.
  if (req.mode === 'navigate' || req.headers.get('accept')?.includes('text/html')) {
    const spaUrl = new URL('spa.html', self.registration.scope).href;
    event.respondWith(
      (async () => {
        try {
          const fresh = await fetch(req);
          const cache = await caches.open(CACHE);
          await cache.put(req, fresh.clone());
          return fresh;
        } catch {
          return (
            (await caches.match(req)) ||
            (await caches.match(spaUrl)) ||
            new Response('Offline', { status: 503, headers: { 'Content-Type': 'text/plain' } })
          );
        }
      })()
    );
    return;
  }

  // Hashed build assets → cache-first (immutable URLs).
  if (url.pathname.includes('/assets/')) {
    event.respondWith(
      (async () => {
        const hit = await caches.match(req);
        if (hit) return hit;
        const fresh = await fetch(req);
        const cache = await caches.open(CACHE);
        await cache.put(req, fresh.clone());
        return fresh;
      })()
    );
    return;
  }

  // Everything else same-origin → network-first, cache fallback.
  event.respondWith(
    (async () => {
      try {
        const fresh = await fetch(req);
        const cache = await caches.open(CACHE);
        await cache.put(req, fresh.clone());
        return fresh;
      } catch {
        return (await caches.match(req)) || Response.error();
      }
    })()
  );
});