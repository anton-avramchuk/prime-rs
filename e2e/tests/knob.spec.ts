import { expect, test } from '@playwright/test';

test.describe('Knob', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/knob');
        // клики и перетаскивание до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-knob')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(10);
        const svg = page.locator('[data-demo=basic] svg');
        await expect(svg).toHaveAttribute('role', 'slider');
        await expect(svg).toHaveAttribute('aria-label', 'Basic');
        const box = (await svg.boundingBox())!;
        expect(Math.round(box.width)).toBe(100);
        const fill = await page.locator('[data-demo=basic] .p-knob-range').evaluate((el) => getComputedStyle(el).fill);
        expect(fill).toBe('none');
        await expect(page.locator('[data-demo=size] svg')).toHaveCSS('width', '200px');
    });

    test('клавиши и клик по шкале', async ({ page }) => {
        const svg = page.locator('[data-demo=basic] svg');
        await svg.focus();
        await page.keyboard.press('ArrowRight');
        await expect(page.getByTestId('echo')).toHaveText('1');
        await page.keyboard.press('PageUp');
        await expect(page.getByTestId('echo')).toHaveText('11');
        await page.keyboard.press('End');
        await expect(page.getByTestId('echo')).toHaveText('100');
        await page.keyboard.press('Home');
        await expect(page.getByTestId('echo')).toHaveText('0');

        const box = (await svg.boundingBox())!;
        const [cx, cy] = [box.x + box.width / 2, box.y + box.height / 2];
        await page.mouse.click(cx, cy - 40);
        await expect(page.getByTestId('echo')).toHaveText('50');
        // разрыв внизу шкалы значение не меняет
        await page.mouse.click(cx, cy + 40);
        await expect(page.getByTestId('echo')).toHaveText('50');
        await expect(svg).toHaveAttribute('aria-valuenow', '50');
    });

    test('перетаскивание следует за указателем даже за пределами шкалы', async ({ page }) => {
        const svg = page.locator('[data-demo=basic] svg');
        const box = (await svg.boundingBox())!;
        const [cx, cy] = [box.x + box.width / 2, box.y + box.height / 2];
        await page.mouse.move(cx - 34, cy + 20);
        await page.mouse.down();
        await expect(svg).toBeFocused();
        await page.mouse.move(cx, cy - 150, { steps: 4 });
        const top = Number(await page.getByTestId('echo').textContent());
        expect(top).toBeGreaterThanOrEqual(49);
        expect(top).toBeLessThanOrEqual(51);
        await page.mouse.move(cx + 150, cy - 10, { steps: 4 });
        const right = Number(await page.getByTestId('echo').textContent());
        expect(right).toBeGreaterThan(70);
        await page.mouse.up();
        // после отпускания перемещение мыши ничего не меняет
        await page.mouse.move(cx - 150, cy - 10, { steps: 3 });
        await expect(page.getByTestId('echo')).toHaveText(String(right));
    });

    test('шаблон, цвета, границы, шаг, readonly и disabled', async ({ page }) => {
        await expect(page.locator('[data-demo=template] text')).toHaveText('60%');
        await expect(page.locator('[data-demo=color] .p-knob-value')).toHaveAttribute('stroke', 'SlateGray');
        await expect(page.locator('[data-demo=color] .p-knob-range')).toHaveAttribute('stroke', 'MediumTurquoise');
        await expect(page.locator('[data-demo=stroke] path').first()).toHaveAttribute('stroke-width', '5');
        const minmax = page.locator('[data-demo=minmax] svg');
        await expect(minmax).toHaveAttribute('aria-valuemin', '-50');
        await expect(minmax).toHaveAttribute('aria-valuemax', '50');
        await expect(minmax).toHaveAttribute('aria-valuenow', '10');

        const stepped = page.locator('[data-demo=step] svg');
        await stepped.scrollIntoViewIfNeeded();
        const box = (await stepped.boundingBox())!;
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2 - 40);
        await expect(stepped).toHaveAttribute('aria-valuenow', '50');

        for (const demo of ['readonly', 'disabled']) {
            const svg = page.locator(`[data-demo=${demo}] svg`);
            await expect(svg).toHaveAttribute('tabindex', '-1');
            await svg.scrollIntoViewIfNeeded();
            const b = (await svg.boundingBox())!;
            const before = await svg.getAttribute('aria-valuenow');
            await page.mouse.click(b.x + b.width / 2 - 40, b.y + b.height / 2);
            await expect(svg).toHaveAttribute('aria-valuenow', before!);
        }
        await expect(page.locator('[data-demo=disabled] .p-knob')).toHaveClass(/p-disabled/);
    });

    test('кнопки управляют readonly-ручкой', async ({ page }) => {
        const svg = page.locator('[data-demo=reactive] svg[role=slider]');
        const buttons = page.locator('[data-demo=reactive] button');
        await expect(buttons.nth(1)).toBeDisabled();
        await buttons.nth(0).click();
        await buttons.nth(0).click();
        await expect(svg).toHaveAttribute('aria-valuenow', '2');
        await expect(buttons.nth(1)).toBeEnabled();
        await buttons.nth(1).click();
        await expect(svg).toHaveAttribute('aria-valuenow', '1');
    });
});
