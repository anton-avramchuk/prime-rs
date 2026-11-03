import { expect, test } from '@playwright/test';

test.describe('Dock', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/dock');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-dock')).toBeAttached();
    });

    test('демонстрация отрисована со стилями темы', async ({ page }) => {
        const dock = page.locator('[data-demo=basic] .p-dock');
        await expect(dock).toHaveClass(/p-dock-bottom/);
        await expect(dock.locator('.p-dock-item')).toHaveCount(6);
        expect(await dock.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const window = page.getByTestId('window');
        const w = (await window.boundingBox())!;
        const d = (await dock.boundingBox())!;
        // нижний док прижат к низу контейнера и растянут на его ширину
        expect(Math.abs(d.y + d.height - (w.y + w.height))).toBeLessThan(2);
        expect(Math.abs(d.width - w.width)).toBeLessThan(2);
        const container = dock.locator('.p-dock-list-container');
        expect(await container.evaluate((el) => getComputedStyle(el).borderTopLeftRadius)).not.toBe('0px');
        const size = (await dock.locator('.p-dock-item-link').first().boundingBox())!;
        expect(size.width).toBeGreaterThan(40);
    });

    test('выбор положения переносит док и меняет ориентацию', async ({ page }) => {
        const dock = page.locator('[data-demo=basic] .p-dock');
        const window = page.getByTestId('window');
        const w = (await window.boundingBox())!;
        await page.getByLabel('Left').check();
        await expect(dock).toHaveClass(/p-dock-left/);
        await expect(dock.locator('.p-dock-list')).toHaveAttribute('aria-orientation', 'vertical');
        const left = (await dock.boundingBox())!;
        expect(Math.abs(left.x - w.x)).toBeLessThan(2);
        expect(Math.abs(left.height - w.height)).toBeLessThan(2);
        const first = (await dock.locator('.p-dock-item').nth(0).boundingBox())!;
        const second = (await dock.locator('.p-dock-item').nth(1).boundingBox())!;
        expect(second.y).toBeGreaterThan(first.y + first.height - 2);
        await page.getByLabel('Top').check();
        await expect(dock).toHaveClass(/p-dock-top/);
        const top = (await dock.boundingBox())!;
        expect(Math.abs(top.y - w.y)).toBeLessThan(2);
        await page.getByLabel('Right').check();
        await expect(dock).toHaveClass(/p-dock-right/);
    });

    test('клавиатура: фокус на списке, стрелки, Home и End', async ({ page }) => {
        const dock = page.locator('[data-demo=basic] .p-dock');
        const list = dock.locator('.p-dock-list');
        await list.focus();
        await expect(dock.locator('.p-dock-item').first()).toHaveClass(/p-focus/);
        const id = await dock.locator('.p-dock-item').first().getAttribute('id');
        await expect(list).toHaveAttribute('aria-activedescendant', id!);
        await page.keyboard.press('ArrowRight');
        await expect(dock.locator('.p-dock-item').nth(1)).toHaveClass(/p-focus/);
        await page.keyboard.press('End');
        await expect(dock.locator('.p-dock-item').nth(5)).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(dock.locator('.p-dock-item').nth(5)).toHaveClass(/p-focus/);
        await page.keyboard.press('Home');
        await expect(dock.locator('.p-dock-item').first()).toHaveClass(/p-focus/);
        const ring = await dock.locator('.p-dock-item.p-focus').evaluate((el) => getComputedStyle(el).outlineStyle);
        expect(ring).not.toBe('none');
    });

    test('подсказка появляется у значка при наведении', async ({ page }) => {
        const dock = page.locator('[data-demo=basic] .p-dock');
        await dock.locator('.p-dock-item').first().hover();
        await expect(page.locator('.p-tooltip')).toBeVisible();
        await expect(page.locator('.p-tooltip')).toContainText('Finder');
    });
});
