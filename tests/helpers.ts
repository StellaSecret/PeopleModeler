import { type Locator, type Page } from '@playwright/test';

// Steppers replace drag sliders in the editing form: `[−] value [+]`.
// Deltas are based on the currently displayed value (default start: 5).
export async function setStepper(slider: Locator, target: number) {
  const txt = (await slider.locator('.step-val').innerText()).trim();
  const match = txt.match(/-?\d+/);
  const current = match ? Number(match[0]) : 5;
  let delta = target - current;
  if (delta === 0 && !match) {
    // "—" means unset: the stepper shows its default (5) but nothing is
    // stored, and unset is distinct from an explicit 5. Materialize it —
    // any click from unset lands on 4 (-) or 6 (+), so step up to 6, wait
    // for the re-render, then step back down to target.
    await slider.locator('button.step-plus').click();
    let now = 0;
    for (let i = 0; i < 100; i++) {
      const v = (await slider.locator('.step-val').innerText()).trim();
      const m = v.match(/-?\d+/);
      if (m) {
        now = Number(m[0]);
        break;
      }
      await new Promise((r) => setTimeout(r, 25));
    }
    delta = target - (now || 6);
  }
  const button = delta < 0 ? 'button.step-minus' : 'button.step-plus';
  for (let i = 0; i < Math.abs(delta); i++) {
    await slider.locator(button).click();
  }
}

export async function dismissTutorial(page: Page) {
  const skip = page.locator('.tut-modal .btn-ghost');
  try {
    await skip.waitFor({ state: 'visible', timeout: 500 });
    await skip.click();
    await page.waitForTimeout(300);
  } catch {
    // tutorial already dismissed or never shown
  }
}

export async function gotoNewPerson(page: Page) {
  await page.goto('/PeopleModeler/person/new');
  await page.waitForTimeout(1000);
  await dismissTutorial(page);
  await page.getByText('Blank (start from scratch)').click();
}

export async function clearStorage(page: Page) {
  await page.goto('/PeopleModeler/');
  await dismissTutorial(page);
  await page.evaluate(() => {
    localStorage.clear();
    localStorage.setItem('pm_tutorial_done', '1');
  });
  await page.reload();
  await page.waitForTimeout(1000);
}

export async function createPerson(page: Page, name: string): Promise<string> {
  await gotoNewPerson(page);
  await page.locator('label:has-text("Name") + input').fill(name);
  await page.click('button:has-text("Save")');
  await page.waitForURL(/\/person\//);
  return page.url().split('/').pop()!;
}

export async function setOcean(
  page: Page,
  vals: [number, number, number, number, number],
) {
  const fieldset = page.locator('fieldset.ocean-inputs').first();
  const sliders = fieldset.locator('div.ocean-slider .stepper-slider');
  for (let i = 0; i < 5; i++) {
    await setStepper(sliders.nth(i), vals[i]);
  }
}

export async function addPattern(
  page: Page,
  trigger: string,
  outcome: string,
  notes: string,
) {
  const fs = page.locator(
    'fieldset.section:has(legend:text-is("Behavioral Patterns"))',
  );
  await fs.locator('select').nth(0).selectOption(trigger);
  await fs.locator('select').nth(1).selectOption(outcome);
  await fs.locator('input[type="text"][placeholder="Notes"]').fill(notes);
  await fs.locator("button[aria-label='Add pattern']").click();
}

export async function addMotivation(
  page: Page,
  mtype: string,
  intensity: number,
) {
  const fs = page.locator(
    'fieldset.section:has(legend:text-is("Motivations"))',
  );
  await fs.locator('select').selectOption(mtype);
  await setStepper(fs.locator('.stepper-slider').first(), intensity);
  await fs.locator("button[aria-label='Add motivation']").click();
}

export async function addBias(page: Page, btype: string, intensity: number) {
  const fs = page.locator(
    'fieldset.section:has(legend:text-is("Biases"))',
  );
  await fs.locator('select').selectOption(btype);
  await setStepper(fs.locator('.stepper-slider').first(), intensity);
  await fs.locator("button[aria-label='Add bias']").click();
}

export async function enableAndSetRep(
  page: Page,
  dimIndex: number,
  value: number,
) {
  const fieldset = page.locator('fieldset.ocean-inputs').nth(1);
  const slider = fieldset.locator('div.ocean-slider').nth(dimIndex);
  await slider.locator("label.dim-toggle input[type='checkbox']").check();
  await slider.locator('input[type="range"]').fill(String(value));
}

export async function addRelationship(
  page: Page,
  otherName: string,
  type: string,
  strength: number,
) {
  const openButton = page.locator('.rel-controls button');
  if (!(await openButton.isVisible().catch(() => false))) {
    await page.getByRole('tab', { name: /Relations/ }).click();
  }
  await openButton.click();
  await page.locator('.rel-autocomplete-input').fill(otherName);
  await page
    .locator(`.rel-person-check-row:has-text("${otherName}")`)
    .click();
  await page.locator('.rel-type-select').selectOption(type);
  await page.locator('.rel-strength-slider').fill(String(strength));
  await page.locator('.rel-add-actions button.btn-primary').click();
  await page.waitForTimeout(300);
}

export async function setConfidence(page: Page, conf: number) {
  const fieldset = page.locator('fieldset.reliability');
  await setStepper(fieldset.locator('.stepper-slider').first(), conf);
}

export async function openLogTab(page: Page) {
  await page.getByRole('tab', { name: /Log/ }).click();
  await page.waitForTimeout(300);
}

export async function addLogEntry(
  page: Page,
  text: string,
  opts: { valence?: number; trigger?: string; target?: string } = {},
) {
  if (opts.valence !== undefined) {
    const label = opts.valence > 0 ? `+${opts.valence}` : String(opts.valence);
    await page.locator(`.valence-btn:text-is("${label}")`).click();
  }
  if (opts.trigger !== undefined) {
    await page.locator('.log-trigger-select').selectOption(opts.trigger);
  }
  if (opts.target !== undefined) {
    await page.locator('.log-target-select').selectOption(opts.target);
  }
  await page.getByLabel('New log entry').fill(text);
  await page.getByRole('button', { name: 'Add entry' }).click();
  await page.waitForTimeout(300);
}
