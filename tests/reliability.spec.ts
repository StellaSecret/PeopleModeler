import { test, expect, type Page } from '@playwright/test';
import { clearStorage, createPerson, setConfidence } from './helpers';

test.describe('Data Reliability UX', () => {
  test.beforeEach(({ page }) => clearStorage(page));

  async function createPersonWithConfidence(
    page: Page,
    name: string,
    conf: number,
  ): Promise<string> {
    const id = await createPerson(page, name);
    await page.goto(`/PeopleModeler/person/${id}/edit`);
    const fieldset = page.locator('fieldset.reliability');
    await expect(fieldset).toBeVisible();
    await setConfidence(page, conf);
    await expect(fieldset).toContainText(`${conf}/10`);
    await page.getByRole('button', { name: /Save/ }).click();
    await page.waitForURL(/\/person\//);
    return id;
  }

  test('edit form groups confidence in a reliability section', async ({
    page,
  }) => {
    await createPerson(page, 'Alice');
    const id = page.url().split('/').pop()!;
    await page.goto(`/PeopleModeler/person/${id}/edit`);
    await page.waitForTimeout(300);

    const fieldset = page.locator('fieldset.reliability');
    await expect(fieldset).toBeVisible();
    await expect(fieldset.locator('legend')).toContainText('Data quality');
    await expect(fieldset.locator('.reliability-hint')).toContainText(
      'rough sketch',
    );
    await expect(fieldset.locator('label')).toContainText(
      'Profile confidence (1-10)',
    );
    await expect(fieldset.locator('input[type="range"]')).toHaveCount(1);
  });

  test('detail page groups confidence and completeness as data quality', async ({
    page,
  }) => {
    await createPerson(page, 'Alice');
    await page.waitForTimeout(300);

    const dq = page.locator('.data-quality');
    await expect(dq).toBeVisible();
    await expect(dq.locator('.data-quality-title')).toContainText(
      'Data quality',
    );
    await expect(dq.locator('.confidence-badge')).toContainText(
      'Profile confidence: 5/10',
    );
    await expect(dq.locator('.completeness-badge')).toContainText('Compl.');
    await expect(page.locator('.ps-band-range')).toContainText('±8');
  });

  test('profile score band widens as confidence drops', async ({ page }) => {
    await createPersonWithConfidence(page, 'Skeptic', 2);
    await expect(page.locator('.ps-band-range')).toContainText('±12');

    await createPersonWithConfidence(page, 'Verified', 10);
    await expect(page.locator('.ps-band-range')).toContainText('±4');
  });

  test(
    'compare band is the widest of relationship and profile confidence',
    { timeout: 30000 },
    async ({ page }) => {
      const id1 = await createPersonWithConfidence(page, 'CarefulA', 10);
      const id2 = await createPersonWithConfidence(page, 'CarefulB', 10);

      await page.goto(`/PeopleModeler/compare/${id1}/${id2}`);
      await page.waitForTimeout(500);
      await page.locator('.rel-context-row select').selectOption('Partner');
      await page.locator('.rel-context-row input[type="range"]').fill('10');
      await expect(page.locator('.scale-band-hint')).toContainText('±4%');

      const low1 = await createPersonWithConfidence(page, 'GuessA', 2);
      const low2 = await createPersonWithConfidence(page, 'GuessB', 10);

      await page.goto(`/PeopleModeler/compare/${low1}/${low2}`);
      await page.waitForTimeout(500);
      await page.locator('.rel-context-row select').selectOption('Partner');
      await page.locator('.rel-context-row input[type="range"]').fill('10');
      await expect(page.locator('.scale-band-hint')).toContainText('±12%');
    },
  );
});
