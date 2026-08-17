import { test, expect } from '@playwright/test';
import {
  clearStorage,
  createPerson,
  dismissTutorial,
  gotoNewPerson,
} from './helpers';

test.describe('UX Polish', () => {
  test.describe('Delete Confirmations', () => {
    test('person delete shows confirmation then cancels', async ({ page }) => {
      await createPerson(page, 'ToDelete');
      await expect(page.locator('h1')).toContainText('ToDelete');
      await page.locator('button:has-text("Delete")').click();
      await expect(
        page.locator('button:has-text("Delete this person")'),
      ).toBeVisible();
      await page.locator('button:has-text("Cancel")').click();
      await expect(page.locator('h1')).toContainText('ToDelete');
    });

    test('person delete confirmation executes delete', async ({ page }) => {
      await createPerson(page, 'ToDelete');
      await page.locator('button:has-text("Delete")').click();
      await page.locator('button:has-text("Delete this person")').click();
      await expect(page).toHaveURL(/\/PeopleModeler\/?$/);
      await expect(page.locator('.page')).toContainText(/no people/i);
    });
  });

  test.describe('Search No-Results', () => {
    test('shows no-results message when search matches nothing', async ({
      page,
    }) => {
      await createPerson(page, 'Alice');
      await page.goto('/PeopleModeler/');
      await page.locator('.search-input').fill('ZzzNonExistent');
      await expect(page.locator('.empty-state')).toContainText(
        /no results/i,
      );
    });

    test('shows no-results message containing the query', async ({ page }) => {
      await createPerson(page, 'Alice');
      await page.goto('/PeopleModeler/');
      await page.locator('.search-input').fill('Zzz');
      await expect(page.locator('.empty-state p')).toContainText('Zzz');
    });
  });

  test.describe('Mobile Table', () => {
    test('name and score columns visible on mobile', async ({ page }) => {
      await createPerson(page, 'MobileTest');
      await page.setViewportSize({ width: 375, height: 812 });
      await page.goto('/PeopleModeler/');
      await expect(page.locator('.pt-name-cell')).toBeVisible();
      await expect(page.locator('.pt-score')).toBeVisible();
    });

    test('detail columns hidden on mobile', async ({ page }) => {
      await createPerson(page, 'MobileTest');
      await page.setViewportSize({ width: 375, height: 812 });
      await page.goto('/PeopleModeler/');
      await expect(page.locator('.pt-col-sub').first()).not.toBeVisible();
    });
  });

  test.describe('Team Page Navigation', () => {
    test('team nav link exists and navigates', async ({ page }) => {
      await page.goto('/PeopleModeler/');
      await dismissTutorial(page);
      const teamLink = page.locator('.nav-links a').filter({ hasText: /Team|Équipe/ });
      await expect(teamLink).toBeVisible();
      await teamLink.click();
      await expect(page).toHaveURL(/\/team/);
    });
  });
});
