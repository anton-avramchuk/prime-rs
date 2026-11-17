import { expect, test } from '@playwright/test';

test.describe('FloatLabel', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/floatlabel');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-floatlabel')).toBeVisible();
    });

    test('demos and variant classes', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(3);
        await expect(page.locator('[data-demo=basic] .p-floatlabel')).toHaveClass(/p-floatlabel-over/);
        await expect(page.locator('[data-demo=variants] .p-floatlabel-in')).toHaveCount(1);
        await expect(page.locator('[data-demo=variants] .p-floatlabel-on')).toHaveCount(1);
        await expect(page.locator('[data-demo=variants] .p-floatlabel-over')).toHaveCount(1);
    });

    test('over: the label rises on focus and stays while typing', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const label = demo.locator('label');
        const input = demo.locator('input');
        const before = (await label.boundingBox())!.y;
        await input.focus();
        await expect.poll(async () => (await label.boundingBox())!.y).toBeLessThan(before);
        await input.fill('abc');
        await input.blur();
        await expect.poll(async () => (await label.boundingBox())!.y).toBeLessThan(before);
        await input.fill('');
        await input.blur();
        await expect.poll(async () => (await label.boundingBox())!.y).toBeCloseTo(before, 0);
    });

    test('in: the label stays inside the field', async ({ page }) => {
        const demo = page.locator('[data-demo=variants] .p-floatlabel-in');
        const input = demo.locator('input');
        await input.focus();
        await page.waitForTimeout(400);
        const label = (await demo.locator('label').boundingBox())!;
        const field = (await input.boundingBox())!;
        expect(label.y).toBeGreaterThanOrEqual(field.y);
        expect(label.y + label.height).toBeLessThanOrEqual(field.y + field.height);
    });

    test('on: the label sits on the top border of the field', async ({ page }) => {
        const demo = page.locator('[data-demo=variants] .p-floatlabel-on');
        const input = demo.locator('input');
        await input.focus();
        const field = (await input.boundingBox())!;
        await expect.poll(async () => (await demo.locator('label').boundingBox())!.y).toBeLessThan(field.y);
        const label = (await demo.locator('label').boundingBox())!;
        expect(label.y + label.height).toBeGreaterThan(field.y);
    });

    test('invalid: the p-invalid class follows whether the field is empty', async ({ page }) => {
        const input = page.locator('[data-demo=invalid] input');
        await expect(input).toHaveClass(/p-invalid/);
        await input.fill('x');
        await expect(input).not.toHaveClass(/p-invalid/);
        await input.fill('');
        await expect(input).toHaveClass(/p-invalid/);
    });
});
