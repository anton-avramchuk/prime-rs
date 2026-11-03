import { expect, test } from '@playwright/test';

test.describe('Image', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/image');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=preview] .p-image')).toBeAttached();
    });

    test('обычная картинка без кнопки просмотра', async ({ page }) => {
        const image = page.locator('[data-demo=basic] .p-image');
        await expect(image.locator('img')).toBeVisible();
        await expect(image.locator('.p-image-preview-mask')).toHaveCount(0);
    });

    test('кнопка просмотра накрывает картинку и проявляется при наведении', async ({ page }) => {
        const image = page.locator('[data-demo=preview] .p-image');
        const mask = image.locator('.p-image-preview-mask');
        expect(await image.evaluate((el) => getComputedStyle(el).position)).toBe('relative');
        const img = (await image.locator('img').boundingBox())!;
        const btn = (await mask.boundingBox())!;
        expect(Math.abs(btn.x - img.x)).toBeLessThan(2);
        expect(Math.abs(btn.width - img.width)).toBeLessThan(2);
        expect(Math.abs(btn.height - img.height)).toBeLessThan(2);
        expect(await mask.evaluate((el) => getComputedStyle(el).opacity)).toBe('0');
        await image.hover();
        await expect.poll(() => mask.evaluate((el) => getComputedStyle(el).opacity)).toBe('1');
    });

    test('просмотр: открытие, панель, поворот, масштаб и закрытие', async ({ page }) => {
        await page.locator('[data-demo=preview] .p-image-preview-mask').click();
        const mask = page.locator('.p-image-mask');
        await expect(mask).toBeVisible();
        const box = (await mask.boundingBox())!;
        const viewport = page.viewportSize()!;
        expect(Math.abs(box.width - viewport.width)).toBeLessThan(2);
        expect(Math.abs(box.height - viewport.height)).toBeLessThan(2);
        expect(await mask.evaluate((el) => getComputedStyle(el).position)).toBe('fixed');
        expect(await page.evaluate(() => document.body.classList.contains('p-overflow-hidden'))).toBe(true);
        await expect(page.locator('.p-image-toolbar button')).toHaveCount(5);
        await expect(page.locator('.p-image-close-button')).toBeFocused();
        const original = page.locator('img.p-image-original');
        await page.locator('.p-image-rotate-right-button').click();
        await expect(original).toHaveCSS('transform', /matrix\(/);
        await page.locator('.p-image-zoom-in-button').click();
        await expect(mask).toBeVisible();
        for (let i = 0; i < 3; i++) await page.locator('.p-image-zoom-in-button').click();
        await expect(page.locator('.p-image-zoom-in-button')).toBeDisabled();
        await page.locator('.p-image-close-button').click();
        await expect(mask).toHaveCount(0);
        expect(await page.evaluate(() => document.body.classList.contains('p-overflow-hidden'))).toBe(false);
    });

    test('клик по маске и Escape закрывают, клик по картинке — нет', async ({ page }) => {
        const trigger = page.locator('[data-demo=preview] .p-image-preview-mask');
        await trigger.click();
        await expect(page.locator('.p-image-mask')).toBeVisible();
        await page.locator('img.p-image-original').click();
        await expect(page.locator('.p-image-mask')).toBeVisible();
        await page.mouse.click(5, 5);
        await expect(page.locator('.p-image-mask')).toHaveCount(0);
        await trigger.click();
        await expect(page.locator('.p-image-mask')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-image-mask')).toHaveCount(0);
        await expect(trigger).toBeFocused();
    });

    test('просмотр показывает вторую картинку, если она задана', async ({ page }) => {
        await page.locator('[data-demo=previewimagesource] .p-image-preview-mask').click();
        const src = await page.locator('img.p-image-original').getAttribute('src');
        expect(decodeURIComponent(src!)).toContain('Large');
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-image-mask')).toHaveCount(0);
    });

    test('шаблоны: свой индикатор и свой просмотр', async ({ page }) => {
        const demo = page.locator('[data-demo=template]');
        await expect(demo.getByTestId('custom-indicator')).toBeAttached();
        await demo.locator('.p-image-preview-mask').click();
        await expect(page.locator('img.p-image-original')).toBeVisible();
        await page.locator('.p-image-rotate-left-button').click();
        await expect(page.locator('img.p-image-original')).toHaveCSS('transform', /matrix\(/);
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-image-mask')).toHaveCount(0);
    });
});
