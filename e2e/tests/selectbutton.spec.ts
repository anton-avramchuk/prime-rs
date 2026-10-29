import { expect, test } from '@playwright/test';

test.describe('SelectButton', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/selectbutton');
        await expect(page.locator('[data-demo=basic] .p-selectbutton')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(7);
        const basic = page.locator('[data-demo=basic] .p-selectbutton');
        await expect(basic).toHaveAttribute('role', 'group');
        await expect(basic).toHaveAttribute('aria-labelledby', 'basic');
        const radius = await basic.locator('.p-togglebutton').first().evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
    });

    test('одиночный выбор переключается и снимается', async ({ page }) => {
        const buttons = page.locator('[data-demo=basic] .p-togglebutton');
        await expect(buttons.nth(0)).toHaveAttribute('aria-pressed', 'true');
        await expect(page.getByTestId('echo')).toHaveText('one-way');
        await buttons.nth(1).click();
        await expect(buttons.nth(0)).toHaveAttribute('aria-pressed', 'false');
        await expect(buttons.nth(1)).toHaveAttribute('aria-pressed', 'true');
        await expect(page.getByTestId('echo')).toHaveText('return');
        await buttons.nth(1).click();
        await expect(buttons.nth(1)).toHaveAttribute('aria-pressed', 'false');
        await expect(page.getByTestId('echo')).toHaveText('');
    });

    test('множественный выбор и клавиши', async ({ page }) => {
        const buttons = page.locator('[data-demo=multiple] .p-togglebutton');
        await buttons.nth(2).click();
        await buttons.nth(0).focus();
        await page.keyboard.press('Space');
        await expect(page.getByTestId('echo-multiple')).toHaveText('3,1');
        await page.keyboard.press('Enter');
        await expect(page.getByTestId('echo-multiple')).toHaveText('3');
        await expect(buttons.nth(2)).toHaveAttribute('aria-pressed', 'true');
    });

    test('шаблон, размеры, invalid, disabled и fluid', async ({ page }) => {
        await expect(page.locator('[data-demo=template] .p-togglebutton strong')).toHaveCount(4);
        await expect(page.locator('[data-demo=sizes] .p-togglebutton-sm')).toHaveCount(2);
        await expect(page.locator('[data-demo=sizes] .p-togglebutton-lg')).toHaveCount(2);
        const invalid = page.locator('[data-demo=invalid] .p-selectbutton');
        await expect(invalid).toHaveClass(/p-invalid/);
        await invalid.locator('.p-togglebutton').first().click();
        await expect(invalid).not.toHaveClass(/p-invalid/);
        const disabled = page.locator('[data-demo=disabled] .p-selectbutton');
        await expect(disabled.first().locator('.p-disabled')).toHaveCount(2);
        await expect(disabled.nth(1).locator('.p-disabled')).toHaveCount(1);
        await disabled.nth(1).locator('.p-togglebutton').nth(1).click({ force: true });
        await expect(disabled.nth(1).locator('.p-togglebutton').nth(0)).toHaveAttribute('aria-pressed', 'true');
        await expect(page.locator('[data-demo=fluid] .p-selectbutton-fluid')).toHaveCount(1);
    });
});
