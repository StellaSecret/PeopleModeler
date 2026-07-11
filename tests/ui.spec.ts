import { test, expect } from '@playwright/test';
import { gotoNewPerson, clearStorage } from './helpers';

test.describe('UI Features', () => {

  test.beforeEach(({ page }) => clearStorage(page));

  // ── Empty state ────────────────────────────────────────

  test('insights shows empty state when no persons exist', async ({ page }) => {
    await page.goto('/PeopleModeler/insights');
    await expect(page.locator('.empty-state')).toBeVisible();
    await expect(page.locator('.empty-state')).toContainText('No persons yet');
  });

  // ── Toast notifications ────────────────────────────────

  test('toast appears on save', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Toast Test');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.toast')).toBeVisible({ timeout: 3000 });
    await expect(page.locator('.toast')).toContainText('Saved');
  });

  test('toast appears on delete', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Delete Toast');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);

    await page.click('button:has-text("Delete")');
    await expect(page.locator('.toast')).toBeVisible({ timeout: 3000 });
    await expect(page.locator('.toast')).toContainText('Deleted');
  });

  // ── Reorder buttons ────────────────────────────────────

  test('reorder buttons on motivations when items exist', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Reorder Mot');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    const personId = page.url().split('/').pop()!;

    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(300);

    // No reorder buttons when list is empty
    await expect(page.locator('.reorder-btn')).toHaveCount(0);

    // Add two motivations via the fieldset's add button
    const motFieldset = page.locator('fieldset').filter({ hasText: 'Motivations' });
    const addBtns = motFieldset.locator('.add-row .btn');
    await addBtns.first().click();
    await addBtns.first().click();

    // 2 items × 2 buttons (▲▼) = 4
    await expect(page.locator('.reorder-btn')).toHaveCount(4);
  });

  test('reorder buttons on biases when items exist', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Reorder Bias');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    const personId = page.url().split('/').pop()!;

    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(300);

    await expect(page.locator('.reorder-btn')).toHaveCount(0);

    // Add two biases via the fieldset's add button
    const biasFieldset = page.locator('fieldset').filter({ hasText: 'Biases' });
    const addBtns = biasFieldset.locator('.add-row .btn');
    await addBtns.first().click();
    await addBtns.first().click();

    await expect(page.locator('.reorder-btn')).toHaveCount(4);
  });
});
