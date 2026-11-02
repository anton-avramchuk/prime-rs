import { expect, test } from '@playwright/test';

test.describe('Menu', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/menu');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-menu')).toBeVisible();
    });

    test('basic: два пункта со значками', async ({ page }) => {
        const items = page.locator('[data-demo=basic] .p-menu-item');
        await expect(items).toHaveCount(2);
        await expect(items.nth(0)).toHaveText('New');
        await expect(items.nth(1)).toHaveText('Search');
        await expect(items.nth(0).locator('svg')).toBeVisible();
        await expect(page.locator('[data-demo=basic] .p-menu-list')).toHaveAttribute('role', 'menu');
        await expect(items.nth(0)).toHaveAttribute('role', 'menuitem');
    });

    test('group: заголовки групп', async ({ page }) => {
        const demo = page.locator('[data-demo=group]');
        await expect(demo.locator('.p-menu-submenu-label')).toHaveText(['Documents', 'Profile']);
        await expect(demo.locator('.p-menu-item')).toHaveCount(4);
    });

    test('command: клик по пункту показывает сообщение', async ({ page }) => {
        const demo = page.locator('[data-demo=command]');
        await demo.locator('.p-menu-item', { hasText: 'New' }).click();
        await expect(demo.locator('.p-toast-message-success .p-toast-detail')).toHaveText('File created');
        await demo.locator('.p-menu-item', { hasText: 'Search' }).click();
        await expect(demo.locator('.p-toast-message-warn .p-toast-detail')).toHaveText('No results found');
    });

    test('клавиши: стрелки, Home, End меняют активный пункт', async ({ page }) => {
        const list = page.locator('[data-demo=group] .p-menu-list');
        await list.focus();
        const items = page.locator('[data-demo=group] .p-menu-item');
        await expect(items.nth(0)).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowDown');
        await expect(items.nth(1)).toHaveClass(/p-focus/);
        await page.keyboard.press('End');
        await expect(items.nth(3)).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowUp');
        await expect(items.nth(2)).toHaveClass(/p-focus/);
        await page.keyboard.press('Home');
        await expect(items.nth(0)).toHaveClass(/p-focus/);
        const id = await items.nth(0).getAttribute('id');
        await expect(list).toHaveAttribute('aria-activedescendant', id!);
    });

    test('popup: показ у кнопки, закрытие кликом снаружи, Escape и выбором пункта', async ({ page }) => {
        const demo = page.locator('[data-demo=popup]');
        const button = demo.getByRole('button', { name: 'Options' });
        await expect(demo.locator('.p-menu')).toHaveCount(0);
        await button.click();
        const menu = demo.locator('.p-menu');
        await expect(menu).toBeVisible();
        await expect(menu).toHaveClass(/p-menu-overlay/);
        const [m, b] = [(await menu.boundingBox())!, (await button.boundingBox())!];
        expect(Math.abs(m.y - (b.y + b.height))).toBeLessThan(8);
        expect(Math.abs(m.x - b.x)).toBeLessThan(8);
        await expect(demo.locator('.p-menu-list')).toBeFocused();

        await page.mouse.click(5, 5);
        await expect(menu).toHaveCount(0);

        await button.click();
        await expect(menu).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(menu).toHaveCount(0);
        await expect(button).toBeFocused();

        await button.click();
        await demo.locator('.p-menu-item', { hasText: 'Refresh' }).click();
        await expect(menu).toHaveCount(0);
    });

    test('template: свои представления пункта, заголовка, начала и конца', async ({ page }) => {
        const demo = page.locator('[data-demo=template]');
        await expect(demo.locator('.p-menu-start')).toContainText('PRIMEAPP');
        await expect(demo.locator('.p-menu-end')).toContainText('Amy Elsner');
        await expect(demo.locator('.p-menu-separator')).toHaveCount(2);
        await expect(demo.locator('.p-menu-submenu-label')).toHaveText(['Documents', 'Profile']);
        await expect(demo.locator('.p-menu-item').first()).toContainText('⌘+N');
    });
});
