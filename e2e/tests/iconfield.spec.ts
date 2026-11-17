import { expect, test } from '@playwright/test';

test.describe('IconField', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/iconfield');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-iconfield').first()).toBeVisible();
    });

    test('demos render', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        await expect(page.locator('[data-demo=sizes] .p-iconfield')).toHaveCount(3);
    });

    test('position classes and the order of children', async ({ page }) => {
        const [left, right] = [page.locator('[data-demo=basic] .p-iconfield').nth(0), page.locator('[data-demo=basic] .p-iconfield').nth(1)];
        await expect(left).toHaveClass(/p-iconfield-left/);
        await expect(right).toHaveClass(/p-iconfield-right/);
        const kinds = (loc: typeof left) => loc.evaluate((el) => [...el.children].map((c) => (c.classList.contains('p-inputicon') ? 'icon' : c.tagName.toLowerCase())));
        expect(await kinds(left)).toEqual(['icon', 'input']);
        expect(await kinds(right)).toEqual(['input', 'icon']);
    });

    test('the icon on the left is inside the field bounds, on the right it is at the right edge', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const inputL = (await demo.locator('.p-iconfield').nth(0).locator('input').boundingBox())!;
        const iconL = (await demo.locator('.p-iconfield').nth(0).locator('.p-inputicon').boundingBox())!;
        expect(iconL.x).toBeGreaterThanOrEqual(inputL.x);
        expect(iconL.x + iconL.width).toBeLessThan(inputL.x + inputL.width / 2);
        const inputR = (await demo.locator('.p-iconfield').nth(1).locator('input').boundingBox())!;
        const iconR = (await demo.locator('.p-iconfield').nth(1).locator('.p-inputicon').boundingBox())!;
        expect(iconR.x).toBeGreaterThan(inputR.x + inputR.width / 2);
        expect(iconR.x + iconR.width).toBeLessThanOrEqual(inputR.x + inputR.width);
    });

    test('the field gets an inner padding for the icon', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const padLeft = await demo.locator('.p-iconfield').nth(0).locator('input').evaluate((el) => parseFloat(getComputedStyle(el).paddingLeft));
        const padRight = await demo.locator('.p-iconfield').nth(1).locator('input').evaluate((el) => parseFloat(getComputedStyle(el).paddingRight));
        const plain = await page.locator('[data-demo=sizes] input').nth(1).evaluate((el) => parseFloat(getComputedStyle(el).paddingRight));
        expect(padLeft).toBeGreaterThan(plain);
        expect(padRight).toBeGreaterThan(plain);
    });

    test('template: a custom SVG inside p-inputicon', async ({ page }) => {
        await expect(page.locator('[data-demo=template] .p-inputicon [data-testid=custom-icon]')).toHaveCount(1);
    });

    test('floatlabel and iftalabel: the icon in the field inside the label wrapper', async ({ page }) => {
        await expect(page.locator('[data-demo=floatlabel] .p-floatlabel .p-iconfield')).toHaveCount(2);
        await expect(page.locator('[data-demo=iftalabel] .p-iftalabel .p-iconfield')).toHaveCount(1);
    });
});
