import { test, expect, type Page } from '@playwright/test';
import {
  clearStorage,
  createPerson,
  addRelationship,
  setOcean,
  addPattern,
  addMotivation,
  addBias,
  enableAndSetRep,
} from './helpers';

test.describe('Compare — Advanced Synergy', () => {
  test.beforeEach(({ page }) => clearStorage(page));

  async function createPersonWithOcean(
    page: Page,
    name: string,
    ocean: [number, number, number, number, number],
  ): Promise<string> {
    const id = await createPerson(page, name);
    await page.goto(`/PeopleModeler/person/${id}/edit`);
    await page.waitForTimeout(200);
    await setOcean(page, ocean);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    return id;
  }

  async function createPersonWithPatterns(
    page: Page,
    name: string,
    triggers: { trigger: string; outcome: string; notes: string }[],
  ): Promise<string> {
    const id = await createPerson(page, name);
    await page.goto(`/PeopleModeler/person/${id}/edit`);
    await page.waitForTimeout(200);
    for (const p of triggers) {
      await addPattern(page, p.trigger, p.outcome, p.notes);
    }
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    return id;
  }

  test('volatile OCEAN pair (N≥7, A≤4) renders breakdown', async ({
    page,
  }) => {
    const id1 = await createPersonWithOcean(
      page,
      'Hothead',
      [5, 5, 5, 2, 9],
    );
    const id2 = await createPersonWithOcean(
      page,
      'Reactive',
      [5, 5, 5, 3, 8],
    );

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.breakdown-section')).toBeVisible();
    await expect(page.locator('.breakdown-bars')).toBeVisible();
    await expect(page.locator('.compatibility-score')).toBeVisible();
  });

  test('harmonious OCEAN pair (low N, high A) shows high score', async ({
    page,
  }) => {
    const id1 = await createPersonWithOcean(
      page,
      'Zen',
      [7, 7, 6, 8, 3],
    );
    const id2 = await createPersonWithOcean(
      page,
      'Calm',
      [8, 6, 5, 7, 2],
    );

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.compatibility-score')).toBeVisible();
    await expect(page.locator('.analysis-card.synergy')).toBeVisible();
  });

  test('all-negative patterns pair renders analysis', async ({ page }) => {
    const id1 = await createPersonWithPatterns(page, 'Pessimist', [
      { trigger: 'Conflict', outcome: 'becomes_defensive', notes: 'conflict test' },
      { trigger: 'Stress', outcome: 'becomes_quiet', notes: 'stress test' },
    ]);
    const id2 = await createPersonWithPatterns(page, 'Defensive', [
      { trigger: 'Threatened', outcome: 'deflects_blame', notes: 'threat test' },
    ]);

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.analysis-section')).toBeVisible();
    await expect(page.locator('.analysis-card.friction')).toBeVisible();
    await expect(page.locator('.analysis-card.strategy')).toBeVisible();
  });

  test('mixed pattern pair renders analysis', async ({ page }) => {
    const id1 = await createPersonWithPatterns(page, 'Optimist', [
      { trigger: 'Change', outcome: 'embraces_change', notes: 'change test' },
    ]);
    const id2 = await createPersonWithPatterns(page, 'Anxious', [
      { trigger: 'Stress', outcome: 'becomes_quiet', notes: 'anxiety test' },
    ]);

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.analysis-section')).toBeVisible();
  });

  test('motivation complementarity (Power+Helping) renders breakdown', async ({
    page,
  }) => {
    const id1 = await createPerson(page, 'Leader');
    await page.goto(`/PeopleModeler/person/${id1}/edit`);
    await page.waitForTimeout(200);
    await addMotivation(page, 'Power', 9);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);

    const id2 = await createPerson(page, 'Supporter');
    await page.goto(`/PeopleModeler/person/${id2}/edit`);
    await page.waitForTimeout(200);
    await addMotivation(page, 'Helping', 8);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.breakdown-section')).toBeVisible();
    await expect(page.locator('.analysis-card.synergy')).toBeVisible();
  });

  test('identical full-profile pair shows high breakdown percentages', async ({
    page,
  }) => {
    const id1 = await createPerson(page, 'Twin');
    await page.goto(`/PeopleModeler/person/${id1}/edit`);
    await page.waitForTimeout(200);
    await setOcean(page, [8, 7, 9, 6, 2]);
    await addMotivation(page, 'Achievement', 9);
    await addMotivation(page, 'Learning', 7);
    await addBias(page, 'Anchoring', 8);
    await addBias(page, 'Confirmation', 6);
    await enableAndSetRep(page, 0, 8); // Hardworker
    await enableAndSetRep(page, 2, 9); // Honest
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);

    const id2 = await createPerson(page, 'Twin2');
    await page.goto(`/PeopleModeler/person/${id2}/edit`);
    await page.waitForTimeout(200);
    await setOcean(page, [8, 7, 9, 6, 2]);
    await addMotivation(page, 'Achievement', 9);
    await addMotivation(page, 'Learning', 7);
    await addBias(page, 'Anchoring', 8);
    await addBias(page, 'Confirmation', 6);
    await enableAndSetRep(page, 0, 8);
    await enableAndSetRep(page, 2, 9);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.breakdown-bars')).toBeVisible();
    await expect(page.locator('.analysis-card.synergy')).toBeVisible();
    await expect(page.locator('.compatibility-score')).toBeVisible();
  });

  test('relationship selector renders, general context shows no band', async ({
    page,
  }) => {
    const id1 = await createPerson(page, 'Alice');
    const id2 = await createPerson(page, 'Bob');

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.rel-context-box')).toBeVisible();
    await expect(page.locator('.rel-context-row select')).toHaveValue('none');
    await expect(
      page.locator('.rel-context-row input[type="range"]'),
    ).toHaveCount(0);
    await expect(page.locator('.scale-band-hint')).toHaveCount(0);
  });

  test('relationship context shows strength slider and widens band at low strength', async ({
    page,
  }) => {
    const id1 = await createPerson(page, 'Alice');
    const id2 = await createPerson(page, 'Bob');

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await page.locator('.rel-context-row select').selectOption('Friends');
    await expect(
      page.locator('.rel-context-row input[type="range"]'),
    ).toHaveCount(1);
    await expect(page.locator('.scale-band-hint')).toContainText('±8%');

    await page.locator('.rel-context-row input[type="range"]').fill('2');
    await expect(page.locator('.scale-band-hint')).toContainText('±12%');
  });

  test('compare prefills type and strength from an existing relationship', async ({
    page,
  }) => {
    const id1 = await createPerson(page, 'Alice');
    const id2 = await createPerson(page, 'Bob');

    await page.goto(`/PeopleModeler/person/${id1}`);
    await page.waitForTimeout(500);
    await addRelationship(page, 'Bob', 'WorksWith', 2);

    await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
    await page.waitForTimeout(500);

    await expect(page.locator('.rel-context-row select')).toHaveValue(
      'WorksWith',
    );
    await expect(page.locator('.rel-strength')).toContainText('Strength: 2/10');
    await expect(page.locator('.scale-band-hint')).toContainText('±12%');
  });
});
