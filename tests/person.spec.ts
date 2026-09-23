import { test, expect } from '@playwright/test';
import { gotoNewPerson, clearStorage, addPattern, addMotivation, addBias, enableAndSetRep, setOcean } from './helpers';

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
    await addPattern(page, 'Stress', 'becomes_quiet', 'test note');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.pattern-item')).toContainText(/stress/i);
  });

  test('fairness rhetoric gap surfaces consistency warning', async ({ page }) => {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(200);
    await addMotivation(page, 'Fairness', 7);
    await enableAndSetRep(page, 8, 2); // FairFavoritism → Favoritism (≤ 3)
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.danger-warning')).toContainText(/fairness/i);
  });

  test('consistent fairness shows no consistency warning', async ({ page }) => {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(200);
    await addMotivation(page, 'Fairness', 7);
    await enableAndSetRep(page, 8, 8); // FairFavoritism → Fair (consistent)
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.danger-warning')).toHaveCount(0);
  });

  test('helping rhetoric gap surfaces consistency warning', async ({ page }) => {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(200);
    await addMotivation(page, 'Helping', 7);
    await enableAndSetRep(page, 7, 2); // GenerousSelfish → Selfish (≤ 3)
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.danger-warning')).toContainText(/helpfulness/i);
  });

  test('open-minded self-image gap surfaces consistency warning', async ({ page }) => {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(200);
    await setOcean(page, [9, 5, 3, 5, 3]); // Openness high, rest neutral
    await enableAndSetRep(page, 12, 2); // AdaptableRigid → Rigid (≤ 3)
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.danger-warning')).toContainText(/open[- ]?minded/i);
  });

  test('calm reputation contradicted by volatile pattern', async ({ page }) => {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(200);
    await enableAndSetRep(page, 5, 8); // CalmReactive → Calm (≥ 8)
    await addPattern(page, 'Stress', 'panics', 'blows up under deadline');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.danger-warning')).toContainText(/calm/i);
  });

  test('impostor self-image gap surfaces consistency warning', async ({ page }) => {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(200);
    await addBias(page, 'Impostor', 7);
    await enableAndSetRep(page, 4, 8); // HumbleArrogant → Arrogant (≥ 8)
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('.danger-warning')).toContainText(/arrogant/i);
  });

  test('work-only person uses Work as primary context, not Personal life', async ({ page }) => {
    // Second person: pick the Work context first, then start from scratch.
    await page.goto('/PeopleModeler/person/new');
    await page.waitForTimeout(500);
    await page.locator('.template-card', { hasText: 'At work' }).click();
    await page.locator('.template-skip button').last().click();
    await page.locator('label:has-text("Name") + input').fill('Work Wendy');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    // Editor: the anchor tab opens active and is labeled with the suffix that
    // disambiguates it from the separate At-Work mask slot.
    await page.click('a:has-text("Edit")');
    await expect(page.locator('.facet-btn.active')).toContainText('At work (main)');
    // The detail header only shows the facet bar once a persona mask exists,
    // so give her an Online persona, save, and confirm the primary wins.
    await page.locator('.facet-btn', { hasText: 'Online' }).click();
    await page
      .locator('.persona-actions:not(.persona-actions-hidden) input[type="checkbox"]')
      .check();
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\/[^/]+$/);
    await expect(page.locator('.facet-btn.active')).toContainText('At work (main)');
  });
});
