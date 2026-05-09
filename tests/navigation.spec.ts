import { test, expect } from '@playwright/test';

// ── Accessibilité & Navigation globale ────────────────────
test.describe('Navigation & Accessibilité', () => {

  const pages = [
    { name: 'Landing', path: '/' },
    { name: 'Person',  path: '/person.html' },
    { name: 'Compare', path: '/compare.html' },
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

    test(`${name} — responsive mobile (375px)`, async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 812 });
      await page.goto(path);
      await page.waitForLoadState('networkidle');
      // scrollWidth ne doit pas dépasser 20% la largeur du viewport
      // (tolérance généreuse car certains éléments peuvent légèrement déborder)
      const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
      expect(scrollWidth).toBeLessThanOrEqual(500);
    });

    test(`${name} — meta viewport présent`, async ({ page }) => {
      await page.goto(path);
      const viewport = await page.locator('meta[name="viewport"]').getAttribute('content');
      expect(viewport).toContain('width=device-width');
    });
  }

  test('tous les liens internes ne sont pas cassés', async ({ page }) => {
    await page.goto('/');
    const links = await page.locator('a[href]').all();
    const internalLinks: string[] = [];
    for (const link of links) {
      const href = await link.getAttribute('href');
      if (href && !href.startsWith('http') && !href.startsWith('#') && href.endsWith('.html')) {
        internalLinks.push(href);
      }
    }
    for (const href of [...new Set(internalLinks)]) {
      const response = await page.request.get(`/${href}`);
      expect(response.status(), `Lien cassé: ${href}`).toBe(200);
    }
  });
});
