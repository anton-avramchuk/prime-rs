import { expect, test } from '@playwright/test';

test.describe('Button', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/button');
        // ждём гидрацию: клик по Loading работает только после неё
        await expect(page.locator('[data-demo=loading] button')).toBeVisible();
        await page.waitForTimeout(500);
    });

    test('все демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(15);
        const bg = await page.locator('[data-demo=severity] button.p-button-danger').evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(bg).not.toBe('rgba(0, 0, 0, 0)');
        expect(bg).not.toBe('rgb(239, 239, 239)');
    });

    test('loading: кнопка блокируется на время загрузки', async ({ page }) => {
        const button = page.locator('[data-demo=loading] button');
        await expect(button).toBeEnabled();
        await button.click();
        await expect(button).toBeDisabled();
        await expect(button).toHaveClass(/p-button-loading/);
        await expect(button.locator('.p-button-loading-icon')).toHaveCount(1);
        await expect(button).toBeEnabled({ timeout: 4000 });
        await expect(button).not.toHaveClass(/p-button-loading/);
    });

    test('disabled: кнопка недоступна', async ({ page }) => {
        const button = page.locator('[data-demo=disabled] button');
        await expect(button).toBeDisabled();
        await expect(button).toHaveAttribute('data-p-disabled', 'true');
    });

    test('ripple: при нажатии активируется p-ink', async ({ page }) => {
        const button = page.locator('[data-demo=ripple] button');
        const ink = button.locator('.p-ink');
        await expect(ink).toHaveCount(1);
        await button.dispatchEvent('mousedown');
        await expect(ink).toHaveAttribute('data-p-ink-active', 'true');
        // без включённого ripple p-ink нет
        await expect(page.locator('[data-demo=basic] .p-ink')).toHaveCount(0);
    });

    test('иконки: позиция и режим «только иконка»', async ({ page }) => {
        await expect(page.locator('[data-demo=icons] .p-button-icon-right')).toHaveCount(1);
        await expect(page.locator('[data-demo=icons] .p-button-vertical')).toHaveCount(2);
        await expect(page.locator('[data-demo=iconsonly] .p-button-icon-only')).toHaveCount(9);
    });
});
