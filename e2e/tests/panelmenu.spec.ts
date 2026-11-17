import { expect, test } from '@playwright/test';

test.describe('PanelMenu', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/panelmenu');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_menu')).toBeAttached();
    });

    const header = (page, menu: string, i: number) => page.locator(`#${menu}_${i}_header`);
    const region = (page, menu: string, i: number) => page.locator(`#${menu}_${i}_content`);
    const item = (page, menu: string, key: string) => page.locator(`li[id="${menu}_${key}"]`);
    const content = (page, menu: string, key: string) => item(page, menu, key).locator('> .p-panelmenu-item-content');

    test('theme styles: panels in a column with a border, a collapsed group has a right chevron, content hidden', async ({ page }) => {
        const root = page.locator('#basic_menu');
        expect(await root.evaluate((el) => getComputedStyle(el).flexDirection)).toBe('column');
        const panel = page.locator('#basic_menu .p-panelmenu-panel').first();
        expect(await panel.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        const first = (await header(page, 'basic_menu', 0).boundingBox())!;
        const second = (await header(page, 'basic_menu', 1).boundingBox())!;
        expect(second.y).toBeGreaterThan(first.y + first.height - 2);
        await expect(region(page, 'basic_menu', 0)).toBeHidden();
        await expect(header(page, 'basic_menu', 0)).toHaveAttribute('aria-expanded', 'false');
        await expect(header(page, 'basic_menu', 0).locator('.p-panelmenu-submenu-icon')).toHaveCount(1);
    });

    test('a click expands a panel, in single mode closes the previous one', async ({ page }) => {
        await header(page, 'basic_menu', 0).click();
        await expect(region(page, 'basic_menu', 0)).toBeVisible();
        await expect(header(page, 'basic_menu', 0)).toHaveClass(/p-panelmenu-header-active/);
        await expect(item(page, 'basic_menu', '0_0')).toBeVisible();
        await header(page, 'basic_menu', 1).click();
        await expect(region(page, 'basic_menu', 1)).toBeVisible();
        await expect(region(page, 'basic_menu', 0)).toBeHidden();
        await header(page, 'basic_menu', 1).click();
        await expect(region(page, 'basic_menu', 1)).toBeHidden();
    });

    test('nested branches expand with an indent to the right', async ({ page }) => {
        await header(page, 'basic_menu', 0).click();
        await content(page, 'basic_menu', '0_0').click();
        const parent = (await content(page, 'basic_menu', '0_0').boundingBox())!;
        const child = item(page, 'basic_menu', '0_0_0').locator('> .p-panelmenu-item-content');
        await expect(child).toBeVisible();
        const box = (await child.boundingBox())!;
        expect(box.y).toBeGreaterThan(parent.y);
        const label = (await item(page, 'basic_menu', '0_0_0').locator('.p-panelmenu-item-label').first().boundingBox())!;
        const parentLabel = (await item(page, 'basic_menu', '0_0').locator('.p-panelmenu-item-label').first().boundingBox())!;
        expect(label.x).toBeGreaterThan(parentLabel.x + 8);
    });

    test('multiple mode keeps several panels open', async ({ page }) => {
        await header(page, 'multiple_menu', 0).scrollIntoViewIfNeeded();
        await header(page, 'multiple_menu', 0).click();
        await header(page, 'multiple_menu', 1).click();
        await expect(region(page, 'multiple_menu', 0)).toBeVisible();
        await expect(region(page, 'multiple_menu', 1)).toBeVisible();
    });

    test('Toggle All expands and collapses everything', async ({ page }) => {
        const button = page.getByRole('button', { name: 'Toggle All' });
        await button.scrollIntoViewIfNeeded();
        await button.click();
        for (const i of [0, 1, 2]) await expect(region(page, 'controlled_menu', i)).toBeVisible();
        await expect(item(page, 'controlled_menu', '0_0')).toHaveAttribute('aria-expanded', 'true');
        await button.click();
        for (const i of [0, 1, 2]) await expect(region(page, 'controlled_menu', i)).toBeHidden();
    });

    test('keyboard: ↓ from a header into the tree, → expands, ← collapses, ↓ after the last — to the next header', async ({ page }) => {
        await header(page, 'basic_menu', 0).click();
        await expect(region(page, 'basic_menu', 0)).toBeVisible();
        await header(page, 'basic_menu', 0).focus();
        await page.keyboard.press('ArrowDown');
        await expect(page.locator('#basic_menu_0_list')).toBeFocused();
        await expect(item(page, 'basic_menu', '0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(item(page, 'basic_menu', '0_0')).toHaveAttribute('aria-expanded', 'true');
        await page.keyboard.press('ArrowDown');
        await expect(item(page, 'basic_menu', '0_0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowLeft');
        await expect(item(page, 'basic_menu', '0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowLeft');
        await expect(item(page, 'basic_menu', '0_0')).toHaveAttribute('aria-expanded', 'false');
        await page.keyboard.press('End');
        await expect(item(page, 'basic_menu', '0_1')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowDown');
        await expect(header(page, 'basic_menu', 1)).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(region(page, 'basic_menu', 1)).toBeVisible();
        await expect(region(page, 'basic_menu', 0)).toBeHidden();
    });

    test('a leaf command triggers a notification', async ({ page }) => {
        await header(page, 'command_menu', 0).scrollIntoViewIfNeeded();
        await header(page, 'command_menu', 0).click();
        await content(page, 'command_menu', '0_0').click();
        await expect(page.locator('.p-toast-message').first()).toContainText('File created');
    });

    test('the template replaces the item link', async ({ page }) => {
        await header(page, 'template_menu', 0).scrollIntoViewIfNeeded();
        await header(page, 'template_menu', 0).click();
        await expect(page.locator('#template_menu [data-testid="template-item"]').first()).toBeVisible();
        await expect(page.locator('#template_menu .p-panelmenu-item-link')).toHaveCount(0);
    });
});
