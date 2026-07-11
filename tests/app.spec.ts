import { test, expect } from '@playwright/test';
import { gotoNewPerson, clearStorage } from './helpers';

test.describe('People Modeler Dioxus App', () => {

  test.beforeEach(({ page }) => clearStorage(page));

  // ── Create Person ─────────────────────────────────────

  test('create a person and see detail', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Marie Curie');
    await page.locator('label:has-text("Role") + input').fill('Physicist');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('h1')).toContainText('Marie Curie');
    await expect(page.locator('.content')).toContainText('Physicist');
  });

  test('create person with ocean scores', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Alan Turing');
    const sliders = page.locator('.ocean-inputs input[type="range"]');
    await expect(sliders).toHaveCount(5);
    await sliders.nth(0).fill('8');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('h1')).toContainText('Alan Turing');
  });

  // ── Edit Person ───────────────────────────────────────

  test('edit person name', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Old Name');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await page.click('a:has-text("Edit")');
    await page.waitForURL(/\/edit/);
    await page.locator('label:has-text("Name") + input').fill('New Name');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('h1')).toContainText('New Name');
  });

  test('delete person', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('To Delete');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    page.once('dialog', d => d.accept());
    await page.click('button:has-text("Delete")');
    await expect(page).toHaveURL(/\/PeopleModeler\/?$/);
  });

  // ── Person list ───────────────────────────────────────

  test('person appears in list after creation', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Ada Lovelace');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await page.locator('.logo').click();
    await expect(page).toHaveURL(/\/PeopleModeler\/?$/);
    await expect(page.locator('.person-card')).toHaveCount(1);
    await expect(page.locator('.person-card')).toContainText('Ada Lovelace');
  });

  // ── Person detail sections ────────────────────────────

  test('person detail shows motivations, biases, patterns', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Test Person');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await expect(page.locator('h2:has-text("Motivations")')).toBeVisible();
    await expect(page.locator('h2:has-text("Behavioral Patterns")')).toBeVisible();
    await page.locator('button:has-text("Biases")').click();
    await expect(page.locator('h2:has-text("Biases")')).toBeVisible();
  });

  // ── Predictions ───────────────────────────────────────

  test('add prediction for person', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Predictable');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    const personId = page.url().split('/').pop()!;

    await page.goto(`/PeopleModeler/person/${personId}`);
    await page.locator('button:has-text("Predictions")').click();
    await page.fill('input[placeholder="Context..."]', 'Meeting tomorrow');
    await page.fill('input[placeholder^="Predicted"]', 'Will negotiate');
    await page.click('button:has-text("Add")');
    await expect(page.locator('.prediction-card')).toHaveCount(1);
    await expect(page.locator('.prediction-card')).toContainText('Meeting tomorrow');
  });

  test('resolve prediction', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Resolver');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    const personId = page.url().split('/').pop()!;

    await page.goto(`/PeopleModeler/person/${personId}`);
    await page.locator('button:has-text("Predictions")').click();
    await page.fill('input[placeholder="Context..."]', 'Test context');
    await page.fill('input[placeholder^="Predicted"]', 'Will happen');
    await page.click('button:has-text("Add")');
    await expect(page.locator('.prediction-card')).toHaveCount(1);

    await page.locator('.prediction-card').locator('button:has-text("Resolve")').click();
    await expect(page.locator('.resolve-form')).toBeVisible({ timeout: 3000 });
    await page.locator('.resolve-form input[placeholder="Actual outcome..."]').fill('It happened');
    await page.locator('.resolve-form .btn-primary').click();
    await expect(page.locator('.prediction-card')).toContainText('It happened');
  });

  test('delete prediction', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Deleter');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    const personId = page.url().split('/').pop()!;

    await page.goto(`/PeopleModeler/person/${personId}`);
    await page.locator('button:has-text("Predictions")').click();
    await page.fill('input[placeholder="Context..."]', 'To delete');
    await page.fill('input[placeholder^="Predicted"]', 'Will be removed');
    await page.click('button:has-text("Add")');
    await expect(page.locator('.prediction-card')).toHaveCount(1);

    await page.locator('.prediction-card').locator('button:has-text("Delete")').click();
    await expect(page.locator('.prediction-card')).toHaveCount(0);
  });

  // ── Insights ──────────────────────────────────────────

  test('insights shows persons', async ({ page }) => {
    await gotoNewPerson(page);
    await page.locator('label:has-text("Name") + input').fill('Insight Person');
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);

    await page.goto('/PeopleModeler/insights');
    await expect(page.locator('.person-card')).toContainText('Insight Person');
  });

  // ── Sync Page ─────────────────────────────────────────

  test('sync page shows export/import buttons', async ({ page }) => {
    await page.goto('/PeopleModeler/sync');
    await expect(page.getByText('Export JSON')).toBeVisible();
    await expect(page.getByText('Import JSON')).toBeVisible();
    await expect(page.getByText('Sign in with Google')).toBeVisible();
  });
});
