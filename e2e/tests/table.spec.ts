import { expect, test } from '@playwright/test';

test.describe('Table', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1800 });
        await page.goto('/table');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_table')).toBeAttached();
    });

    const names = (page, id: string) => page.locator(`#${id} tbody > tr:not(.p-datatable-row-expansion) > td:nth-child(2)`).allTextContents();
    const nameCol = (page, id: string, offset = 2) => page.locator(`#${id} tbody > tr:not(.p-datatable-row-expansion) > td:nth-child(${offset})`).allTextContents();

    test('стили темы: рамка заголовка, ячейки с отступами, строки в таблице', async ({ page }) => {
        const table = page.locator('#basic_table .p-datatable-table');
        expect(await table.evaluate((el) => getComputedStyle(el).borderSpacing)).toBe('0px');
        expect(await table.evaluate((el) => getComputedStyle(el).width)).not.toBe('0px');
        const th = page.locator('#basic_table thead th').first();
        expect(parseFloat(await th.evaluate((el) => getComputedStyle(el).paddingLeft))).toBeGreaterThan(8);
        expect(await th.evaluate((el) => getComputedStyle(el).fontWeight)).toBe('600');
        await expect(page.locator('#basic_table tbody tr')).toHaveCount(5);
        const a = (await page.locator('#basic_table tbody tr').nth(0).boundingBox())!;
        const b = (await page.locator('#basic_table tbody tr').nth(1).boundingBox())!;
        expect(b.y).toBeGreaterThan(a.y);
    });

    test('сортировка по одному столбцу', async ({ page }) => {
        const header = page.locator('#sort_table thead th').nth(1);
        await header.click();
        expect(await names(page, 'sort_table')).toEqual(['Bamboo Watch', 'Black Watch', 'Blue Band', 'Blue T-Shirt', 'Bracelet']);
        await expect(header).toHaveAttribute('aria-sort', 'ascending');
        await expect(header).toHaveClass(/p-datatable-column-sorted/);
        await header.click();
        expect((await names(page, 'sort_table'))[0]).toBe('Bracelet');
        await expect(header).toHaveAttribute('aria-sort', 'descending');
        // сортировка клавишей
        const quantity = page.locator('#sort_table thead th').nth(3);
        await quantity.focus();
        await page.keyboard.press('Enter');
        expect(await nameCol(page, 'sort_table', 4)).toEqual(['2', '24', '25', '61', '73']);
        await expect(header).toHaveAttribute('aria-sort', 'none');
    });

    test('многоуровневая сортировка: Ctrl-клик добавляет столбец и показывает счётчик', async ({ page }) => {
        const category = page.locator('#multisort_table thead th').nth(2);
        const quantity = page.locator('#multisort_table thead th').nth(3);
        await category.click();
        await quantity.click({ modifiers: ['Control'] });
        const cats = await nameCol(page, 'multisort_table', 3);
        expect(cats.slice(0, 2)).toEqual(['Accessories', 'Accessories']);
        const qty = await nameCol(page, 'multisort_table', 4);
        expect(qty.slice(0, 6)).toEqual(['0', '5', '23', '24', '61', '73']);
        await expect(page.locator('#multisort_table .p-sortable-column-badge')).toHaveCount(2);
    });

    test('пагинатор и возврат на первую страницу при сортировке', async ({ page }) => {
        const view = page.locator('#paginator_table');
        await view.scrollIntoViewIfNeeded();
        await expect(view.locator('tbody tr')).toHaveCount(4);
        await expect(view.locator('.p-datatable-paginator-bottom')).toBeVisible();
        await expect(view.locator('.p-paginator-current')).toContainText('Showing 1 to 4 of 10');
        await view.locator('.p-paginator-next').click();
        await expect(view.locator('tbody tr').first()).toContainText('h456wer53');
        await view.locator('thead th').nth(1).click();
        await expect(view.locator('tbody tr').first()).toContainText('Bamboo Watch');
        await view.locator('.p-paginator-last').click();
        await expect(view.locator('tbody tr')).toHaveCount(2);
    });

    test('выбор одной строки и нескольких с Ctrl', async ({ page }) => {
        const single = page.locator('#single_table');
        await single.scrollIntoViewIfNeeded();
        await single.locator('tbody tr').nth(1).click();
        await expect(single.locator('tbody tr').nth(1)).toHaveClass(/p-datatable-row-selected/);
        await expect(page.getByTestId('single-selection')).toHaveText('nvklal433');
        await single.locator('tbody tr').nth(2).click();
        await expect(page.getByTestId('single-selection')).toHaveText('zz21cz3c1');
        await single.locator('tbody tr').nth(2).click();
        await expect(page.getByTestId('single-selection')).toHaveText('');

        const multiple = page.locator('#multiple_table');
        await multiple.scrollIntoViewIfNeeded();
        await multiple.locator('tbody tr').nth(0).click();
        await multiple.locator('tbody tr').nth(2).click();
        await expect(page.getByTestId('multiple-selection')).toHaveText('zz21cz3c1');
        await multiple.locator('tbody tr').nth(0).click({ modifiers: ['Control'] });
        await expect(page.getByTestId('multiple-selection')).toHaveText('zz21cz3c1, f230fh0g3');
    });

    test('флажки: строка и «выбрать все» в заголовке', async ({ page }) => {
        const view = page.locator('#checkbox_table');
        await view.scrollIntoViewIfNeeded();
        await view.locator('tbody .p-checkbox').nth(1).click();
        await view.locator('tbody .p-checkbox').nth(3).click();
        await expect(page.getByTestId('checkbox-selection')).toHaveText('2');
        await view.locator('thead .p-checkbox').click();
        await expect(page.getByTestId('checkbox-selection')).toHaveText('5');
        await expect(view.locator('thead .p-checkbox')).toHaveClass(/p-checkbox-checked/);
        await view.locator('thead .p-checkbox').click();
        await expect(page.getByTestId('checkbox-selection')).toHaveText('0');
    });

    test('радиокнопки выбирают одну строку', async ({ page }) => {
        const view = page.locator('#radio_table');
        await view.scrollIntoViewIfNeeded();
        await view.locator('tbody .p-radiobutton').nth(1).click();
        await expect(page.getByTestId('radio-selection')).toHaveText('nvklal433');
        await view.locator('tbody .p-radiobutton').nth(3).click();
        await expect(page.getByTestId('radio-selection')).toHaveText('244wgerg2');
        await expect(view.locator('.p-radiobutton-checked')).toHaveCount(1);
    });

    test('раскрытие строк', async ({ page }) => {
        const view = page.locator('#expansion_table');
        await view.scrollIntoViewIfNeeded();
        await view.locator('.p-datatable-row-toggle-button').nth(0).click();
        await expect(view.locator('.p-datatable-row-expansion')).toHaveCount(1);
        await expect(view.locator('.expansion-body')).toHaveText('Orders for Bamboo Watch');
        await view.locator('.p-datatable-row-toggle-button').nth(2).click();
        await expect(view.locator('.p-datatable-row-expansion')).toHaveCount(2);
        await view.locator('.p-datatable-row-toggle-button').nth(0).click();
        await expect(view.locator('.p-datatable-row-expansion')).toHaveCount(1);
    });

    test('полосатость, сетка и размеры', async ({ page }) => {
        const tables = page.locator('#looks_table .p-datatable');
        await tables.first().scrollIntoViewIfNeeded();
        await expect(tables.nth(0)).toHaveClass(/p-datatable-striped/);
        await expect(tables.nth(1)).toHaveClass(/p-datatable-gridlines/);
        await expect(tables.nth(2)).toHaveClass(/p-datatable-sm/);
        await expect(tables.nth(3)).toHaveClass(/p-datatable-lg/);
        const padding = async (i: number) => parseFloat(await tables.nth(i).locator('tbody td').first().evaluate((el) => getComputedStyle(el).paddingTop));
        expect(await padding(2)).toBeLessThan(await padding(0));
        expect(await padding(3)).toBeGreaterThan(await padding(0));
        const odd = tables.nth(0).locator('tbody tr').nth(1);
        const even = tables.nth(0).locator('tbody tr').nth(0); // полосатыми считаются нечётные по счёту (1-я, 3-я)
        expect(await odd.evaluate((el) => getComputedStyle(el).backgroundColor)).not.toBe(await even.evaluate((el) => getComputedStyle(el).backgroundColor));
    });

    test('вертикальная прокрутка и загрузка', async ({ page }) => {
        const scroll = page.locator('#scroll_table .p-datatable-table-container');
        await scroll.scrollIntoViewIfNeeded();
        const box = (await scroll.boundingBox())!;
        expect(Math.round(box.height)).toBeLessThanOrEqual(222);
        expect(await scroll.evaluate((el) => el.scrollHeight > el.clientHeight)).toBe(true);
        const mask = page.locator('#loading_table .p-datatable-mask');
        await mask.scrollIntoViewIfNeeded();
        await expect(mask).toBeVisible();
    });
});
