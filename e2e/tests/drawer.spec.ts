import { expect, test } from '@playwright/test';

test.describe('Drawer', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/drawer');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] button')).toBeVisible();
    });

    test('панели нет, пока её не открыли', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        await expect(page.locator('.p-drawer')).toHaveCount(0);
        await expect(page.locator('.p-drawer-mask')).toHaveCount(0);
    });

    test('открытие выводит панель и маску в body', async ({ page }) => {
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

    test('закрытие кнопкой убирает панель и маску', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await page.getByRole('button', { name: 'Close' }).click();
        await expect(page.locator('.p-drawer')).toHaveCount(0);
        await expect(page.locator('.p-drawer-mask')).toHaveCount(0);
    });

    test('клик по маске закрывает, клик по панели нет', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await page.locator('.p-drawer-content').click();
        await expect(page.locator('.p-drawer')).toBeVisible();
        await page.locator('.p-drawer-mask').click({ position: { x: 900, y: 300 } });
        await expect(page.locator('.p-drawer')).toHaveCount(0);
    });

    test('Escape закрывает панель', async ({ page }) => {
        await page.locator('[data-demo=basic] button').click();
        await expect(page.locator('.p-drawer')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-drawer')).toHaveCount(0);
    });

    test('позиции: справа, сверху, снизу', async ({ page }) => {
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

    test('fullscreen занимает весь экран', async ({ page }) => {
        await page.locator('[data-demo=fullscreen] button').click();
        const viewport = page.viewportSize()!;
        // панель въезжает 0,5 с: размеры сравниваем после анимации входа
        await expect
            .poll(async () => {
                const box = (await page.locator('.p-drawer').boundingBox())!;
                return box.width >= viewport.width - 1 && box.height >= viewport.height - 1;
            })
            .toBe(true);
    });

    test('template: свой заголовок и подвал, кнопки закрытия нет', async ({ page }) => {
        await page.locator('[data-demo=template] button').click();
        const panel = page.locator('.p-drawer');
        await expect(panel.locator('.p-drawer-header')).toContainText('Amy Elsner');
        await expect(panel.locator('.p-drawer-footer')).toContainText('Account');
        await expect(panel.locator('.p-drawer-close-button')).toHaveCount(0);
    });

    test('headless: только своё содержимое', async ({ page }) => {
        await page.locator('[data-demo=headless]').getByRole('button', { name: 'Show' }).click();
        const panel = page.locator('.p-drawer');
        await expect(panel).toContainText('Your Logo');
        await expect(panel.locator('.p-drawer-header')).toHaveCount(0);
        await panel.getByRole('button').first().click();
        await expect(panel).toHaveCount(0);
    });

    test('Tab и Shift+Tab не уводят фокус за пределы панели', async ({ page }) => {
        await page.locator('[data-demo=template] button').click();
        const panel = page.locator('.p-drawer');
        await expect(panel).toBeVisible();
        // фокус при открытии остаётся на кнопке-открывалке (как в PrimeNG): ловушка работает, когда он уже внутри панели
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
