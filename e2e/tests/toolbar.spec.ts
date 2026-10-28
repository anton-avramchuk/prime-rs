import { expect, test } from '@playwright/test';

test.describe('Toolbar', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/toolbar');
        await expect(page.locator('[data-demo=basic] .p-toolbar')).toBeVisible();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const toolbar = page.locator('[data-demo=basic] .p-toolbar');
        const display = await toolbar.evaluate((el) => getComputedStyle(el).display);
        expect(display).toBe('flex');
        const radius = await toolbar.evaluate((el) => getComputedStyle(el).borderRadius);
        expect(radius).not.toBe('0px');
    });

    test('basic: роль и три области по порядку', async ({ page }) => {
        const toolbar = page.locator('[data-demo=basic] .p-toolbar');
        await expect(toolbar).toHaveAttribute('role', 'toolbar');
        const areas = await toolbar.evaluate((el) => [...el.children].map((c) => c.className.split(' ')[0]));
        expect(areas).toEqual(['p-toolbar-start', 'p-toolbar-center', 'p-toolbar-end']);
        await expect(toolbar.locator('.p-toolbar-start button')).toHaveCount(3);
        await expect(toolbar.locator('.p-toolbar-center input')).toHaveAttribute('placeholder', 'Search');
        await expect(toolbar.locator('.p-toolbar-end')).toContainText('Save');
    });

    test('basic: область начала слева, конца справа', async ({ page }) => {
        const toolbar = page.locator('[data-demo=basic] .p-toolbar');
        const box = await toolbar.boundingBox();
        const start = await toolbar.locator('.p-toolbar-start').boundingBox();
        const end = await toolbar.locator('.p-toolbar-end').boundingBox();
        expect(start!.x).toBeLessThan(box!.x + box!.width / 3);
        expect(end!.x + end!.width).toBeGreaterThan(box!.x + (box!.width * 2) / 3);
    });

    test('custom: стиль корня и пустой центр', async ({ page }) => {
        const toolbar = page.locator('[data-demo=custom] .p-toolbar');
        const radius = await toolbar.evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).toBe('48px');
        await expect(toolbar.locator('.p-toolbar-center')).toHaveCount(0);
        await expect(toolbar.locator('.p-toolbar-start')).toContainText('Files');
        await expect(toolbar.locator('.p-toolbar-end')).toContainText('Share');
    });
});
