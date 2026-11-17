import { expect, test } from '@playwright/test';

test.describe('Textarea', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/textarea');
        await expect(page.locator('[data-demo=basic] textarea.p-textarea')).toBeAttached();
    });

    test('demos render with the theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(9);
        const border = await page.locator('[data-demo=basic] textarea').evaluate((el) => getComputedStyle(el).borderTopWidth);
        expect(border).not.toBe('0px');
    });

    test('input reaches the signal and sets p-filled', async ({ page }) => {
        const area = page.locator('[data-demo=basic] textarea');
        await expect(area).not.toHaveClass(/p-filled/);
        // until hydration completes the event may be lost, so the input is repeated
        await expect(async () => {
            await area.fill(['hello', 'world'].join('\n'));
            await expect(page.getByTestId('echo')).toHaveText('11', { timeout: 500 });
        }).toPass({ timeout: 8000 });
        await expect(area).toHaveClass(/p-filled/);
    });

    test('auto_resize grows the height on input', async ({ page }) => {
        const area = page.locator('[data-demo=autoresize] textarea');
        await expect(area).toHaveClass(/p-textarea-resizable/);
        const before = (await area.boundingBox())!.height;
        await expect(async () => {
            await area.fill(Array.from({ length: 15 }, (_, i) => `line ${i}`).join('\n'));
            expect((await area.boundingBox())!.height).toBeGreaterThan(before + 50);
        }).toPass({ timeout: 8000 });
    });

    test('classes for size, variant, invalid, fluid and disabled', async ({ page }) => {
        await expect(page.locator('[data-demo=sizes] .p-textarea-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-textarea-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=invalid] .p-invalid')).toHaveCount(1);
        await expect(page.locator('[data-demo=fluid] .p-textarea-fluid')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] textarea')).toBeDisabled();
    });

    test('floatlabel: the label in the wrapper rises on input', async ({ page }) => {
        await page.waitForLoadState('networkidle');
        const demo = page.locator('[data-demo=floatlabel]');
        await expect(demo.locator('.p-floatlabel.p-floatlabel-over textarea.p-textarea')).toHaveCount(1);
        const label = demo.locator('.p-floatlabel > label');
        await expect(label).toHaveCount(1);
        const before = (await label.boundingBox())!.y;
        await demo.locator('.p-floatlabel textarea.p-textarea').first().focus();
        await expect.poll(async () => (await label.boundingBox())!.y).toBeLessThan(before);
    });

    test('iftalabel: the label in the wrapper above the value', async ({ page }) => {
        const demo = page.locator('[data-demo=iftalabel]');
        await expect(demo.locator('.p-iftalabel textarea.p-textarea')).toHaveCount(1);
        const label = demo.locator('.p-iftalabel > label');
        const box = (await label.boundingBox())!;
        const root = (await demo.locator('.p-iftalabel').boundingBox())!;
        expect(box.y).toBeGreaterThanOrEqual(root.y);
        expect(box.y + box.height).toBeLessThanOrEqual(root.y + root.height);
    });
});
