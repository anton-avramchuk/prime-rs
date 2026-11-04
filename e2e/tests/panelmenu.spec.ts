import { expect, test } from '@playwright/test';

test.describe('PanelMenu', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/panelmenu');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_menu')).toBeAttached();
    });

    const header = (page, menu: string, i: number) => page.locator(`#${menu}_${i}_header`);
    const region = (page, menu: string, i: number) => page.locator(`#${menu}_${i}_content`);
    const item = (page, menu: string, key: string) => page.locator(`li[id="${menu}_${key}"]`);
    const content = (page, menu: string, key: string) => item(page, menu, key).locator('> .p-panelmenu-item-content');

    test('стили темы: панели в столбец с рамкой, у свёрнутой группы шеврон вправо, содержимое скрыто', async ({ page }) => {
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

    test('клик раскрывает панель, в одиночном режиме закрывает прежнюю', async ({ page }) => {
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

    test('вложенные ветки раскрываются отступом вправо', async ({ page }) => {
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

    test('множественный режим держит открытыми несколько панелей', async ({ page }) => {
        await header(page, 'multiple_menu', 0).scrollIntoViewIfNeeded();
        await header(page, 'multiple_menu', 0).click();
        await header(page, 'multiple_menu', 1).click();
        await expect(region(page, 'multiple_menu', 0)).toBeVisible();
        await expect(region(page, 'multiple_menu', 1)).toBeVisible();
    });

    test('Toggle All раскрывает и сворачивает всё', async ({ page }) => {
        const button = page.getByRole('button', { name: 'Toggle All' });
        await button.scrollIntoViewIfNeeded();
        await button.click();
        for (const i of [0, 1, 2]) await expect(region(page, 'controlled_menu', i)).toBeVisible();
        await expect(item(page, 'controlled_menu', '0_0')).toHaveAttribute('aria-expanded', 'true');
        await button.click();
        for (const i of [0, 1, 2]) await expect(region(page, 'controlled_menu', i)).toBeHidden();
    });

    test('клавиатура: ↓ с заголовка в дерево, → раскрывает, ← сворачивает, ↓ после последнего — к следующему заголовку', async ({ page }) => {
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

    test('команда листа вызывает уведомление', async ({ page }) => {
        await header(page, 'command_menu', 0).scrollIntoViewIfNeeded();
        await header(page, 'command_menu', 0).click();
        await content(page, 'command_menu', '0_0').click();
        await expect(page.locator('.p-toast-message').first()).toContainText('File created');
    });

    test('шаблон заменяет ссылку пункта', async ({ page }) => {
        await header(page, 'template_menu', 0).scrollIntoViewIfNeeded();
        await header(page, 'template_menu', 0).click();
        await expect(page.locator('#template_menu [data-testid="template-item"]').first()).toBeVisible();
        await expect(page.locator('#template_menu .p-panelmenu-item-link')).toHaveCount(0);
    });
});
