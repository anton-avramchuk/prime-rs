import { expect, test } from '@playwright/test';

test.describe('InputNumber', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/inputnumber');
        // ввод до гидратации (загрузка wasm) теряется
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=locale] .p-inputnumber').first()).toBeAttached();
    });

    test('демонстрации отрисованы, форматы локалей и валют', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(14);
        await expect(page.locator('#locale-us')).toHaveValue('1,234.50');
        await expect(page.locator('#locale-german')).toHaveValue('1.234,50');
        await expect(page.locator('#locale-indian')).toHaveValue('1,234.50');
        await expect(page.locator('#currency-us')).toHaveValue('$1,234.50');
        await expect(page.locator('#currency-germany')).toHaveValue(/^1\.234,50\s€$/);
        await expect(page.locator('#currency-india')).toHaveValue(/^INR\s1,234\.50$/);
        await expect(page.locator('#currency-japan')).toHaveValue('￥1,235');
        await expect(page.locator('#mile')).toHaveValue('50 mi');
        await expect(page.locator('#percent')).toHaveValue('%20');
        await expect(page.locator('#expiry')).toHaveValue('Expires in 10 days');
        await expect(page.locator('#temperature')).toHaveValue('↑ 20℃');
        await expect(page.locator('#integeronly')).toHaveValue('42,723');
        await expect(page.locator('#withoutgrouping')).toHaveValue('58151');
        const radius = await page.locator('#locale-us').evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
    });

    test('набор с клавиатуры форматирует на лету, Backspace и границы', async ({ page }) => {
        const field = page.locator('#locale-us');
        await field.click();
        await page.keyboard.press('Control+A');
        await page.keyboard.press('Backspace');
        await expect(field).toHaveValue('');
        await expect(page.getByTestId('echo-us')).toHaveText('');
        await page.keyboard.type('1234');
        await expect(field).toHaveValue('1,234.00');
        await expect(page.getByTestId('echo-us')).toHaveText('1234');
        await page.keyboard.type('ab');
        await expect(field).toHaveValue('1,234.00');
        await page.keyboard.press('Backspace');
        await expect(field).toHaveValue('123.00');

        const bounded = page.locator('#minmax');
        await bounded.click();
        await page.keyboard.press('Control+A');
        await page.keyboard.type('250');
        await page.keyboard.press('Tab');
        await expect(bounded).toHaveValue('100');
        await expect(page.getByTestId('echo-minmax')).toHaveText('100');
    });

    test('стрелки меняют значение, а курсор остаётся рядом с цифрами', async ({ page }) => {
        const bounded = page.locator('#minmax');
        await bounded.click();
        await page.keyboard.press('ArrowUp');
        await expect(bounded).toHaveValue('51');
        await page.keyboard.press('ArrowDown');
        await page.keyboard.press('ArrowDown');
        await expect(bounded).toHaveValue('49');
        await page.keyboard.press('End');
        await expect(bounded).toHaveValue('100');
        await page.keyboard.press('ArrowUp');
        await expect(bounded).toHaveValue('100');
        await page.keyboard.press('Home');
        await expect(bounded).toHaveValue('0');

        const price = page.locator('#currency-us');
        await price.click();
        const caret = await price.evaluate((el: HTMLInputElement) => el.selectionStart);
        expect(caret).toBeGreaterThanOrEqual(1);
    });

    test('кнопки: шаг, удержание, раскладки и значки', async ({ page }) => {
        const root = page.locator('[data-demo=buttons] .p-inputnumber').first();
        await expect(root).toHaveClass(/p-inputnumber-stacked/);
        const up = root.locator('.p-inputnumber-increment-button');
        await up.click();
        await expect(page.getByTestId('echo-stacked')).toHaveText('58152');
        const box = (await up.boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
        await page.mouse.down();
        await page.waitForTimeout(900);
        await page.mouse.up();
        const held = Number(await page.getByTestId('echo-stacked').textContent());
        expect(held).toBeGreaterThan(58160);
        await page.waitForTimeout(200);
        await expect(page.getByTestId('echo-stacked')).toHaveText(String(held));

        const horizontal = page.locator('[data-demo=buttons] .p-inputnumber').nth(2);
        await expect(horizontal).toHaveClass(/p-inputnumber-horizontal/);
        await horizontal.locator('.p-inputnumber-increment-button').click();
        await expect(page.locator('#horizontal')).toHaveValue(/^2[.,]75\s?€?$|^€2\.75$/);
        await expect(horizontal.locator('.p-inputnumber-button-group')).toHaveCount(0);
        await expect(page.locator('[data-demo=vertical] .p-inputnumber')).toHaveClass(/p-inputnumber-vertical/);
        await page.locator('[data-demo=vertical] .p-inputnumber-decrement-button').click();
        await expect(page.locator('#vertical-input')).toHaveValue('49');
    });

    test('очистка, размеры, вариант, invalid и disabled', async ({ page }) => {
        const clearable = page.locator('[data-demo=clearicon] .p-inputnumber');
        await expect(page.locator('#price_input')).toHaveValue('$3,000.00');
        await clearable.locator('.p-inputnumber-clear-icon').click();
        await expect(page.locator('#price_input')).toHaveValue('');
        await expect(clearable.locator('.p-inputnumber-clear-icon')).toHaveCount(0);

        await expect(page.locator('[data-demo=sizes] .p-inputtext-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-inputtext-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] input')).toBeDisabled();
        await expect(page.locator('[data-demo=fluid] .p-inputnumber-fluid')).toHaveCount(1);

        const invalid = page.locator('[data-demo=invalid] .p-inputnumber').first();
        await expect(invalid).toHaveClass(/p-invalid/);
        await invalid.locator('input').click();
        await page.keyboard.type('5');
        await expect(invalid).not.toHaveClass(/p-invalid/);
    });

    test('floatlabel: метка в обёртке и поднимается при вводе', async ({ page }) => {
        await page.waitForLoadState('networkidle');
        const demo = page.locator('[data-demo=floatlabel]');
        await expect(demo.locator('.p-floatlabel.p-floatlabel-over input.p-inputnumber-input')).toHaveCount(1);
        const label = demo.locator('.p-floatlabel > label');
        await expect(label).toHaveCount(1);
        const before = (await label.boundingBox())!.y;
        await demo.locator('.p-floatlabel input.p-inputnumber-input').first().focus();
        await expect.poll(async () => (await label.boundingBox())!.y).toBeLessThan(before);
    });

    test('iftalabel: метка в обёртке над значением', async ({ page }) => {
        const demo = page.locator('[data-demo=iftalabel]');
        await expect(demo.locator('.p-iftalabel input.p-inputnumber-input')).toHaveCount(1);
        const label = demo.locator('.p-iftalabel > label');
        const box = (await label.boundingBox())!;
        const root = (await demo.locator('.p-iftalabel').boundingBox())!;
        expect(box.y).toBeGreaterThanOrEqual(root.y);
        expect(box.y + box.height).toBeLessThanOrEqual(root.y + root.height);
    });
});
