import { expect, test } from '@playwright/test';

test.describe('MegaMenu', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 900 });
        await page.goto('/megamenu');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_menu')).toBeAttached();
    });

    const item = (page, menu: string, key: string) => page.locator(`li[id="${menu}_${key}"]`);
    const content = (page, menu: string, key: string) => item(page, menu, key).locator('> .p-megamenu-item-content');

    test('стили темы: пункты в одну строку, кнопка скрыта, у группы стрелка вниз', async ({ page }) => {
        const root = page.locator('#basic_menu');
        expect(await root.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        expect(await root.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        const first = (await item(page, 'basic_menu', '0').boundingBox())!;
        const second = (await item(page, 'basic_menu', '1').boundingBox())!;
        expect(second.x).toBeGreaterThan(first.x + first.width - 2);
        expect(Math.abs(second.y - first.y)).toBeLessThan(2);
        expect(await page.locator('#basic_menu .p-megamenu-button').evaluate((el) => getComputedStyle(el).display)).toBe('none');
        await expect(item(page, 'basic_menu', '0').locator('.p-megamenu-submenu-icon')).toHaveCount(1);
        await expect(page.locator('#basic_menu .p-megamenu-overlay')).toHaveCount(0);
    });

    test('клик раскрывает наложение под пунктом с четырьмя колонками в ряд', async ({ page }) => {
        await content(page, 'basic_menu', '0').click();
        const overlay = item(page, 'basic_menu', '0').locator('> .p-megamenu-overlay');
        await expect(overlay).toBeVisible();
        expect(await overlay.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const parent = (await content(page, 'basic_menu', '0').boundingBox())!;
        const ob = (await overlay.boundingBox())!;
        expect(ob.y).toBeGreaterThanOrEqual(parent.y + parent.height - 2);
        const cols = overlay.locator('.p-megamenu-col-3');
        await expect(cols).toHaveCount(4);
        const a = (await cols.nth(0).boundingBox())!;
        const b = (await cols.nth(1).boundingBox())!;
        expect(b.x).toBeGreaterThan(a.x + a.width - 2);
        expect(Math.abs(b.y - a.y)).toBeLessThan(2);
        await expect(overlay.locator('.p-megamenu-submenu-label').first()).toHaveText('Living Room');
        await page.mouse.click(5, 5);
        await expect(page.locator('#basic_menu .p-megamenu-overlay')).toHaveCount(0);
    });

    test('после клика наведение переключает наложение', async ({ page }) => {
        await content(page, 'basic_menu', '0').click();
        await content(page, 'basic_menu', '1').hover();
        await expect(item(page, 'basic_menu', '1')).toHaveClass(/p-megamenu-item-active/);
        await expect(item(page, 'basic_menu', '0')).not.toHaveClass(/p-megamenu-item-active/);
    });

    test('клавиатура: ↓ раскрывает и входит, → переходит в колонку, Escape возвращает на корень', async ({ page }) => {
        const list = page.locator('#basic_menu_list');
        await list.focus();
        await expect(item(page, 'basic_menu', '0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowDown');
        await expect(item(page, 'basic_menu', '0_0_0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(item(page, 'basic_menu', '0_1_0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('Escape');
        await expect(item(page, 'basic_menu', '0')).toHaveClass(/p-focus/);
        await expect(page.locator('#basic_menu .p-megamenu-overlay')).toHaveCount(0);
    });

    test('вертикальная ориентация: пункты в столбец, наложение правее', async ({ page }) => {
        const root = page.locator('#vertical_menu');
        await root.scrollIntoViewIfNeeded();
        await expect(root).toHaveClass(/p-megamenu-vertical/);
        const first = (await item(page, 'vertical_menu', '0').boundingBox())!;
        const second = (await item(page, 'vertical_menu', '1').boundingBox())!;
        expect(second.y).toBeGreaterThan(first.y + first.height - 2);
        await content(page, 'vertical_menu', '0').click();
        const overlay = item(page, 'vertical_menu', '0').locator('> .p-megamenu-overlay');
        await expect(overlay).toBeVisible();
        const ob = (await overlay.boundingBox())!;
        const row = (await content(page, 'vertical_menu', '0').boundingBox())!;
        expect(ob.x).toBeGreaterThan(row.x + row.width / 2);
        await expect(item(page, 'vertical_menu', '0').locator('.p-megamenu-submenu-icon')).toHaveCount(1);
    });

    test('команда листа вызывает уведомление', async ({ page }) => {
        await content(page, 'command_menu', '0').scrollIntoViewIfNeeded();
        await content(page, 'command_menu', '0').click();
        await content(page, 'command_menu', '0_0_0_1').click();
        await expect(page.locator('.p-toast-message').first()).toContainText('Armchair');
    });

    test('шаблон: слоты start и end, свой пункт', async ({ page }) => {
        await expect(page.locator('[data-testid="megamenu-start"]')).toBeAttached();
        await expect(page.locator('[data-testid="megamenu-end"]')).toBeAttached();
        await expect(page.locator('#template_menu [data-testid="template-item"]').first()).toBeAttached();
        await expect(page.locator('#template_menu .p-megamenu-item-link')).toHaveCount(0);
    });

    test('узкое окно: кнопка вместо списка, колонки друг под другом', async ({ page }) => {
        await page.setViewportSize({ width: 600, height: 900 });
        const root = page.locator('#basic_menu');
        await expect(root).toHaveClass(/p-megamenu-mobile/);
        const button = page.locator('#basic_menu .p-megamenu-button');
        await expect(button).toBeVisible();
        await expect(page.locator('#basic_menu_list')).toBeHidden();
        await button.click();
        await expect(root).toHaveClass(/p-megamenu-mobile-active/);
        await expect(page.locator('#basic_menu_list')).toBeVisible();
        await content(page, 'basic_menu', '0').click();
        const cols = item(page, 'basic_menu', '0').locator('.p-megamenu-col-12');
        await expect(cols).toHaveCount(4);
        const a = (await cols.nth(0).boundingBox())!;
        const b = (await cols.nth(1).boundingBox())!;
        expect(b.y).toBeGreaterThan(a.y + a.height - 2);
    });
});
