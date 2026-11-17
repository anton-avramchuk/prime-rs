import { expect, test } from '@playwright/test';

test.describe('Galleria', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/galleria');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_galleria')).toBeAttached();
    });

    test('theme styles: frame, thumbnails in a row', async ({ page }) => {
        const root = page.locator('#basic_galleria');
        await expect(root).toHaveClass(/p-galleria-thumbnails-bottom/);
        expect(await root.evaluate((el) => getComputedStyle(el).overflow)).toBe('hidden');
        const items = root.locator('.p-galleria-thumbnail-item');
        await expect(items).toHaveCount(8);
        const first = (await items.nth(0).boundingBox())!;
        const second = (await items.nth(1).boundingBox())!;
        expect(Math.abs(first.y - second.y)).toBeLessThan(1);
        expect(second.x).toBeGreaterThan(first.x);
        await expect(root.locator('.p-galleria-item img')).toHaveCount(1);
    });

    test('clicking a thumbnail shows the frame and shifts the strip', async ({ page }) => {
        const root = page.locator('#basic_galleria');
        const before = await root.locator('.p-galleria-item img').getAttribute('src');
        await root.locator('.p-galleria-thumbnail').nth(4).click();
        await expect(root.locator('.p-galleria-thumbnail-item').nth(4)).toHaveClass(/p-galleria-thumbnail-item-current/);
        expect(await root.locator('.p-galleria-item img').getAttribute('src')).not.toBe(before);
        const ribbon = await root.locator('.p-galleria-thumbnail-items').getAttribute('style');
        expect(ribbon).toContain('translate3d(-');
    });

    test('page buttons and thumbnail keys', async ({ page }) => {
        const root = page.locator('#basic_galleria');
        const next = root.locator('.p-galleria-thumbnail-next-button');
        await next.click();
        await expect(root.locator('.p-galleria-thumbnail-item').nth(1)).toHaveClass(/p-galleria-thumbnail-item-current/);
        await root.locator('.p-galleria-thumbnail').nth(1).focus();
        await page.keyboard.press('ArrowRight');
        await expect(root.locator('.p-galleria-thumbnail').nth(2)).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(root.locator('.p-galleria-thumbnail-item').nth(2)).toHaveClass(/p-galleria-thumbnail-item-current/);
        await page.keyboard.press('End');
        await page.keyboard.press('Space');
        await expect(root.locator('.p-galleria-thumbnail-item').nth(7)).toHaveClass(/p-galleria-thumbnail-item-current/);
        await expect(next).toHaveClass(/p-disabled/);
    });

    test('auto-advance runs and stops on click', async ({ page }) => {
        const root = page.locator('#autoplay_galleria');
        await root.scrollIntoViewIfNeeded();
        const current = root.locator('.p-galleria-thumbnail-item-current');
        await expect(current).toHaveCount(1);
        const initial = await root.locator('.p-galleria-item img').getAttribute('src');
        await expect.poll(async () => root.locator('.p-galleria-item img').getAttribute('src'), { timeout: 8000 }).not.toBe(initial);
        await root.locator('.p-galleria-thumbnail').nth(0).click();
        const stopped = await root.locator('.p-galleria-item img').getAttribute('src');
        await page.waitForTimeout(2600);
        expect(await root.locator('.p-galleria-item img').getAttribute('src')).toBe(stopped);
    });

    test('caption, frame buttons and indicators', async ({ page }) => {
        const root = page.locator('#caption_galleria');
        await root.scrollIntoViewIfNeeded();
        await expect(root.locator('.p-galleria-caption')).toContainText('Title 1');
        await root.locator('.p-galleria-next-button').click();
        await expect(root.locator('.p-galleria-caption')).toContainText('Title 2');
        await root.locator('.p-galleria-indicator').nth(3).click();
        await expect(root.locator('.p-galleria-caption')).toContainText('Title 4');
        await expect(root.locator('.p-galleria-indicator').nth(3)).toHaveClass(/p-galleria-indicator-active/);
        await expect(root.locator('.p-galleria-thumbnails')).toHaveCount(0);
    });

    test('vertical thumbnails', async ({ page }) => {
        const root = page.locator('#thumbnail_galleria');
        await root.scrollIntoViewIfNeeded();
        await page.getByRole('button', { name: 'Left' }).click();
        await expect(root).toHaveClass(/p-galleria-thumbnails-left/);
        const viewport = (await root.locator('.p-galleria-thumbnails-viewport').boundingBox())!;
        expect(Math.round(viewport.height)).toBe(250);
        await expect(root.locator('.p-galleria-thumbnail-items')).toHaveAttribute('style', /translate3d\(0, /);
    });

    test('fullscreen mode: opening, closing by mask and button', async ({ page }) => {
        await page.locator('.fs-thumb').nth(2).scrollIntoViewIfNeeded();
        await page.locator('.fs-thumb').nth(2).click();
        const mask = page.locator('.p-galleria-mask');
        await expect(mask).toBeVisible();
        await expect(mask.locator('.p-galleria')).toHaveClass(/p-galleria-fullscreen/);
        await expect(mask.locator('.p-galleria-thumbnail-item').nth(2)).toHaveClass(/p-galleria-thumbnail-item-current/);
        // click on the frame does not close
        await mask.locator('.p-galleria-item').click();
        await expect(mask).toBeVisible();
        // click on the mask closes
        await page.mouse.click(5, 5);
        await expect(mask).toHaveCount(0);
        // close button
        await page.locator('.fs-thumb').nth(0).click();
        await expect(mask).toBeVisible();
        await page.locator('.p-galleria-close-button').click();
        await expect(mask).toHaveCount(0);
    });
});
