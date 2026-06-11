import { test, expect } from '@playwright/test';

// ── Accessibilité & Navigation globale ────────────────────
test.describe('Navigation & Accessibilité', () => {

  const pages = [
    { name: 'Landing', path: '/' },
    { name: 'Person',  path: '/person.html' },
    { name: 'Compare', path: '/compare.html' },
    { name: 'App',     path: '/app.html' },
  ];

  for (const { name, path } of pages) {

    test(`${name} — titre de page défini`, async ({ page }) => {
      await page.goto(path);
      const title = await page.title();
      expect(title.length).toBeGreaterThan(0);
      expect(title).toContain('People Modeler');
    });

    test(`${name} — pas d'erreur console critique`, async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => errors.push(err.message));
      await page.goto(path);
      await page.waitForTimeout(500);
      const critical = errors.filter(e =>
        !e.includes('favicon') && !e.includes('fonts.googleapis')
      );
      expect(critical).toHaveLength(0);
    });

    test(`${name} — responsive mobile 375px — pas de scroll horizontal`, async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 812 });
      await page.goto(path);
      await page.waitForLoadState('networkidle');

      const { scrollWidth, clientWidth } = await page.evaluate(() => ({
        scrollWidth: document.documentElement.scrollWidth,
        clientWidth: document.documentElement.clientWidth,
      }));

      expect(
        scrollWidth,
        `Overflow horizontal sur ${name} à 375px : scrollWidth=${scrollWidth} > clientWidth=${clientWidth}`
      ).toBeLessThanOrEqual(clientWidth + 20);
    });

    test(`${name} — responsive tablette 768px`, async ({ page }) => {
      await page.setViewportSize({ width: 768, height: 1024 });
      await page.goto(path);
      await page.waitForLoadState('networkidle');

      const { scrollWidth, clientWidth } = await page.evaluate(() => ({
        scrollWidth: document.documentElement.scrollWidth,
        clientWidth: document.documentElement.clientWidth,
      }));

      expect(scrollWidth).toBeLessThanOrEqual(clientWidth + 5);
    });

    test(`${name} — meta viewport présent`, async ({ page }) => {
      await page.goto(path);
      const viewport = await page.locator('meta[name="viewport"]').getAttribute('content');
      expect(viewport).toContain('width=device-width');
    });
  }

  test('tous les liens internes ne sont pas cassés', async ({ page }) => {
    const pagesToCrawl = ['/', '/person.html', '/compare.html', '/app.html'];
    const allLinks = new Set<string>();

    for (const path of pagesToCrawl) {
      await page.goto(path);
      const links = await page.locator('a[href]').all();
      for (const link of links) {
        const href = await link.getAttribute('href');
        if (href && !href.startsWith('http') && !href.startsWith('#') && href.endsWith('.html')) {
          allLinks.add(href);
        }
      }
    }

    for (const href of allLinks) {
      // href is already relative (e.g. "app.html"), resolve against baseURL
      const response = await page.request.get(href);
      expect(response.status(), `Lien cassé: ${href}`).toBe(200);
    }
  });

  test('navigation Landing → Person fonctionne', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Démo');
    await expect(page).toHaveURL(/person/);
  });

  test('navigation Landing → Compare fonctionne', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Comparer');
    await expect(page).toHaveURL(/compare/);
  });

  test('navigation Landing → App fonctionne', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Ouvrir l\'app');
    await expect(page).toHaveURL(/app/);
  });

  test('navigation retour Compare → Landing fonctionne', async ({ page }) => {
    await page.goto('/compare.html');
    await page.click('.nav-logo');
    await expect(page).toHaveURL(/index\.html|\/$/)
  });

  test('navigation retour App → Landing fonctionne', async ({ page }) => {
    await page.goto('/app.html');
    await page.click('.nav-logo');
    await expect(page).toHaveURL(/index\.html|\/$/)
  });
});
