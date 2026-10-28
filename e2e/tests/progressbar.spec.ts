import { expect, test } from '@playwright/test';

test.describe('ProgressBar', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/progressbar');
        await expect(page.locator('[data-demo=basic] .p-progressbar')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        const height = await page.locator('[data-demo=basic] .p-progressbar').evaluate((el) => getComputedStyle(el).height);
        expect(height).toBe('20px');
    });

    test('ARIA и подпись определённого режима', async ({ page }) => {
        const bar = page.locator('[data-demo=basic] .p-progressbar');
        await expect(bar).toHaveAttribute('role', 'progressbar');
        await expect(bar).toHaveAttribute('aria-valuenow', '50');
        await expect(bar).toHaveAttribute('aria-level', '50%');
        await expect(bar).toHaveAttribute('data-p', 'determinate');
        await expect(bar.locator('.p-progressbar-value')).toHaveCSS('width', /^\d+(\.\d+)?px$/);
        await expect(bar.locator('.p-progressbar-label')).toHaveText('50%');
    });

    test('dynamic растёт со временем', async ({ page }) => {
        const bar = page.locator('[data-demo=dynamic] .p-progressbar');
        await expect(async () => {
            const now = Number(await bar.getAttribute('aria-valuenow'));
            expect(now).toBeGreaterThanOrEqual(20);
        }).toPass({ timeout: 8000 });
    });

    test('indeterminate и шаблон', async ({ page }) => {
        const ind = page.locator('[data-demo=indeterminate] .p-progressbar');
        await expect(ind).toHaveClass(/p-progressbar-indeterminate/);
        await expect(ind.locator('.p-progressbar-label')).toHaveCount(0);
        await expect(page.locator('[data-demo=template] .p-progressbar-label')).toHaveText('50/100');
    });
});
