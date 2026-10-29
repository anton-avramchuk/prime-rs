import { expect, test } from '@playwright/test';

test.describe('Slider', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/slider');
        // клики и перетаскивание до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-slider')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        const root = page.locator('[data-demo=basic] .p-slider');
        await expect(root).toHaveAttribute('data-p', 'horizontal');
        const height = await root.evaluate((el) => getComputedStyle(el).height);
        expect(height).toBe('3px');
        const handle = page.locator('[data-demo=basic] .p-slider-handle');
        await expect(handle).toHaveAttribute('role', 'slider');
        await expect(handle).toHaveAttribute('aria-label', 'Basic');
    });

    test('клавиши двигают ручку', async ({ page }) => {
        const handle = page.locator('[data-demo=basic] .p-slider-handle');
        await handle.focus();
        await page.keyboard.press('ArrowRight');
        await expect(page.getByTestId('echo')).toHaveText('1');
        await page.keyboard.press('PageUp');
        await expect(page.getByTestId('echo')).toHaveText('11');
        await page.keyboard.press('End');
        await expect(page.getByTestId('echo')).toHaveText('100');
        await expect(handle).toHaveAttribute('aria-valuenow', '100');
        await expect(handle).toBeFocused();
        await page.keyboard.press('Home');
        await expect(page.getByTestId('echo')).toHaveText('0');
    });

    test('клик по полосе и перетаскивание мышью', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-slider');
        const box = (await root.boundingBox())!;
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
        await expect.poll(async () => Number(await page.getByTestId('echo').textContent())).toBeGreaterThanOrEqual(48);
        await expect.poll(async () => Number(await page.getByTestId('echo').textContent())).toBeLessThanOrEqual(50);

        const handle = page.locator('[data-demo=basic] .p-slider-handle');
        const hb = (await handle.boundingBox())!;
        await page.mouse.move(hb.x + hb.width / 2, hb.y + hb.height / 2);
        await page.mouse.down();
        await expect(root).toHaveAttribute('data-p-sliding', 'true');
        await page.mouse.move(box.x + box.width * 0.75, hb.y + hb.height / 2, { steps: 5 });
        await page.mouse.up();
        await expect(root).toHaveAttribute('data-p-sliding', 'false');
        const dragged = Number(await page.getByTestId('echo').textContent());
        expect(dragged).toBeGreaterThanOrEqual(73);
        expect(dragged).toBeLessThanOrEqual(76);
        // после отпускания перемещение мыши ничего не меняет
        await page.mouse.move(box.x + 5, hb.y + hb.height / 2, { steps: 3 });
        await expect(page.getByTestId('echo')).toHaveText(String(dragged));
    });

    test('диапазон, шаг и вертикальная ориентация', async ({ page }) => {
        const handles = page.locator('[data-demo=range] .p-slider-handle');
        await expect(handles).toHaveCount(2);
        await handles.nth(1).focus();
        await page.keyboard.press('ArrowLeft');
        await expect(page.getByTestId('echo-range')).toHaveText('20,79');
        await handles.nth(0).focus();
        await page.keyboard.press('End');
        await expect(page.getByTestId('echo-range')).toHaveText('79,100');
        await expect(handles.nth(1)).toBeFocused();

        const stepped = page.locator('[data-demo=step] .p-slider-handle');
        await stepped.focus();
        await page.keyboard.press('ArrowRight');
        await expect(page.getByTestId('echo-step')).toHaveText('40');

        const vertical = page.locator('[data-demo=vertical] .p-slider-handle');
        await expect(page.locator('[data-demo=vertical] .p-slider')).toHaveClass(/p-slider-vertical/);
        await vertical.focus();
        await page.keyboard.press('ArrowUp');
        await expect(vertical).toHaveAttribute('aria-valuenow', '51');
        await expect(vertical).toHaveAttribute('aria-orientation', 'vertical');
    });

    test('фильтр и связь с полем ввода', async ({ page }) => {
        const target = page.getByTestId('filter-target');
        await expect(target).toHaveCSS('filter', /contrast\(1\)/);
        await page.locator('[data-demo=filter] .p-togglebutton').nth(1).click();
        const handle = page.locator('[data-demo=filter] .p-slider-handle');
        await handle.focus();
        await page.keyboard.press('End');
        await expect(target).toHaveCSS('filter', /brightness\(2\)/);

        const input = page.locator('[data-demo=input] input');
        const slider = page.locator('[data-demo=input] .p-slider-handle');
        await expect(slider).toHaveAttribute('aria-valuenow', '50');
        await input.fill('30');
        await expect(slider).toHaveAttribute('aria-valuenow', '30');
        await slider.focus();
        await page.keyboard.press('ArrowRight');
        await expect(input).toHaveValue('31');
    });
});
