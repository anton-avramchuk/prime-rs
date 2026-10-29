import { expect, test } from '@playwright/test';

test.describe('InputText', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/inputtext');
        await expect(page.locator('[data-demo=basic] input.p-inputtext')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(8);
        const radius = await page.locator('[data-demo=basic] input').evaluate((el) => getComputedStyle(el).borderTopWidth);
        expect(radius).not.toBe('0px');
    });

    test('ввод попадает в сигнал и ставит p-filled', async ({ page }) => {
        const input = page.locator('[data-demo=basic] input');
        await expect(input).not.toHaveClass(/p-filled/);
        await input.fill('привет');
        await expect(page.getByTestId('echo')).toHaveText('привет');
        await expect(input).toHaveClass(/p-filled/);
        await input.fill('');
        await expect(input).not.toHaveClass(/p-filled/);
    });

    test('классы размера, варианта, invalid и fluid', async ({ page }) => {
        await expect(page.locator('[data-demo=sizes] .p-inputtext-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-inputtext-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=invalid] .p-invalid')).toHaveCount(1);
        await expect(page.locator('[data-demo=fluid] .p-inputtext-fluid')).toHaveCount(1);
    });

    test('disabled недоступно', async ({ page }) => {
        await expect(page.locator('[data-demo=disabled] input')).toBeDisabled();
    });

    test('floatlabel: метка в обёртке и поднимается при вводе', async ({ page }) => {
        await page.waitForLoadState('networkidle');
        const demo = page.locator('[data-demo=floatlabel]');
        await expect(demo.locator('.p-floatlabel.p-floatlabel-over input.p-inputtext')).toHaveCount(1);
        const label = demo.locator('.p-floatlabel > label');
        await expect(label).toHaveCount(1);
        const before = (await label.boundingBox())!.y;
        await demo.locator('.p-floatlabel input.p-inputtext').first().focus();
        await expect.poll(async () => (await label.boundingBox())!.y).toBeLessThan(before);
    });

    test('iftalabel: метка в обёртке над значением', async ({ page }) => {
        const demo = page.locator('[data-demo=iftalabel]');
        await expect(demo.locator('.p-iftalabel input.p-inputtext')).toHaveCount(1);
        const label = demo.locator('.p-iftalabel > label');
        const box = (await label.boundingBox())!;
        const root = (await demo.locator('.p-iftalabel').boundingBox())!;
        expect(box.y).toBeGreaterThanOrEqual(root.y);
        expect(box.y + box.height).toBeLessThanOrEqual(root.y + root.height);
    });
});
