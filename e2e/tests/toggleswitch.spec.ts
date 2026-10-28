import { expect, test } from '@playwright/test';

test.describe('ToggleSwitch', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/toggleswitch');
        await expect(page.locator('[data-demo=basic] .p-toggleswitch')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        const radius = await page.locator('[data-demo=basic] .p-toggleswitch-slider').evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
    });

    test('клик по дорожке переключает состояние', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-toggleswitch');
        await expect(root).not.toHaveClass(/p-toggleswitch-checked/);
        await root.locator('.p-toggleswitch-slider').click({ force: true });
        await expect(root).toHaveClass(/p-toggleswitch-checked/);
        await expect(root.locator('input')).toHaveAttribute('aria-checked', 'true');
        await expect(page.getByTestId('echo')).toHaveText('true');
        await root.locator('input').click({ force: true });
        await expect(root).not.toHaveClass(/p-toggleswitch-checked/);
        await expect(page.getByTestId('echo')).toHaveText('false');
    });

    test('предвыбор, шаблон ручки, invalid и disabled', async ({ page }) => {
        await expect(page.locator('[data-demo=preselection] .p-toggleswitch-checked')).toHaveCount(1);
        const tpl = page.locator('[data-demo=template] .p-toggleswitch');
        await expect(tpl.locator('.p-toggleswitch-handle svg')).toBeAttached();
        await tpl.locator('.p-toggleswitch-slider').click({ force: true });
        await expect(tpl).toHaveClass(/p-toggleswitch-checked/);
        await expect(page.locator('[data-demo=invalid] .p-invalid')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] .p-disabled')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] input')).toBeDisabled();
    });
});
