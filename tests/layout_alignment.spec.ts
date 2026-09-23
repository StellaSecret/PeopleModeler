import { test, expect, type Page } from '@playwright/test';
import { clearStorage, createPerson, dismissTutorial } from './helpers';

// This whole file exists because every alignment fix in this app's history
// was checked by hand, screenshot by screenshot, across several rounds of
// "still off by a few pixels". These tests turn that into an automated
// regression guard: whatever the Personal life / Work Persona toggle looks
// like, switching between the two must never move a section's on-screen
// position, on any viewport.

// All eight accordion sections, in DOM order, using their exact English
// titles (see app/src/i18n.rs). Every one of these exists identically in
// both facets — it's the *content* inside them, and the header badges
// beside them, that differ, never their position.
const SECTION_TITLES = [
  'Resilience & Risk Appetite',
  'OCEAN Scores (1-10)',
  'Motivations',
  'Biases',
  'Reputation',
  'Behavioral Patterns',
  'Personal Styles',
  'Values',
];

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

// A freshly-created person has no work mask, and the Work Persona facet only
// renders its edit sections once the mask is enabled (otherwise it just shows
// the "enable" row). "Copy from base profile" is the copy button in the
// currently visible persona-actions row (only the active, non-anchor row is
// visible — the others are visibility:hidden); clicking it enables the mask
// and fills every bucket, so the Work facet exposes the full section chrome
// the alignment guard compares against Personal life. Positional so it stays
// locale-independent.
async function enableWorkMask(page: Page) {
  await page
    .locator('.persona-actions:not(.persona-actions-hidden)')
    .locator('button')
    .first()
    .click();
}

async function gotoEdit(page: Page, personId: string) {
  await page.goto(`/PeopleModeler/person/${personId}/edit`);
  await page.waitForTimeout(300);
  await dismissTutorial(page);
}

// Records every section's bounding box, keyed by title.
async function captureLayout(page: Page): Promise<Record<string, { x: number; y: number }>> {
  const layout: Record<string, { x: number; y: number }> = {};
  for (const title of SECTION_TITLES) {
    const box = await editSection(page, title).boundingBox();
    if (!box) {
      throw new Error(`section "${title}" has no bounding box — is it in the DOM?`);
    }
    layout[title] = { x: box.x, y: box.y };
  }
  return layout;
}

async function assertLayoutsMatch(
  page: Page,
  before: Record<string, { x: number; y: number }>,
) {
  const after = await captureLayout(page);
  for (const title of SECTION_TITLES) {
    expect(after[title].x, `${title}: x position shifted`).toBe(before[title].x);
    expect(after[title].y, `${title}: y position shifted`).toBe(before[title].y);
  }
}

test.describe('Facet tab pixel alignment', () => {
  test('every section stays in the exact same place when switching facets (desktop)', async ({
    page,
  }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Alignment Test Desktop');
    await gotoEdit(page, personId);

    const personalLayout = await captureLayout(page);

    await facetOption(page, 'Work Persona').click();
    // Work Persona renders extra header content (the override checkboxes,
    // the "Copy from base profile"/"Clear mask" row) — if any of that ever
    // changes the layout's *height* rather than staying within its
    // reserved space, every section below it would shift, and this is
    // exactly what would catch that.
    await enableWorkMask(page);
    await assertLayoutsMatch(page, personalLayout);

    await facetOption(page, 'Personal life').click();
    await assertLayoutsMatch(page, personalLayout);
  });

  test('every section stays in the exact same place when switching facets (mobile)', async ({
    page,
  }) => {
    // Narrow viewport specifically: the header-wrap and persona-actions
    // height-mismatch bugs this guards against only ever showed up below
    // ~480px, never on desktop.
    await page.setViewportSize({ width: 390, height: 844 });

    await clearStorage(page);
    const personId = await createPerson(page, 'Alignment Test Mobile');
    await gotoEdit(page, personId);

    const personalLayout = await captureLayout(page);

    await facetOption(page, 'Work Persona').click();
    await enableWorkMask(page);
    await assertLayoutsMatch(page, personalLayout);

    await facetOption(page, 'Personal life').click();
    await assertLayoutsMatch(page, personalLayout);
  });

  test('alignment holds even with long, wrapping translated labels (mobile, FR)', async ({
    page,
  }) => {
    // French labels run noticeably longer than English ones — this is
    // what originally tipped the persona-actions row into wrapping and
    // desyncing the two facets' heights on a narrow phone.
    await page.setViewportSize({ width: 390, height: 844 });
    await clearStorage(page);
    // Create the person while still in English — the creation form's own
    // labels are localized too, and this test only cares about the layout of
    // the *edit* page it then switches to French.
    const personId = await createPerson(page, 'Alignment Test FR');
    await gotoEdit(page, personId);
    await page.locator('button.lang-toggle').click();
    // The toggle shows the *target* language, so "EN" means FR is now active
    // — wait for the re-render before measuring the (longer) FR labels.
    await expect(page.locator('button.lang-toggle')).toHaveText('EN');

    // Titles are localized under FR, so just compare the *count* and
    // *set of y-positions* of edit-sections between facets rather than
    // matching by (now-French) title text.
    const personalBoxes = await page.locator('.edit-section').evaluateAll((els) =>
      els.map((el) => el.getBoundingClientRect().y),
    );

    const workToggle = page
      .locator('div.facet-bar .facet-toggle .facet-btn')
      .nth(1);
    await workToggle.click();
    await enableWorkMask(page);
    // enableWorkMask only toggles state; wait for Dioxus to re-render the
    // Work facet's sections before snapshotting their positions.
    await expect(page.locator('.edit-section')).toHaveCount(8);

    const workBoxes = await page.locator('.edit-section').evaluateAll((els) =>
      els.map((el) => el.getBoundingClientRect().y),
    );

    expect(workBoxes).toEqual(personalBoxes);
  });

  test('the hidden persona-actions row still reserves its real layout height', async ({
    page,
  }) => {
    // Regression guard for the reserved-space fix itself: Personal life
    // renders the *same* checkbox+buttons markup as Work Persona, just
    // visibility:hidden — never display:none — specifically so it keeps
    // occupying space. If that ever regresses to display:none (or back to
    // a guessed min-height placeholder), this is what would catch it.
    // There is one reserved row per mask (Base first, then Work, then
    // Online), so scope each lookup to its row instead of matching all.
    await clearStorage(page);
    const personId = await createPerson(page, 'Persona Actions Height Test');
    await gotoEdit(page, personId);

    const workActions = page.locator('.persona-actions').nth(1);
    const onlineActions = page.locator('.persona-actions').nth(2);

    const hiddenWorkBox = await workActions.boundingBox();
    const hiddenOnlineBox = await onlineActions.boundingBox();
    expect(hiddenWorkBox, 'work persona-actions row should still occupy layout space when hidden')
      .not.toBeNull();
    expect(
      hiddenOnlineBox,
      'online persona-actions row should still occupy layout space when hidden',
    ).not.toBeNull();
    expect(hiddenWorkBox!.height).toBeGreaterThan(0);
    expect(hiddenOnlineBox!.height).toBeGreaterThan(0);

    await facetOption(page, 'Work Persona').click();
    const visibleBox = await workActions.boundingBox();
    expect(visibleBox).not.toBeNull();

    // Heights must match exactly — this is the whole point of rendering
    // identical markup rather than a guessed placeholder height.
    expect(hiddenWorkBox!.height).toBe(visibleBox!.height);
  });
});

test.describe('OCEAN stepper regressions', () => {
  test('clicking + from an unset field increments on every click, including the first', async ({
    page,
  }) => {
    // Regression guard: OceanSlider used to special-case "field is
    // currently unset" by hard-coding the result to 5 instead of applying
    // the computed +1, silently dropping the very first click. Three
    // clicks of "+" from unset should reach 8, not 7.
    await clearStorage(page);
    const personId = await createPerson(page, 'Ocean Stepper Test');
    await gotoEdit(page, personId);

    const fieldset = page.locator('fieldset.ocean-inputs').first();
    const slider = fieldset.locator('div.ocean-slider .stepper-slider').first();

    await expect(slider.locator('.step-val')).toHaveText('—');

    await slider.locator('button.step-plus').click();
    await slider.locator('button.step-plus').click();
    await slider.locator('button.step-plus').click();

    await expect(slider.locator('.step-val')).toHaveText('8/10');
  });

  test('the display always shows the /10 suffix once set', async ({ page }) => {
    await clearStorage(page);
    const personId = await createPerson(page, 'Ocean Suffix Test');
    await gotoEdit(page, personId);

    const fieldset = page.locator('fieldset.ocean-inputs').first();
    const slider = fieldset.locator('div.ocean-slider .stepper-slider').first();

    await slider.locator('button.step-plus').click();
    await expect(slider.locator('.step-val')).toHaveText(/^\d+\/10$/);
  });
});
