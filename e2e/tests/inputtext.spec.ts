import { expect, test } from '@playwright/test';

test.describe('InputText', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/inputtext');
        await expect(page.locator('[data-demo=basic] input.p-inputtext')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
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
});
