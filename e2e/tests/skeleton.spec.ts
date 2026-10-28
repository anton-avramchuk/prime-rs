import { expect, test } from '@playwright/test';

test.describe('Skeleton', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/skeleton');
        await expect(page.locator('[data-demo=shapes] .p-skeleton').first()).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(3);
        const bg = await page.locator('[data-demo=shapes] .p-skeleton').first().evaluate((el) => getComputedStyle(el).overflow);
        expect(bg).toBe('hidden');
    });

    test('круги и квадраты: размеры и форма', async ({ page }) => {
        const circles = page.locator('[data-demo=shapes] .p-skeleton-circle');
        await expect(circles).toHaveCount(4);
        const box = await circles.nth(2).boundingBox();
        expect(Math.round(box!.width)).toBe(64);
        expect(Math.round(box!.height)).toBe(64);
        const radius = await circles.first().evaluate((el) => getComputedStyle(el).borderRadius);
        expect(radius).toMatch(/50%|px/);
    });

    test('aria-hidden, data-p и скругление', async ({ page }) => {
        const first = page.locator('[data-demo=shapes] .p-skeleton').first();
        await expect(first).toHaveAttribute('aria-hidden', 'true');
        await expect(first).toHaveAttribute('data-p', 'rectangle');
        const rounded = page.locator('[data-demo=shapes] .p-skeleton[style*="border-radius: 16px"]');
        await expect(rounded).toHaveCount(5);
    });

    test('card и list содержат заглушки', async ({ page }) => {
        await expect(page.locator('[data-demo=card] .p-skeleton')).toHaveCount(7);
        await expect(page.locator('[data-demo=list] .p-skeleton')).toHaveCount(12);
    });
});
