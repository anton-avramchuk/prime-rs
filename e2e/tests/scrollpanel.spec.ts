import { expect, test } from '@playwright/test';

test.describe('ScrollPanel', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/scrollpanel');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-scrollpanel')).toBeVisible();
        // bars are computed after hydration
        await expect(page.locator('[data-demo=basic] .p-scrollpanel-bar-y')).toHaveAttribute('data-p-scrollpanel-hidden', 'false');
    });

    test('demos are rendered with theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        expect(await root.evaluate((el) => el.getBoundingClientRect().height)).toBeCloseTo(150, 0);
        const content = page.locator('[data-demo=basic] .p-scrollpanel-content');
        expect(await content.evaluate((el) => getComputedStyle(el).scrollbarWidth)).toBe('none');
    });

    test('bar ARIA and link with the content', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        const id = await root.locator('.p-scrollpanel-content').getAttribute('id');
        expect(id).toMatch(/^pn_id_\d+_content$/);
        const y = root.locator('.p-scrollpanel-bar-y');
        await expect(y).toHaveAttribute('role', 'scrollbar');
        await expect(y).toHaveAttribute('aria-orientation', 'vertical');
        await expect(y).toHaveAttribute('aria-controls', id!);
        await expect(root.locator('.p-scrollpanel-bar-x')).toHaveAttribute('aria-orientation', 'horizontal');
    });

    test('the horizontal bar is hidden, the vertical one is visible', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        await expect(root.locator('.p-scrollpanel-bar-x')).toHaveClass(/p-scrollpanel-hidden/);
        await expect(root.locator('.p-scrollpanel-bar-y')).not.toHaveClass(/p-scrollpanel-hidden/);
    });

    test('wheel scrolling moves the bar and updates aria-valuenow', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        const y = root.locator('.p-scrollpanel-bar-y');
        const before = await y.evaluate((el) => (el as HTMLElement).style.top);
        await root.hover();
        await page.mouse.wheel(0, 80);
        await expect.poll(() => root.locator('.p-scrollpanel-content').evaluate((el) => el.scrollTop)).toBeGreaterThan(0);
        await expect.poll(() => y.evaluate((el) => (el as HTMLElement).style.top)).not.toBe(before);
        await expect.poll(async () => Number(await y.getAttribute('aria-valuenow'))).toBeGreaterThan(0);
    });

    test('dragging the vertical bar scrolls the content', async ({ page }) => {
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

    test('arrow down on the bar scrolls the content', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-scrollpanel');
        await root.locator('.p-scrollpanel-bar-y').focus();
        // as in PrimeNG, scrolling happens 40 ms after the press and keyup cancels it: the key must be held down
        await page.keyboard.down('ArrowDown');
        await expect.poll(() => root.locator('.p-scrollpanel-content').evaluate((el) => el.scrollTop)).toBe(5);
        await page.keyboard.up('ArrowDown');
    });

    test('custom: the bar color comes from primary', async ({ page }) => {
        const custom = page.locator('[data-demo=custom] .p-scrollpanel');
        await expect(custom).toHaveClass(/custombar/);
        const bar = await custom.locator('.p-scrollpanel-bar-y').evaluate((el) => getComputedStyle(el).backgroundColor);
        const plain = await page.locator('[data-demo=basic] .p-scrollpanel-bar-y').evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(bar).not.toBe(plain);
    });
});
