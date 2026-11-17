import { expect, test } from '@playwright/test';

test.describe('InputGroup', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/inputgroup');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-inputgroup').first()).toBeVisible();
    });

    test('demos are rendered with theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        const group = page.locator('[data-demo=basic] .p-inputgroup').first();
        expect(await group.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        const addon = group.locator('.p-inputgroupaddon');
        expect(await addon.evaluate((el) => getComputedStyle(el).borderRightWidth)).not.toBe('1px');
    });

    test('basic: addons around the input in order', async ({ page }) => {
        const group = page.locator('[data-demo=basic] .p-inputgroup').nth(1);
        const kinds = await group.evaluate((el) => [...el.children].map((c) => (c.classList.contains('p-inputgroupaddon') ? 'addon' : c.tagName.toLowerCase())));
        expect(kinds).toEqual(['addon', 'input', 'addon']);
        await expect(group.locator('.p-inputgroupaddon').first()).toHaveText('$');
        await expect(group.locator('.p-inputgroupaddon').last()).toHaveText('.00');
    });

    test('basic: outer corners are rounded, inner ones are square', async ({ page }) => {
        const group = page.locator('[data-demo=basic] .p-inputgroup').nth(1);
        const radius = (loc: ReturnType<typeof group.locator>, prop: string) => loc.evaluate((el, p) => getComputedStyle(el).getPropertyValue(p), prop);
        expect(await radius(group.locator('.p-inputgroupaddon').first(), 'border-top-left-radius')).not.toBe('0px');
        expect(await radius(group.locator('input'), 'border-top-left-radius')).toBe('0px');
        expect(await radius(group.locator('input'), 'border-top-right-radius')).toBe('0px');
        expect(await radius(group.locator('.p-inputgroupaddon').last(), 'border-top-right-radius')).not.toBe('0px');
    });

    test('button: buttons and addons with buttons', async ({ page }) => {
        const groups = page.locator('[data-demo=button] .p-inputgroup');
        await expect(groups).toHaveCount(3);
        await expect(groups.nth(0).locator('button')).toContainText('Search');
        await expect(groups.nth(2).locator('.p-inputgroupaddon button')).toHaveCount(2);
    });

    test('checkbox: the checkbox in an addon works', async ({ page }) => {
        const demo = page.locator('[data-demo=checkbox]');
        await expect(demo.getByTestId('echo')).toHaveText('false');
        await demo.locator('.p-inputgroupaddon .p-checkbox').click();
        await expect(demo.getByTestId('echo')).toHaveText('true');
    });

    test('multiple: several addons in one group', async ({ page }) => {
        const group = page.locator('[data-demo=multiple] .p-inputgroup');
        await expect(group.locator('.p-inputgroupaddon')).toHaveCount(4);
        expect(await group.evaluate((el) => el.getBoundingClientRect().width)).toBeLessThanOrEqual(480);
    });
});
