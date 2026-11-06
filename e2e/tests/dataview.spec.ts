import { expect, test } from '@playwright/test';

test.describe('DataView', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1600 });
        await page.goto('/dataview');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_dataview')).toBeAttached();
    });

        test('стили темы: корень блочный, позиционирован, содержимое с отступами', async ({ page }) => {
        const root = page.locator('#basic_dataview .p-dataview');
        await expect(root).toHaveClass(/p-dataview-list/);
        expect(await root.evaluate((el) => getComputedStyle(el).position)).toBe('relative');
        expect(await page.locator('#basic_dataview .p-dataview-content').evaluate((el) => getComputedStyle(el).display)).toBe('block');
        await expect(page.locator('#basic_dataview .product-row')).toHaveCount(5);
        await expect(page.locator('#basic_dataview .p-paginator')).toHaveCount(0);
    });

    test('переключение между списком и сеткой', async ({ page }) => {
        const root = page.locator('#layout_dataview .p-dataview');
        await expect(root).toHaveClass(/p-dataview-list/);
        await expect(page.locator('#layout_dataview .product-row')).toHaveCount(6);
        await page.locator('#layout_dataview').getByRole('button', { name: 'Grid' }).click();
        await expect(root).toHaveClass(/p-dataview-grid/);
        await expect(page.locator('#layout_dataview .product-card')).toHaveCount(6);
        await expect(page.locator('#layout_dataview .product-row')).toHaveCount(0);
        const grid = page.locator('#layout_dataview .products-grid');
        expect(await grid.evaluate((el) => getComputedStyle(el).display)).toBe('grid');
        await page.locator('#layout_dataview').getByRole('button', { name: 'List' }).click();
        await expect(page.locator('#layout_dataview .product-row')).toHaveCount(6);
    });

    test('пагинатор: страницы по пять', async ({ page }) => {
        const view = page.locator('#pagination_dataview');
        await view.scrollIntoViewIfNeeded();
        await expect(view.locator('.product-row')).toHaveCount(5);
        await expect(view.locator('.p-dataview-paginator-bottom')).toBeVisible();
        await view.locator('.p-paginator-next').click();
        await expect(view.locator('.product-row').first()).toContainText('Brown Purse');
        await view.locator('.p-paginator-last').click();
        await expect(view.locator('.product-row')).toHaveCount(2);
        await view.locator('.p-paginator-first').click();
        await expect(view.locator('.product-row').first()).toContainText('Bamboo Watch');
    });

    test('сортировка по цене возвращает на первую страницу', async ({ page }) => {
        const view = page.locator('#sorting_dataview');
        await view.scrollIntoViewIfNeeded();
        await view.locator('.p-paginator-next').click();
        await view.getByRole('button', { name: 'Price low to high' }).click();
        await expect(view.locator('.product-row').first()).toContainText('Bracelet');
        await expect(view.locator('.product-price').first()).toHaveText('$15');
        await view.getByRole('button', { name: 'Price high to low' }).click();
        await expect(view.locator('.product-row').first()).toContainText('Gaming Set');
        await expect(view.locator('.product-price').first()).toHaveText('$299');
    });

    test('загрузка: маска и значок, нет сообщения о пустых данных', async ({ page }) => {
        const view = page.locator('#loading_dataview');
        await view.scrollIntoViewIfNeeded();
        await expect(view.locator('.p-dataview-loading-overlay')).toBeVisible();
        expect(await view.locator('.p-dataview-loading-overlay').evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        await expect(view.locator('.p-dataview-empty-message')).toHaveCount(0);
    });
});
