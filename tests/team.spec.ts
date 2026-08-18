import { test, expect } from '@playwright/test';
import {
  clearStorage,
  createPerson,
  dismissTutorial,
} from './helpers';

test.describe('Team Page (via /team/all)', () => {
  test.beforeEach(async ({ page }) => {
    await clearStorage(page);
  });

  test('/team/all loads and shows empty state with < 2 people', async ({
    page,
  }) => {
    await page.goto('/PeopleModeler/team/all');
    await dismissTutorial(page);
    await expect(page.locator('h2')).toContainText('All People');
    await expect(page.locator('.empty-state')).toBeVisible();
  });

  test('team page shows synergy grid when >= 2 people', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await page.goto('/PeopleModeler/team/all');
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
    await page.goto('/PeopleModeler/team/all');
    await dismissTutorial(page);
    await expect(page.locator('.team-summary')).toBeVisible();
    await expect(page.locator('.summary-card.highlight-good')).toBeVisible();
    await expect(page.locator('.summary-card.highlight-warn')).toBeVisible();
  });

  test('team page shows context averages', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await page.goto('/PeopleModeler/team/all');
    await dismissTutorial(page);
    await expect(page.locator('.ctx-bars')).toBeVisible();
    const ctxRows = page.locator('.ctx-row');
    await expect(ctxRows).toHaveCount(6);
  });

  test('team page shows 3 pair cards for 3 people', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await createPerson(page, 'Carol');
    await page.goto('/PeopleModeler/team/all');
    await dismissTutorial(page);
    await expect(page.locator('.team-pair-card')).toHaveCount(3);
  });
});

async function createTeamViaUI(page: any, name: string, icon?: string) {
  await page.goto('/PeopleModeler/teams/new');
  await dismissTutorial(page);
  await page.waitForTimeout(300);
  await page.locator('#team-name-input').fill(name);
  if (icon) {
    await page.locator(`.emoji-btn[aria-label="Icon ${icon}"]`).click();
  }
  await page.waitForTimeout(200);
  await page.locator('button:has-text("Save")').click();
  await page.waitForTimeout(500);
}

test.describe('Teams List Page', () => {
  test.beforeEach(async ({ page }) => {
    await clearStorage(page);
  });

  test('/teams shows All People row and create button', async ({ page }) => {
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await expect(page.locator('.teams-row-all')).toBeVisible();
    await expect(page.locator('.teams-row-name').filter({ hasText: 'All People' })).toBeVisible();
    await expect(page.locator('.btn-primary').filter({ hasText: /\+/ })).toBeVisible();
  });

  test('create a team and see it in the list', async ({ page }) => {
    await createTeamViaUI(page, 'Alpha Squad');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await expect(page.locator('.teams-row-name').filter({ hasText: 'Alpha Squad' })).toBeVisible();
  });

  test('create team then navigate to detail page', async ({ page }) => {
    await createTeamViaUI(page, 'Beta Team');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'Beta Team' }).click();
    await expect(page.locator('h2')).toContainText('Beta Team');
  });

  test('delete team with confirmation', async ({ page }) => {
    await createTeamViaUI(page, 'To Delete');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await expect(page.locator('.teams-row-name').filter({ hasText: 'To Delete' })).toBeVisible();
    await page.locator('.teams-del-btn').click();
    await page.locator('.btn-danger').filter({ hasText: 'Delete' }).click();
    await expect(page.locator('.teams-row-name').filter({ hasText: 'To Delete' })).not.toBeVisible();
  });
});

test.describe('Team Member Management', () => {
  test.beforeEach(async ({ page }) => {
    await clearStorage(page);
  });

  test('Members tab shows all persons as checkboxes', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await createTeamViaUI(page, 'MyTeam');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'MyTeam' }).click();
    await expect(page.locator('h2')).toContainText('MyTeam');
    await page.locator('button:has-text("Members")').click();
    await page.waitForTimeout(300);
    await expect(page.locator('.team-member-row')).toHaveCount(2);
    await expect(page.locator('.team-member-name').filter({ hasText: 'Alice' })).toBeVisible();
    await expect(page.locator('.team-member-name').filter({ hasText: 'Bob' })).toBeVisible();
  });

  test('toggle checkbox adds person to team', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await createTeamViaUI(page, 'AddTeam');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'AddTeam' }).click();
    await page.locator('button:has-text("Members")').click();
    await page.waitForTimeout(300);
    const aliceCheckbox = page.locator('.team-member-name').filter({ hasText: 'Alice' }).locator('..').locator('input[type="checkbox"]');
    await aliceCheckbox.check({ force: true });
    await page.waitForTimeout(300);
    const bobCheckbox = page.locator('.team-member-name').filter({ hasText: 'Bob' }).locator('..').locator('input[type="checkbox"]');
    await bobCheckbox.check({ force: true });
    await page.waitForTimeout(300);
    await page.locator('button:has-text("Synergy")').click();
    await page.waitForTimeout(500);
    await expect(page.locator('.team-pair-card')).toHaveCount(1);
    await expect(page.locator('.pair-names')).toContainText('Alice');
    await expect(page.locator('.pair-names')).toContainText('Bob');
  });

  test('uncheck removes person from team', async ({ page }) => {
    await createPerson(page, 'Alice');
    await createPerson(page, 'Bob');
    await createTeamViaUI(page, 'RemoveTeam');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'RemoveTeam' }).click();
    await page.locator('button:has-text("Members")').click();
    await page.waitForTimeout(300);
    const aliceCheckbox = page.locator('.team-member-name').filter({ hasText: 'Alice' }).locator('..').locator('input[type="checkbox"]');
    await aliceCheckbox.check({ force: true });
    await page.waitForTimeout(300);
    const bobCheckbox = page.locator('.team-member-name').filter({ hasText: 'Bob' }).locator('..').locator('input[type="checkbox"]');
    await bobCheckbox.check({ force: true });
    await page.waitForTimeout(300);
    await aliceCheckbox.uncheck({ force: true });
    await page.waitForTimeout(300);
    await page.locator('button:has-text("Synergy")').click();
    await page.waitForTimeout(500);
    await expect(page.locator('.empty-state')).toBeVisible();
  });

  test('All People team shows no-edit message on Members tab', async ({ page }) => {
    await page.goto('/PeopleModeler/team/all');
    await dismissTutorial(page);
    await page.locator('button:has-text("Members")').click();
    await page.waitForTimeout(300);
    await expect(page.locator('.empty-state')).toBeVisible();
  });
});

test.describe('Team Icon', () => {
  test.beforeEach(async ({ page }) => {
    await clearStorage(page);
  });

  test('create team with icon and see it in the list', async ({ page }) => {
    await createTeamViaUI(page, 'Icon Team', '🦁');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await expect(page.locator('.teams-row-name').filter({ hasText: 'Icon Team' })).toBeVisible();
    const row = page.locator('.teams-row').filter({ hasText: 'Icon Team' });
    await expect(row.locator('.teams-row-emoji')).toContainText('🦁');
  });

  test('create team without icon shows default emoji', async ({ page }) => {
    await createTeamViaUI(page, 'Default Icon Team');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    const row = page.locator('.teams-row').filter({ hasText: 'Default Icon Team' });
    await expect(row.locator('.teams-row-emoji')).toContainText('🎯');
  });

  test('team detail shows icon in header', async ({ page }) => {
    await createTeamViaUI(page, 'Detail Icon', '🦊');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'Detail Icon' }).click();
    await expect(page.locator('.team-name-row .teams-row-emoji')).toContainText('🦊');
  });
});

test.describe('Team Rename', () => {
  test.beforeEach(async ({ page }) => {
    await clearStorage(page);
  });

  test('rename team via edit button', async ({ page }) => {
    await createTeamViaUI(page, 'Old Name');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'Old Name' }).click();
    await page.waitForTimeout(300);
    await page.locator('.team-edit-btn').click();
    await page.waitForTimeout(200);
    const input = page.locator('#team-rename-input');
    await input.clear();
    await input.fill('New Name');
    await page.locator('button:has-text("Save")').last().click();
    await page.waitForTimeout(300);
    await expect(page.locator('h2')).toContainText('New Name');
  });

  test('rename persists in list after navigating back', async ({ page }) => {
    await createTeamViaUI(page, 'Persist Name');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'Persist Name' }).click();
    await page.waitForTimeout(300);
    await page.locator('.team-edit-btn').click();
    await page.waitForTimeout(200);
    const input = page.locator('#team-rename-input');
    await input.clear();
    await input.fill('Persisted');
    await page.locator('button:has-text("Save")').last().click();
    await page.waitForTimeout(300);
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await expect(page.locator('.teams-row-name').filter({ hasText: 'Persisted' })).toBeVisible();
  });

  test('cancel rename does not save', async ({ page }) => {
    await createTeamViaUI(page, 'Stay Name');
    await page.goto('/PeopleModeler/teams');
    await dismissTutorial(page);
    await page.locator('.teams-row-link').filter({ hasText: 'Stay Name' }).click();
    await page.waitForTimeout(300);
    await page.locator('.team-edit-btn').click();
    await page.waitForTimeout(200);
    const input = page.locator('#team-rename-input');
    await input.clear();
    await input.fill('Should Not Save');
    await page.locator('button:has-text("Cancel")').last().click();
    await page.waitForTimeout(200);
    await expect(page.locator('h2')).toContainText('Stay Name');
  });

  test('All People team has no edit button', async ({ page }) => {
    await page.goto('/PeopleModeler/team/all');
    await dismissTutorial(page);
    await expect(page.locator('.team-edit-btn')).not.toBeVisible();
  });
});
