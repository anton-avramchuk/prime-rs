import { expect, test } from '@playwright/test';

test.describe('OrderList', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1200 });
        await page.goto('/orderlist');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_orderlist')).toBeAttached();
    });

    const root = (page, id: string) => page.locator(`#${id} .p-orderlist`);
    const labels = (page, id: string) => page.locator(`#${id} .p-listbox-option`).allTextContents().then((t) => t.map((s) => s.trim()));

    test('стили темы: кнопки слева столбцом, список справа', async ({ page }) => {
        const orderlist = root(page, 'basic_orderlist');
        await expect(orderlist).toHaveClass(/p-orderlist-controls-left/);
        expect(await orderlist.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        const controls = orderlist.locator('.p-orderlist-controls');
        expect(await controls.evaluate((el) => getComputedStyle(el).flexDirection)).toBe('column');
        const buttons = controls.locator('.p-button');
        await expect(buttons).toHaveCount(4);
        for (let i = 0; i < 4; i++) await expect(buttons.nth(i)).toBeDisabled();
        const c = (await controls.boundingBox())!;
        const l = (await orderlist.locator('.p-listbox').boundingBox())!;
        expect(c.x).toBeLessThan(l.x);
    });

    test('кнопки двигают выбранный элемент', async ({ page }) => {
        const orderlist = root(page, 'basic_orderlist');
        await orderlist.locator('.p-listbox-option', { hasText: 'Bracelet' }).first().click();
        const buttons = orderlist.locator('.p-orderlist-controls .p-button');
        await expect(buttons.first()).toBeEnabled();
        await orderlist.locator('.p-orderlist-move-up-button').click();
        expect((await labels(page, 'basic_orderlist')).slice(2, 5)).toEqual(["Blue Band", "Bracelet", "Blue T-Shirt"]);
        await orderlist.locator('.p-orderlist-move-top-button').click();
        expect((await labels(page, 'basic_orderlist'))[0]).toBe('Bracelet');
        await orderlist.locator('.p-orderlist-move-bottom-button').click();
        const all = await labels(page, 'basic_orderlist');
        expect(all[all.length - 1]).toBe('Bracelet');
        await orderlist.locator('.p-orderlist-move-down-button').click();
        expect((await labels(page, 'basic_orderlist')).at(-1)).toBe('Bracelet');
        await expect(page.getByTestId('basic-order')).toContainText('Galaxy Earrings, Bracelet');
    });

    test('несколько выбранных: Ctrl-клик и перемещение вверх', async ({ page }) => {
        const orderlist = root(page, 'basic_orderlist');
        await orderlist.locator('.p-listbox-option', { hasText: 'Blue Band' }).click();
        await orderlist.locator('.p-listbox-option', { hasText: 'Brown Purse' }).click();
        await orderlist.locator('.p-orderlist-move-up-button').click();
        const all = await labels(page, 'basic_orderlist');
        expect(all.indexOf('Blue Band')).toBe(1);
        expect(all.indexOf('Brown Purse')).toBe(4);
    });

    test('фильтр: двигаем элемент, найденный поиском', async ({ page }) => {
        const orderlist = root(page, 'filter_orderlist');
        await orderlist.locator('input.p-listbox-filter').fill('Galaxy');
        await expect(orderlist.locator('.p-listbox-option')).toHaveCount(1);
        await orderlist.locator('.p-listbox-option').click();
        await orderlist.locator('.p-orderlist-move-top-button').click();
        await orderlist.locator('input.p-listbox-filter').fill('');
        expect((await labels(page, 'filter_orderlist'))[0]).toBe('Galaxy Earrings');
    });

    test('кнопки справа', async ({ page }) => {
        const orderlist = root(page, 'controls_orderlist');
        await expect(orderlist).toHaveClass(/p-orderlist-controls-right/);
        const c = (await orderlist.locator('.p-orderlist-controls').boundingBox())!;
        const l = (await orderlist.locator('.p-listbox').boundingBox())!;
        expect(c.x).toBeGreaterThan(l.x);
    });
});
