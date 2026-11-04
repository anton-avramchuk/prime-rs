import { expect, test } from '@playwright/test';

test.describe('AutoComplete', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/autocomplete');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_ac')).toBeAttached();
    });

    const root = (page, id: string) => page.locator(`#${id}`);
    const input = (page, id: string) => root(page, id).locator('input.p-autocomplete-input');
    const options = (page, id: string) => root(page, id).locator('li.p-autocomplete-option');

    test('стили темы: поле как InputText, подсказка', async ({ page }) => {
        const i = input(page, 'basic_ac');
        expect(await i.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        await expect(i).toHaveAttribute('placeholder', 'Search');
        expect(await root(page, 'basic_ac').evaluate((el) => getComputedStyle(el).position)).toBe('relative');
        await expect(root(page, 'basic_ac').locator('.p-autocomplete-overlay')).toHaveCount(0);
    });

    test('ввод запрашивает варианты и открывает панель под полем; клик выбирает', async ({ page }) => {
        await input(page, 'basic_ac').fill('alg');
        const panel = root(page, 'basic_ac').locator('.p-autocomplete-overlay');
        await expect(panel).toBeVisible();
        await expect(options(page, 'basic_ac')).toHaveCount(1); // Algeria
        const fb = (await input(page, 'basic_ac').boundingBox())!;
        const pb = (await panel.boundingBox())!;
        expect(pb.y).toBeGreaterThanOrEqual(fb.y + fb.height - 2);
        expect(await panel.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        await options(page, 'basic_ac').first().click();
        await expect(input(page, 'basic_ac')).toHaveValue('Algeria');
        await expect(panel).toHaveCount(0);
    });

    test('клавиатура: ↓ ходит по вариантам, Enter выбирает, Escape закрывает', async ({ page }) => {
        const i = input(page, 'basic_ac');
        await i.fill('an');
        await expect(options(page, 'basic_ac').first()).toBeVisible();
        await page.keyboard.press('ArrowDown');
        await expect(options(page, 'basic_ac').first()).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowDown');
        await expect(options(page, 'basic_ac').nth(1)).toHaveClass(/p-focus/);
        await page.keyboard.press('Enter');
        await expect(i).not.toHaveValue('an');
        await expect(root(page, 'basic_ac').locator('.p-autocomplete-overlay')).toHaveCount(0);
        await i.fill('ch');
        await expect(options(page, 'basic_ac').first()).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(root(page, 'basic_ac').locator('.p-autocomplete-overlay')).toHaveCount(0);
    });

    test('кнопка-стрелка запрашивает все варианты', async ({ page }) => {
        const r = root(page, 'dropdown_ac');
        await r.scrollIntoViewIfNeeded();
        await r.locator('.p-autocomplete-dropdown').click();
        await expect(options(page, 'dropdown_ac').first()).toBeVisible();
        expect(await options(page, 'dropdown_ac').count()).toBeGreaterThan(10);
        await r.locator('.p-autocomplete-dropdown').click();
        await expect(r.locator('.p-autocomplete-overlay')).toHaveCount(0);
    });

    test('множественный режим: чипы, повтор не добавляется, удаление', async ({ page }) => {
        const r = root(page, 'multiple_ac');
        await r.scrollIntoViewIfNeeded();
        const i = input(page, 'multiple_ac');
        await i.fill('germ');
        await options(page, 'multiple_ac').first().click();
        await expect(r.locator('.p-autocomplete-chip-item')).toHaveCount(1);
        await expect(r.locator('.p-autocomplete-chip-item').first()).toContainText('Germany');
        await i.fill('germ');
        await options(page, 'multiple_ac').first().click();
        await expect(r.locator('.p-autocomplete-chip-item')).toHaveCount(1);
        await i.fill('fra');
        await options(page, 'multiple_ac').first().click();
        await expect(r.locator('.p-autocomplete-chip-item')).toHaveCount(2);
        await r.locator('.p-autocomplete-chip-item').first().locator('.p-chip-remove-icon').click();
        await expect(r.locator('.p-autocomplete-chip-item')).toHaveCount(1);
    });

    test('группы: заголовки не выбираются', async ({ page }) => {
        const r = root(page, 'group_ac');
        await r.scrollIntoViewIfNeeded();
        await r.locator('.p-autocomplete-dropdown').click();
        await expect(r.locator('li.p-autocomplete-option-group')).toHaveCount(2);
        await expect(r.locator('li.p-autocomplete-option')).toHaveCount(8);
        await r.locator('li.p-autocomplete-option-group').first().click({ force: true });
        await expect(input(page, 'group_ac')).toHaveValue('');
    });

    test('очистка, отключённое и неверное состояния', async ({ page }) => {
        const clear = root(page, 'clear_ac');
        await clear.scrollIntoViewIfNeeded();
        await expect(input(page, 'clear_ac')).toHaveValue('Canada');
        await clear.locator('.p-autocomplete-clear-icon').click();
        await expect(input(page, 'clear_ac')).toHaveValue('');
        await expect(input(page, 'disabled_ac')).toBeDisabled();
        await expect(root(page, 'invalid_ac')).toHaveClass(/p-invalid/);
    });

    test('шаблон варианта', async ({ page }) => {
        const r = root(page, 'template_ac');
        await r.scrollIntoViewIfNeeded();
        await input(page, 'template_ac').fill('jap');
        await expect(r.locator('[data-testid="item-tpl"]').first()).toHaveText('Japan');
    });
});
