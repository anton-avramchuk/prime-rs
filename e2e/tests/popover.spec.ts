import { expect, test } from '@playwright/test';

test.describe('Popover', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/popover');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] button')).toBeVisible();
    });

    test('панель скрыта, пока её не показали', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        await expect(page.locator('.p-popover')).toHaveCount(0);
    });

    test('клик по кнопке показывает и скрывает панель', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const button = demo.getByRole('button', { name: 'Share' });
        await button.click();
        const panel = demo.locator('.p-popover');
        await expect(panel).toBeVisible();
        await expect(panel).toHaveAttribute('role', 'dialog');
        await expect(panel).toContainText('Share this document');
        await button.click();
        await expect(panel).toHaveCount(0);
    });

    test('панель расположена под кнопкой', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const button = demo.getByRole('button', { name: 'Share' });
        await button.click();
        const [b, p] = [(await button.boundingBox())!, (await demo.locator('.p-popover').boundingBox())!];
        expect(p.y).toBeGreaterThanOrEqual(b.y + b.height - 1);
    });

    test('клик вне панели закрывает, клик внутри нет', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        await demo.getByRole('button', { name: 'Share' }).click();
        const panel = demo.locator('.p-popover');
        await panel.getByText('Team Members').click();
        await expect(panel).toBeVisible();
        await page.locator('h2').click();
        await expect(panel).toHaveCount(0);
    });

    test('Escape закрывает панель', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        await demo.getByRole('button', { name: 'Share' }).click();
        await expect(demo.locator('.p-popover')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(demo.locator('.p-popover')).toHaveCount(0);
    });

    test('target: панель выровнена по блоку, а не по кнопке', async ({ page }) => {
        const demo = page.locator('[data-demo=target]');
        await demo.getByRole('button', { name: 'Show' }).click();
        const panel = demo.locator('.p-popover');
        await expect(panel).toBeVisible();
        const [t, p] = [(await demo.getByText('Target Element', { exact: true }).locator('..').boundingBox())!, (await panel.boundingBox())!];
        expect(p.y).toBeGreaterThanOrEqual(t.y + t.height - 1);
    });

    test('selectdata: выбор участника меняет подпись и закрывает панель', async ({ page }) => {
        const demo = page.locator('[data-demo=selectdata]');
        await demo.getByRole('button', { name: 'Select Member' }).click();
        await demo.locator('.p-popover li', { hasText: 'Ioni Bowcher' }).click();
        await expect(demo.locator('.p-popover')).toHaveCount(0);
        await expect(demo.getByRole('button', { name: 'Ioni Bowcher' })).toBeVisible();
    });

    test('панель одной демонстрации не мешает другой', async ({ page }) => {
        await page.locator('[data-demo=template]').getByRole('button', { name: 'Show' }).click();
        await expect(page.locator('[data-demo=template] .p-popover')).toContainText('Custom Content');
        await page.locator('[data-demo=basic]').getByRole('button', { name: 'Share' }).click();
        await expect(page.locator('.p-popover')).toHaveCount(1);
    });
});
