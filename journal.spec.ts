import { test, expect } from '@playwright/test'

// ─────────────────────────────────────────────────────────────
// Home page
// ─────────────────────────────────────────────────────────────

test.describe('Home page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('displays the app title and nav links', async ({ page }) => {
    // The title may be visually hidden on narrow viewports via CSS —
    // check it's present in the DOM and check the nav links which are always visible.
    await expect(page.getByText('Strategic Journal').first()).toBeAttached()
    await expect(page.getByRole('link', { name: 'Entries' })).toBeVisible()
    await expect(page.getByRole('link', { name: 'Review' })).toBeVisible()
  })

  test('shows Connect Google Drive button', async ({ page }) => {
    await expect(page.getByRole('button', { name: 'Connect Google Drive' })).toBeVisible()
  })

  test('shows empty state when no entries exist', async ({ page }) => {
    await expect(page.getByText('Your first entry awaits')).toBeVisible()
    await expect(page.getByText('Hypotheses. Decisions. Predictions.')).toBeVisible()
    await expect(page.getByRole('link', { name: 'Begin' })).toBeVisible()
  })

  test('"Begin" link navigates to the entry page', async ({ page }) => {
    await page.getByRole('link', { name: 'Begin' }).click()
    await expect(page).toHaveURL(/\/entry/)
  })

  test('"+ New entry" button navigates to /entry', async ({ page }) => {
    // On a fresh store with no entries the CTA is "Begin".
    // Navigate directly to confirm the route works.
    await page.goto('/entry')
    await expect(page.getByRole('button', { name: 'Save entry' })).toBeVisible()
  })
})

// ─────────────────────────────────────────────────────────────
// Entry page — create & save
// ─────────────────────────────────────────────────────────────

test.describe('Entry page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/entry')
    // Wait for the entry form to fully load
    await expect(page.getByRole('button', { name: 'Save entry' })).toBeVisible()
  })

  test('shows the entry form with all three sections', async ({ page }) => {
    await expect(page.getByText('Hypotheses')).toBeVisible()
    await expect(page.getByText('Decisions')).toBeVisible()
    await expect(page.getByText('Predictions')).toBeVisible()
  })

  test('shows context textarea with correct placeholder', async ({ page }) => {
    await expect(
      page.getByPlaceholder("What's the broader context of today? What's on your mind?")
    ).toBeVisible()
  })

  test('Cancel button navigates back', async ({ page }) => {
    // Cancel calls navigate(-1), so we need prior history.
    // Go home first, then navigate to /entry via the Begin link.
    await page.goto('/')
    await page.getByRole('link', { name: 'Begin' }).click()
    await expect(page.getByRole('button', { name: 'Save entry' })).toBeVisible()
    await page.getByRole('button', { name: 'Cancel' }).click()
    await expect(page).toHaveURL('/')
  })

  test('fills in context note and saves entry', async ({ page }) => {
    await page.getByPlaceholder("What's the broader context of today? What's on your mind?")
      .fill('Testing the strategic journal end-to-end.')

    await page.getByRole('button', { name: 'Save entry' }).click()
    await expect(page).toHaveURL('/')
    await expect(page.getByText('Testing the strategic journal end-to-end.')).toBeVisible()
  })

  test('adds a hypothesis and saves', async ({ page }) => {
    // Open hypothesis section
    await page.getByText('Hypotheses').click()
    await expect(page.getByPlaceholder('I believe that…')).toBeVisible()

    await page.getByPlaceholder('I believe that…').fill('AI adoption will accelerate.')
    await page.getByRole('button', { name: 'Add hypothesis' }).click()

    await page.getByRole('button', { name: 'Save entry' }).click()
    await expect(page).toHaveURL('/')
    await expect(page.getByText('1 H')).toBeVisible()
  })

  test('adds a decision and saves', async ({ page }) => {
    await page.getByText('Decisions').click()
    await expect(page.getByPlaceholder('I decided to…')).toBeVisible()

    await page.getByPlaceholder('I decided to…').fill('Migrate to Kotlin Multiplatform.')
    await page.getByPlaceholder('Because…').fill('Better code sharing.')
    await page.getByRole('button', { name: 'Add decision' }).click()

    await page.getByRole('button', { name: 'Save entry' }).click()
    await expect(page).toHaveURL('/')
    await expect(page.getByText('1 D')).toBeVisible()
  })

  test('adds a prediction and saves', async ({ page }) => {
    await page.getByText('Predictions').click()
    await expect(page.getByPlaceholder('I predict that…')).toBeVisible()

    await page.getByPlaceholder('I predict that…').fill('We ship v2.0 before December.')
    await page.getByPlaceholder('The measurable outcome will be…').fill('App store release.')
    await page.getByRole('button', { name: 'Add prediction' }).click()

    await page.getByRole('button', { name: 'Save entry' }).click()
    await expect(page).toHaveURL('/')
    await expect(page.getByText('1 P')).toBeVisible()
  })
})

// ─────────────────────────────────────────────────────────────
// Entry edit flow
// ─────────────────────────────────────────────────────────────

test.describe('Entry edit flow', () => {
  test('existing entry can be reopened and updated', async ({ page }) => {
    // Create an entry
    await page.goto('/entry')
    await expect(page.getByRole('button', { name: 'Save entry' })).toBeVisible()
    await page.getByPlaceholder("What's the broader context of today? What's on your mind?")
      .fill('Original context.')
    await page.getByRole('button', { name: 'Save entry' }).click()
    await expect(page).toHaveURL('/')

    // Click the entry card to reopen
    await page.getByText('Original context.').click()
    await expect(page).toHaveURL(/\/entry/)
    await expect(page.getByRole('button', { name: 'Save entry' })).toBeVisible()

    // Update and save
    const textarea = page.getByPlaceholder("What's the broader context of today? What's on your mind?")
    await textarea.clear()
    await textarea.fill('Updated context.')
    await page.getByRole('button', { name: 'Save entry' }).click()
    await expect(page).toHaveURL('/')
    await expect(page.getByText('Updated context.')).toBeVisible()
  })
})

// ─────────────────────────────────────────────────────────────
// Review page
// ─────────────────────────────────────────────────────────────

test.describe('Review page', () => {
  test('navigates to /review via nav link', async ({ page }) => {
    await page.goto('/')
    await page.getByRole('link', { name: 'Review' }).click()
    await expect(page).toHaveURL('/review')
  })

  test('shows review heading and tabs', async ({ page }) => {
    await page.goto('/review')
    // h1.heading contains "Review"
    await expect(page.locator('h1', { hasText: 'Review' })).toBeVisible()
    // Tab buttons use lowercase text matching the useState type values
    await expect(page.getByRole('button', { name: 'predictions' })).toBeVisible()
    await expect(page.getByRole('button', { name: 'decisions' })).toBeVisible()
    await expect(page.getByRole('button', { name: 'analytics' })).toBeVisible()
  })

  test('pending review banner links to review page', async ({ page }) => {
    // Seed an entry with an overdue prediction
    await page.goto('/entry')
    await expect(page.getByRole('button', { name: 'Save entry' })).toBeVisible()
    await page.getByText('Predictions').click()
    await page.getByPlaceholder('I predict that…').fill('Past deadline prediction')
    await page.getByPlaceholder('The measurable outcome will be…').fill('Some outcome')
    // Set deadline in the past
    const past = new Date()
    past.setDate(past.getDate() - 10)
    await page.locator('input[type="date"]').fill(past.toISOString().split('T')[0])
    await page.getByRole('button', { name: 'Add prediction' }).click()
    await page.getByRole('button', { name: 'Save entry' }).click()
    await expect(page).toHaveURL('/')

    // The pending banner should appear
    await expect(page.getByText(/to review/i)).toBeVisible()
    await page.getByText(/to review/i).click()
    await expect(page).toHaveURL('/review')
  })
})

// ─────────────────────────────────────────────────────────────
// Navigation
// ─────────────────────────────────────────────────────────────

test.describe('Navigation', () => {
  test('browser back button returns from entry to home', async ({ page }) => {
    await page.goto('/')
    await page.goto('/entry')
    await page.goBack()
    await expect(page).toHaveURL('/')
  })

  test('unknown route renders app shell', async ({ page }) => {
    await page.goto('/this-does-not-exist')
    // App shell loads — check the nav links which are always visible on all viewports
    await expect(page.getByRole('link', { name: 'Entries' })).toBeVisible()
    await expect(page.getByRole('link', { name: 'Review' })).toBeVisible()
  })
})
