import { expect, test } from '@playwright/test';

// Регрессия: всплывающие панели (Menu, Popover, ConfirmPopup, TieredMenu) закрывались «сами» по `scroll`, когда браузер
// сдвигал страницу при вставке панели или при фокусе, хотя пользователь ничего не прокручивал. Теперь прокрутка документа
// закрывает панель, только если цель действительно сдвинулась, а фокус при показе не прокручивает страницу.

const cases = [
    { name: 'Menu', url: '/menu', trigger: '[data-demo=popup] button', panel: '.p-menu-overlay' },
    { name: 'Popover', url: '/popover', trigger: '[data-demo=basic] button', panel: '.p-popover' },
    { name: 'ConfirmPopup', url: '/confirmpopup', trigger: '[data-demo=basic] button >> nth=0', panel: '.p-confirmpopup' },
    { name: 'TieredMenu', url: '/tieredmenu', trigger: '[data-demo=popup] button', panel: '#popup_menu' },
];

for (const c of cases) {
    test.describe(c.name, () => {
        test('панель, открытая в узком окне, остаётся открытой, прокрутка страницы цели её закрывает', async ({ page }) => {
            await page.setViewportSize({ width: 1280, height: 900 });
            await page.goto(c.url);
            await page.waitForLoadState('networkidle');
            await page.setViewportSize({ width: 360, height: 700 });
            await page.waitForTimeout(400);
            for (let i = 0; i < 4; i++) {
                await page.locator(c.trigger).first().click();
                const panel = page.locator(c.panel).first();
                await expect(panel).toBeVisible();
                // даём пройти поздним событиям `scroll` и `resize`
                await page.waitForTimeout(500);
                await expect(panel).toBeVisible();
                // настоящая прокрутка страницы сдвигает цель и закрывает панель
                await page.evaluate(() => window.scrollBy(0, 40));
                await page.waitForTimeout(200);
                const moved = await page.evaluate(() => window.scrollY > 0);
                if (moved) {
                    await expect(panel).toHaveCount(0);
                    await page.evaluate(() => window.scrollTo(0, 0));
                } else {
                    // страница не прокручивается (нет высоты): панель остаётся
                    await page.keyboard.press('Escape');
                    await expect(panel).toHaveCount(0);
                }
                await page.waitForTimeout(100);
            }
        });
    });
}
