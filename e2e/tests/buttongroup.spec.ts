import { expect, test } from '@playwright/test';

test.describe('ButtonGroup', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/buttongroup');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-buttongroup')).toBeVisible();
    });

    test('демонстрации и разметка корня', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const group = page.locator('[data-demo=basic] .p-buttongroup');
        await expect(group).toHaveAttribute('role', 'group');
        await expect(group).toHaveClass(/p-component/);
        await expect(group.locator('button.p-button')).toHaveCount(3);
        expect(await group.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
    });

    test('кнопки слитные: скруглены только крайние углы', async ({ page }) => {
        const buttons = page.locator('[data-demo=basic] .p-buttongroup button');
        const radius = (i: number, prop: string) => buttons.nth(i).evaluate((el, p) => getComputedStyle(el).getPropertyValue(p), prop);
        expect(await radius(0, 'border-top-left-radius')).not.toBe('0px');
        expect(await radius(0, 'border-top-right-radius')).toBe('0px');
        expect(await radius(1, 'border-top-left-radius')).toBe('0px');
        expect(await radius(1, 'border-top-right-radius')).toBe('0px');
        expect(await radius(2, 'border-top-left-radius')).toBe('0px');
        expect(await radius(2, 'border-top-right-radius')).not.toBe('0px');
    });

    test('icons: три кнопки с одними значками', async ({ page }) => {
        const buttons = page.locator('[data-demo=icons] .p-buttongroup button');
        await expect(buttons).toHaveCount(3);
        await expect(buttons.first()).toHaveClass(/p-button-icon-only/);
    });
});
