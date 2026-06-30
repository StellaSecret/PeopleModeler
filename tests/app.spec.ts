import { test, expect } from '@playwright/test';

// ── Web App (/app.html) ───────────────────────────────────
test.describe('Web App', () => {

  test.beforeEach(async ({ page }) => {
    // Clear localStorage before each test for a clean state
    await page.goto('/app.html');
    await page.evaluate(() => localStorage.clear());
    await page.reload();
    await page.waitForTimeout(300);
  });

  // ── Empty state ────────────────────────────────────────

  test('affiche l\'état vide au premier lancement', async ({ page }) => {
    await expect(page.locator('#emptyState')).toBeVisible();
    await expect(page.locator('#profileView')).not.toBeVisible();
  });

  test('affiche le bouton Créer un profil', async ({ page }) => {
    await expect(page.locator('.app-empty .btn-primary')).toBeVisible();
  });

  test('affiche le bouton Sync Google Drive', async ({ page }) => {
    await expect(page.locator('#gdriveSyncBtn')).toBeVisible();
  });

  // ── Création de profil ─────────────────────────────────

  test('ouvre la modal de création de profil', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await expect(page.locator('.modal-overlay')).toHaveClass(/open/);
    await expect(page.locator('#fName')).toBeVisible();
  });

  test('crée un profil et affiche la vue profil', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Marie Curie');
    await page.fill('#fRole', 'Chercheuse');
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('#profileView')).toBeVisible();
    await expect(page.locator('#emptyState')).not.toBeVisible();
    await expect(page.locator('#pName')).toContainText('Marie Curie');
  });

  test('le profil créé apparaît dans la sidebar', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Ada Lovelace');
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('#profileList .pe-name')).toContainText('Ada Lovelace');
  });

  test('la modal se ferme avec Annuler', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.click('.modal-actions .btn-ghost');
    await expect(page.locator('.modal-overlay')).not.toHaveClass(/open/);
  });

  test('refus de créer un profil sans nom', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    await page.click('.app-empty .btn-primary');
    await page.click('.modal-actions .btn-primary');
    // Modal stays open — profile was not created
    await expect(page.locator('.modal-overlay')).toHaveClass(/open/);
  });

  // ── Persistence localStorage ───────────────────────────

  test('le profil persiste après rechargement', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Alan Turing');
    await page.click('.modal-actions .btn-primary');
    await page.reload();
    await page.waitForTimeout(300);
    await expect(page.locator('#pName')).toContainText('Alan Turing');
  });

  // ── Onglets ────────────────────────────────────────────

  test('les 5 onglets sont présents après création d\'un profil', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('[data-panel]')).toHaveCount(5);
  });

  test('navigation entre onglets fonctionne', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="biases"]');
    await expect(page.locator('#panel-biases')).toHaveClass(/active/);
    await expect(page.locator('#panel-motivations')).not.toHaveClass(/active/);
  });

  // ── Motivations ────────────────────────────────────────

  test('peut ajouter une motivation', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('.btn-add');
    await expect(page.locator('.modal-overlay')).toHaveClass(/open/);
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('#motivationList .mot-item')).toHaveCount(1);
  });

  test('peut supprimer une motivation', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('.btn-add');
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('#motivationList .mot-item')).toHaveCount(1);
    await page.click('#motivationList .btn-delete');
    await expect(page.locator('#motivationList .mot-item')).toHaveCount(0);
  });

  test('peut supprimer un biais', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="biases"]');
    await page.click('#panel-biases .btn-add');
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('#biasList .bias-item-row')).toHaveCount(1);
    await page.click('#biasList .btn-delete');
    await expect(page.locator('#biasList .bias-item-row')).toHaveCount(0);
  });

  // ── Biais ──────────────────────────────────────────────

  test('peut ajouter un biais', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="biases"]');
    await page.click('#panel-biases .btn-add');
    await expect(page.locator('.modal-overlay')).toHaveClass(/open/);
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('#biasList .bias-item-row')).toHaveCount(1);
  });

  // ── OCEAN ──────────────────────────────────────────────

  test('les 5 sliders OCEAN sont présents', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="ocean"]');
    await expect(page.locator('.ocean-slider')).toHaveCount(5);
  });

  test('slider OCEAN met à jour la valeur affichée', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="ocean"]');
    const slider = page.locator('#sO');
    await slider.fill('9');
    await slider.dispatchEvent('input');
    await expect(page.locator('#vO')).toHaveText('9');
  });

  test('l\'interprétation OCEAN se met à jour', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="ocean"]');
    const slider = page.locator('#sE');
    await slider.fill('9');
    await slider.dispatchEvent('input');
    await expect(page.locator('#oceanInterp')).not.toBeEmpty();
  });

  // ── Prédictions ────────────────────────────────────────

  test('peut ajouter une prédiction', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="predictions"]');
    await page.fill('#predCtx', 'Réunion lundi');
    await page.fill('#predOut', 'Il va négocier');
    await page.click('.btn-primary.btn-submit');
    await expect(page.locator('#predictionList .pred-item').first()).toContainText('Réunion lundi');
  });

  // ── Insights ───────────────────────────────────────────

  test('insights — sélectionner un contexte affiche une analyse', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Test');
    await page.click('.modal-actions .btn-primary');
    await page.click('[data-panel="insights"]');
    await page.click('button[onclick*="stress"]');
    await expect(page.locator('#insightOutput')).toContainText('Test');
  });

  // ── Édition & Suppression ──────────────────────────────

  test('peut éditer un profil', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Ancien Nom');
    await page.click('.modal-actions .btn-primary');
    await page.click('.btn-edit');
    await page.fill('#fName', 'Nouveau Nom');
    await page.click('.modal-actions .btn-primary');
    await expect(page.locator('#pName')).toContainText('Nouveau Nom');
  });

  test('peut supprimer un profil', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'À supprimer');
    await page.click('.modal-actions .btn-primary');
    await page.click('[aria-label="Supprimer le profil"]');
    await expect(page.locator('#emptyState')).toBeVisible();
  });

  // ── Multi-profils ──────────────────────────────────────

  test('peut créer plusieurs profils et basculer entre eux', async ({ page }) => {
    // Créer profil 1
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Profil Alpha');
    await page.click('.modal-actions .btn-primary');
    // Créer profil 2
    await page.click('.btn-new');
    await page.fill('#fName', 'Profil Beta');
    await page.click('.modal-actions .btn-primary');
    // Basculer sur profil 1
    await page.click('.profile-entry:has-text("Profil Alpha")');
    await expect(page.locator('#pName')).toContainText('Profil Alpha');
    // Basculer sur profil 2
    await page.click('.profile-entry:has-text("Profil Beta")');
    await expect(page.locator('#pName')).toContainText('Profil Beta');
  });

  // ── Export ─────────────────────────────────────────────

  test('le bouton Export déclenche un téléchargement', async ({ page }) => {
    await page.click('.app-empty .btn-primary');
    await page.fill('#fName', 'Export Test');
    await page.click('.modal-actions .btn-primary');
    const [download] = await Promise.all([
      page.waitForEvent('download'),
      page.click('button[onclick="exportData()"]'),
    ]);
    expect(download.suggestedFilename()).toMatch(/PeopleModeler.*\.json/);
  });

  // ── Navigation ─────────────────────────────────────────

  test('le logo nav renvoie vers l\'accueil', async ({ page }) => {
    await expect(page.locator('.nav-logo')).toHaveAttribute('href', 'index.html');
  });

  test('le lien Démo pointe vers person.html', async ({ page }) => {
    await expect(page.locator('a[href="person.html"]')).toBeVisible();
  });
});
