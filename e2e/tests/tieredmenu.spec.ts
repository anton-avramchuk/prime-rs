import { expect, test } from '@playwright/test';

test.describe('TieredMenu', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 900 });
        await page.goto('/tieredmenu');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_menu')).toBeAttached();
    });

    const item = (page, menu: string, key: string) => page.locator(`li[id="${menu}_${key}"]`);
    const content = (page, menu: string, key: string) => item(page, menu, key).locator('> .p-tieredmenu-item-content');

    test('theme styles: the menu is vertical, the label and the group arrow are on one line', async ({ page }) => {
        const root = page.locator('#basic_menu');
        expect(await root.evaluate((el) => getComputedStyle(el).backgroundColor)).not.toBe('rgba(0, 0, 0, 0)');
        expect(await root.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        const first = (await item(page, 'basic_menu', '0').boundingBox())!;
        const second = (await item(page, 'basic_menu', '1').boundingBox())!;
        expect(second.y).toBeGreaterThan(first.y + first.height - 2);
        const link = item(page, 'basic_menu', '0').locator('.p-tieredmenu-item-link');
        expect(await link.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        const label = (await link.locator('.p-tieredmenu-item-label').boundingBox())!;
        const arrow = (await link.locator('.p-tieredmenu-submenu-icon').boundingBox())!;
        expect(arrow.x).toBeGreaterThan(label.x + label.width - 2);
        await expect(item(page, 'basic_menu', '3')).toHaveAttribute('aria-haspopup', 'menu');
        await expect(page.locator('#basic_menu .p-tieredmenu-submenu')).toHaveCount(0);
    });

    test('a click expands the submenu to the right of the item, the nested one further right', async ({ page }) => {
        await content(page, 'basic_menu', '0').click();
        const sub = item(page, 'basic_menu', '0').locator('> .p-tieredmenu-submenu');
        await expect(sub).toBeVisible();
        expect(await sub.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const parent = (await item(page, 'basic_menu', '0').boundingBox())!;
        const box = (await sub.boundingBox())!;
        expect(box.x).toBeGreaterThanOrEqual(parent.x + parent.width - 2);
        expect(Math.abs(box.y - parent.y)).toBeLessThan(3);
        // hover after a click expands the nested group (a subsequent click on it collapses it, as in PrimeNG)
        await content(page, 'basic_menu', '0_0').hover();
        const deeper = item(page, 'basic_menu', '0_0').locator('> .p-tieredmenu-submenu');
        await expect(deeper).toBeVisible();
        const sub2 = (await deeper.boundingBox())!;
        // the nested submenu starts at the right edge of the parent item (which is narrower than the submenu's border and padding)
        expect(sub2.x).toBeGreaterThanOrEqual(box.x + box.width - 10);
        await content(page, 'basic_menu', '0_0').click();
        await expect(deeper).toHaveCount(0);
    });

    test('after a click, hover expands sibling groups, an outside click closes', async ({ page }) => {
        await content(page, 'basic_menu', '0').click();
        await content(page, 'basic_menu', '1').hover();
        await expect(item(page, 'basic_menu', '1')).toHaveClass(/p-tieredmenu-item-active/);
        await expect(item(page, 'basic_menu', '0')).not.toHaveClass(/p-tieredmenu-item-active/);
        await page.mouse.click(5, 5);
        await expect(page.locator('#basic_menu .p-tieredmenu-submenu')).toHaveCount(0);
    });

    test('keyboard: arrows across levels, Escape closes', async ({ page }) => {
        await page.locator('#basic_menu_list').focus();
        await page.keyboard.press('ArrowDown');
        await expect(item(page, 'basic_menu', '0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(item(page, 'basic_menu', '0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(item(page, 'basic_menu', '0_0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowLeft');
        await expect(item(page, 'basic_menu', '0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowLeft');
        await expect(item(page, 'basic_menu', '0')).toHaveClass(/p-focus/);
        await expect(page.locator('#basic_menu .p-tieredmenu-submenu')).toHaveCount(0);
        await page.keyboard.press('ArrowRight');
        await expect(page.locator('#basic_menu .p-tieredmenu-submenu')).toHaveCount(1);
        await page.keyboard.press('Escape');
        await expect(page.locator('#basic_menu .p-tieredmenu-submenu')).toHaveCount(0);
        await page.keyboard.type('s');
        await expect(item(page, 'basic_menu', '2')).toHaveClass(/p-focus/);
    });

    test('popup: the button opens the menu below itself, a leaf and an outside click close it', async ({ page }) => {
        const button = page.locator('[data-demo=popup]').getByRole('button', { name: 'Toggle' });
        await button.click();
        const menu = page.locator('#popup_menu');
        await expect(menu).toBeVisible();
        await expect(menu).toHaveClass(/p-tieredmenu-overlay/);
        const m = (await menu.boundingBox())!;
        const b = (await button.boundingBox())!;
        // the menu is next to the button: below it or (if there is no room below) above it
        expect(m.y >= b.y + b.height - 2 || m.y + m.height <= b.y + 2).toBe(true);
        await content(page, 'popup_menu', '2').click();
        await expect(menu).toHaveCount(0);
        await button.click();
        await expect(menu).toBeVisible();
        await page.mouse.click(5, 5);
        await expect(menu).toHaveCount(0);
        await button.click();
        await page.keyboard.press('Escape');
        await expect(menu).toHaveCount(0);
        await expect(button).toBeFocused();
    });

    test('at the right edge of the window the submenu opens to the left', async ({ page }) => {
        await page.setViewportSize({ width: 360, height: 700 });
        // a narrow window enables mobile mode, where the submenu goes inside the item: we check the popup menu at the right edge
        await page.evaluate(() => {
            const wrap = document.querySelector('[data-demo=popup] > div, [data-demo=popup]') as HTMLElement;
            wrap.style.cssText += 'display:flex;justify-content:flex-end';
        });
        await page.waitForTimeout(400); // let resize run, otherwise it closes the menu that was just opened
        const button = page.locator('[data-demo=popup]').getByRole('button', { name: 'Toggle' });
        await button.click();
        await expect(page.locator('#popup_menu')).toBeVisible();
        await content(page, 'popup_menu', '0').click();
        const sub = item(page, 'popup_menu', '0').locator('> .p-tieredmenu-submenu');
        await expect(sub).toBeVisible();
        const box = (await sub.boundingBox())!;
        const view = page.viewportSize()!;
        expect(box.x).toBeGreaterThanOrEqual(-1);
        expect(box.x + box.width).toBeLessThanOrEqual(view.width + 1);
    });

    test('a command shows a message, a template replaces the link', async ({ page }) => {
        await content(page, 'command_menu', '0').click();
        await content(page, 'command_menu', '0_1').click();
        await expect(page.locator('.p-toast-message').first()).toContainText('Preparing to print');
        await expect(page.locator('#command_menu .p-tieredmenu-submenu')).toHaveCount(0);
        await expect(page.getByTestId('template-item').first()).toBeVisible();
        await expect(page.locator('#template_menu .p-tieredmenu-item-link')).toHaveCount(0);
    });
});
