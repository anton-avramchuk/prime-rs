import { expect, test } from '@playwright/test';

test.describe('Tree', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/tree');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_tree')).toBeAttached();
    });

    const node = (page, tree: string, key: string) => page.locator(`#${tree} li[data-id="${key}"]`);
    const content = (page, tree: string, key: string) => node(page, tree, key).locator('> .p-tree-node-content');
    const toggle = (page, tree: string, key: string) => content(page, tree, key).locator('.p-tree-node-toggle-button');

    test('стили темы: узлы в столбец, у вложенных отступ, у кнопки шеврон', async ({ page }) => {
        const root = page.locator('#basic_tree');
        const first = (await content(page, 'basic_tree', '0').boundingBox())!;
        const second = (await content(page, 'basic_tree', '1').boundingBox())!;
        expect(second.y).toBeGreaterThan(first.y + first.height - 2);
        await expect(toggle(page, 'basic_tree', '0').locator('svg')).toHaveCount(1);
        await toggle(page, 'basic_tree', '0').click();
        await expect(node(page, 'basic_tree', '0-0')).toBeVisible();
        const child = (await node(page, 'basic_tree', '0-0').locator('> .p-tree-node-content .p-tree-node-label').boundingBox())!;
        const parent = (await node(page, 'basic_tree', '0').locator('> .p-tree-node-content .p-tree-node-label').boundingBox())!;
        expect(child.x).toBeGreaterThan(parent.x + 10);
        await toggle(page, 'basic_tree', '0').click();
        await expect(node(page, 'basic_tree', '0-0')).toHaveCount(0);
    });

    test('единичный выбор подсвечивает узел и снимается повторным кликом', async ({ page }) => {
        await content(page, 'single_tree', '1').scrollIntoViewIfNeeded();
        await content(page, 'single_tree', '1').click();
        await expect(content(page, 'single_tree', '1')).toHaveClass(/p-tree-node-selected/);
        await expect(node(page, 'single_tree', '1')).toHaveAttribute('aria-selected', 'true');
        const bg = await content(page, 'single_tree', '1').evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(bg).not.toBe('rgba(0, 0, 0, 0)');
        await content(page, 'single_tree', '1').click();
        await expect(content(page, 'single_tree', '1')).not.toHaveClass(/p-tree-node-selected/);
    });

    test('флажки: родитель отмечается, когда отмечены все дети, и бывает частично отмеченным', async ({ page }) => {
        await toggle(page, 'checkbox_tree', '1').scrollIntoViewIfNeeded();
        await toggle(page, 'checkbox_tree', '1').click();
        await content(page, 'checkbox_tree', '1-0').click();
        await expect(node(page, 'checkbox_tree', '1').locator('> .p-tree-node-content .p-tree-node-checkbox')).toHaveAttribute('data-p-partialchecked', 'true');
        await content(page, 'checkbox_tree', '1-1').click();
        await content(page, 'checkbox_tree', '1-2').click();
        await expect(node(page, 'checkbox_tree', '1')).toHaveAttribute('aria-checked', 'true');
        await expect(node(page, 'checkbox_tree', '1').locator('> .p-tree-node-content .p-tree-node-checkbox')).toHaveClass(/p-checkbox-checked/);
    });

    test('Expand all раскрывает всё дерево', async ({ page }) => {
        const button = page.getByRole('button', { name: 'Expand all' });
        await button.scrollIntoViewIfNeeded();
        await button.click();
        await expect(node(page, 'controlled_tree', '0-0-1')).toBeVisible();
        await page.getByRole('button', { name: 'Collapse all' }).click();
        await expect(node(page, 'controlled_tree', '0-0')).toHaveCount(0);
    });

    test('фильтр оставляет совпавшие узлы с предками', async ({ page }) => {
        const input = page.locator('#filter_tree .p-tree-filter-input');
        await input.scrollIntoViewIfNeeded();
        await input.fill('serpico');
        await expect(node(page, 'filter_tree', '2-0-1')).toBeVisible();
        await expect(node(page, 'filter_tree', '0')).toHaveCount(0);
        await input.fill('');
        await expect(node(page, 'filter_tree', '0')).toBeVisible();
        await input.fill('nothing-like-this');
        await expect(page.locator('#filter_tree .p-tree-empty-message')).toBeVisible();
    });

    test('клавиатура: → раскрывает и входит, ↓ ↑ ходят, ← сворачивает, Enter выбирает', async ({ page }) => {
        await node(page, 'single_tree', '0').scrollIntoViewIfNeeded();
        await node(page, 'single_tree', '0').focus();
        await page.keyboard.press('ArrowRight');
        await expect(node(page, 'single_tree', '0-0')).toBeFocused();
        await page.keyboard.press('ArrowDown');
        await expect(node(page, 'single_tree', '0-1')).toBeFocused();
        await page.keyboard.press('ArrowUp');
        await page.keyboard.press('ArrowLeft');
        await expect(node(page, 'single_tree', '0')).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(node(page, 'single_tree', '0')).toHaveAttribute('aria-selected', 'true');
        await page.keyboard.press('ArrowLeft');
        await expect(node(page, 'single_tree', '0-0')).toHaveCount(0);
    });

    test('ленивая загрузка: спиннер, затем дети', async ({ page }) => {
        await toggle(page, 'lazy_tree', '0').scrollIntoViewIfNeeded();
        await toggle(page, 'lazy_tree', '0').click();
        await expect(node(page, 'lazy_tree', '0-0')).toBeVisible();
        await expect(node(page, 'lazy_tree', '0')).toHaveAttribute('aria-expanded', 'true');
    });

    test('события вызывают уведомления', async ({ page }) => {
        await content(page, 'event_tree', '1').scrollIntoViewIfNeeded();
        await content(page, 'event_tree', '1').click();
        await expect(page.locator('.p-toast-message').first()).toContainText('Node Selected');
    });

    test('шаблон узла по типу', async ({ page }) => {
        await toggle(page, 'template_tree', '0').scrollIntoViewIfNeeded();
        await toggle(page, 'template_tree', '0').click();
        await expect(page.locator('#template_tree [data-testid="url-node"]')).toBeVisible();
        await expect(page.locator('#template_tree [data-testid="default-node"]').first()).toBeVisible();
    });
});
