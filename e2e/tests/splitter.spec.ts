import { expect, test, type Locator } from '@playwright/test';

async function width(el: Locator) {
    return (await el.boundingBox())!.width;
}

async function height(el: Locator) {
    return (await el.boundingBox())!.height;
}

test.describe('Splitter', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/splitter');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=horizontal] .p-splitter')).toBeVisible();
    });

    test('структура: четыре демонстрации, панели и разделители', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        const demo = page.locator('[data-demo=horizontal]');
        await expect(demo.locator('.p-splitterpanel')).toHaveCount(2);
        await expect(demo.locator('.p-splitter-gutter')).toHaveCount(1);
        await expect(demo.locator('.p-splitter-gutter')).toHaveAttribute('role', 'separator');
        await expect(demo.locator('.p-splitter-gutter-handle')).toHaveAttribute('tabindex', '0');
        await expect(page.locator('[data-demo=vertical] .p-splitter')).toHaveClass(/p-splitter-vertical/);
    });

    test('размеры: поровну и заданные', async ({ page }) => {
        const equal = page.locator('[data-demo=horizontal] .p-splitterpanel');
        expect(Math.abs((await width(equal.nth(0))) - (await width(equal.nth(1))))).toBeLessThan(2);
        const sized = page.locator('[data-demo=size] .p-splitterpanel');
        const [a, b] = [await width(sized.nth(0)), await width(sized.nth(1))];
        expect(a / (a + b)).toBeCloseTo(0.25, 1);
    });

    test('перетаскивание мышью сдвигает границу и снимает состояние после отпускания', async ({ page }) => {
        const demo = page.locator('[data-demo=horizontal]');
        const panels = demo.locator('.p-splitterpanel');
        const before = await width(panels.nth(0));
        const box = (await demo.locator('.p-splitter-gutter').boundingBox())!;
        const x = box.x + box.width / 2;
        const y = box.y + box.height / 2;
        await page.mouse.move(x, y);
        await page.mouse.down();
        await page.mouse.move(x + 60, y, { steps: 6 });
        await expect(demo.locator('.p-splitter')).toHaveClass(/p-splitter-resizing/);
        await expect(demo.locator('.p-splitter-gutter')).toHaveClass(/p-splitter-gutter-resizing/);
        await page.mouse.up();
        await expect(demo.locator('.p-splitter')).not.toHaveClass(/p-splitter-resizing/);
        expect(Math.abs((await width(panels.nth(0))) - before - 60)).toBeLessThan(3);
        // после отпускания движение мыши ничего не меняет
        const after = await width(panels.nth(0));
        await page.mouse.move(x - 80, y);
        expect(Math.abs((await width(panels.nth(0))) - after)).toBeLessThan(1);
    });

    test('вертикальный: перетаскивание меняет высоту', async ({ page }) => {
        const demo = page.locator('[data-demo=vertical]');
        await demo.locator('.p-splitter').scrollIntoViewIfNeeded();
        const panels = demo.locator('.p-splitterpanel');
        const before = await height(panels.nth(0));
        const box = (await demo.locator('.p-splitter-gutter').boundingBox())!;
        const x = box.x + box.width / 2;
        const y = box.y + box.height / 2;
        await page.mouse.move(x, y);
        await page.mouse.down();
        await page.mouse.move(x, y + 40, { steps: 4 });
        await page.mouse.up();
        expect(Math.abs((await height(panels.nth(0))) - before - 40)).toBeLessThan(3);
    });

    test('стрелки на ручке двигают границу на шаг', async ({ page }) => {
        const demo = page.locator('[data-demo=horizontal]');
        const panels = demo.locator('.p-splitterpanel');
        const handle = demo.locator('.p-splitter-gutter-handle');
        const before = await width(panels.nth(0));
        await handle.focus();
        await page.keyboard.press('ArrowRight');
        await page.keyboard.press('ArrowRight');
        expect(Math.abs((await width(panels.nth(0))) - before - 10)).toBeLessThan(1.5);
        await page.keyboard.press('ArrowLeft');
        expect(Math.abs((await width(panels.nth(0))) - before - 5)).toBeLessThan(1.5);
        await expect(handle).toHaveAttribute('aria-valuenow', /^\d+\.\d{4}$/);
    });

    test('вложенный: минимальный размер первой панели не нарушается', async ({ page }) => {
        const demo = page.locator('[data-demo=nested]');
        const outer = demo.locator('.p-splitter').first();
        const first = outer.locator(':scope > .p-splitterpanel').first();
        const total = await width(outer);
        const box = (await outer.locator(':scope > .p-splitter-gutter').boundingBox())!;
        const x = box.x + box.width / 2;
        const y = box.y + box.height / 2;
        await page.mouse.move(x, y);
        await page.mouse.down();
        await page.mouse.move(x - total * 0.5, y, { steps: 10 });
        await page.mouse.up();
        expect(await width(first)).toBeGreaterThanOrEqual(total * 0.1 - 6);
        await expect(demo.locator('.p-splitter')).toHaveCount(3);
        await expect(demo.locator('.p-splitterpanel-nested').first()).toBeVisible();
    });
});
