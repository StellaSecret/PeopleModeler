import { test, expect, type Page } from '@playwright/test';
import {
  clearStorage,
  createPerson,
  addMotivation,
  setStepper,
  dismissTutorial,
} from './helpers';

test.describe('Edit form state machines', () => {
  // Locates a `.edit-section` by the aria-label of its toggle button
  // (same axe the buttons expose to screen readers).
  function editSection(page: Page, title: string) {
    return page
      .locator('.edit-section')
      .filter({ has: page.locator(`.edit-section-toggle[aria-label="${title}"]`) });
  }

  function facetOption(page: Page, name: string) {
    return page
      .locator('div.facet-bar .facet-toggle .facet-btn')
      .filter({ hasText: name });
  }

  async function gotoEdit(page: Page, personId: string) {
    await page.goto(`/PeopleModeler/person/${personId}/edit`);
    await page.waitForTimeout(300);
    await dismissTutorial(page);
  }

  async function opennessSlider(page: Page, title: string) {
    return editSection(page, title)
      .locator('.ocean-inputs')
      .first()
      .locator('.stepper-slider')
      .first();
  }

  test('accordion sections open and close independently', async ({ page }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Accordion Test');
    await gotoEdit(page, personId);

    // All eight sections start expanded (deliberate default).
    await expect(page.locator('.edit-section.open')).toHaveCount(8);

    const mot = editSection(page, 'Motivations');
    await expect(mot).toHaveClass(/open/);

    // Closing Motivations collapses only it...
    await mot.locator('.edit-section-toggle').click();
    await expect(mot).not.toHaveClass(/open/);
    await expect(mot.locator('.edit-section-toggle')).toHaveAttribute(
      'aria-expanded',
      'false',
    );
    await expect(mot.locator('.edit-section-body')).toHaveClass(/collapsed/);
    await expect(page.locator('.edit-section.open')).toHaveCount(7);

    // ...and closing OCEAN leaves Motivations closed (independent state).
    const ocean = editSection(page, 'OCEAN Scores (1-10)');
    await ocean.locator('.edit-section-toggle').click();
    await expect(mot).not.toHaveClass(/open/);
    await expect(page.locator('.edit-section.open')).toHaveCount(6);

    // Reopening works and restores content without touching the other.
    await mot.locator('.edit-section-toggle').click();
    await expect(mot).toHaveClass(/open/);
    await expect(mot.locator('.edit-section-toggle')).toHaveAttribute(
      'aria-expanded',
      'true',
    );
    await expect(page.locator('.edit-section.open')).toHaveCount(7);
  });

  test('discard reverts to saved values, not to the just-edited ones', async ({
    page,
  }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Discard Test');
    await gotoEdit(page, personId);

    // A freshly created person's OCEAN scores are genuinely unset (shown
    // as "—", not defaulted to 5) — that's deliberate, matching how
    // Reputation also treats "undefined" as distinct from "explicitly set
    // to some value". So the saved baseline this test discards back to
    // has to be established for real: set 5, save, and reload, rather
    // than assuming a fresh/unsaved person already reads as 5/10.
    await expect(
      (await opennessSlider(page, 'OCEAN Scores (1-10)')).locator('.step-val'),
    ).toHaveText('—');
    await setStepper(await opennessSlider(page, 'OCEAN Scores (1-10)'), 5);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await gotoEdit(page, personId);

    const slider = await opennessSlider(page, 'OCEAN Scores (1-10)');
    await expect(slider.locator('.step-val')).toHaveText('5/10');

    await setStepper(slider, 8);
    await expect(slider.locator('.step-val')).toHaveText('8/10');

    // Regression guard: this used to restore the live (just-edited) value,
    // making Discard look like a no-op.
    await editSection(page, 'OCEAN Scores (1-10)')
      .locator('.edit-section-discard')
      .click();
    await expect(slider.locator('.step-val')).toHaveText('5/10');
  });

  test('discard removes list items added in this session', async ({ page }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Discard List Test');
    await gotoEdit(page, personId);

    const mot = editSection(page, 'Motivations');
    await addMotivation(page, 'Achievement', 5);
    await expect(mot.locator('.list-item')).toHaveCount(1);

    await mot.locator('.edit-section-discard').click();
    await expect(mot.locator('.list-item')).toHaveCount(0);
  });

  test('work persona edits stay isolated from the base profile', async ({
    page,
  }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Facet Test');
    await gotoEdit(page, personId);

    // Base: openness 8.
    await setStepper(await opennessSlider(page, 'OCEAN Scores (1-10)'), 8);
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);

    // Work: copy from base, then move openness to 3.
    await gotoEdit(page, personId);
    await facetOption(page, 'Work Persona').click();
    await page.getByRole('button', { name: 'Copy from base profile' }).click();
    await expect(
      (await opennessSlider(page, 'OCEAN Scores (1-10)')).locator('.step-val'),
    ).toHaveText('8/10');
    await setStepper(await opennessSlider(page, 'OCEAN Scores (1-10)'), 3);

    // Switch back to Personal life: base value is untouched.
    await facetOption(page, 'Personal life').click();
    await expect(
      (await opennessSlider(page, 'OCEAN Scores (1-10)')).locator('.step-val'),
    ).toHaveText('8/10');

    // Work still remembers its own 3 within the same session.
    await facetOption(page, 'Work Persona').click();
    await expect(
      (await opennessSlider(page, 'OCEAN Scores (1-10)')).locator('.step-val'),
    ).toHaveText('3/10');

    // Save, reload, and confirm BOTH facets persist independently.
    await page.click('button:has-text("Save")');
    await page.waitForURL(/\/person\//);
    await gotoEdit(page, personId);
    await expect(
      (await opennessSlider(page, 'OCEAN Scores (1-10)')).locator('.step-val'),
    ).toHaveText('8/10');
    await facetOption(page, 'Work Persona').click();
    await expect(
      (await opennessSlider(page, 'OCEAN Scores (1-10)')).locator('.step-val'),
    ).toHaveText('3/10');
  });
});