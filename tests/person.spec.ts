import { test, expect, type Page } from '@playwright/test';

async function gotoNewPerson(page: Page) {
  await page.goto('/PeopleModeler/person/new');
  await page.getByText('Blank (start from scratch)').click();
}

test.describe('Person Page — Dioxus', () => {
  let personId: string;

  test.beforeEach(async ({ page }) => {
    await page.goto('/PeopleModeler/');
    await page.evaluate(() => localStorage.clear());
    await page.reload();
    await page.waitForTimeout(500);

    // Create a person to work with
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
});
