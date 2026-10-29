import { expect, test } from '@playwright/test';

test.describe('IftaLabel', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/iftalabel');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-iftalabel')).toBeVisible();
    });

    test('демонстрации и разметка', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const root = page.locator('[data-demo=basic] .p-iftalabel');
        expect(await root.evaluate((el) => [...el.children].map((c) => c.tagName.toLowerCase()))).toEqual(['input', 'label']);
    });

    test('метка внутри границ поля над значением', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const label = (await demo.locator('label').boundingBox())!;
        const field = (await demo.locator('input').boundingBox())!;
        expect(label.y).toBeGreaterThanOrEqual(field.y);
        expect(label.y + label.height).toBeLessThan(field.y + field.height);
        expect(label.x).toBeGreaterThanOrEqual(field.x);
    });

    test('метка не двигается при фокусе', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const before = (await demo.locator('label').boundingBox())!.y;
        await demo.locator('input').focus();
        await demo.locator('input').fill('abc');
        expect((await demo.locator('label').boundingBox())!.y).toBeCloseTo(before, 0);
    });

    test('invalid: класс p-invalid следует за пустотой поля', async ({ page }) => {
        const input = page.locator('[data-demo=invalid] input');
        await expect(input).toHaveClass(/p-invalid/);
        await input.fill('x');
        await expect(input).not.toHaveClass(/p-invalid/);
    });
});
