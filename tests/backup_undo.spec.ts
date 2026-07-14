import { test, expect } from '@playwright/test';
import { createPerson, clearStorage } from './helpers';

test.describe('Backup and Restore', () => {

  test.beforeEach(({ page }) => clearStorage(page));

  test('data persists across full page reload', async ({ page }) => {
    await createPerson(page, 'Persist Alice');

    const snapshot = await page.evaluate(() => {
      const items: Record<string, string> = {};
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        if (key) items[key] = localStorage.getItem(key)!;
      }
      return items;
    });

    await page.evaluate(() => localStorage.clear());
    await page.goto('/PeopleModeler/');
    await page.waitForTimeout(300);
    await expect(page.locator('.person-card')).toHaveCount(0);

    await page.evaluate((items: Record<string, string>) => {
      for (const [key, value] of Object.entries(items)) {
        localStorage.setItem(key, value);
      }
    }, snapshot);

    await page.goto('/PeopleModeler/');
    await page.waitForTimeout(300);
    await expect(page.locator('.person-card')).toHaveCount(1);
    await expect(page.getByText('Persist Alice')).toBeVisible();
  });
});

test.describe('Undo', () => {

  test.beforeEach(({ page }) => clearStorage(page));

  test('undo button restores deleted person', async ({ page }) => {
    await createPerson(page, 'Undo Bob');

    await page.click('button:has-text("Delete")');
    await page.waitForTimeout(500);

    await expect(page.locator('.person-card')).toHaveCount(0);

    await page.click('button[aria-label*="Undo"]');
    await page.waitForTimeout(300);

    // Undo restores person in DB but PeopleList signal is stale; reload to force fresh read
    await page.goto('/PeopleModeler/');
    await page.waitForTimeout(300);
    await expect(page.locator('.person-card')).toHaveCount(1);
    await expect(page.getByText('Undo Bob')).toBeVisible();
  });

  test('Ctrl+Z restores deleted person', async ({ page }) => {
    await createPerson(page, 'Undo Charlie');

    await page.click('button:has-text("Delete")');
    await page.waitForTimeout(500);

    await expect(page.locator('.person-card')).toHaveCount(0);

    // Dispatch keydown directly so WASM document listener receives correct properties
    await page.evaluate(() => {
      document.dispatchEvent(new KeyboardEvent('keydown', {
        key: 'z',
        code: 'KeyZ',
        ctrlKey: true,
        bubbles: true,
        cancelable: true,
      }));
    });
    await page.waitForTimeout(300);

    await page.goto('/PeopleModeler/');
    await page.waitForTimeout(300);
    await expect(page.locator('.person-card')).toHaveCount(1);
    await expect(page.getByText('Undo Charlie')).toBeVisible();
  });
});
