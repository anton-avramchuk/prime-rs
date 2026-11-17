import { expect, test } from '@playwright/test';

// Regression: popup panels (Menu, Popover, ConfirmPopup, TieredMenu) closed "by themselves" on `scroll` when the browser
// shifted the page when the panel was inserted or on focus, although the user had not scrolled anything. Now document scrolling
// closes the panel only if the target has actually moved, and focus on show does not scroll the page.

const cases = [
    { name: 'Menu', url: '/menu', trigger: '[data-demo=popup] button', panel: '.p-menu-overlay' },
    { name: 'Popover', url: '/popover', trigger: '[data-demo=basic] button', panel: '.p-popover' },
    { name: 'ConfirmPopup', url: '/confirmpopup', trigger: '[data-demo=basic] button >> nth=0', panel: '.p-confirmpopup' },
    { name: 'TieredMenu', url: '/tieredmenu', trigger: '[data-demo=popup] button', panel: '#popup_menu' },
];

for (const c of cases) {
    test.describe(c.name, () => {
        test('a panel opened in a narrow window stays open, scrolling the target page closes it', async ({ page }) => {
            await page.setViewportSize({ width: 1280, height: 900 });
            await page.goto(c.url);
            await page.waitForLoadState('networkidle');
            await page.setViewportSize({ width: 360, height: 700 });
            await page.waitForTimeout(400);
            for (let i = 0; i < 4; i++) {
                await page.locator(c.trigger).first().click();
                const panel = page.locator(c.panel).first();
                await expect(panel).toBeVisible();
                // let late `scroll` and `resize` events pass
                await page.waitForTimeout(500);
                await expect(panel).toBeVisible();
                // real page scrolling moves the target and closes the panel
                await page.evaluate(() => window.scrollBy(0, 40));
                await page.waitForTimeout(200);
                const moved = await page.evaluate(() => window.scrollY > 0);
                if (moved) {
                    await expect(panel).toHaveCount(0);
                    await page.evaluate(() => window.scrollTo(0, 0));
                } else {
                    // the page does not scroll (no height): the panel stays
                    await page.keyboard.press('Escape');
                    await expect(panel).toHaveCount(0);
                }
                await page.waitForTimeout(100);
            }
        });
    });
}
