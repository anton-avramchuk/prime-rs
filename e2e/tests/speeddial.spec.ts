import { expect, test } from '@playwright/test';

test.describe('SpeedDial', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/speeddial');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=linear] .p-speeddial').first()).toBeAttached();
    });

    const demo = (page, name: string) => page.locator(`[data-demo=${name}]`);

    test('линейный: список скрыт, открывается вверх и пункты выстроены в колонку', async ({ page }) => {
        const sd = demo(page, 'linear').locator('.p-speeddial-direction-up');
        const button = sd.locator('.p-speeddial-button');
        const items = sd.locator('.p-speeddial-item');
        expect(await items.first().evaluate((el) => getComputedStyle(el).opacity)).toBe('0');
        expect(await button.evaluate((el) => getComputedStyle(el).borderRadius)).not.toBe('0px');
        await button.click();
        await expect(sd).toHaveClass(/p-speeddial-open/);
        await expect.poll(() => items.first().evaluate((el) => getComputedStyle(el).opacity)).toBe('1');
        const b = (await button.boundingBox())!;
        const first = (await items.nth(0).boundingBox())!;
        const second = (await items.nth(1).boundingBox())!;
        expect(first.y + first.height).toBeLessThanOrEqual(b.y + 2);
        expect(second.y + second.height).toBeLessThanOrEqual(first.y + 2);
        expect(Math.abs(first.x + first.width / 2 - (b.x + b.width / 2))).toBeLessThan(6);
        expect(await button.evaluate((el) => getComputedStyle(el).transform)).not.toBe('none');
    });

    test('все четыре направления раскрывают список с нужной стороны кнопки', async ({ page }) => {
        const cases: [string, (b: any, f: any) => boolean][] = [
            ['up', (b, f) => f.y + f.height <= b.y + 2],
            ['down', (b, f) => f.y >= b.y + b.height - 2],
            ['left', (b, f) => f.x + f.width <= b.x + 2],
            ['right', (b, f) => f.x >= b.x + b.width - 2],
        ];
        for (const [dir, ok] of cases) {
            const sd = demo(page, 'linear').locator(`.p-speeddial-direction-${dir}`);
            await sd.locator('.p-speeddial-button').click();
            await expect(sd).toHaveClass(/p-speeddial-open/);
            await page.waitForTimeout(400);
            const b = (await sd.locator('.p-speeddial-button').boundingBox())!;
            const f = (await sd.locator('.p-speeddial-item').first().boundingBox())!;
            expect(ok(b, f), `${dir}: ${JSON.stringify([b, f])}`).toBe(true);
            await sd.locator('.p-speeddial-button').click();
            await expect(sd).not.toHaveClass(/p-speeddial-open/);
        }
    });

    test('пункт вызывает команду, показывает сообщение и закрывает список', async ({ page }) => {
        const sd = demo(page, 'linear').locator('.p-speeddial-direction-up');
        await sd.locator('.p-speeddial-button').click();
        await page.waitForTimeout(400);
        await sd.locator('.p-speeddial-action').nth(2).click();
        await expect(sd).not.toHaveClass(/p-speeddial-open/);
        await expect(page.locator('.p-toast-message').first()).toContainText('Data Deleted');
    });

    test('клик вне списка и Escape закрывают его', async ({ page }) => {
        const sd = demo(page, 'linear').locator('.p-speeddial-direction-up');
        await sd.locator('.p-speeddial-button').click();
        await expect(sd).toHaveClass(/p-speeddial-open/);
        await page.mouse.click(5, 5);
        await expect(sd).not.toHaveClass(/p-speeddial-open/);
        await sd.locator('.p-speeddial-button').click();
        await expect(sd).toHaveClass(/p-speeddial-open/);
        await page.keyboard.press('Escape');
        await expect(sd).not.toHaveClass(/p-speeddial-open/);
        await expect(sd.locator('.p-speeddial-button')).toBeFocused();
    });

    test('клавиатура: стрелка открывает список и ведёт по пунктам, Enter активирует', async ({ page }) => {
        const sd = demo(page, 'linear').locator('.p-speeddial-direction-up');
        await sd.locator('.p-speeddial-button').focus();
        await page.keyboard.press('ArrowDown');
        await expect(sd).toHaveClass(/p-speeddial-open/);
        const items = sd.locator('.p-speeddial-item');
        await expect(items.nth(0)).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowUp');
        await expect(items.nth(1)).toHaveClass(/p-focus/);
        await page.keyboard.press('End');
        await expect(items.nth(4)).toHaveClass(/p-focus/);
        await page.keyboard.press('Home');
        await page.keyboard.press('Enter');
        await expect(sd).not.toHaveClass(/p-speeddial-open/);
        await expect(page.locator('.p-toast-message').first()).toContainText('Data Added');
    });

    test('круг: пункты разложены вокруг кнопки на заданном радиусе', async ({ page }) => {
        const sd = demo(page, 'circle').locator('.p-speeddial');
        await sd.locator('.p-speeddial-button').click();
        await page.waitForTimeout(500);
        const b = (await sd.locator('.p-speeddial-button').boundingBox())!;
        const centre = { x: b.x + b.width / 2, y: b.y + b.height / 2 };
        const count = await sd.locator('.p-speeddial-item').count();
        for (let i = 0; i < count; i++) {
            const f = (await sd.locator('.p-speeddial-item').nth(i).boundingBox())!;
            const dist = Math.hypot(f.x + f.width / 2 - centre.x, f.y + f.height / 2 - centre.y);
            // пункты смещены на половину разницы размеров кнопки и пункта (--item-diff), поэтому допуск шире
            expect(Math.abs(dist - 80), `пункт ${i}: ${dist}`).toBeLessThan(14);
        }
    });

    test('полукруг и четверть круга раскрываются в своей полуплоскости', async ({ page }) => {
        const semi = demo(page, 'semicircle').locator('.p-speeddial-direction-up');
        await semi.locator('.p-speeddial-button').click();
        await page.waitForTimeout(500);
        const b = (await semi.locator('.p-speeddial-button').boundingBox())!;
        const count = await semi.locator('.p-speeddial-item').count();
        for (let i = 0; i < count; i++) {
            const f = (await semi.locator('.p-speeddial-item').nth(i).boundingBox())!;
            expect(f.y + f.height / 2, `пункт ${i}`).toBeLessThanOrEqual(b.y + b.height / 2 + 6);
        }
        const quarter = demo(page, 'quartercircle').locator('.p-speeddial-direction-up-left');
        await quarter.locator('.p-speeddial-button').click();
        await page.waitForTimeout(500);
        const qb = (await quarter.locator('.p-speeddial-button').boundingBox())!;
        const last = (await quarter.locator('.p-speeddial-item').last().boundingBox())!;
        expect(last.x + last.width / 2).toBeLessThanOrEqual(qb.x + qb.width / 2 + 14);
        expect(last.y + last.height / 2).toBeLessThanOrEqual(qb.y + qb.height / 2 + 14);
    });

    test('маска закрывает страницу под списком только пока он открыт', async ({ page }) => {
        const sd = demo(page, 'mask').locator('.p-speeddial');
        await expect(demo(page, 'mask').locator('.p-speeddial-mask')).toHaveCount(0);
        await sd.locator('.p-speeddial-button').click();
        const mask = demo(page, 'mask').locator('.p-speeddial-mask');
        await expect(mask).toBeVisible();
        // тема делает маску absolute: она закрывает ближайший позиционированный предок (контейнер демонстрации)
        expect(await mask.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const m = (await mask.boundingBox())!;
        const box = (await demo(page, 'mask').locator('div[style*="height: 350px"]').boundingBox())!;
        expect(Math.abs(m.width - box.width)).toBeLessThan(2);
        expect(Math.abs(m.height - box.height)).toBeLessThan(2);
        await page.mouse.click(5, 5);
        await expect(mask).toHaveCount(0);
    });

    test('подсказка пункта появляется при наведении', async ({ page }) => {
        const sd = demo(page, 'tooltip').locator('.p-speeddial').first();
        await sd.locator('.p-speeddial-button').click();
        await page.waitForTimeout(500);
        await sd.locator('.p-speeddial-item').first().hover();
        await expect(page.locator('.p-tooltip')).toBeVisible();
        await expect(page.locator('.p-tooltip')).toContainText('Add');
    });

    test('шаблоны: своя кнопка и свои пункты', async ({ page }) => {
        const d = demo(page, 'template');
        await d.getByTestId('custom-toggler').getByRole('button').click();
        await expect(d.locator('.p-speeddial')).toHaveClass(/p-speeddial-open/);
        await page.waitForTimeout(800);
        await d.getByTestId('custom-item').nth(1).click();
        await expect(d.locator('.p-speeddial')).not.toHaveClass(/p-speeddial-open/);
        await expect(page.locator('.p-toast-message').first()).toContainText('Data Updated');
    });
});
