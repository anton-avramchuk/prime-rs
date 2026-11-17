import { expect, test } from '@playwright/test';

test.describe('Drawer', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/drawer');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] button')).toBeVisible();
    });

    test('there is no panel until it is opened', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        await expect(page.locator('.p-drawer')).toHaveCount(0);
        await expect(page.locator('.p-drawer-mask')).toHaveCount(0);
    });

    test('opening renders the panel and mask in body', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        const panel = page.locator('.p-drawer');
        await expect(panel).toBeVisible();
        await expect(panel).toHaveAttribute('role', 'complementary');
        await expect(panel.locator('.p-drawer-title')).toHaveText('Drawer');
        await expect(page.locator('body > div .p-drawer, body > div > .p-drawer')).toHaveCount(1);
        await expect(page.locator('.p-drawer-mask')).toBeVisible();
        const box = (await panel.boundingBox())!;
        expect(box.x).toBeLessThanOrEqual(1);
        expect(box.width).toBeGreaterThan(250);
    });

    test('closing with the button removes the panel and mask', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await page.getByRole('button', { name: 'Close' }).click();
        await expect(page.locator('.p-drawer')).toHaveCount(0);
        await expect(page.locator('.p-drawer-mask')).toHaveCount(0);
    });

    test('clicking the mask closes, clicking the panel does not', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await page.locator('.p-drawer-content').click();
        await expect(page.locator('.p-drawer')).toBeVisible();
        await page.locator('.p-drawer-mask').click({ position: { x: 900, y: 300 } });
        await expect(page.locator('.p-drawer')).toHaveCount(0);
    });

    test('Escape closes the panel', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await expect(page.locator('.p-drawer')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-drawer')).toHaveCount(0);
    });

    test('positions: right, top, bottom', async ({ page }) => {
        const demo = page.locator('[data-demo=position]');
        const viewport = page.viewportSize()!;
        const buttons = demo.getByRole('button');
        await buttons.nth(1).click();
        let box = (await page.locator('.p-drawer').boundingBox())!;
        expect(box.x + box.width).toBeGreaterThanOrEqual(viewport.width - 1);
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-drawer')).toHaveCount(0);
        await buttons.nth(2).click();
        box = (await page.locator('.p-drawer').boundingBox())!;
        expect(box.y).toBeLessThanOrEqual(1);
        expect(box.width).toBeGreaterThanOrEqual(viewport.width - 1);
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-drawer')).toHaveCount(0);
        await buttons.nth(3).click();
        box = (await page.locator('.p-drawer').boundingBox())!;
        expect(box.y + box.height).toBeGreaterThanOrEqual(viewport.height - 1);
    });

    test('fullscreen takes the whole screen', async ({ page }) => {
        await page.locator('[data-demo=fullscreen] button').click();
        const viewport = page.viewportSize()!;
        // the panel slides in over 0.5 s: compare sizes after the enter animation
        await expect
            .poll(async () => {
                const box = (await page.locator('.p-drawer').boundingBox())!;
                return box.width >= viewport.width - 1 && box.height >= viewport.height - 1;
            })
            .toBe(true);
    });

    test('template: custom header and footer, no close button', async ({ page }) => {
        await page.locator('[data-demo=template] button').click();
        const panel = page.locator('.p-drawer');
        await expect(panel.locator('.p-drawer-header')).toContainText('Amy Elsner');
        await expect(panel.locator('.p-drawer-footer')).toContainText('Account');
        await expect(panel.locator('.p-drawer-close-button')).toHaveCount(0);
    });

    test('headless: only custom content', async ({ page }) => {
        await page.locator('[data-demo=headless]').getByRole('button', { name: 'Show' }).click();
        const panel = page.locator('.p-drawer');
        await expect(panel).toContainText('Your Logo');
        await expect(panel.locator('.p-drawer-header')).toHaveCount(0);
        await panel.getByRole('button').first().click();
        await expect(panel).toHaveCount(0);
    });

    test('Tab and Shift+Tab do not move focus outside the panel', async ({ page }) => {
        await page.locator('[data-demo=template] button').click();
        const panel = page.locator('.p-drawer');
        await expect(panel).toBeVisible();
        // focus on open stays on the opener button (as in PrimeNG): the trap works once it is already inside the panel
        await panel.getByRole('button', { name: 'Logout' }).focus();
        const inside = () => page.evaluate(() => !!document.activeElement?.closest('.p-drawer'));
        for (let i = 0; i < 4; i++) {
            await page.keyboard.press('Tab');
            expect(await inside()).toBe(true);
        }
        for (let i = 0; i < 4; i++) {
            await page.keyboard.press('Shift+Tab');
            expect(await inside()).toBe(true);
        }
    });
});
