import { expect, test } from '@playwright/test';

test.describe('Password', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/password');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=meter] .p-password')).toBeVisible();
    });

    test('структура: тринадцать демонстраций, поле скрывает ввод', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(13);
        await expect(page.locator('[data-demo=meter] input.p-password-input')).toHaveAttribute('type', 'password');
        await expect(page.locator('[data-demo=meter] .p-password-overlay')).toHaveCount(0);
    });

    test('фокус открывает панель с подсказкой, потеря фокуса закрывает', async ({ page }) => {
        const demo = page.locator('[data-demo=meter]');
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

    test('уровни: слабый, средний, сильный', async ({ page }) => {
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
            // ширина меняется плавным переходом темы, поэтому ждём установившегося значения
            await expect
                .poll(() => meter.evaluate((el) => (el.getBoundingClientRect().width / el.parentElement!.getBoundingClientRect().width) * 100))
                .toBeCloseTo(width, 0);
        }
    });

    test('Escape закрывает панель, ввод открывает снова', async ({ page }) => {
        const demo = page.locator('[data-demo=meter]');
        const input = demo.locator('input');
        await input.click();
        await expect(demo.locator('.p-password-overlay')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(demo.locator('.p-password-overlay')).toHaveCount(0);
        await page.keyboard.type('x');
        await expect(demo.locator('.p-password-overlay')).toBeVisible();
    });

    test('подписи locale', async ({ page }) => {
        const demo = page.locator('[data-demo=locale]');
        await demo.locator('input').click();
        await expect(demo.locator('.p-password-meter-text')).toHaveText('Choose a password');
        await page.keyboard.type('a');
        await expect(demo.locator('.p-password-meter-text')).toHaveText('Too simple');
    });

    test('template: заголовок и подвал внутри панели', async ({ page }) => {
        const demo = page.locator('[data-demo=template]');
        await demo.locator('input').click();
        const panel = demo.locator('.p-password-overlay');
        await expect(panel.getByText('Reset Password')).toBeVisible();
        await expect(panel.getByText('Minimum 8 characters')).toBeVisible();
    });

    test('togglemask: значок переключает тип и не отнимает фокус', async ({ page }) => {
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

    test('clearicon: очищает значение и пропадает', async ({ page }) => {
        const demo = page.locator('[data-demo=clearicon]');
        await expect(demo.locator('input')).toHaveValue('secret');
        await demo.locator('.p-password-clear-icon').click();
        await expect(demo.locator('input')).toHaveValue('');
        await expect(demo.locator('.p-password-clear-icon')).toHaveCount(0);
    });

    test('disabled и feedback=false: панели нет', async ({ page }) => {
        await expect(page.locator('[data-demo=disabled] input')).toBeDisabled();
        const basic = page.locator('[data-demo=basic]');
        await basic.locator('input').click();
        await page.keyboard.type('abc');
        await expect(basic.locator('.p-password-overlay')).toHaveCount(0);
    });
});
