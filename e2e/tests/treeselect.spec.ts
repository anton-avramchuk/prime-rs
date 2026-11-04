import { expect, test } from '@playwright/test';

test.describe('TreeSelect', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/treeselect');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_select')).toBeAttached();
    });

    const field = (page) => page.locator('#basic_select');
    const panel = (page) => field(page).locator('.p-treeselect-overlay');
    const node = (page, key: string) => panel(page).locator(`li[data-id="${key}"]`);

    test('стили темы: поле с рамкой, подсказка, кнопка со стрелкой', async ({ page }) => {
        const root = field(page);
        expect(await root.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        expect(await root.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        await expect(root.locator('.p-treeselect-label')).toHaveText('Select Item');
        await expect(root.locator('.p-treeselect-label')).toHaveClass(/p-placeholder/);
        await expect(root.locator('.p-treeselect-dropdown svg')).toHaveCount(1);
        await expect(panel(page)).toHaveCount(0);
    });

    test('клик открывает панель под полем, выбор закрывает и подставляет подпись', async ({ page }) => {
        await field(page).click();
        await expect(panel(page)).toBeVisible();
        const fb = (await field(page).boundingBox())!;
        const pb = (await panel(page).boundingBox())!;
        expect(pb.y).toBeGreaterThanOrEqual(fb.y + fb.height - 2);
        expect(await panel(page).evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        await node(page, '0').locator('> .p-tree-node-content .p-tree-node-toggle-button').click();
        await node(page, '0-1').locator('> .p-tree-node-content').click();
        await expect(panel(page)).toHaveCount(0);
        await expect(field(page).locator('.p-treeselect-label')).toHaveText('Home');
        await field(page).click();
        await expect(node(page, '0-1')).toBeVisible();
        await page.mouse.click(5, 5);
        await expect(panel(page)).toHaveCount(0);
    });

    test('клавиатура: ↓ открывает и фокусирует узел, Escape закрывает', async ({ page }) => {
        const input = field(page).locator('input');
        await input.focus();
        await page.keyboard.press('ArrowDown');
        await expect(panel(page)).toBeVisible();
        await expect(node(page, '0')).toBeFocused();
        await page.keyboard.press('ArrowRight');
        await expect(node(page, '0-0')).toBeFocused();
        await input.focus();
        await page.keyboard.press('Escape');
        await expect(panel(page)).toHaveCount(0);
    });

    test('множественный выбор с чипами и очисткой', async ({ page }) => {
        const multi = page.locator('[data-demo="multiple"] .p-treeselect');
        await multi.scrollIntoViewIfNeeded();
        await multi.click();
        const p = multi.locator('.p-treeselect-overlay');
        await p.locator('li[data-id="1"] > .p-tree-node-content').click();
        await p.locator('li[data-id="2"] > .p-tree-node-content').click();
        await expect(multi.locator('.p-treeselect-chip-item')).toHaveCount(2);
        await expect(p).toBeVisible();
    });

    test('флажки: отметка родителя отмечает поддерево', async ({ page }) => {
        const cb = page.locator('[data-demo="checkbox"] .p-treeselect');
        await cb.scrollIntoViewIfNeeded();
        await cb.click();
        const p = cb.locator('.p-treeselect-overlay');
        await p.locator('li[data-id="1"] > .p-tree-node-content').click();
        await expect(p.locator('li[data-id="1"]')).toHaveAttribute('aria-checked', 'true');
        await expect(cb.locator('.p-treeselect-label')).toContainText('barcelona.jpg');
    });

    test('значок очистки сбрасывает значение', async ({ page }) => {
        const clear = page.locator('[data-demo="clearicon"] .p-treeselect');
        await clear.scrollIntoViewIfNeeded();
        await expect(clear.locator('.p-treeselect-label')).toHaveText('Work');
        await clear.locator('.p-treeselect-clear-icon').click();
        await expect(clear.locator('.p-treeselect-label')).toHaveText('Select Item');
        await expect(clear.locator('.p-treeselect-overlay')).toHaveCount(0);
    });

    test('фильтр в панели сужает дерево', async ({ page }) => {
        const f = page.locator('[data-demo="filter"] .p-treeselect');
        await f.scrollIntoViewIfNeeded();
        await f.click();
        const input = f.locator('.p-tree-filter-input');
        await input.fill('scarface');
        await expect(f.locator('li[data-id="2-0-0"]')).toBeVisible();
        await expect(f.locator('li[data-id="0"]')).toHaveCount(0);
    });

    test('отключённое поле не открывается', async ({ page }) => {
        const d = page.locator('[data-demo="disabled"] .p-treeselect');
        await d.scrollIntoViewIfNeeded();
        await expect(d).toHaveClass(/p-disabled/);
        await d.click({ force: true });
        await expect(d.locator('.p-treeselect-overlay')).toHaveCount(0);
    });

    test('своё представление значения', async ({ page }) => {
        await expect(page.locator('[data-testid="value-view"]')).toHaveText('1 selected');
    });
});
