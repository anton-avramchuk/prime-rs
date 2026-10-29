import { expect, test } from '@playwright/test';

test.describe('Rating', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/rating');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-rating')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        await expect(page.locator('[data-demo=basic] .p-rating-option')).toHaveCount(5);
        await expect(page.locator('[data-demo=numberofstars] .p-rating-option')).toHaveCount(10);
        const size = await page.locator('[data-demo=basic] .p-rating-icon').first().evaluate((el) => getComputedStyle(el).width);
        expect(size).not.toBe('14px');
    });

    test('клик выбирает звезду, повторный клик сбрасывает', async ({ page }) => {
        const options = page.locator('[data-demo=basic] .p-rating-option');
        await options.nth(2).click();
        await expect(page.locator('[data-demo=basic] .p-rating-option-active')).toHaveCount(3);
        await expect(page.getByTestId('echo')).toHaveText('3');
        await expect(options.nth(2).locator('input')).toBeChecked();
        await options.nth(2).click();
        await expect(page.locator('[data-demo=basic] .p-rating-option-active')).toHaveCount(0);
        await expect(page.getByTestId('echo')).toHaveText('');
    });

    test('стрелки клавиатуры переключают звёзды', async ({ page }) => {
        const options = page.locator('[data-demo=basic] .p-rating-option');
        await options.nth(1).click();
        await expect(page.getByTestId('echo')).toHaveText('2');
        await page.keyboard.press('ArrowRight');
        await expect(page.getByTestId('echo')).toHaveText('3');
        await expect(options.nth(2).locator('input')).toBeFocused();
        await expect(options.nth(2)).toHaveClass(/p-focus-visible/);
        await page.keyboard.press('ArrowLeft');
        await page.keyboard.press('ArrowLeft');
        await expect(page.getByTestId('echo')).toHaveText('1');
        await expect(page.locator('[data-demo=basic] .p-rating-option-active')).toHaveCount(1);
    });

    test('disabled, readonly и шаблон иконок', async ({ page }) => {
        const disabled = page.locator('[data-demo=disabled] .p-rating');
        await expect(disabled).toHaveClass(/p-disabled/);
        await expect(disabled.locator('input').first()).toBeDisabled();
        await disabled.locator('.p-rating-option').nth(4).click({ force: true });
        await expect(disabled.locator('.p-rating-option-active')).toHaveCount(3);
        const readonly = page.locator('[data-demo=readonly] .p-rating');
        await expect(readonly).toHaveClass(/p-readonly/);
        await readonly.locator('.p-rating-option').nth(0).click();
        await expect(readonly.locator('.p-rating-option-active')).toHaveCount(5);
        const template = page.locator('[data-demo=template] .p-rating');
        await expect(template.locator('.p-rating-on-icon')).toHaveText(['♥', '♥', '♥']);
        await expect(template.locator('.p-rating-off-icon')).toHaveText(['♡', '♡']);
        await template.locator('.p-rating-option').nth(4).click();
        await expect(template.locator('.p-rating-on-icon')).toHaveCount(5);
    });
});
