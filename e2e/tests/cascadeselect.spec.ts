import { expect, test } from '@playwright/test';

test.describe('CascadeSelect', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/cascadeselect');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_select')).toBeAttached();
    });

    const field = (page) => page.locator('#basic_select');
    const option = (page, key: string) => page.locator(`li[id="basic_select_${key}"]`);
    const content = (page, key: string) => option(page, key).locator('> .p-cascadeselect-option-content');

    test('стили темы: поле с рамкой, подсказка, кнопка со стрелкой', async ({ page }) => {
        const root = field(page);
        expect(await root.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        await expect(root.locator('.p-cascadeselect-label')).toHaveText('Select a City');
        await expect(root.locator('.p-cascadeselect-label')).toHaveClass(/p-placeholder/);
        await expect(root.locator('.p-cascadeselect-dropdown svg')).toHaveCount(1);
        await expect(root.locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('клик открывает панель под полем; вложенный список правее родителя', async ({ page }) => {
        await field(page).click();
        const panel = field(page).locator('.p-cascadeselect-overlay').first();
        await expect(panel).toBeVisible();
        const fb = (await field(page).boundingBox())!;
        const pb = (await panel.boundingBox())!;
        expect(pb.y).toBeGreaterThanOrEqual(fb.y + fb.height - 2);
        // наведение раскрывает группу (как в PrimeNG; последующий клик по раскрытой группе её свернул бы)
        await content(page, '0').hover();
        const sub = option(page, '0').locator('> ul.p-cascadeselect-option-list');
        await expect(sub).toBeVisible();
        const row = (await content(page, '0').boundingBox())!;
        const sb = (await sub.boundingBox())!;
        expect(sb.x).toBeGreaterThan(row.x + row.width / 2);
        await content(page, '0_0').hover();
        await expect(option(page, '0_0').locator('> ul.p-cascadeselect-option-list')).toBeVisible();
        await expect(content(page, '0')).toHaveClass(/p-cascadeselect-option-content/);
        await expect(option(page, '0')).toHaveClass(/p-cascadeselect-option-active/);
        await expect(option(page, '0').locator('.p-cascadeselect-group-icon svg').first()).toBeVisible();
    });

    test('выбор листа подставляет подпись и закрывает панель', async ({ page }) => {
        await field(page).click();
        await content(page, '1').hover();
        await content(page, '1_1').hover();
        await expect(option(page, '1_1').locator('> ul.p-cascadeselect-option-list')).toBeVisible();
        await content(page, '1_1_1').click();
        await expect(field(page).locator('.p-cascadeselect-label')).toHaveText('Toronto');
        await expect(field(page).locator('.p-cascadeselect-overlay')).toHaveCount(0);
        await field(page).click();
        await expect(option(page, '1_1_1')).toHaveClass(/p-cascadeselect-option-selected/);
        await page.mouse.click(5, 5);
        await expect(field(page).locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('клавиатура: ↓ открывает, → входит в группу, Enter выбирает лист', async ({ page }) => {
        const input = field(page).locator('input');
        await input.focus();
        await page.keyboard.press('ArrowDown');
        await expect(field(page).locator('.p-cascadeselect-overlay').first()).toBeVisible();
        await page.keyboard.press('ArrowDown');
        await expect(option(page, '0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(option(page, '0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(option(page, '0_0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('Enter');
        await expect(field(page).locator('.p-cascadeselect-label')).toHaveText('Sydney');
        await expect(field(page).locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('значок очистки сбрасывает значение', async ({ page }) => {
        const clear = page.locator('#clear_select');
        await clear.scrollIntoViewIfNeeded();
        await expect(clear.locator('.p-cascadeselect-label')).toHaveText('Sydney');
        await clear.locator('.p-cascadeselect-clear-icon').click();
        await expect(clear.locator('.p-cascadeselect-label')).toHaveText('Select a City');
        await expect(clear.locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('отключённое поле, загрузка и неверное значение', async ({ page }) => {
        const disabled = page.locator('#disabled_select');
        await disabled.scrollIntoViewIfNeeded();
        await expect(disabled).toHaveClass(/p-disabled/);
        await disabled.click({ force: true });
        await expect(disabled.locator('.p-cascadeselect-overlay')).toHaveCount(0);
        await expect(page.locator('#loading_select .p-cascadeselect-loading-icon')).toBeAttached();
        await expect(page.locator('#invalid_select')).toHaveClass(/p-invalid/);
    });

    test('шаблон варианта', async ({ page }) => {
        const t = page.locator('#template_select');
        await t.scrollIntoViewIfNeeded();
        await t.click();
        await expect(t.locator('[data-testid="option-tpl"]').first()).toContainText('Australia');
    });
});
