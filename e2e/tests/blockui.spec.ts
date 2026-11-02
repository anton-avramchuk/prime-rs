import { expect, test } from '@playwright/test';

test.describe('BlockUI', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/blockui');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-panel')).toBeAttached();
    });

    test('без блокировки маски нет, демонстрации отрисованы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        await expect(page.locator('.p-blockui')).toHaveCount(0);
        await expect(page.locator('body')).not.toHaveClass(/p-overflow-hidden/);
    });

    test('Block закрывает панель маской, Unblock снимает её', async ({ page }) => {
        const target = page.getByTestId('target');
        await page.getByRole('button', { name: 'Block' }).first().click();
        const mask = target.locator('.p-blockui');
        await expect(mask).toBeVisible();
        await expect(mask).toHaveAttribute('aria-busy', 'true');
        await expect(mask).toHaveClass(/p-overlay-mask/);
        // маска растянута на цель (position: absolute по теме) и несёт индикатор по центру
        expect(await mask.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const box = (await target.boundingBox())!;
        const maskBox = (await mask.boundingBox())!;
        expect(Math.abs(maskBox.width - box.width)).toBeLessThan(2);
        expect(Math.abs(maskBox.height - box.height)).toBeLessThan(2);
        await expect(mask.locator('.p-progressspinner')).toBeVisible();
        expect(await target.evaluate((el) => (el as HTMLElement).style.position)).toBe('relative');
        // клик под маской не доходит до содержимого панели
        const hit = await page.evaluate(
            ([x, y]) => document.elementFromPoint(x, y)?.className ?? '',
            [box.x + 20, box.y + 20]
        );
        expect(String(hit)).toContain('p-blockui');
        await expect(page.locator('body')).not.toHaveClass(/p-overflow-hidden/);
        await page.getByRole('button', { name: 'Unblock' }).click();
        await expect(target.locator('.p-blockui')).toHaveCount(0);
        expect(await target.evaluate((el) => (el as HTMLElement).style.position)).toBe('');
    });

    test('документ блокируется на три секунды', async ({ page }) => {
        const demo = page.locator('[data-demo=document]');
        await demo.getByRole('button', { name: 'Block' }).click();
        const mask = page.locator('body > div > .p-blockui');
        await expect(mask).toBeVisible();
        await expect(mask).toHaveClass(/p-blockui-mask-document/);
        expect(await mask.evaluate((el) => getComputedStyle(el).position)).toBe('fixed');
        await expect(page.locator('body')).toHaveClass(/p-overflow-hidden/);
        const z = await mask.evaluate((el) => Number(getComputedStyle(el).zIndex));
        expect(z).toBeGreaterThanOrEqual(1100);
        await expect(page.locator('.p-blockui')).toHaveCount(0, { timeout: 6000 });
        await expect(page.locator('body')).not.toHaveClass(/p-overflow-hidden/);
    });
});
