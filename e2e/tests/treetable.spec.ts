import { expect, test } from '@playwright/test';

test.describe('TreeTable', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 2000 });
        await page.goto('/treetable');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_treetable')).toBeAttached();
    });

    const names = (page, id: string) => page.locator(`#${id} tbody > tr > td:nth-child(1)`).allTextContents().then((t) => t.map((s) => s.trim()));

    test('theme styles: cell indents, expand button, rows in the table', async ({ page }) => {
        const th = page.locator('#basic_treetable thead th').first();
        expect(parseFloat(await th.evaluate((el) => getComputedStyle(el).paddingLeft))).toBeGreaterThan(8);
        const toggler = page.locator('#basic_treetable .p-treetable-toggler').first();
        const box = (await toggler.boundingBox())!;
        expect(box.width).toBeGreaterThan(16);
        expect(await toggler.evaluate((el) => getComputedStyle(el).display)).toBe('inline-flex');
        await expect(page.locator('#basic_treetable tbody tr')).toHaveCount(5);
    });

    test('expanding and collapsing nodes with an indent by level', async ({ page }) => {
        const view = page.locator('#basic_treetable');
        await view.locator('.p-treetable-toggler').nth(0).click();
        expect((await names(page, 'basic_treetable')).slice(0, 4)).toEqual(['Applications', 'React', 'editor.app', 'settings.app']);
        await view.locator('.p-treetable-toggler').nth(1).click();
        await expect(view.locator('tbody tr').nth(2)).toContainText('react.app');
        const level1 = await view.locator('.p-treetable-toggler').nth(1).evaluate((el) => (el as HTMLElement).style.marginInlineStart);
        const level2 = await view.locator('.p-treetable-toggler').nth(2).evaluate((el) => (el as HTMLElement).style.marginInlineStart);
        expect(level1).toBe('16px');
        expect(level2).toBe('32px');
        await view.locator('.p-treetable-toggler').nth(0).click();
        expect(await names(page, 'basic_treetable')).toEqual(['Applications', 'Cloud', 'Desktop', 'Documents', 'Downloads']);
    });

    test('controlled expansion: all and collapse all', async ({ page }) => {
        const view = page.locator('#controlled_treetable');
        await view.scrollIntoViewIfNeeded();
        expect(await names(page, 'controlled_treetable')).toHaveLength(8);
        await view.getByRole('button', { name: 'Expand all' }).click();
        expect(await names(page, 'controlled_treetable')).toHaveLength(14);
        await view.getByRole('button', { name: 'Collapse all' }).click();
        expect(await names(page, 'controlled_treetable')).toEqual(['Applications', 'Cloud', 'Desktop', 'Documents', 'Downloads']);
    });

    test('sorting applies at every level', async ({ page }) => {
        const view = page.locator('#sort_treetable');
        await view.scrollIntoViewIfNeeded();
        await view.locator('thead th').nth(0).click();
        await view.locator('thead th').nth(0).click();
        const roots = await names(page, 'sort_treetable');
        expect(roots).toEqual(['Downloads', 'Documents', 'Desktop', 'Cloud', 'Applications']);
        await expect(view.locator('thead th').nth(0)).toHaveAttribute('aria-sort', 'descending');
        await view.locator('.p-treetable-toggler').nth(4).click();
        expect((await names(page, 'sort_treetable')).slice(4)).toEqual(['Applications', 'settings.app', 'editor.app', 'React']);
    });

    test('paginator counts root nodes', async ({ page }) => {
        const view = page.locator('#paginator_treetable');
        await view.scrollIntoViewIfNeeded();
        await expect(view.locator('tbody tr')).toHaveCount(2);
        await view.locator('.p-paginator-next').click();
        expect(await names(page, 'paginator_treetable')).toEqual(['Desktop', 'Documents']);
        await view.locator('.p-paginator-last').click();
        expect(await names(page, 'paginator_treetable')).toEqual(['Downloads']);
    });

    test('node selection by click and with Ctrl', async ({ page }) => {
        const single = page.locator('#single_treetable');
        await single.scrollIntoViewIfNeeded();
        await single.locator('tbody tr').nth(1).click();
        await expect(single.locator('tbody tr').nth(1)).toHaveClass(/p-treetable-row-selected/);
        await expect(page.getByTestId('single-selection')).toHaveText('1');
        const multiple = page.locator('#multiple_treetable');
        await multiple.scrollIntoViewIfNeeded();
        await multiple.locator('tbody tr').nth(0).click();
        await multiple.locator('tbody tr').nth(2).click({ modifiers: ['Control'] });
        await expect(page.getByTestId('multiple-selection')).toHaveText('0, 2');
        // a click on the expand button does not select
        await multiple.locator('.p-treetable-toggler').nth(1).click();
        await expect(page.getByTestId('multiple-selection')).toHaveText('0, 2');
    });

    test('checkboxes: selection propagates to descendants and parents', async ({ page }) => {
        const view = page.locator('#checkbox_treetable');
        await view.scrollIntoViewIfNeeded();
        const boxes = view.locator('tbody .p-treetable-node-checkbox');
        await boxes.nth(2).click(); // react.app
        await expect(page.getByTestId('checkbox-selection')).toHaveText('0-0-0, 0-0');
        await expect(boxes.nth(1)).toHaveClass(/p-checkbox-checked/); // React: the only child is selected
        await expect(boxes.nth(0)).not.toHaveClass(/p-checkbox-checked/); // Applications: partial
        await boxes.nth(0).click(); // Applications
        const text = await page.getByTestId('checkbox-selection').textContent();
        expect(text!.split(', ').sort()).toEqual(['0', '0-0', '0-0-0', '0-1', '0-2'].sort());
        await boxes.nth(0).click();
        await expect(page.getByTestId('checkbox-selection')).toHaveText('');
    });

    test('grid lines, sizes and loading', async ({ page }) => {
        const looks = page.locator('#looks_treetable .p-treetable');
        await looks.first().scrollIntoViewIfNeeded();
        await expect(looks.nth(0)).toHaveClass(/p-treetable-gridlines/);
        await expect(looks.nth(1)).toHaveClass(/p-treetable-sm/);
        await expect(looks.nth(2)).toHaveClass(/p-treetable-lg/);
        const pad = async (i: number) => parseFloat(await looks.nth(i).locator('tbody td').first().evaluate((el) => getComputedStyle(el).paddingTop));
        expect(await pad(1)).toBeLessThan(await pad(0));
        expect(await pad(2)).toBeGreaterThan(await pad(0));
        const mask = page.locator('#loading_treetable .p-treetable-mask');
        await mask.scrollIntoViewIfNeeded();
        await expect(mask).toBeVisible();
    });
});
