import { expect, test } from '@playwright/test';

test.describe('Steps', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 900 });
        await page.goto('/steps');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-steps')).toBeAttached();
    });

    const steps = (page, name: string) => page.locator(`[data-demo=${name}] .p-steps`);
    const item = (page, name: string, i: number) => steps(page, name).locator('.p-steps-item').nth(i);

    test('стили темы: шаги в ряд с номерами в кружках, активный выделен', async ({ page }) => {
        const s = steps(page, 'basic');
        expect(await s.locator('.p-steps-list').evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        const a = (await item(page, 'basic', 0).boundingBox())!;
        const b = (await item(page, 'basic', 1).boundingBox())!;
        expect(b.x).toBeGreaterThan(a.x + a.width - 2);
        const number = s.locator('.p-steps-item-number').nth(1);
        const box = (await number.boundingBox())!;
        expect(Math.abs(box.width - box.height)).toBeLessThan(1);
        expect(await number.evaluate((el) => getComputedStyle(el).borderTopLeftRadius)).not.toBe('0px');
        // активный номер отличается от остальных цветом рамки или текста (фон в светлой теме у обоих белый)
        const look = (i: number) =>
            s.locator('.p-steps-item-number').nth(i).evaluate((el) => {
                const c = getComputedStyle(el);
                return `${c.backgroundColor}|${c.borderTopColor}|${c.color}`;
            });
        expect(await look(1)).not.toBe(await look(2));
        expect(await look(2)).toBe(await look(3));
        await expect(item(page, 'basic', 1)).toHaveClass(/p-steps-item-active/);
        await expect(item(page, 'basic', 1)).toHaveAttribute('aria-current', 'step');
        await expect(s).toHaveClass(/p-readonly/);
    });

    test('в режиме чтения клик ничего не меняет', async ({ page }) => {
        await item(page, 'basic', 3).locator('a').click({ force: true });
        await expect(item(page, 'basic', 1)).toHaveClass(/p-steps-item-active/);
    });

    test('управляемая демонстрация: кнопки меняют шаг', async ({ page }) => {
        const demo = page.locator('[data-demo=controlled]');
        await expect(item(page, 'controlled', 0)).toHaveClass(/p-steps-item-active/);
        await expect(demo.getByRole('button', { name: 'Back' })).toBeDisabled();
        await demo.getByRole('button', { name: 'Next' }).click();
        await demo.getByRole('button', { name: 'Next' }).click();
        await expect(item(page, 'controlled', 2)).toHaveClass(/p-steps-item-active/);
        await demo.getByRole('button', { name: 'Back' }).click();
        await expect(item(page, 'controlled', 1)).toHaveClass(/p-steps-item-active/);
        await demo.getByRole('button', { name: 'Next' }).click();
        await demo.getByRole('button', { name: 'Next' }).click();
        await expect(demo.getByRole('button', { name: 'Next' })).toBeDisabled();
    });

    test('интерактивная демонстрация: клик и клавиатура переключают шаги, отключённый шаг недоступен', async ({ page }) => {
        await item(page, 'interactive', 1).locator('a').click();
        await expect(item(page, 'interactive', 1)).toHaveClass(/p-steps-item-active/);
        await expect(page.locator('.p-toast-message').first()).toContainText('Seat — step 2');
        await item(page, 'interactive', 3).locator('a').click({ force: true });
        await expect(item(page, 'interactive', 1)).toHaveClass(/p-steps-item-active/);
        await expect(item(page, 'interactive', 3)).toHaveClass(/p-disabled/);
        await item(page, 'interactive', 1).locator('a').focus();
        await page.keyboard.press('ArrowRight');
        await expect(item(page, 'interactive', 2).locator('a')).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(item(page, 'interactive', 2)).toHaveClass(/p-steps-item-active/);
        await page.keyboard.press('Home');
        await expect(item(page, 'interactive', 0).locator('a')).toBeFocused();
        await page.keyboard.press('Space');
        await expect(item(page, 'interactive', 0)).toHaveClass(/p-steps-item-active/);
    });
});
