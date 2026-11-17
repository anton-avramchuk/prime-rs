import { expect, test } from '@playwright/test';

test.describe('Button', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/button');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        // wait for hydration: a click on Loading works only after it
        await expect(page.locator('[data-demo=loading] button')).toBeVisible();
        await page.waitForTimeout(500);
    });

    test('all demos render with the theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(15);
        const bg = await page.locator('[data-demo=severity] button.p-button-danger').evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(bg).not.toBe('rgba(0, 0, 0, 0)');
        expect(bg).not.toBe('rgb(239, 239, 239)');
    });

    test('loading: the button is blocked while loading', async ({ page }) => {
        const button = page.locator('[data-demo=loading] button');
        await expect(button).toBeEnabled();
        await button.click();
        await expect(button).toBeDisabled();
        await expect(button).toHaveClass(/p-button-loading/);
        await expect(button.locator('.p-button-loading-icon')).toHaveCount(1);
        await expect(button).toBeEnabled({ timeout: 4000 });
        await expect(button).not.toHaveClass(/p-button-loading/);
    });

    test('disabled: the button is unavailable', async ({ page }) => {
        const button = page.locator('[data-demo=disabled] button');
        await expect(button).toBeDisabled();
        await expect(button).toHaveAttribute('data-p-disabled', 'true');
    });

    test('ripple: p-ink is activated on press', async ({ page }) => {
        const button = page.locator('[data-demo=ripple] button');
        const ink = button.locator('.p-ink');
        await expect(ink).toHaveCount(1);
        await button.dispatchEvent('mousedown');
        await expect(ink).toHaveAttribute('data-p-ink-active', 'true');
        // without ripple enabled there is no p-ink
        await expect(page.locator('[data-demo=basic] .p-ink')).toHaveCount(0);
    });

    test('icons: position and the icon-only mode', async ({ page }) => {
        await expect(page.locator('[data-demo=icons] .p-button-icon-right')).toHaveCount(1);
        await expect(page.locator('[data-demo=icons] .p-button-vertical')).toHaveCount(2);
        await expect(page.locator('[data-demo=iconsonly] .p-button-icon-only')).toHaveCount(9);
    });
});
