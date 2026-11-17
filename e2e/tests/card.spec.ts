import { expect, test } from '@playwright/test';

test.describe('Card', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/card');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-card')).toBeVisible();
    });

    test('demos are rendered with theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const radius = await page.locator('[data-demo=basic] .p-card').evaluate((el) => getComputedStyle(el).borderRadius);
        expect(radius).not.toBe('0px');
    });

    test('basic: title and content', async ({ page }) => {
        const card = page.locator('[data-demo=basic] .p-card');
        await expect(card.locator('.p-card-title')).toHaveText('Simple Card');
        await expect(card.locator('.p-card-content')).toContainText('Lorem ipsum');
        await expect(card.locator('.p-card-header, .p-card-subtitle, .p-card-footer')).toHaveCount(0);
    });

    test('advanced: block order and a footer with buttons', async ({ page }) => {
        const card = page.locator('[data-demo=advanced] .p-card');
        const top = await card.evaluate((el) => [...el.children].map((c) => c.className.split(' ')[0]));
        expect(top).toEqual(['p-card-header', 'p-card-body']);
        const body = await card.locator('.p-card-body').evaluate((el) => [...el.children].map((c) => c.className.split(' ')[0]));
        expect(body).toEqual(['p-card-title', 'p-card-subtitle', 'p-card-content', 'p-card-footer']);
        await expect(card.locator('.p-card-footer button.p-button')).toHaveCount(2);
        await expect(card.locator('.p-card-title')).toHaveText('Advanced Card');
    });
});
