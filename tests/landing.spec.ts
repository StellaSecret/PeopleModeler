import { test, expect } from '@playwright/test';

// ── Landing Page ──────────────────────────────────────────
test.describe('Landing Page', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('affiche le titre principal', async ({ page }) => {
    await expect(page.locator('.hero-title')).toBeVisible();
    await expect(page.locator('.hero-title [data-i18n="hero_title_1"]')).toBeVisible();
  });

  test('affiche les 6 feature cards', async ({ page }) => {
    const cards = page.locator('.feature-card');
    await expect(cards).toHaveCount(6);
  });

  test('les feature cards sont visibles après scroll', async ({ page }) => {
    await page.evaluate(() => window.scrollTo(0, 600));
    await expect(page.locator('.feature-card').first()).toBeVisible();
  });

  test('lien Démo navigue vers la page person', async ({ page }) => {
    await page.locator('a[href="person.html"]').first().click();
    // serve peut servir /person ou /person.html selon config
    await expect(page).toHaveURL(/person/);
  });

  test('lien Comparer navigue vers la page compare', async ({ page }) => {
    await page.locator('a[data-i18n="nav_compare"]').first().click();
    await expect(page).toHaveURL(/compare/);
  });

  test('les 4 étapes "Comment ça fonctionne" sont présentes', async ({ page }) => {
    const steps = page.locator('.step');
    await expect(steps).toHaveCount(4);
  });

  test('la nav est fixe en haut', async ({ page }) => {
    const nav = page.locator('nav.nav');
    await expect(nav).toHaveCSS('position', 'fixed');
  });

  test('note éthique visible', async ({ page }) => {
    await expect(page.locator('.ethics')).toBeVisible();
    await expect(page.locator('.ethics [data-i18n="ethics_title"]')).toBeVisible();
  });
});
