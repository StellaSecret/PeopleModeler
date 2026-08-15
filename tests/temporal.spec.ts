import { test, expect } from '@playwright/test';
import {
  clearStorage,
  createPerson,
  openLogTab,
  addLogEntry,
} from './helpers';

test.describe('Temporal layer (Phase 3)', () => {
  test.beforeEach(({ page }) => clearStorage(page));

  test('log tab offers valence, trigger and target inputs', async ({
    page,
  }) => {
    await createPerson(page, 'Alice');
    await openLogTab(page);

    await expect(page.locator('.valence-row .valence-btn')).toHaveCount(7);
    await expect(page.locator('.log-trigger-select option')).toContainText([
      'No trigger',
      'Stress',
      'Conflict',
      'Feedback',
    ]);
    await expect(page.locator('.log-target-select')).toBeVisible();
  });

  test('entries render valence/trigger/target badges', async ({ page }) => {
    await createPerson(page, 'Alice');
    const id1 = page.url().split('/').pop()!;
    const id2 = await createPerson(page, 'Bob');
    await page.goto(`/PeopleModeler/person/${id1}`);
    await page.waitForTimeout(300);
    await openLogTab(page);

    await addLogEntry(page, 'Great session', {
      valence: 2,
      trigger: 'Success',
      target: id2,
    });

    await expect(page.locator('.log-entry .log-valence')).toHaveText('+2');
    await expect(page.locator('.log-entry .log-trigger')).toContainText(
      'Success',
    );
    await expect(page.locator('.log-entry .log-target')).toContainText('Bob');
  });

  test('positive entries push the personal trend to Improving', async ({
    page,
  }) => {
    await createPerson(page, 'Alice');
    await openLogTab(page);

    await addLogEntry(page, 'Went well', { valence: 2 });

    const chip = page.locator('.log-head .trend-chip');
    await expect(chip).toBeVisible();
    await expect(chip).toContainText('Improving');
    await expect(chip).toContainText('+7');
  });

  test('negative entries push the personal trend to Deteriorating', async ({
    page,
  }) => {
    await createPerson(page, 'Alice');
    await openLogTab(page);

    await addLogEntry(page, 'Rough patch', { valence: -2 });

    const chip = page.locator('.log-head .trend-chip');
    await expect(chip).toBeVisible();
    await expect(chip).toContainText('Deteriorating');
    await expect(chip).toContainText('-7');
  });

  test('compare trend chip counts only targeted interactions', async ({
    page,
  }) => {
    const id1 = await createPerson(page, 'Alice');
    const id2 = await createPerson(page, 'Bob');

    await page.goto(`/PeopleModeler/person/${id1}`);
    await page.waitForTimeout(300);
    await openLogTab(page);
    await addLogEntry(page, 'About Bob', { valence: 2, target: id2 });

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    const chip = page.locator('.trend-chip');
    await expect(chip).toBeVisible();
    await expect(chip).toContainText('Improving');
    await expect(chip).toContainText('+7');
  });

  test('compare shows no trend chip without targeted entries', async ({
    page,
  }) => {
    const id1 = await createPerson(page, 'Alice');
    const id2 = await createPerson(page, 'Bob');

    await page.goto(`/PeopleModeler/person/${id1}`);
    await page.waitForTimeout(300);
    await openLogTab(page);
    await addLogEntry(page, 'Self note', { valence: 2 });

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.trend-chip')).toHaveCount(0);
  });
});
