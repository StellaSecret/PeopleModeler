import { test, expect } from '@playwright/test';
import { clearStorage, createPerson } from './helpers';

test.describe('Compare Persons Page', () => {
  let id1: string;
  let id2: string;

  test.beforeEach(({ page }) => clearStorage(page));

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
    await expect(page.locator('.scale-score')).toContainText('%');
    await expect(page.locator('.scale-band-hero')).toBeVisible();
    await expect(page.locator('.asymmetric-scores')).toBeVisible();
    await expect(page.locator('.asym-direction').first()).toContainText('←');
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
