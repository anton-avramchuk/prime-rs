import { expect, test } from '@playwright/test';

test.describe('ColorPicker', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/colorpicker');
        // events before hydration (wasm load) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_cp')).toBeAttached();
    });

    test('theme styles: 24×24 swatch, panel closed', async ({ page }) => {
        const preview = page.locator('#basic_cp .p-colorpicker-preview');
        const box = (await preview.boundingBox())!;
        expect(Math.round(box.width)).toBe(24);
        expect(Math.round(box.height)).toBe(24);
        await expect(page.locator('#basic_cp .p-colorpicker-panel')).toHaveCount(0);
        await expect(preview).toHaveAttribute('aria-label', 'Select Color');
    });

    test('click opens the panel, Escape and outside click close it', async ({ page }) => {
        const preview = page.locator('#basic_cp .p-colorpicker-preview');
        const panel = page.locator('#basic_cp .p-colorpicker-panel');
        await preview.click();
        await expect(panel).toBeVisible();
        const box = (await panel.boundingBox())!;
        expect(Math.round(box.width)).toBe(193);
        expect(Math.round(box.height)).toBe(166);
        await preview.focus();
        await page.keyboard.press('Escape');
        await expect(panel).toHaveCount(0);
        await preview.focus();
        await page.keyboard.press('Space');
        await expect(panel).toBeVisible();
        await page.mouse.click(5, 500);
        await expect(panel).toHaveCount(0);
    });

    test('click on the field and scale changes the color', async ({ page }) => {
        const panel = page.locator('[data-demo=inline] .p-colorpicker-panel');
        await expect(panel).toBeVisible();
        const field = page.locator('[data-demo=inline] .p-colorpicker-color-selector');
        const hue = page.locator('[data-demo=inline] .p-colorpicker-hue');
        const f = (await field.boundingBox())!;
        await page.mouse.click(f.x + 150, f.y + 0.5);
        await expect(page.getByTestId('inline-echo')).toHaveText(/^#f[c-f]0[0-3]0[0-3]$/);
        await page.mouse.click(f.x + 75, f.y + 75);
        await expect(page.getByTestId('inline-echo')).toHaveText(/^#[0-9a-f]{6}$/);
        const h = (await hue.boundingBox())!;
        await page.mouse.click(h.x + 8, h.y + 50);
        // hue 240 → the blue channel dominates
        const echo = (await page.getByTestId('inline-echo').textContent())!;
        expect(parseInt(echo.slice(5, 7), 16)).toBeGreaterThan(parseInt(echo.slice(1, 3), 16));
    });

    test('dragging over the field is limited to the area', async ({ page }) => {
        await page.locator('[data-demo=inline] .p-colorpicker-panel').scrollIntoViewIfNeeded();
        const field = page.locator('[data-demo=inline] .p-colorpicker-color-selector');
        const f = (await field.boundingBox())!;
        await page.mouse.move(f.x + 10, f.y + 10);
        await page.mouse.down();
        await expect(page.locator('[data-demo=inline] .p-colorpicker')).toHaveClass(/p-colorpicker-dragging/);
        await page.mouse.move(f.x + 400, f.y + 400, { steps: 5 });
        await expect(page.getByTestId('inline-echo')).toHaveText('#000000');
        await page.mouse.up();
        await expect(page.locator('[data-demo=inline] .p-colorpicker')).not.toHaveClass(/p-colorpicker-dragging/);
    });

    test('rgb and hsb formats', async ({ page }) => {
        for (const [id, echo, pattern] of [
            ['cp-rgb', 'rgb-echo', /^r:\d+ g:\d+ b:\d+$/],
            ['cp-hsb', 'hsb-echo', /^h:\d+ s:\d+ b:\d+$/],
        ] as const) {
            await page.locator(`#${id}`).click();
            const panel = page.locator('[data-demo=format] .p-colorpicker-panel');
            await expect(panel).toBeVisible();
            const f = (await panel.locator('.p-colorpicker-color-selector').boundingBox())!;
            await page.mouse.click(f.x + 150, f.y + 0.5);
            await expect(page.getByTestId(echo)).toHaveText(pattern);
            await page.locator('h2').click();
            await expect(panel).toHaveCount(0);
        }
        await expect(page.getByTestId('rgb-echo')).toHaveText(/^r:255 g:[0-3] b:[0-3]$/);
    });

    test('a disabled one does not open', async ({ page }) => {
        const preview = page.locator('#disabled_cp .p-colorpicker-preview');
        await expect(preview).toBeDisabled();
        await preview.click({ force: true });
        await expect(page.locator('#disabled_cp .p-colorpicker-panel')).toHaveCount(0);
    });
});
