import { expect, test, type Page } from '@playwright/test';

const message = (page: Page, demo: string) => page.locator(`[data-demo=${demo}] .p-toast-message`);

// сообщение въезжает 300 мс (масштаб 0.6 → 1): размеры сравниваем после анимации входа
const stableBox = async (locator: ReturnType<Page['locator']>) => {
    let last = JSON.stringify(await locator.boundingBox());
    await expect
        .poll(async () => {
            const now = JSON.stringify(await locator.boundingBox());
            const same = now === last;
            last = now;
            return same;
        })
        .toBe(true);
    return (await locator.boundingBox())!;
};

test.describe('Toast', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/toast');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] button')).toBeVisible();
    });

    test('контейнеры пусты, пока не показали сообщение', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(11);
        await expect(page.locator('.p-toast-message')).toHaveCount(0);
        await expect(page.locator('[data-demo=basic] .p-toast')).toHaveClass(/p-toast-top-right/);
    });

    test('basic: сообщение справа вверху, закрывается кнопкой', async ({ page }) => {
        await page.locator('[data-demo=basic]').getByRole('button', { name: 'Show' }).click();
        const m = message(page, 'basic');
        await expect(m).toBeVisible();
        await expect(m).toHaveClass(/p-toast-message-success/);
        await expect(m).toHaveAttribute('role', 'alert');
        await expect(m.locator('.p-toast-summary')).toHaveText('Success Message');
        await expect(m.locator('.p-toast-detail')).toHaveText('Message Content');
        const [b, v] = [await stableBox(m), page.viewportSize()!];
        expect(v.width - (b.x + b.width)).toBeLessThan(40);
        expect(b.y).toBeLessThan(60);
        await m.getByRole('button', { name: 'Close' }).click();
        await expect(m).toHaveCount(0);
    });

    test('сообщение закрывается само через life (3 секунды)', async ({ page }) => {
        await page.locator('[data-demo=basic]').getByRole('button', { name: 'Show' }).click();
        await expect(message(page, 'basic')).toBeVisible();
        await expect(message(page, 'basic')).toHaveCount(0, { timeout: 6000 });
    });

    test('наведение мыши останавливает таймер, уход запускает заново', async ({ page }) => {
        await page.locator('[data-demo=basic]').getByRole('button', { name: 'Show' }).click();
        const m = message(page, 'basic');
        await expect(m).toBeVisible();
        await m.hover();
        await page.waitForTimeout(3600);
        await expect(m).toBeVisible();
        await page.mouse.move(5, 5);
        await expect(m).toHaveCount(0, { timeout: 6000 });
    });

    test('severity: класс по серьёзности', async ({ page }) => {
        const demo = page.locator('[data-demo=severity]');
        for (const [name, cls] of [['Error', 'error'], ['Warn', 'warn'], ['Contrast', 'contrast']] as const) {
            await demo.getByRole('button', { name, exact: true }).click();
            await expect(demo.locator(`.p-toast-message-${cls}`)).toBeVisible();
        }
    });

    test('multiple: три сообщения сразу', async ({ page }) => {
        await page.locator('[data-demo=multiple]').getByRole('button', { name: 'Multiple' }).click();
        await expect(message(page, 'multiple')).toHaveCount(3);
    });

    test('position: нижний левый угол', async ({ page }) => {
        await page.locator('[data-demo=position]').getByRole('button', { name: 'Bottom Left' }).click();
        const m = message(page, 'position');
        await expect(m).toBeVisible();
        await expect(page.locator('[data-demo=position] .p-toast')).toHaveClass(/p-toast-bottom-left/);
        const [b, v] = [await stableBox(m), page.viewportSize()!];
        expect(b.x).toBeLessThan(40);
        expect(v.height - (b.y + b.height)).toBeLessThan(60);
    });

    test('sticky остаётся, clear закрывает', async ({ page }) => {
        await page.locator('[data-demo=sticky]').getByRole('button', { name: 'Sticky' }).click();
        await expect(message(page, 'sticky')).toBeVisible();
        await page.waitForTimeout(3600);
        await expect(message(page, 'sticky')).toBeVisible();

        const demo = page.locator('[data-demo=clear]');
        await demo.getByRole('button', { name: 'Show' }).click();
        await expect(message(page, 'clear')).toBeVisible();
        await demo.getByRole('button', { name: 'Clear' }).click();
        await expect(message(page, 'clear')).toHaveCount(0);
    });

    test('target: сообщение попадает в Toast со своим ключом', async ({ page }) => {
        const demo = page.locator('[data-demo=target]');
        await demo.getByRole('button', { name: 'Show Success' }).click();
        await expect(demo.locator('.p-toast-top-left .p-toast-message')).toHaveCount(1);
        await expect(demo.locator('.p-toast-bottom-right .p-toast-message')).toHaveCount(0);
        await demo.getByRole('button', { name: 'Show Warning' }).click();
        await expect(demo.locator('.p-toast-bottom-right .p-toast-message')).toHaveCount(1);
        await expect(demo.locator('.p-toast-top-left .p-toast-message')).toHaveCount(0);
    });

    test('responsive: на узком экране ширина 50%', async ({ page }) => {
        await page.setViewportSize({ width: 800, height: 700 });
        await page.locator('[data-demo=responsive]').getByRole('button', { name: 'Show' }).click();
        const toast = page.locator('[data-demo=responsive] .p-toast');
        await expect(message(page, 'responsive')).toBeVisible();
        const b = (await toast.boundingBox())!;
        expect(Math.abs(b.width - 400)).toBeLessThan(3);
    });

    test('template: свой вид сообщения, Reply закрывает', async ({ page }) => {
        await page.locator('[data-demo=template]').getByRole('button', { name: 'View' }).click();
        const m = message(page, 'template');
        await expect(m).toContainText('Amy Elsner');
        await expect(m).toContainText('Need to reply?');
        await m.getByRole('button', { name: 'Reply' }).click();
        await expect(m).toHaveCount(0);
    });

    test('headless: только своё содержимое, кнопка закрывает', async ({ page }) => {
        await page.locator('[data-demo=headless]').getByRole('button', { name: 'Confirm' }).click();
        const m = message(page, 'headless');
        await expect(m).toContainText('Uploading your files.');
        await expect(m.locator('.p-toast-message-content')).toHaveCount(0);
        await m.getByRole('button', { name: 'Cancel' }).click();
        await expect(m).toHaveCount(0);
    });
});
