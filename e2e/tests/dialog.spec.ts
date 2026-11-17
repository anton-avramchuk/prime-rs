import { expect, test, type Page } from '@playwright/test';

const dialog = (page: Page) => page.locator('.p-dialog');
const box = async (page: Page) => (await dialog(page).boundingBox())!;

// the window appears over 300 ms: compare sizes and position after the enter animation
const settled = (page: Page) =>
    expect
        .poll(async () => {
            const a = await box(page);
            await page.waitForTimeout(60);
            const b = await box(page);
            return Math.abs(a.x - b.x) + Math.abs(a.y - b.y) + Math.abs(a.width - b.width) < 1;
        })
        .toBe(true);

test.describe('Dialog', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/dialog');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] button')).toBeVisible();
    });

    test('no window until opened', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(9);
        await expect(dialog(page)).toHaveCount(0);
        await expect(page.locator('.p-dialog-mask')).toHaveCount(0);
    });

    test('basic: dialog centered in body, mask, close via button', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        const d = dialog(page);
        await expect(d).toBeVisible();
        await expect(d).toHaveAttribute('role', 'dialog');
        await expect(d.locator('.p-dialog-title')).toHaveText('Edit Profile');
        await expect(page.locator('body > div > .p-dialog-mask > .p-dialog')).toHaveCount(1);
        await settled(page);
        const [b, v] = [await box(page), page.viewportSize()!];
        expect(Math.abs(b.x + b.width / 2 - v.width / 2)).toBeLessThan(2);
        expect(Math.abs(b.y + b.height / 2 - v.height / 2)).toBeLessThan(2);
        await expect(page.locator('.p-dialog-mask')).toHaveClass(/p-overlay-mask/);
        await d.getByRole('button', { name: 'Close' }).click();
        await expect(d).toHaveCount(0);
        await expect(page.locator('.p-dialog-mask')).toHaveCount(0);
    });

    test('Cancel closes, Escape closes, a mask click does not', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        await demo.getByRole('button', { name: 'Show' }).click();
        await page.locator('.p-dialog-mask').click({ position: { x: 5, y: 5 } });
        await expect(dialog(page)).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(dialog(page)).toHaveCount(0);
        await demo.getByRole('button', { name: 'Show' }).click();
        await dialog(page).getByRole('button', { name: 'Cancel' }).click();
        await expect(dialog(page)).toHaveCount(0);
    });

    test('modal locks scrolling, non-modal lets clicks through', async ({ page }) => {
        await page.locator('[data-demo=modal]').getByRole('button', { name: 'Show' }).click();
        await expect(page.locator('body')).toHaveClass(/p-overflow-hidden/);
        await page.keyboard.press('Escape');
        await expect(dialog(page)).toHaveCount(0);
        await expect(page.locator('body')).not.toHaveClass(/p-overflow-hidden/);

        await page.locator('[data-demo=withoutmodal]').getByRole('button', { name: 'Show' }).click();
        await expect(dialog(page)).toBeVisible();
        await expect(page.locator('body')).not.toHaveClass(/p-overflow-hidden/);
        // a click on the page behind the window reaches it (the mask does not intercept) and the window stays
        await page.locator('h2').click();
        await expect(dialog(page)).toBeVisible();
    });

    test('position: bottom right corner', async ({ page }) => {
        await page.locator('[data-demo=position]').getByRole('button', { name: 'BottomRight', exact: true }).click();
        await expect(page.locator('.p-dialog-mask')).toHaveClass(/p-dialog-bottomright/);
        await settled(page);
        const [b, v] = [await box(page), page.viewportSize()!];
        expect(b.x + b.width + 16).toBeGreaterThan(v.width - 2);
        expect(b.y + b.height + 16).toBeGreaterThan(v.height - 2);
    });

    test('dragging by the header with a real mouse', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await settled(page);
        const before = await box(page);
        const title = dialog(page).locator('.p-dialog-title');
        const t = (await title.boundingBox())!;
        await page.mouse.move(t.x + 10, t.y + 5);
        await page.mouse.down();
        await page.mouse.move(t.x + 10 + 120, t.y + 5 + 60, { steps: 6 });
        await page.mouse.up();
        const after = await box(page);
        expect(Math.abs(after.x - before.x - 120)).toBeLessThan(3);
        expect(Math.abs(after.y - before.y - 60)).toBeLessThan(3);
        // after release the window stays put
        await page.mouse.move(10, 10);
        expect(Math.abs((await box(page)).x - after.x)).toBeLessThan(1);
    });

    test('window cannot be dragged off screen', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await settled(page);
        const t = (await dialog(page).locator('.p-dialog-title').boundingBox())!;
        await page.mouse.move(t.x + 10, t.y + 5);
        await page.mouse.down();
        await page.mouse.move(t.x + 10 - 3000, t.y + 5, { steps: 8 });
        await page.mouse.up();
        expect((await box(page)).x).toBeGreaterThanOrEqual(-1);
    });

    test('resizing by the handle', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await settled(page);
        const before = await box(page);
        const h = (await dialog(page).locator('.p-resizable-handle').boundingBox())!;
        await page.mouse.move(h.x + 6, h.y + 6);
        await page.mouse.down();
        await page.mouse.move(h.x + 6 + 40, h.y + 6 + 20, { steps: 5 });
        await page.mouse.up();
        const after = await box(page);
        expect(after.width).toBeGreaterThan(before.width + 30);
    });

    test('maximizable: expands to full screen and restores', async ({ page }) => {
        await page.locator('[data-demo=maximizable]').getByRole('button', { name: 'Show' }).click();
        await settled(page);
        const normal = await box(page);
        const button = dialog(page).getByRole('button', { name: 'Maximize' });
        await button.click();
        await expect(dialog(page)).toHaveClass(/p-dialog-maximized/);
        const v = page.viewportSize()!;
        await expect.poll(async () => (await box(page)).width).toBeGreaterThanOrEqual(v.width - 1);
        await dialog(page).getByRole('button', { name: 'Minimize' }).click();
        await expect(dialog(page)).not.toHaveClass(/p-dialog-maximized/);
        await expect.poll(async () => Math.round((await box(page)).width)).toBe(Math.round(normal.width));
    });

    test('longcontent: content scrolls inside the window', async ({ page }) => {
        await page.locator('[data-demo=longcontent]').getByRole('button', { name: 'Show' }).click();
        const content = dialog(page).locator('.p-dialog-content');
        const [scroll, client] = await content.evaluate((el) => [el.scrollHeight, el.clientHeight]);
        expect(scroll).toBeGreaterThan(client);
        const v = page.viewportSize()!;
        expect((await box(page)).height).toBeLessThanOrEqual(v.height * 0.9 + 2);
    });

    test('responsive: 90vw width on a narrow screen', async ({ page }) => {
        await page.setViewportSize({ width: 500, height: 800 });
        await page.locator('[data-demo=responsive]').getByRole('button', { name: 'Show' }).click();
        await settled(page);
        expect(Math.abs((await box(page)).width - 450)).toBeLessThan(3);
    });

    test('template: custom header content and footer', async ({ page }) => {
        await page.locator('[data-demo=template]').getByRole('button', { name: 'Show' }).click();
        const d = dialog(page);
        await expect(d.locator('.p-dialog-header')).toContainText('Amy Elsner');
        await expect(d.locator('.p-dialog-footer')).toContainText('Save');
        await d.getByRole('button', { name: 'Cancel' }).click();
        await expect(d).toHaveCount(0);
    });

    test('headless: only custom content', async ({ page }) => {
        await page.locator('[data-demo=headless]').getByRole('button', { name: 'Login' }).click();
        const d = dialog(page);
        await expect(d).toContainText('Your Logo');
        await expect(d.locator('.p-dialog-header')).toHaveCount(0);
        await d.getByRole('button', { name: 'Cancel' }).click();
        await expect(d).toHaveCount(0);
    });

    test('Tab and Shift+Tab do not move focus outside the window', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await expect(dialog(page)).toBeVisible();
        const inside = () => page.evaluate(() => !!document.activeElement?.closest('.p-dialog'));
        await expect.poll(inside).toBe(true); // focus moved to the first field
        for (let i = 0; i < 6; i++) {
            await page.keyboard.press('Tab');
            expect(await inside()).toBe(true);
        }
        for (let i = 0; i < 6; i++) {
            await page.keyboard.press('Shift+Tab');
            expect(await inside()).toBe(true);
        }
    });
});
