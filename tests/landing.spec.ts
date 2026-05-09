import { test, expect } from '@playwright/test';

// ── Landing Page ──────────────────────────────────────────
test.describe('Landing Page', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('affiche le titre principal', async ({ page }) => {
    await expect(page.locator('.hero-title')).toBeVisible();
    await expect(page.locator('.hero-title')).toContainText('systèmes');
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
    await page.click('text=Démo');
    // serve peut servir /person ou /person.html selon config
    await expect(page).toHaveURL(/person/);
  });

  test('lien Comparer navigue vers la page compare', async ({ page }) => {
    await page.click('text=Comparer');
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
    await expect(page.locator('.ethics')).toContainText('éthique');
  });
});
