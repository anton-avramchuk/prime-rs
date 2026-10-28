import { expect, test } from '@playwright/test';

test.describe('Tag', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/tag');
        await expect(page.locator('[data-demo=basic] .p-tag')).toBeVisible();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        const bg = await page.locator('[data-demo=severity] .p-tag-danger').evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(bg).not.toBe('rgba(0, 0, 0, 0)');
    });

    test('классы серьёзности и скругления', async ({ page }) => {
        for (const sev of ['secondary', 'success', 'info', 'warn', 'danger', 'contrast']) {
            await expect(page.locator(`[data-demo=severity] .p-tag-${sev}`)).toHaveCount(1);
        }
        await expect(page.locator('[data-demo=pill] .p-tag-rounded')).toHaveCount(7);
        await expect(page.locator('[data-demo=severity] .p-tag-rounded')).toHaveCount(0);
    });

    test('порядок содержимого: иконка перед меткой', async ({ page }) => {
        const first = page.locator('[data-demo=icon] .p-tag').first();
        await expect(first.locator('.p-tag-icon')).toHaveCount(1);
        const order = await first.evaluate((el) => [...el.children].map((c) => c.className.split(' ')[0]));
        expect(order).toEqual(['p-tag-icon', 'p-tag-label']);
        await expect(page.locator('[data-demo=basic] .p-tag-icon')).toHaveCount(0);
    });

    test('template: содержимое и пользовательский стиль', async ({ page }) => {
        const tag = page.locator('[data-demo=template] .p-tag');
        await expect(tag).toContainText('Italy');
        expect(await tag.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('2px');
    });
});
