import { expect, test } from '@playwright/test';

test.describe('ImageCompare', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/imagecompare');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-imagecompare')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const root = page.locator('[data-demo=basic] .p-imagecompare');
        await expect(root.locator('img')).toHaveCount(2);
        await expect(root.locator('input[type=range]')).toHaveValue('50');
        expect(await root.evaluate((el) => getComputedStyle(el).position)).toBe('relative');
        expect(await root.evaluate((el) => getComputedStyle(el).overflow)).toBe('hidden');
        const box = (await root.boundingBox())!;
        expect(Math.abs(box.width / box.height - 16 / 9)).toBeLessThan(0.05);
        // правое изображение обрезано темой до 50 %
        const clip = await root.locator('img').nth(1).evaluate((el) => getComputedStyle(el).clipPath);
        expect(clip).toContain('polygon');
    });

    test('клик по полосе двигает ручку и обрезает правое изображение', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-imagecompare');
        const box = (await root.boundingBox())!;
        await page.mouse.click(box.x + box.width * 0.25, box.y + box.height / 2);
        const value = Number(await root.locator('input[type=range]').inputValue());
        expect(value).toBeGreaterThan(15);
        expect(value).toBeLessThan(35);
        const inline = await root.locator('img').nth(1).evaluate((el) => (el as HTMLElement).style.clipPath);
        expect(inline).toContain(`${value}%`);
        const left = await root.locator('img').first().evaluate((el) => (el as HTMLElement).style.clipPath);
        expect(left).toBe('');
    });

    test('перетаскивание ручки меняет значение', async ({ page }) => {
        const root = page.locator('[data-demo=responsive] .p-imagecompare');
        // демонстрация ниже экрана: без прокрутки координаты мыши лежат вне окна
        await root.scrollIntoViewIfNeeded();
        const box = (await root.boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
        await page.mouse.down();
        await page.mouse.move(box.x + box.width * 0.8, box.y + box.height / 2, { steps: 5 });
        await page.mouse.up();
        const value = Number(await root.locator('input[type=range]').inputValue());
        expect(value).toBeGreaterThan(70);
    });

    test('клавиши стрелок меняют значение ползунка', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-imagecompare');
        const slider = root.locator('input[type=range]');
        await slider.focus();
        await page.keyboard.press('ArrowRight');
        await page.keyboard.press('ArrowRight');
        await expect(slider).toHaveValue('52');
        const inline = await root.locator('img').nth(1).evaluate((el) => (el as HTMLElement).style.clipPath);
        expect(inline).toContain('52%');
    });
});
