import { test, expect, type Page } from '@playwright/test';

async function gotoNewPerson(page: Page) {
  await page.goto('/PeopleModeler/person/new');
  await page.getByText('Blank (start from scratch)').click();
}

test.describe('Compare Persons Page', () => {
  let id1: string;
  let id2: string;

  test.beforeEach(async ({ page }) => {
    await page.goto('/PeopleModeler/');
    await page.evaluate(() => localStorage.clear());
    await page.reload();
    await page.waitForTimeout(500);
  });

  async function createPerson(page: Page, name: string) {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill(name);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    return page.url().split('/').pop()!;
  }

  test('compare page shows both persons and analysis', async ({ page }) => {
    id1 = await createPerson(page, 'Alice');
    id2 = await createPerson(page, 'Bob');

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    const names = page.locator('.compare-card h3');
    await expect(names).toHaveCount(2);
    await expect(names.nth(0)).toContainText('Alice');
    await expect(names.nth(1)).toContainText('Bob');
  });

  test('compare page shows compatibility score', async ({ page }) => {
    id1 = await createPerson(page, 'Charlie');
    id2 = await createPerson(page, 'Diana');

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.compatibility-score')).toBeVisible();
    await expect(page.locator('.compat-label')).toContainText('%');
  });

  test('compare page shows analysis section with synergy, friction, strategy', async ({ page }) => {
    id1 = await createPerson(page, 'Eve');
    id2 = await createPerson(page, 'Frank');

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.analysis-section')).toBeVisible();
    await expect(page.locator('.analysis-card.synergy')).toBeVisible();
    await expect(page.locator('.analysis-card.friction')).toBeVisible();
    await expect(page.locator('.analysis-card.strategy')).toBeVisible();
  });

  test('compare page shows ethics banner', async ({ page }) => {
    id1 = await createPerson(page, 'Grace');
    id2 = await createPerson(page, 'Heidi');

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.ethics-banner')).toBeVisible();
  });
});
