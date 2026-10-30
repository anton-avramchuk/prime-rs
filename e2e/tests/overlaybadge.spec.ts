import { expect, test } from '@playwright/test';

test.describe('OverlayBadge', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/overlaybadge');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-overlaybadge').first()).toBeVisible();
    });

    test('разметка: три обёртки, бейдж после содержимого', async ({ page }) => {
        const roots = page.locator('[data-demo=basic] .p-overlaybadge');
        await expect(roots).toHaveCount(3);
        await expect(roots.first().locator('> .p-badge')).toHaveText('2');
        await expect(roots.nth(1).locator('> .p-badge')).toHaveClass(/p-badge-danger/);
        await expect(roots.nth(2).locator('> .p-badge')).toHaveClass(/p-badge-dot/);
    });

    test('бейдж лежит на правом верхнем углу содержимого', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-overlaybadge').first();
        const box = (await root.boundingBox())!;
        const badge = (await root.locator('> .p-badge').boundingBox())!;
        const cx = badge.x + badge.width / 2;
        const cy = badge.y + badge.height / 2;
        expect(Math.abs(cx - (box.x + box.width))).toBeLessThan(2);
        expect(Math.abs(cy - box.y)).toBeLessThan(2);
        expect(await root.evaluate((el) => getComputedStyle(el).position)).toBe('relative');
    });
});
