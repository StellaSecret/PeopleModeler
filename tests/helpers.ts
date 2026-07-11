import { type Page } from '@playwright/test';

export async function gotoNewPerson(page: Page) {
  await page.goto('/PeopleModeler/person/new');
  await page.getByText('Blank (start from scratch)').click();
}

export async function clearStorage(page: Page) {
  await page.goto('/PeopleModeler/');
  await page.evaluate(() => localStorage.clear());
  await page.reload();
  await page.waitForTimeout(500);
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
  const sliders = fieldset.locator('div.ocean-slider input[type="range"]');
  for (let i = 0; i < 5; i++) {
    await sliders.nth(i).fill(String(vals[i]));
  }
}

export async function addPattern(
  page: Page,
  trigger: string,
  outcome: string,
  confidence: number,
) {
  const fs = page.locator(
    'fieldset.section:has(legend:text-is("Behavioral Patterns"))',
  );
  await fs.locator('select').selectOption(trigger);
  await fs
    .locator("input[placeholder='Predicted outcome...']")
    .fill(outcome);
  await fs.locator('input[type="range"]').fill(String(confidence));
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
  await fs
    .locator('input[type="range"]')
    .first()
    .fill(String(intensity));
  await fs.locator("button[aria-label='Add motivation']").click();
}

export async function addBias(page: Page, btype: string, intensity: number) {
  const fs = page.locator(
    'fieldset.section:has(legend:text-is("Biases"))',
  );
  await fs.locator('select').selectOption(btype);
  await fs
    .locator('input[type="range"]')
    .first()
    .fill(String(intensity));
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
