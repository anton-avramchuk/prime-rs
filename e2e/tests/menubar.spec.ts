import { expect, test } from '@playwright/test';

test.describe('Menubar', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 900 });
        await page.goto('/menubar');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_menu')).toBeAttached();
    });

    const item = (page, menu: string, key: string) => page.locator(`li[id="${menu}_${key}"]`);
    const content = (page, menu: string, key: string) => item(page, menu, key).locator('> .p-menubar-item-content');

    test('theme styles: items in one row, button hidden, a group has a down arrow', async ({ page }) => {
        const root = page.locator('#basic_menu');
        expect(await root.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        expect(await root.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        const first = (await item(page, 'basic_menu', '0').boundingBox())!;
        const second = (await item(page, 'basic_menu', '1').boundingBox())!;
        expect(second.x).toBeGreaterThan(first.x + first.width - 2);
        expect(Math.abs(second.y - first.y)).toBeLessThan(2);
        expect(await page.locator('#basic_menu .p-menubar-button').evaluate((el) => getComputedStyle(el).display)).toBe('none');
        await expect(item(page, 'basic_menu', '2').locator('.p-menubar-submenu-icon')).toHaveCount(1);
        await expect(page.locator('#basic_menu .p-menubar-submenu')).toHaveCount(0);
    });

    test('a click expands the submenu under the item, a nested one to the right', async ({ page }) => {
        await content(page, 'basic_menu', '2').click();
        const sub = item(page, 'basic_menu', '2').locator('> .p-menubar-submenu');
        await expect(sub).toBeVisible();
        expect(await sub.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const parent = (await content(page, 'basic_menu', '2').boundingBox())!;
        const sb = (await sub.boundingBox())!;
        expect(sb.y).toBeGreaterThanOrEqual(parent.y + parent.height - 2);
        // after a click hover expands the nested group (a repeated click would collapse it, as in PrimeNG)
        await content(page, 'basic_menu', '2_4').hover();
        const deeper = item(page, 'basic_menu', '2_4').locator('> .p-menubar-submenu');
        await expect(deeper).toBeVisible();
        const row = (await content(page, 'basic_menu', '2_4').boundingBox())!;
        const db = (await deeper.boundingBox())!;
        expect(db.x).toBeGreaterThan(row.x + row.width / 2);
        await page.mouse.click(5, 5);
        await expect(page.locator('#basic_menu .p-menubar-submenu')).toHaveCount(0);
    });

    test('after a click hover over a neighboring item closes the previous branch', async ({ page }) => {
        await content(page, 'command_menu', '0').scrollIntoViewIfNeeded();
        await content(page, 'command_menu', '0').click();
        await expect(item(page, 'command_menu', '0')).toHaveClass(/p-menubar-item-active/);
        await content(page, 'command_menu', '1').hover();
        // "Search" has no submenu: the expanded File branch closes
        await expect(page.locator('#command_menu .p-menubar-submenu')).toHaveCount(0);
    });

    test('keyboard: → along the root, ↓ expands the group, ← closes', async ({ page }) => {
        const list = page.locator('#basic_menu_list');
        await list.focus();
        await expect(item(page, 'basic_menu', '0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await page.keyboard.press('ArrowRight');
        await expect(item(page, 'basic_menu', '2')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowDown');
        await expect(item(page, 'basic_menu', '2')).toHaveClass(/p-menubar-item-active/);
        await expect(item(page, 'basic_menu', '2_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowLeft');
        await expect(page.locator('#basic_menu .p-menubar-submenu')).toHaveCount(0);
        await expect(item(page, 'basic_menu', '2')).toHaveClass(/p-focus/);
    });

    test('command: a leaf invokes the command and closes the submenu', async ({ page }) => {
        await content(page, 'command_menu', '0').scrollIntoViewIfNeeded();
        await content(page, 'command_menu', '0').click();
        await content(page, 'command_menu', '0_1').click();
        await expect(page.locator('.p-toast-message')).toContainText('Printing');
        await expect(page.locator('#command_menu .p-menubar-submenu')).toHaveCount(0);
    });

    test('template: start and end slots and a custom item', async ({ page }) => {
        await expect(page.locator('[data-testid=menubar-start]')).toBeVisible();
        await expect(page.locator('[data-testid=menubar-end]')).toBeVisible();
        const start = (await page.locator('[data-testid=menubar-start]').boundingBox())!;
        const first = (await page.locator('#template_menu [data-testid=template-item]').first().boundingBox())!;
        const end = (await page.locator('[data-testid=menubar-end]').boundingBox())!;
        expect(start.x).toBeLessThan(first.x);
        expect(end.x).toBeGreaterThan(first.x);
        await expect(page.locator('#template_menu .p-menubar-item-link')).toHaveCount(0);
    });

    test('narrow window: the list is hidden, the button opens it', async ({ page }) => {
        await page.setViewportSize({ width: 600, height: 800 });
        const menu = page.locator('#basic_menu');
        await menu.scrollIntoViewIfNeeded();
        const button = page.locator('#basic_menu .p-menubar-button');
        await expect(button).toBeVisible();
        await expect(page.locator('#basic_menu_list')).toBeHidden();
        await button.click();
        await expect(menu).toHaveClass(/p-menubar-mobile-active/);
        await expect(page.locator('#basic_menu_list')).toBeVisible();
        await expect(button).toHaveAttribute('aria-expanded', 'true');
        await content(page, 'basic_menu', '2').click();
        // the submenu is in the flow, not on top
        const sub = item(page, 'basic_menu', '2').locator('> .p-menubar-submenu');
        await expect(sub).toBeVisible();
        expect(await sub.evaluate((el) => getComputedStyle(el).position)).toBe('static');
        await button.click();
        await expect(page.locator('#basic_menu_list')).toBeHidden();
    });
});
