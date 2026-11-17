import { expect, test } from '@playwright/test';

test.describe('Password', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/password');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=meter] .p-password')).toBeVisible();
    });

    test('structure: thirteen demos, the field hides input', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(13);
        await expect(page.locator('[data-demo=meter] input.p-password-input')).toHaveAttribute('type', 'password');
        await expect(page.locator('[data-demo=meter] .p-password-overlay')).toHaveCount(0);
    });

    test('focus opens the panel with a hint, focus loss closes it', async ({ page }) => {
        const demo = page.locator('[data-demo=meter]');
        // near the bottom edge of the window the panel would open upward: put the field at the top of the window
        await demo.evaluate((el) => el.scrollIntoView({ block: 'start' }));
        await demo.locator('input').click();
        const panel = demo.locator('.p-password-overlay');
        await expect(panel).toBeVisible();
        await expect(panel.locator('.p-password-meter-text')).toHaveText('Enter a password');
        const field = (await demo.locator('input').boundingBox())!;
        const box = (await panel.boundingBox())!;
        expect(box.y).toBeGreaterThanOrEqual(field.y + field.height - 1);
        await page.locator('h2').click();
        await expect(panel).toHaveCount(0);
    });

    test('levels: weak, medium, strong', async ({ page }) => {
        const demo = page.locator('[data-demo=meter]');
        const input = demo.locator('input');
        await input.click();
        for (const [text, level, label, width] of [
            ['a', 'weak', 'Weak', 33.33],
            ['abcDEF', 'medium', 'Medium', 66.66],
            ['abcDEF12', 'strong', 'Strong', 100],
        ] as const) {
            await input.fill(text);
            const meter = demo.locator('.p-password-meter-label');
            await expect(meter).toHaveClass(new RegExp(`p-password-meter-${level}`));
            await expect(demo.locator('.p-password-meter-text')).toHaveText(label);
            // width changes with a smooth theme transition, so wait for the settled value
            await expect
                .poll(() => meter.evaluate((el) => (el.getBoundingClientRect().width / el.parentElement!.getBoundingClientRect().width) * 100))
                .toBeCloseTo(width, 0);
        }
    });

    test('Escape closes the panel, input opens it again', async ({ page }) => {
        const demo = page.locator('[data-demo=meter]');
        const input = demo.locator('input');
        await input.click();
        await expect(demo.locator('.p-password-overlay')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(demo.locator('.p-password-overlay')).toHaveCount(0);
        await page.keyboard.type('x');
        await expect(demo.locator('.p-password-overlay')).toBeVisible();
    });

    test('locale labels', async ({ page }) => {
        const demo = page.locator('[data-demo=locale]');
        await demo.locator('input').click();
        await expect(demo.locator('.p-password-meter-text')).toHaveText('Choose a password');
        await page.keyboard.type('a');
        await expect(demo.locator('.p-password-meter-text')).toHaveText('Too simple');
    });

    test('template: header and footer inside the panel', async ({ page }) => {
        const demo = page.locator('[data-demo=template]');
        await demo.locator('input').click();
        const panel = demo.locator('.p-password-overlay');
        await expect(panel.getByText('Reset Password')).toBeVisible();
        await expect(panel.getByText('Minimum 8 characters')).toBeVisible();
    });

    test('togglemask: the icon switches the type and does not take focus', async ({ page }) => {
        const demo = page.locator('[data-demo=togglemask]');
        const input = demo.locator('input');
        await input.fill('secret');
        await input.focus();
        await demo.locator('.p-password-unmask-icon').click();
        await expect(input).toHaveAttribute('type', 'text');
        await expect(input).toBeFocused();
        await demo.locator('.p-password-mask-icon').click();
        await expect(input).toHaveAttribute('type', 'password');
    });

    test('clearicon: clears the value and disappears', async ({ page }) => {
        const demo = page.locator('[data-demo=clearicon]');
        await expect(demo.locator('input')).toHaveValue('secret');
        await demo.locator('.p-password-clear-icon').click();
        await expect(demo.locator('input')).toHaveValue('');
        await expect(demo.locator('.p-password-clear-icon')).toHaveCount(0);
    });

    test('disabled and feedback=false: no panel', async ({ page }) => {
        await expect(page.locator('[data-demo=disabled] input')).toBeDisabled();
        const basic = page.locator('[data-demo=basic]');
        await basic.locator('input').click();
        await page.keyboard.type('abc');
        await expect(basic.locator('.p-password-overlay')).toHaveCount(0);
    });
});
