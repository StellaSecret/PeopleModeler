import { test, expect } from '@playwright/test';
import { gotoNewPerson, clearStorage, addPattern } from './helpers';

test.describe('Person Page — Dioxus', () => {
  let personId: string;

  test.beforeEach(async ({ page }) => {
    await clearStorage(page);

    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Alexandre');
    await page.locator('label:has-text("Role") + input').fill('Manager');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    personId = page.url().split('/').pop()!;
  });

  test('person header shows name, emoji, role', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Alexandre');
    await expect(page.locator('.avatar-lg')).toBeVisible();
    await expect(page.locator('.content')).toContainText('Manager');
  });

  test('ocean chart is visible', async ({ page }) => {
    await expect(page.locator('.content')).toContainText('OCEAN Scores');
  });

  test('edit link exists', async ({ page }) => {
    await expect(page.locator('a:has-text("Edit")')).toHaveAttribute('href', `/PeopleModeler/person/${personId}/edit`);
  });

  test('delete button exists', async ({ page }) => {
    await expect(page.locator('button:has-text("Delete")')).toBeVisible();
  });

  test('add pattern then verify on detail page', async ({ page }) => {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(200);
    await addPattern(page, 'Stress', 'becomes_quiet', 7);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.pattern-item')).toContainText('⚡7/10');
    await expect(page.locator('.pattern-item')).toContainText(/stress/i);
  });
});
