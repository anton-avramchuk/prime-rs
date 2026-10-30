import { expect, test } from '@playwright/test';

test.describe('MeterGroup', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/metergroup');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-metergroup')).toBeVisible();
    });

    test('демонстрации и ARIA', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        const group = page.locator('[data-demo=multiple] .p-metergroup');
        await expect(group).toHaveAttribute('role', 'meter');
        await expect(group).toHaveAttribute('aria-valuemin', '0');
        await expect(group).toHaveAttribute('aria-valuemax', '100');
        await expect(group).toHaveAttribute('aria-valuenow', '58');
        await expect(group).toHaveClass(/p-metergroup-horizontal/);
    });

    test('ширина отрезков пропорциональна значению', async ({ page }) => {
        const group = page.locator('[data-demo=multiple] .p-metergroup-meters');
        const total = (await group.boundingBox())!.width;
        const meters = group.locator('.p-metergroup-meter');
        await expect(meters).toHaveCount(4);
        const w = (await meters.nth(2).boundingBox())!.width;
        expect(Math.abs(w / total - 0.24)).toBeLessThan(0.01);
    });

    test('minmax: max=200 уменьшает отрезки вдвое', async ({ page }) => {
        const group = page.locator('[data-demo=minmax] .p-metergroup-meters');
        const total = (await group.boundingBox())!.width;
        const w = (await group.locator('.p-metergroup-meter').nth(2).boundingBox())!.width;
        expect(Math.abs(w / total - 0.12)).toBeLessThan(0.01);
    });

    test('подписи: текст с процентом и цветом маркера', async ({ page }) => {
        const labels = page.locator('[data-demo=multiple] .p-metergroup-label');
        await expect(labels).toHaveCount(4);
        await expect(labels.first().locator('.p-metergroup-label-text')).toHaveText('Apps (16%)');
        const color = await labels.first().locator('.p-metergroup-label-marker').evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(color).toBe('rgb(52, 211, 153)');
    });

    test('label: подписи перед шкалой, вертикальный список', async ({ page }) => {
        const demo = page.locator('[data-demo=label] .p-metergroup');
        await expect(demo.locator('.p-metergroup-label-list')).toHaveClass(/p-metergroup-label-list-vertical/);
        const list = (await demo.locator('.p-metergroup-label-list').boundingBox())!;
        const meters = (await demo.locator('.p-metergroup-meters').boundingBox())!;
        expect(list.y).toBeLessThan(meters.y);
    });

    test('vertical: отрезки задают высоту', async ({ page }) => {
        const demo = page.locator('[data-demo=vertical] .p-metergroup');
        await expect(demo).toHaveClass(/p-metergroup-vertical/);
        const meters = (await demo.locator('.p-metergroup-meters').boundingBox())!;
        const meter = demo.locator('.p-metergroup-meter').nth(2);
        const h = (await meter.boundingBox())!.height;
        expect(Math.abs(h / meters.height - 0.24)).toBeLessThan(0.02);
    });

    test('icon: значки вместо маркеров', async ({ page }) => {
        const labels = page.locator('[data-demo=icon] .p-metergroup-label');
        await expect(labels.first().locator('.p-metergroup-label-icon svg')).toBeVisible();
        await expect(labels.first().locator('.p-metergroup-label-marker')).toHaveCount(0);
    });
});
