import { expect, test } from '@playwright/test';

test.describe('InputOtp', () => {
    test.beforeEach(async ({ page, context }) => {
        await context.grantPermissions(['clipboard-read', 'clipboard-write']).catch(() => undefined);
        await page.goto('/inputotp');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-inputotp')).toBeVisible();
    });

    test('structure: five demos and the number of cells', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        await expect(page.locator('[data-demo=basic] input.p-inputotp-input')).toHaveCount(4);
        await expect(page.locator('[data-demo=template] input.custom-otp-input')).toHaveCount(6);
        await expect(page.locator('[data-demo=mask] input[type=password]')).toHaveCount(4);
        await expect(page.locator('[data-demo=integeronly] input[inputmode=numeric]')).toHaveCount(4);
    });

    test('input: characters fill the cells with auto-advance', async ({ page }) => {
        const cells = page.locator('[data-demo=basic] input');
        await cells.nth(0).click();
        await page.keyboard.type('12ab');
        for (const [i, v] of ['1', '2', 'a', 'b'].entries()) await expect(cells.nth(i)).toHaveValue(v);
        await expect(page.getByTestId('echo-basic')).toHaveText('12ab');
        await expect(cells.nth(3)).toBeFocused();
    });

    test('overflow: the fifth character is not entered', async ({ page }) => {
        const cells = page.locator('[data-demo=basic] input');
        await cells.nth(0).click();
        await page.keyboard.type('12345');
        await expect(page.getByTestId('echo-basic')).toHaveText('1234');
    });

    test('Backspace in an empty cell and arrows move focus', async ({ page }) => {
        const cells = page.locator('[data-demo=basic] input');
        await cells.nth(0).click();
        await page.keyboard.type('1');
        await expect(cells.nth(1)).toBeFocused();
        await page.keyboard.press('Backspace');
        await expect(cells.nth(0)).toBeFocused();
        await page.keyboard.press('ArrowRight');
        await expect(cells.nth(1)).toBeFocused();
        await page.keyboard.press('ArrowLeft');
        await expect(cells.nth(0)).toBeFocused();
    });

    test('integer_only: letters are not entered', async ({ page }) => {
        const cells = page.locator('[data-demo=integeronly] input');
        await cells.nth(0).click();
        await page.keyboard.type('a1b2');
        await expect(page.getByTestId('echo-integer')).toHaveText('12');
    });

    test('pasting a code is distributed across cells', async ({ page }) => {
        const cells = page.locator('[data-demo=basic] input');
        await cells.nth(0).click();
        await page.evaluate(() => {
            const el = document.activeElement as HTMLInputElement;
            const data = new DataTransfer();
            data.setData('text', '987654');
            el.dispatchEvent(new ClipboardEvent('paste', { clipboardData: data, bubbles: true, cancelable: true }));
        });
        await expect(page.getByTestId('echo-basic')).toHaveText('9876');
        for (const [i, v] of ['9', '8', '7', '6'].entries()) await expect(cells.nth(i)).toHaveValue(v);
    });

    test('template: custom fields and a separator after the third cell', async ({ page }) => {
        const cells = page.locator('[data-demo=template] input.custom-otp-input');
        await cells.nth(0).click();
        await page.keyboard.type('123456');
        for (const [i, v] of ['1', '2', '3', '4', '5', '6'].entries()) await expect(cells.nth(i)).toHaveValue(v);
        await expect(page.locator('[data-demo=template] .p-inputotp > div')).toHaveCount(1);
    });
});
