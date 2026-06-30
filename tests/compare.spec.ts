import { test, expect } from '@playwright/test';

// ── Compare Page ──────────────────────────────────────────
test.describe('Compare Page', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/compare.html');
  });

  test('affiche deux cartes de profil', async ({ page }) => {
    await expect(page.locator('.compare-card')).toHaveCount(2);
  });

  test('affiche le score de synergie', async ({ page }) => {
    await expect(page.locator('.compat-label')).toBeVisible();
    const text = await page.locator('.compat-label').textContent();
    expect(text).toContain('%');
  });

  test('affiche les 3 sections d\'analyse', async ({ page }) => {
    await expect(page.locator('.analysis-card.synergy')).toBeVisible();
    await expect(page.locator('.analysis-card.friction')).toBeVisible();
    await expect(page.locator('.analysis-card.strategy')).toBeVisible();
  });

  test('affiche la note éthique en bas', async ({ page }) => {
    await expect(page.locator('.ethics-banner')).toBeVisible();
    await expect(page.locator('.ethics-banner [data-i18n="compare_ethics"]')).toBeVisible();
  });

  test('les mini-bars OCEAN sont présentes pour les deux profils', async ({ page }) => {
    const bars = page.locator('.mini-bars');
    await expect(bars).toHaveCount(2);
  });

  test('lien retour vers accueil fonctionne', async ({ page }) => {
    await page.click('.nav-logo');
    await expect(page).toHaveURL(/index\.html|\/$/);
  });
});
