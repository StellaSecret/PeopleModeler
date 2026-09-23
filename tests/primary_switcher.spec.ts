import { test, expect } from '@playwright/test';
import { clearStorage, createPerson } from './helpers';

// The anchor context ("(main)" tab) is not fixed at Personal life: a work-only
// person anchors on At work, and any person can re-point the anchor from the
// editor's main-context bar. That swap must reorder the tabs primary-first,
// relabel the extra tab as the "Personal life" mask, and survive a save.
function tabOrder(page: import('@playwright/test').Page) {
  return page
    .locator('div.facet-bar .facet-toggle .facet-btn')
    .allInnerTexts();
}

test.describe('Primary context switcher', () => {
  test('re-pointing the anchor at At work reorders and relabels the tabs', async ({
    page,
  }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Switch Sam');
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(300);

    // Base-primary editor starts with the canonical order.
    await expect(tabOrder(page)).resolves.toEqual([
      'Personal life (main)',
      'Work Persona',
      'Online',
    ]);

    await page.locator('.main-context-btn', { hasText: 'At work' }).click();
    // Primary first: At work becomes the anchor tab, Personal life becomes a
    // mask tab, and the active tab jumps straight to the new anchor.
    await expect(tabOrder(page)).resolves.toEqual([
      'At work (main)',
      'Personal life Persona',
      'Online',
    ]);
    await expect(page.locator('.facet-btn.active')).toContainText('At work (main)');

    // The anchor tab never shows a persona enable row.
    await expect(
      page.locator('.persona-actions:not(.persona-actions-hidden)'),
    ).toHaveCount(0);

    // Jumping to the mask tab exposes the Personal-life enable row; enabling
    // it materializes the private mask without touching the work profile.
    await page
      .locator('.facet-btn', { hasText: 'Personal life Persona' })
      .click();
    const baseRow = page.locator('.persona-actions:not(.persona-actions-hidden)');
    await expect(baseRow).toHaveCount(1);
    await baseRow.locator('input[type="checkbox"]').check();
    await expect(baseRow.locator('input[type="checkbox"]')).toBeChecked();
  });

  test('anchor swap and private mask persist across a save/reload', async ({
    page,
  }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Persistent Perry');
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(300);

    await page.locator('.main-context-btn', { hasText: 'At work' }).click();
    await page.locator('.facet-btn', { hasText: 'Personal life Persona' }).click();
    await page
      .locator('.persona-actions:not(.persona-actions-hidden) input[type="checkbox"]')
      .check();
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\/[^/]+$/);

    // Detail opens on the anchor tab and the anchor still reads "At work".
    await expect(page.locator('.facet-toggle')).toBeVisible();
    await expect(page.locator('.facet-btn.active')).toContainText('At work (main)');

    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(300);
    await expect(page.locator('.facet-btn.active')).toContainText('At work (main)');
    await expect(tabOrder(page)).resolves.toEqual([
      'At work (main)',
      'Personal life Persona',
      'Online',
    ]);
    await page.locator('.facet-btn', { hasText: 'Personal life Persona' }).click();
    await expect(
      page
        .locator('.persona-actions:not(.persona-actions-hidden) input[type="checkbox"]')
        .first(),
    ).toBeChecked();
  });
});