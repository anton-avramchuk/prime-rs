import { expect, test } from '@playwright/test';

test.describe('Textarea', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/textarea');
        await expect(page.locator('[data-demo=basic] textarea.p-textarea')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(7);
        const border = await page.locator('[data-demo=basic] textarea').evaluate((el) => getComputedStyle(el).borderTopWidth);
        expect(border).not.toBe('0px');
    });

    test('ввод попадает в сигнал и ставит p-filled', async ({ page }) => {
        const area = page.locator('[data-demo=basic] textarea');
        await expect(area).not.toHaveClass(/p-filled/);
        // до завершения гидратации событие может потеряться, поэтому ввод повторяется
        await expect(async () => {
            await area.fill(['hello', 'world'].join('\n'));
            await expect(page.getByTestId('echo')).toHaveText('11', { timeout: 500 });
        }).toPass({ timeout: 8000 });
        await expect(area).toHaveClass(/p-filled/);
    });

    test('auto_resize увеличивает высоту при вводе', async ({ page }) => {
        const area = page.locator('[data-demo=autoresize] textarea');
        await expect(area).toHaveClass(/p-textarea-resizable/);
        const before = (await area.boundingBox())!.height;
        await expect(async () => {
            await area.fill(Array.from({ length: 15 }, (_, i) => `line ${i}`).join('\n'));
            expect((await area.boundingBox())!.height).toBeGreaterThan(before + 50);
        }).toPass({ timeout: 8000 });
    });

    test('классы размера, варианта, invalid, fluid и disabled', async ({ page }) => {
        await expect(page.locator('[data-demo=sizes] .p-textarea-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-textarea-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=invalid] .p-invalid')).toHaveCount(1);
        await expect(page.locator('[data-demo=fluid] .p-textarea-fluid')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] textarea')).toBeDisabled();
    });
});
