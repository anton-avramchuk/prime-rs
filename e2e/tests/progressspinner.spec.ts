import { expect, test } from '@playwright/test';

test.describe('ProgressSpinner', () => {
    const errors: string[] = [];

    test.beforeEach(async ({ page }) => {
        errors.length = 0;
        page.on('console', (m) => m.type() === 'error' && errors.push(m.text()));
        page.on('pageerror', (e) => errors.push(e.message));
        await page.goto('/progressspinner');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-progressspinner')).toBeVisible();
    });

    test('demos render with the theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const spin = page.locator('[data-demo=basic] .p-progressspinner-spin');
        expect(await spin.evaluate((el) => getComputedStyle(el).animationName)).not.toBe('none');
        const root = page.locator('[data-demo=basic] .p-progressspinner');
        expect((await root.boundingBox())!.width).toBeGreaterThan(50);
    });

    test('basic: ARIA and the circle by default', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-progressspinner');
        await expect(root).toHaveAttribute('role', 'progressbar');
        await expect(root).toHaveAttribute('aria-busy', 'true');
        await expect(root).toHaveAttribute('aria-label', 'loading');
        const circle = root.locator('circle.p-progressspinner-circle');
        await expect(circle).toHaveAttribute('stroke-width', '2');
        await expect(circle).toHaveAttribute('fill', 'none');
    });

    test('custom: size, thickness, fill and speed', async ({ page }) => {
        const root = page.locator('[data-demo=custom] .p-progressspinner');
        const box = (await root.boundingBox())!;
        expect(Math.round(box.width)).toBe(50);
        expect(Math.round(box.height)).toBe(50);
        const circle = root.locator('circle');
        await expect(circle).toHaveAttribute('stroke-width', '8');
        await expect(circle).toHaveAttribute('fill', 'transparent');
        expect(await root.locator('svg').evaluate((el) => (el as unknown as SVGElement).style.animationDuration)).toBe('0.5s');
    });

    test('hydration without errors', async ({ page }) => {
        await page.waitForTimeout(500);
        expect(errors).toEqual([]);
    });
});
