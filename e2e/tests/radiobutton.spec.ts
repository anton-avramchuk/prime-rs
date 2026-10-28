import { expect, test } from '@playwright/test';

test.describe('RadioButton', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/radiobutton');
        await expect(page.locator('[data-demo=group] .p-radiobutton').first()).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        const size = await page.locator('[data-demo=group] .p-radiobutton-box').first().evaluate((el) => getComputedStyle(el).width);
        expect(size).toBe('20px');
    });

    test('выбор в группе снимает остальные', async ({ page }) => {
        const roots = page.locator('[data-demo=group] .p-radiobutton');
        await page.locator('label[for=ingredient2]').click();
        await expect(roots.nth(1)).toHaveClass(/p-radiobutton-checked/);
        await expect(page.getByTestId('selected')).toHaveText('Mushroom');
        await page.locator('label[for=ingredient4]').click();
        await expect(roots.nth(3)).toHaveClass(/p-radiobutton-checked/);
        await expect(roots.nth(1)).not.toHaveClass(/p-radiobutton-checked/);
        await expect(roots.nth(3).locator('input')).toHaveAttribute('aria-checked', 'true');
        await expect(page.getByTestId('selected')).toHaveText('Onion');
    });

    test('классы размера, варианта, invalid и disabled', async ({ page }) => {
        await expect(page.locator('[data-demo=sizes] .p-radiobutton-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-radiobutton-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-radiobutton-checked')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=invalid] .p-invalid')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] .p-disabled')).toHaveCount(2);
        await expect(page.locator('[data-demo=disabled] input').first()).toBeDisabled();
    });
});
