import { expect, test } from '@playwright/test';

test.describe('Message', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/message');
        await expect(page.locator('[data-demo=basic] .p-message')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(7);
        const radius = await page.locator('[data-demo=basic] .p-message').evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
    });

    test('role, aria-live и data-p', async ({ page }) => {
        const msg = page.locator('[data-demo=basic] .p-message');
        await expect(msg).toHaveAttribute('role', 'alert');
        await expect(msg).toHaveAttribute('aria-live', 'polite');
        await expect(msg).toHaveAttribute('data-p', 'info');
    });

    test('серьёзности и варианты', async ({ page }) => {
        for (const s of ['success', 'info', 'warn', 'error', 'secondary', 'contrast']) {
            await expect(page.locator(`[data-demo=severity] .p-message-${s}`)).toHaveCount(1);
            await expect(page.locator(`[data-demo=outlined] .p-message-${s}.p-message-outlined`)).toHaveCount(1);
            await expect(page.locator(`[data-demo=simple] .p-message-${s}.p-message-simple`)).toHaveCount(1);
        }
        await expect(page.locator('[data-demo=sizes] .p-message-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-message-lg')).toHaveCount(1);
    });

    test('кнопка закрытия скрывает сообщение', async ({ page }) => {
        const msg = page.locator('[data-demo=closable] .p-message');
        await expect(msg).toBeVisible();
        await msg.locator('.p-message-close-button').click();
        await expect(msg).toBeHidden();
        await expect(msg).toHaveClass(/p-message-leave-active/);
    });

    test('life скрывает сообщение само', async ({ page }) => {
        const demo = page.locator('[data-demo=life]');
        await expect(demo.locator('.p-message')).toHaveCount(0);
        await demo.locator('button').click();
        await expect(demo.locator('.p-message')).toBeVisible();
        await expect(demo.locator('.p-message')).toHaveCount(0, { timeout: 6000 });
    });
});
