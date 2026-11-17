import { expect, test } from '@playwright/test';

test.describe('RadioButton', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/radiobutton');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=group] .p-radiobutton').first()).toBeAttached();
    });

    test('demos render with the theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        const size = await page.locator('[data-demo=group] .p-radiobutton-box').first().evaluate((el) => getComputedStyle(el).width);
        expect(size).toBe('20px');
    });

    test('selecting in a group clears the others', async ({ page }) => {
        const roots = page.locator('[data-demo=group] .p-radiobutton');
        await page.locator('label[for=ingredient2]').click();
        await expect(roots.nth(1)).toHaveClass(/p-radiobutton-checked/);
        await expect(page.getByTestId('selected')).toHaveText('Mushroom');
        await page.locator('label[for=ingredient4]').click();
        await expect(roots.nth(3)).toHaveClass(/p-radiobutton-checked/);
        await expect(roots.nth(1)).not.toHaveClass(/p-radiobutton-checked/);
        await expect(roots.nth(3).locator('input')).toHaveAttribute('aria-checked', 'true');
        await expect(page.getByTestId('selected')).toHaveText('Onion');
    });

    test('classes for size, variant, invalid and disabled', async ({ page }) => {
        await expect(page.locator('[data-demo=sizes] .p-radiobutton-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-radiobutton-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-radiobutton-checked')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=invalid] .p-invalid')).toHaveCount(1);
        await expect(page.locator('[data-demo=disabled] .p-disabled')).toHaveCount(2);
        await expect(page.locator('[data-demo=disabled] input').first()).toBeDisabled();
    });
});
