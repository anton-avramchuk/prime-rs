import { expect, test } from '@playwright/test';

test.describe('ScrollPanel', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/scrollpanel');
        await expect(page.locator('[data-demo=basic] .p-scrollpanel')).toBeVisible();
        // полосы считаются после гидрации
        await expect(page.locator('[data-demo=basic] .p-scrollpanel-bar-y')).toHaveAttribute('data-p-scrollpanel-hidden', 'false');
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        expect(await root.evaluate((el) => el.getBoundingClientRect().height)).toBeCloseTo(150, 0);
        const content = page.locator('[data-demo=basic] .p-scrollpanel-content');
        expect(await content.evaluate((el) => getComputedStyle(el).scrollbarWidth)).toBe('none');
    });

    test('ARIA полос и связь с содержимым', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        const id = await root.locator('.p-scrollpanel-content').getAttribute('id');
        expect(id).toMatch(/^pn_id_\d+_content$/);
        const y = root.locator('.p-scrollpanel-bar-y');
        await expect(y).toHaveAttribute('role', 'scrollbar');
        await expect(y).toHaveAttribute('aria-orientation', 'vertical');
        await expect(y).toHaveAttribute('aria-controls', id!);
        await expect(root.locator('.p-scrollpanel-bar-x')).toHaveAttribute('aria-orientation', 'horizontal');
    });

    test('горизонтальная полоса скрыта, вертикальная видна', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        await expect(root.locator('.p-scrollpanel-bar-x')).toHaveClass(/p-scrollpanel-hidden/);
        await expect(root.locator('.p-scrollpanel-bar-y')).not.toHaveClass(/p-scrollpanel-hidden/);
    });

    test('прокрутка колесом сдвигает полосу и обновляет aria-valuenow', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        const y = root.locator('.p-scrollpanel-bar-y');
        const before = await y.evaluate((el) => (el as HTMLElement).style.top);
        await root.hover();
        await page.mouse.wheel(0, 80);
        await expect.poll(() => root.locator('.p-scrollpanel-content').evaluate((el) => el.scrollTop)).toBeGreaterThan(0);
        await expect.poll(() => y.evaluate((el) => (el as HTMLElement).style.top)).not.toBe(before);
        await expect.poll(async () => Number(await y.getAttribute('aria-valuenow'))).toBeGreaterThan(0);
    });

    test('перетаскивание вертикальной полосы прокручивает содержимое', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        await root.hover();
        const bar = root.locator('.p-scrollpanel-bar-y');
        const box = (await bar.boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + 5);
        await page.mouse.down();
        await expect(bar).toHaveClass(/p-scrollpanel-grabbed/);
        await page.mouse.move(box.x + box.width / 2, box.y + 40, { steps: 5 });
        await expect.poll(() => root.locator('.p-scrollpanel-content').evaluate((el) => el.scrollTop)).toBeGreaterThan(20);
        await page.mouse.up();
        await expect(bar).not.toHaveClass(/p-scrollpanel-grabbed/);
    });

    test('стрелка вниз на полосе прокручивает содержимое', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        await root.locator('.p-scrollpanel-bar-y').focus();
        // как в PrimeNG, прокрутка идёт через 40 мс после нажатия, а keyup отменяет её: клавишу нужно удерживать
        await page.keyboard.down('ArrowDown');
        await expect.poll(() => root.locator('.p-scrollpanel-content').evaluate((el) => el.scrollTop)).toBe(5);
        await page.keyboard.up('ArrowDown');
    });

    test('custom: цвет полосы берётся из primary', async ({ page }) => {
        const custom = page.locator('[data-demo=custom] .p-scrollpanel');
        await expect(custom).toHaveClass(/custombar/);
        const bar = await custom.locator('.p-scrollpanel-bar-y').evaluate((el) => getComputedStyle(el).backgroundColor);
        const plain = await page.locator('[data-demo=basic] .p-scrollpanel-bar-y').evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(bar).not.toBe(plain);
    });
});
