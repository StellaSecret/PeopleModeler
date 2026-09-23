import { test, expect } from '@playwright/test';
import { clearStorage, gotoNewPerson } from './helpers';

// Regression guard for the single-poll-scope bug: a keystroke used to
// re-run the *whole* PersonEditForm (every field, every section). The
// shell now owns only the layout; fields re-render in isolation. The shell
// stamps itself with `data-edit-renders`, so we can assert that typing in
// one field never re-executes the whole form.
test.describe('Edit form render scope', () => {
  test('typing in a field does not re-render the form shell', async ({
    page,
  }) => {
    await clearStorage(page);
    await gotoNewPerson(page);

    const shell = page.locator('.page-edit');
    await expect(shell).toHaveAttribute('data-edit-renders', /.+/);

    const rendersBefore = Number(await shell.getAttribute('data-edit-renders'));
    expect(rendersBefore).toBeGreaterThan(0);

    // Type into the Name field (a scoped child; must not touch the shell).
    await page.locator('label:has-text("Name") + input').type('Render');
    await page.waitForTimeout(150);

    const rendersAfter = Number(await shell.getAttribute('data-edit-renders'));
    expect(rendersAfter).toBe(rendersBefore);
  });

  test('shell only re-renders on structural changes', async ({ page }) => {
    await clearStorage(page);
    await gotoNewPerson(page);

    const shell = page.locator('.page-edit');
    const base = Number(await shell.getAttribute('data-edit-renders'));

    // Editing a section body must stay scoped too.
    await page.locator('label:has-text("Name") + input').type('x');
    await page
      .locator('.edit-section-toggle[aria-label="Motivations"]')
      .waitFor();
    // Keep the shell untouched by field edits and facility interactions.
    const stable = Number(await shell.getAttribute('data-edit-renders'));
    expect(stable).toBe(base);

    // A structural change to the shell (turning a persona facet on) remounts
    // the facet sections — the shell passes that state down, and its own
    // counter can stay put: the point is that it is *not* bumped by every
    // keystroke, whatever it does.
    //
    // The persona-actions row for Work is only visible once the Work facet is
    // the active mode (the row hides with `persona-actions-hidden` otherwise),
    // so switch the mode bar first, like the layout-alignment specs do, and
    // target the one visible row (the anchor and other masks stay hidden).
    await page
      .locator('div.facet-bar .facet-toggle .facet-btn')
      .filter({ hasText: 'Work Persona' })
      .click();
    const workCheckbox = page
      .locator('.persona-actions:not(.persona-actions-hidden)')
      .locator('input[type="checkbox"]');
    await workCheckbox.check();
    await expect(workCheckbox).toBeChecked();
    // Still stable after the persona toggle (no shell re-run).
    expect(Number(await shell.getAttribute('data-edit-renders'))).toBe(stable);
  });
});