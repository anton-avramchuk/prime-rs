import { expect, test } from '@playwright/test';

test.describe('Divider', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/divider');
        await expect(page.locator('[data-demo=basic] .p-divider')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        const border = await page.locator('[data-demo=basic] .p-divider').evaluate((el) => getComputedStyle(el, '::before').borderTopStyle);
        expect(['solid', 'dashed', 'dotted']).toContain(border);
    });

    test('role, aria-orientation и data-p', async ({ page }) => {
        const basic = page.locator('[data-demo=basic] .p-divider');
        await expect(basic).toHaveAttribute('role', 'separator');
        await expect(basic).toHaveAttribute('aria-orientation', 'horizontal');
        await expect(basic).toHaveAttribute('data-p', 'horizontal solid');
        const vertical = page.locator('[data-demo=vertical] .p-divider-vertical').first();
        await expect(vertical).toHaveAttribute('aria-orientation', 'vertical');
    });

    test('классы выравнивания и типа', async ({ page }) => {
        await expect(page.locator('[data-demo=content] .p-divider-left')).toHaveCount(1);
        await expect(page.locator('[data-demo=content] .p-divider-center.p-divider-dotted')).toHaveCount(1);
        await expect(page.locator('[data-demo=content] .p-divider-right.p-divider-dashed')).toHaveCount(1);
        await expect(page.locator('[data-demo=type] .p-divider-solid, [data-demo=type] .p-divider-dotted, [data-demo=type] .p-divider-dashed')).toHaveCount(3);
        await expect(page.locator('[data-demo=vertical] .p-divider-top')).toHaveCount(1);
    });

    test('содержимое внутри p-divider-content и стиль выравнивания', async ({ page }) => {
        const right = page.locator('[data-demo=content] .p-divider-right');
        await expect(right.locator('.p-divider-content')).toContainText('Right');
        expect(await right.evaluate((el) => getComputedStyle(el).justifyContent)).toBe('flex-end');
    });
});
