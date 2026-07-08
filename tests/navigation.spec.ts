import { test, expect, type Page } from '@playwright/test';

async function gotoNewPerson(page: Page) {
  await page.goto('/PeopleModeler/person/new');
  await page.getByText('Blank (start from scratch)').click();
}

test.describe('SPA Navigation', () => {
  test('landing page loads with nav bar', async ({ page }) => {
    await page.goto('/PeopleModeler/');
    await expect(page.locator('.top-bar')).toBeVisible();
    await expect(page.locator('.nav-links')).toBeVisible();
  });

  test('nav links exist and link to correct routes', async ({ page }) => {
    await page.goto('/PeopleModeler/');
    const links = page.locator('.nav-links a');
    await expect(links.nth(0)).toHaveAttribute('href', /PeopleModeler\/?$/);
    await expect(links.nth(1)).toHaveAttribute('href', /relationships/);
    await expect(links.nth(2)).toHaveAttribute('href', /timeline/);
    await expect(links.nth(3)).toHaveAttribute('href', /sync/);
  });

  test('click Sync nav goes to /sync', async ({ page }) => {
    await page.goto('/PeopleModeler/');
    await page.locator('.nav-links a').nth(3).click();
    await expect(page).toHaveURL(/\/sync/);
  });

  test('click logo goes to /', async ({ page }) => {
    await page.goto('/PeopleModeler/sync');
    await page.locator('.logo').click();
    await expect(page).toHaveURL(/\/PeopleModeler\/?$/);
  });

  test('/person/new loads template picker then form after selection', async ({ page }) => {
    await page.goto('/PeopleModeler/person/new');
    await expect(page.locator('h2')).toContainText('New Person');
    // Template picker shown first; click blank to reveal form
    await page.getByText('Blank (start from scratch)').click();
    await expect(page.locator('label:has-text("Name")')).toBeVisible();
  });

  test('/predictions loads', async ({ page }) => {
    await page.goto('/PeopleModeler/predictions');
    await expect(page.locator('h2')).toContainText('All Predictions');
  });

  test('/insights loads', async ({ page }) => {
    await page.goto('/PeopleModeler/insights');
    await expect(page.locator('h2')).toContainText('Insights');
  });

  test('/sync loads', async ({ page }) => {
    await page.goto('/PeopleModeler/sync');
    await expect(page.locator('h2')).toContainText('Sync & Backup');
  });

  test('404 edges return app shell', async ({ page }) => {
    await page.goto('/PeopleModeler/nonexistent');
    const title = await page.title();
    expect(title.length).toBeGreaterThan(0);
  });
});
