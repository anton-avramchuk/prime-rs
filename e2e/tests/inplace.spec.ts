import { expect, test } from '@playwright/test';

test.describe('Inplace', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/inplace');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-inplace-display')).toBeVisible();
    });

    test('structure: four demos, the view is reachable from the keyboard', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        const display = page.locator('[data-demo=basic] .p-inplace-display');
        await expect(display).toHaveAttribute('role', 'button');
        await expect(display).toHaveAttribute('tabindex', '0');
        await expect(page.locator('[data-demo=basic] .p-inplace')).toHaveAttribute('aria-live', 'polite');
        await expect(page.locator('[data-demo=basic] .p-inplace-content')).toHaveCount(0);
    });

    test('click shows the content and hides the view', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        await demo.locator('.p-inplace-display').click();
        await expect(demo.locator('.p-inplace-content')).toContainText('Lorem ipsum');
        await expect(demo.locator('.p-inplace-display')).toHaveCount(0);
    });

    test('Enter on the view activates it', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        await demo.locator('.p-inplace-display').focus();
        await page.keyboard.press('Enter');
        await expect(demo.locator('.p-inplace-content')).toBeVisible();
    });

    test('input: the button closes it and returns the view', async ({ page }) => {
        const demo = page.locator('[data-demo=input]');
        await demo.locator('.p-inplace-display').click();
        await demo.locator('input').fill('hello');
        await demo.locator('.p-inplace-content .p-button').click();
        await expect(demo.locator('.p-inplace-display')).toHaveText('Click to Edit');
        await expect(demo.locator('.p-inplace-content')).toHaveCount(0);
    });

    test('lazy: data is loaded on activation', async ({ page }) => {
        const demo = page.locator('[data-demo=lazy]');
        await expect(demo.locator('tbody tr')).toHaveCount(0);
        await demo.locator('.p-inplace-display').click();
        await expect(demo.locator('tbody tr')).toHaveCount(3);
        await expect(demo.locator('tbody')).toContainText('Bamboo Watch');
    });

    test('image: a view with an icon and a caption', async ({ page }) => {
        const demo = page.locator('[data-demo=image]');
        await expect(demo.locator('.p-inplace-display svg')).toBeVisible();
        await expect(demo.locator('.p-inplace-display')).toContainText('View Photo');
        await demo.locator('.p-inplace-display').click();
        await expect(demo.locator('.p-inplace-content img[alt=Nature]')).toHaveCount(1);
    });
});
