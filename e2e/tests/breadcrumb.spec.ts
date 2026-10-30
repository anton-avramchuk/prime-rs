import { expect, test } from '@playwright/test';

test.describe('Breadcrumb', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/breadcrumb');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-breadcrumb')).toBeVisible();
    });

    test('basic: домой, пять пунктов и пять разделителей', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        const crumb = page.locator('[data-demo=basic] .p-breadcrumb');
        await expect(crumb.locator('li.p-breadcrumb-home-item')).toHaveCount(1);
        await expect(crumb.locator('li.p-breadcrumb-item')).toHaveCount(5);
        await expect(crumb.locator('li.p-breadcrumb-separator')).toHaveCount(5);
        await expect(crumb.locator('.p-breadcrumb-item-label')).toHaveText(['Electronics', 'Computer', 'Accessories', 'Keyboard', 'Wireless']);
        await expect(crumb.locator('li.p-breadcrumb-home-item svg')).toBeVisible();
    });

    test('пункты стоят в одну строку слева направо', async ({ page }) => {
        const items = page.locator('[data-demo=basic] .p-breadcrumb li');
        const a = (await items.nth(0).boundingBox())!;
        const b = (await items.nth(1).boundingBox())!;
        expect(b.x).toBeGreaterThan(a.x);
        expect(Math.abs(b.y - a.y)).toBeLessThan(6);
    });

    test('пункт без url: клик не меняет адрес страницы', async ({ page }) => {
        const before = page.url();
        await page.locator('[data-demo=basic] .p-breadcrumb-item-link').first().click();
        expect(page.url()).toBe(before);
    });

    test('template: свой разделитель и пункты-ссылки со значками', async ({ page }) => {
        const crumb = page.locator('[data-demo=template] .p-breadcrumb');
        await expect(crumb.locator('li.p-breadcrumb-separator').first()).toHaveText('/');
        await expect(crumb.locator('li.p-breadcrumb-separator svg')).toHaveCount(0);
        await expect(crumb.locator('li.p-breadcrumb-item a svg')).toHaveCount(5);
    });
});
