import { expect, test } from '@playwright/test';

test.describe('Carousel', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1600, height: 900 });
        await page.goto('/carousel');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-carousel')).toBeAttached();
    });

    const demo = (page, name: string) => page.locator(`[data-demo=${name}] .p-carousel`);
    const active = (c) => c.locator('.p-carousel-item.p-carousel-item-active');

    test('theme styles: the viewport clips the list, items in a row, three visible', async ({ page }) => {
        const c = demo(page, 'basic');
        const viewport = c.locator('.p-carousel-viewport');
        expect(await viewport.evaluate((el) => getComputedStyle(el).overflow)).toBe('hidden');
        expect(await c.locator('.p-carousel-item-list').evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        await expect(active(c)).toHaveCount(3);
        const v = (await viewport.boundingBox())!;
        const first = (await c.locator('.p-carousel-item').first().boundingBox())!;
        expect(Math.abs(first.width - v.width / 3)).toBeLessThan(2);
        await expect(c.locator('.p-carousel-indicator')).toHaveCount(3);
        await expect(c.locator('.p-carousel-prev-button')).toHaveClass(/p-disabled/);
    });

    test('buttons and dots page by three items', async ({ page }) => {
        const c = demo(page, 'basic');
        await c.locator('.p-carousel-next-button').click();
        await expect(c.locator('.p-carousel-indicator').nth(1)).toHaveClass(/p-carousel-indicator-active/);
        await expect(active(c).first()).toContainText('Blue T-Shirt');
        await c.locator('.p-carousel-indicator-button').nth(2).click();
        await expect(active(c).first()).toContainText('Chakra Bracelet');
        await expect(c.locator('.p-carousel-next-button')).toHaveClass(/p-disabled/);
        await c.locator('.p-carousel-prev-button').click();
        await expect(c.locator('.p-carousel-indicator').nth(1)).toHaveClass(/p-carousel-indicator-active/);
        // after the transition inactive items are hidden
        await page.waitForTimeout(800);
        const hiddenOne = c.locator('.p-carousel-item:not(.p-carousel-item-active)').first();
        expect(await hiddenOne.evaluate((el) => getComputedStyle(el).visibility)).toBe('hidden');
    });

    test('responsiveness: fewer items are visible in a narrow window', async ({ page }) => {
        const c = demo(page, 'responsive');
        await expect(active(c)).toHaveCount(3);
        await page.setViewportSize({ width: 700, height: 900 });
        await expect(active(c)).toHaveCount(2);
        await page.setViewportSize({ width: 500, height: 900 });
        await expect(active(c)).toHaveCount(1);
        await expect(c.locator('.p-carousel-indicator')).toHaveCount(9);
        await page.setViewportSize({ width: 1600, height: 900 });
        await expect(active(c)).toHaveCount(3);
    });

    test('infinite mode with autoplay goes in a circle and stops on click', async ({ page }) => {
        const c = demo(page, 'circular');
        await expect(c.locator('.p-carousel-item-clone')).toHaveCount(6);
        await expect(c.locator('.p-carousel-content')).toHaveAttribute('aria-live', 'polite');
        const dots = c.locator('.p-carousel-indicator');
        await expect(dots.nth(0)).toHaveClass(/p-carousel-indicator-active/);
        await expect(dots.nth(1)).toHaveClass(/p-carousel-indicator-active/, { timeout: 5000 });
        // backward from page zero goes to the last one
        await c.locator('.p-carousel-indicator-button').nth(0).click();
        await expect(c.locator('.p-carousel-content')).toHaveAttribute('aria-live', 'off');
        await expect(dots.nth(0)).toHaveClass(/p-carousel-indicator-active/);
        await c.locator('.p-carousel-prev-button').click();
        await expect(dots.last()).toHaveClass(/p-carousel-indicator-active/);
        expect(await c.locator('.p-carousel-prev-button').getAttribute('class')).not.toContain('p-disabled');
        await page.waitForTimeout(3600);
        await expect(dots.last()).toHaveClass(/p-carousel-indicator-active/);
    });

    test('vertical mode pages along the Y axis in a viewport of a given height', async ({ page }) => {
        const c = demo(page, 'vertical');
        await expect(c).toHaveClass(/p-carousel-vertical/);
        const viewport = c.locator('.p-carousel-viewport');
        const h = (await viewport.boundingBox())!.height;
        expect(Math.abs(h - 330)).toBeLessThan(2);
        expect(await c.locator('.p-carousel-item-list').evaluate((el) => getComputedStyle(el).flexDirection)).toBe('column');
        await c.locator('.p-carousel-next-button').click();
        await page.waitForTimeout(700);
        const transform = await c.locator('.p-carousel-item-list').evaluate((el) => getComputedStyle(el).transform);
        expect(transform).toMatch(/matrix\(1, 0, 0, 1, 0, -?\d/);
        await expect(active(c)).toHaveCount(1);
        await expect(active(c)).toContainText('Black Watch');
    });

    test('keyboard: arrows on dots move focus, Enter opens the page', async ({ page }) => {
        const c = demo(page, 'basic');
        const buttons = c.locator('.p-carousel-indicator-button');
        await buttons.first().focus();
        await page.keyboard.press('ArrowRight');
        await expect(buttons.nth(1)).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(c.locator('.p-carousel-indicator').nth(1)).toHaveClass(/p-carousel-indicator-active/);
    });
});
