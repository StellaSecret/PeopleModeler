import { test, expect } from '@playwright/test';
import {
  clearStorage,
  createPerson,
  dismissTutorial,
} from './helpers';

test.describe('Team Page', () => {
  test.beforeEach(async ({ page }) => {
    await clearStorage(page);
  });

  test('/team loads and shows empty state with < 2 people', async ({
    page,
  }) => {
    await page.goto('/PeopleModeler/team');
    await dismissTutorial(page);
    await expect(page.locator('.page h2')).toContainText('Team');
    await expect(page.locator('.empty-state')).toBeVisible();
  });

  test('team page shows synergy grid when >= 2 people', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await page.goto('/PeopleModeler/team');
    await dismissTutorial(page);
    await expect(page.locator('.team-summary')).toBeVisible();
    await expect(page.locator('.team-pairs-grid')).toBeVisible();
    await expect(page.locator('.team-pair-card')).toHaveCount(1);
    await expect(page.locator('.pair-names')).toContainText('Alice');
    await expect(page.locator('.pair-names')).toContainText('Bob');
  });

  test('team page shows weakest and strongest links', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await createPerson(page, 'Carol');
    await page.goto('/PeopleModeler/team');
    await dismissTutorial(page);
    await expect(page.locator('.team-summary')).toBeVisible();
    await expect(page.locator('.summary-card.highlight-good')).toBeVisible();
    await expect(page.locator('.summary-card.highlight-warn')).toBeVisible();
  });

  test('team page shows context averages', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await page.goto('/PeopleModeler/team');
    await dismissTutorial(page);
    await expect(page.locator('.ctx-bars')).toBeVisible();
    const ctxRows = page.locator('.ctx-row');
    await expect(ctxRows).toHaveCount(6);
  });

  test('team page shows 3 pair cards for 3 people', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await createPerson(page, 'Carol');
    await page.goto('/PeopleModeler/team');
    await dismissTutorial(page);
    await expect(page.locator('.team-pair-card')).toHaveCount(3);
  });
});
