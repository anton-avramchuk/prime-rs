import { expect, test } from '@playwright/test';

test.describe('PickList', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1400 });
        await page.goto('/picklist');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_picklist')).toBeAttached();
    });

    const root = (page, id: string) => page.locator(`#${id} .p-picklist`);
    const options = (page, id: string, side: 'source' | 'target') =>
        page.locator(`#${id} .p-picklist-${side}-list-container .p-listbox-option`).allTextContents().then((t) => t.map((s) => s.trim()));

    test('стили темы: пять блоков в ряд, кнопки столбцом', async ({ page }) => {
        const picklist = root(page, 'basic_picklist');
        expect(await picklist.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        const controls = picklist.locator('.p-picklist-transfer-controls');
        expect(await controls.evaluate((el) => getComputedStyle(el).flexDirection)).toBe('column');
        const src = (await picklist.locator('.p-picklist-source-list-container').boundingBox())!;
        const tgt = (await picklist.locator('.p-picklist-target-list-container').boundingBox())!;
        const ctl = (await controls.boundingBox())!;
        expect(src.x).toBeLessThan(ctl.x);
        expect(ctl.x).toBeLessThan(tgt.x);
        expect(Math.abs(src.width - tgt.width)).toBeLessThan(2);
        await expect(picklist.locator('.p-picklist-source-list-container')).toContainText('Available');
        await expect(picklist.locator('.p-picklist-target-list-container')).toContainText('Selected');
    });

    test('перенос выбранных и всех', async ({ page }) => {
        const picklist = root(page, 'basic_picklist');
        await picklist.locator('.p-picklist-source-list-container .p-listbox-option', { hasText: 'Blue Band' }).click();
        await picklist.locator('.p-picklist-source-list-container .p-listbox-option', { hasText: 'Bracelet' }).first().click();
        await picklist.getByLabel('Move to Target', { exact: true }).click();
        expect(await options(page, 'basic_picklist', 'target')).toEqual(['Blue Band', 'Bracelet']);
        expect(await options(page, 'basic_picklist', 'source')).toHaveLength(5);
        await expect(page.getByTestId('basic-target')).toHaveText('Blue Band, Bracelet');
        await picklist.getByLabel('Move All to Target').click();
        expect(await options(page, 'basic_picklist', 'source')).toHaveLength(0);
        expect(await options(page, 'basic_picklist', 'target')).toHaveLength(7);
        await picklist.getByLabel('Move All to Source').click();
        expect(await options(page, 'basic_picklist', 'target')).toHaveLength(0);
    });

    test('перестановка в цели и двойной клик', async ({ page }) => {
        const picklist = root(page, 'basic_picklist');
        await picklist.getByLabel('Move All to Target').click();
        await picklist.locator('.p-picklist-target-list-container .p-listbox-option', { hasText: 'Bracelet' }).first().click();
        await picklist.locator('.p-picklist-target-controls').getByLabel('Move Top').click();
        expect((await options(page, 'basic_picklist', 'target'))[0]).toBe('Bracelet');
        await picklist.locator('.p-picklist-target-controls').getByLabel('Move Bottom').click();
        expect((await options(page, 'basic_picklist', 'target')).at(-1)).toBe('Bracelet');
        // двойной клик по элементу цели возвращает выбранные
        await picklist.locator('.p-picklist-target-list-container .p-listbox-option', { hasText: 'Bracelet' }).first().dblclick();
        expect(await options(page, 'basic_picklist', 'source')).toEqual(['Bracelet']);
    });

    test('поиск и перенос только найденных', async ({ page }) => {
        const picklist = root(page, 'filter_picklist');
        await picklist.locator('.p-picklist-source-list-container input.p-listbox-filter').fill('Watch');
        await expect(picklist.locator('.p-picklist-source-list-container .p-listbox-option')).toHaveCount(2);
        await picklist.getByLabel('Move All to Target').click();
        expect(await options(page, 'filter_picklist', 'target')).toEqual(['Bamboo Watch', 'Black Watch']);
        await picklist.locator('.p-picklist-source-list-container input.p-listbox-filter').fill('');
        expect(await options(page, 'filter_picklist', 'source')).toHaveLength(5);
    });
});
