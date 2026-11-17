import { expect, test } from '@playwright/test';

test.describe('ContextMenu', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 900 });
        await page.goto('/contextmenu');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_target')).toBeAttached();
    });

    const item = (page, menu: string, key: string) => page.locator(`li[id="${menu}_${key}"]`);
    const content = (page, menu: string, key: string) => item(page, menu, key).locator('> .p-contextmenu-item-content');

    test('no menu until the right button is pressed; then it is at the cursor with theme styles', async ({ page }) => {
        await expect(page.locator('#basic_menu')).toHaveCount(0);
        const target = page.locator('#basic_target');
        await target.scrollIntoViewIfNeeded();
        const box = (await target.boundingBox())!;
        const at = { x: 40, y: 30 };
        await target.click({ button: 'right', position: at });
        const menu = page.locator('#basic_menu');
        await expect(menu).toBeVisible();
        // Position is in page coordinates; the page may have been scrolled
        await expect
            .poll(async () => {
                const m = (await menu.boundingBox())!;
                return [Math.round(m.x - (box.x + at.x)), Math.round(m.y - (box.y + at.y))];
            })
            .toEqual([1, 1]);
        expect(await menu.evaluate((el) => getComputedStyle(el).backgroundColor)).not.toBe('rgba(0, 0, 0, 0)');
        expect(await menu.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        expect(await menu.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const first = (await item(page, 'basic_menu', '0').boundingBox())!;
        const second = (await item(page, 'basic_menu', '1').boundingBox())!;
        expect(second.y).toBeGreaterThan(first.y + first.height - 2);
        await expect(page.locator('#basic_menu_list')).toBeFocused();
    });

    test('a left click outside closes the menu, so does Escape', async ({ page }) => {
        const target = page.locator('#basic_target');
        await target.click({ button: 'right' });
        await expect(page.locator('#basic_menu')).toBeVisible();
        await page.mouse.click(5, 5);
        await expect(page.locator('#basic_menu')).toHaveCount(0);
        await target.click({ button: 'right' });
        await expect(page.locator('#basic_menu')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(page.locator('#basic_menu')).toHaveCount(0);
    });

    test('at the bottom-right window edge the menu flips into the window', async ({ page }) => {
        await page.setViewportSize({ width: 600, height: 500 });
        const target = page.locator('#basic_target');
        await target.scrollIntoViewIfNeeded();
        const box = (await target.boundingBox())!;
        // bottom-right corner of the target, within the window
        const x = Math.min(box.width - 4, 590 - box.x);
        const y = Math.min(box.height - 4, 490 - box.y);
        await target.click({ button: 'right', position: { x, y } });
        const menu = page.locator('#basic_menu');
        await expect(menu).toBeVisible();
        const m = (await menu.boundingBox())!;
        expect(m.x + m.width).toBeLessThanOrEqual(600 + 1);
        expect(m.y + m.height).toBeLessThanOrEqual(500 + 1);
        expect(m.x).toBeLessThan(box.x + x);
    });

    test('a submenu expands to the right, the keyboard walks the levels', async ({ page }) => {
        const target = page.locator('#nested_target');
        await target.scrollIntoViewIfNeeded();
        await target.click({ button: 'right' });
        await expect(page.locator('#nested_menu')).toBeVisible();
        await content(page, 'nested_menu', '0').click();
        const sub = item(page, 'nested_menu', '0').locator('> .p-contextmenu-submenu');
        await expect(sub).toBeVisible();
        expect(await sub.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const parent = (await item(page, 'nested_menu', '0').boundingBox())!;
        const sb = (await sub.boundingBox())!;
        expect(sb.x).toBeGreaterThan(parent.x + parent.width / 2);
        await page.keyboard.press('Escape');
        await expect(page.locator('#nested_menu')).toHaveCount(0);
        await target.click({ button: 'right' });
        await page.locator('#nested_menu_list').focus();
        await page.keyboard.press('ArrowDown');
        await expect(item(page, 'nested_menu', '0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(item(page, 'nested_menu', '0')).toHaveClass(/p-contextmenu-item-active/);
        await expect(item(page, 'nested_menu', '0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowLeft');
        await expect(page.locator('#nested_menu .p-contextmenu-submenu')).toHaveCount(0);
    });

    test('command: right click on a user, a leaf calls the command and closes the menu', async ({ page }) => {
        const user = page.locator('[data-testid=user]').first();
        await user.scrollIntoViewIfNeeded();
        await user.click({ button: 'right' });
        await expect(page.locator('#command_menu')).toBeVisible();
        await content(page, 'command_menu', '0').click();
        await expect(page.locator('#command_menu')).toHaveCount(0);
        await expect(page.locator('.p-toast-message')).toContainText('View requested');
    });

    test('template: a custom item instead of a link', async ({ page }) => {
        const target = page.locator('#template_target');
        await target.scrollIntoViewIfNeeded();
        await target.click({ button: 'right' });
        await expect(page.locator('#template_menu [data-testid=template-item]').first()).toBeVisible();
        await expect(page.locator('#template_menu .p-contextmenu-item-link')).toHaveCount(0);
    });
});
