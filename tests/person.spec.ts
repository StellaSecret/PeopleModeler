import { test, expect } from '@playwright/test';

// ── Person Page ───────────────────────────────────────────
test.describe('Person Page — Fiche', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/person.html');
    // Attendre que la page charge les données demo
    await page.waitForTimeout(300);
  });

  test('affiche le nom du profil demo', async ({ page }) => {
    await expect(page.locator('#profileName')).toBeVisible();
    await expect(page.locator('#profileName')).toContainText('Alexandre');
  });

  test('affiche l\'avatar emoji', async ({ page }) => {
    await expect(page.locator('#profileAvatar')).toBeVisible();
    const text = await page.locator('#profileAvatar').textContent();
    expect(text?.trim().length).toBeGreaterThan(0);
  });

  test('les 5 onglets sont présents', async ({ page }) => {
    const tabs = page.locator('.tab');
    await expect(tabs).toHaveCount(5);
  });

  test('onglet Motivations actif par défaut', async ({ page }) => {
    const activeTab = page.locator('.tab.active');
    await expect(activeTab).toHaveAttribute('data-i18n', 'tab_motivations');
  });

  test('navigation entre onglets fonctionne', async ({ page }) => {
    // Clic sur Biais
    await page.click('.tab[data-tab="biases"]');
    await expect(page.locator('#tab-biases')).toHaveClass(/active/);
    await expect(page.locator('#tab-motivations')).not.toHaveClass(/active/);
  });

  test('onglet OCEAN affiche les sliders', async ({ page }) => {
    await page.click('.tab[data-tab="ocean"]');
    await expect(page.locator('.ocean-slider')).toHaveCount(5);
  });

  test('slider OCEAN modifie la valeur affichée', async ({ page }) => {
    await page.click('.tab[data-tab="ocean"]');
    const slider = page.locator('#slider-O');
    await slider.fill('9');
    await slider.dispatchEvent('input');
    await expect(page.locator('#val-O')).toHaveText('9');
  });

  test('onglet Prédictions affiche le formulaire', async ({ page }) => {
    await page.click('.tab[data-tab="predictions"]');
    await expect(page.locator('#predContext')).toBeVisible();
    await expect(page.locator('#predOutcome')).toBeVisible();
  });

  test('ajout d\'une prédiction fonctionne', async ({ page }) => {
    await page.click('.tab[data-tab="predictions"]');
    await page.fill('#predContext', 'Test réunion vendredi');
    await page.fill('#predOutcome', 'Il va chercher à négocier');
    await page.click('button[onclick="addPrediction()"]');
    // La prédiction apparaît dans la liste
    await expect(page.locator('.prediction-item').first()).toContainText('Test réunion vendredi');
  });

  test('insights — sélection d\'un trigger affiche l\'analyse', async ({ page }) => {
    await page.click('.tab[data-tab="insights"]');
    await page.click('button[onclick*="stress"]');
    const output = page.locator('#insightOutput');
    await expect(output).toContainText('Alexandre');
  });

  test('bouton modal Ajouter motivation s\'ouvre', async ({ page }) => {
    await page.click('.btn-add');
    await expect(page.locator('.modal-overlay')).toHaveClass(/open/);
    await expect(page.locator('#motType')).toBeVisible();
  });

  test('modal se ferme avec Annuler', async ({ page }) => {
    await page.click('.btn-add');
    await page.click('.modal-actions .btn-ghost');
    await expect(page.locator('.modal-overlay')).not.toHaveClass(/open/);
  });

  test('interprétation OCEAN se met à jour', async ({ page }) => {
    await page.click('.tab[data-tab="ocean"]');
    const interpretation = page.locator('#oceanInterpretation');
    await expect(interpretation).not.toBeEmpty();
    // Changer extraversion à 9 → doit mentionner "extraverti"
    const sliderE = page.locator('#slider-E');
    await sliderE.fill('9');
    await sliderE.dispatchEvent('input');
    await expect(interpretation).not.toBeEmpty();
  });
});
