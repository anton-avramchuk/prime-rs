import { expect, test } from '@playwright/test';

test.describe('InputMask', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/inputmask');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_mask')).toBeAttached();
    });

    const field = (page, id: string) => page.locator(`#${id} input`);

    test('theme styles: the field looks like InputText with a border and a hint', async ({ page }) => {
        const i = field(page, 'basic_mask');
        expect(await i.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        await expect(i).toHaveAttribute('placeholder', '99-999999');
        await expect(i).toHaveValue('');
    });

    test('focus shows the buffer, input fills the mask and skips constant characters', async ({ page }) => {
        const i = field(page, 'basic_mask');
        await i.click();
        await expect(i).toHaveValue('__-______');
        await page.keyboard.type('12a3456789');
        await expect(i).toHaveValue('12-345678');
    });

    test('Backspace removes a character, an incomplete value is cleared on blur', async ({ page }) => {
        const i = field(page, 'basic_mask');
        await i.click();
        await page.keyboard.type('1234');
        await expect(i).toHaveValue('12-34____');
        await page.keyboard.press('Backspace');
        await expect(i).toHaveValue('12-3_____');
        await page.locator('h2').click();
        await expect(i).toHaveValue('');
    });

    test('phone and date with their own placeholder', async ({ page }) => {
        const phone = field(page, 'phone_mask');
        await phone.scrollIntoViewIfNeeded();
        await phone.click();
        await page.keyboard.type('5551234567');
        await expect(phone).toHaveValue('(555) 123-4567');
        const date = field(page, 'slot_mask');
        await date.scrollIntoViewIfNeeded();
        await date.click();
        await expect(date).toHaveValue('mm/dd/yyyy');
        await page.keyboard.type('12252024');
        await expect(date).toHaveValue('12/25/2024');
    });

    test('optional part: a value without it counts as complete', async ({ page }) => {
        const i = field(page, 'optional_mask');
        await i.scrollIntoViewIfNeeded();
        await i.click();
        await page.keyboard.type('5551234567');
        await page.locator('h2').click();
        await expect(i).toHaveValue('(555) 123-4567');
    });

    test('unmask returns only the entered characters', async ({ page }) => {
        const i = field(page, 'unmask_mask');
        await i.scrollIntoViewIfNeeded();
        await i.click();
        await page.keyboard.type('5551234567');
        await expect(page.locator('[data-testid="unmasked"]')).toHaveText('5551234567');
    });

    test('the clear icon resets the value; disabled and invalid states', async ({ page }) => {
        const clear = page.locator('#clear_mask');
        await clear.scrollIntoViewIfNeeded();
        await expect(clear.locator('input')).toHaveValue('(555) 123-4567');
        await clear.locator('.p-inputmask-clear-icon').click();
        await expect(clear.locator('input')).toHaveValue('');
        await expect(clear.locator('.p-inputmask-clear-icon')).toHaveCount(0);
        await expect(field(page, 'disabled_mask')).toBeDisabled();
        await expect(field(page, 'invalid_mask')).toHaveClass(/p-invalid/);
    });

    test('the clear icon is inside the field on the right', async ({ page }) => {
        const clear = page.locator('#clear_mask');
        await clear.scrollIntoViewIfNeeded();
        const input = (await clear.locator('input').boundingBox())!;
        const icon = (await clear.locator('.p-inputmask-clear-icon').boundingBox())!;
        expect(icon.x).toBeGreaterThan(input.x + input.width / 2);
        expect(icon.x + icon.width).toBeLessThanOrEqual(input.x + input.width + 1);
    });
});
