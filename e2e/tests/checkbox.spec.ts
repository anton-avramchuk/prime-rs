import { expect, test } from '@playwright/test';

test.describe('Checkbox', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/checkbox');
        await expect(page.locator('[data-demo=basic] .p-checkbox')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(7);
        const size = await page.locator('[data-demo=basic] .p-checkbox-box').evaluate((el) => getComputedStyle(el).width);
        expect(size).toBe('20px');
    });

    test('клик переключает состояние и классы', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-checkbox');
        await expect(root).not.toHaveClass(/p-checkbox-checked/);
        await root.locator('input').click({ force: true });
        await expect(root).toHaveClass(/p-checkbox-checked p-highlight/);
        await expect(root.locator('.p-checkbox-icon')).toBeVisible();
        await expect(page.getByTestId('echo')).toHaveText('true');
        await root.locator('input').click({ force: true });
        await expect(root).not.toHaveClass(/p-checkbox-checked/);
        await expect(page.getByTestId('echo')).toHaveText('false');
    });

    test('набор значений', async ({ page }) => {
        await page.locator('label[for=ny1]').click();
        await page.locator('label[for=ny3]').click();
        await expect(page.getByTestId('selected')).toHaveText('Chicago,New York');
        await page.locator('label[for=ny1]').click();
        await expect(page.getByTestId('selected')).toHaveText('New York');
    });

    test('классы размера, варианта, invalid, disabled и indeterminate', async ({ page }) => {
        await expect(page.locator('[data-demo=sizes] .p-checkbox-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-checkbox-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=invalid] .p-invalid')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] .p-disabled')).toHaveCount(2);
        await expect(page.locator('[data-demo=disabled] input').first()).toBeDisabled();
        const root = page.locator('[data-demo=indeterminate] .p-checkbox');
        await expect(root.locator('.p-checkbox-icon')).toBeVisible();
        await root.locator('input').click({ force: true });
        await expect(root).toHaveClass(/p-checkbox-checked/);
    });
});
