import { expect, test } from '@playwright/test';

test.describe('Chip', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/chip');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-chip').first()).toBeVisible();
        await page.waitForTimeout(500); // hydration
    });

    test('demos render', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        await expect(page.locator('[data-demo=basic] .p-chip')).toHaveCount(4);
        await expect(page.locator('[data-demo=image] .p-chip-image')).toHaveCount(4);
        await expect(page.locator('[data-demo=icon] .p-chip-icon')).toHaveCount(4);
    });

    test('removal with the mouse hides the Chip', async ({ page }) => {
        const chip = page.locator('[data-demo=basic] .p-chip', { hasText: 'Thriller' });
        const remove = chip.locator('.p-chip-remove-icon');
        await expect(remove).toHaveAttribute('role', 'button');
        await expect(remove).toHaveAttribute('aria-label', 'Remove');
        await remove.click();
        await expect(chip).toBeHidden();
        await expect(page.locator('[data-demo=basic] .p-chip:visible')).toHaveCount(3);
    });

    test('removal with the keyboard: Enter and Backspace', async ({ page }) => {
        for (const [demo, key] of [['icon', 'Enter'], ['image', 'Backspace']]) {
            const chip = page.locator(`[data-demo=${demo}] .p-chip`).last();
            const remove = chip.locator('.p-chip-remove-icon');
            await remove.focus();
            await page.keyboard.press(key);
            await expect(chip).toBeHidden();
        }
    });

    test('only removable Chips have the remove element', async ({ page }) => {
        await expect(page.locator('[data-demo=basic] .p-chip-remove-icon')).toHaveCount(1);
        await expect(page.locator('[data-demo=basic] .p-chip[data-p=removable]')).toHaveCount(1);
    });
});
