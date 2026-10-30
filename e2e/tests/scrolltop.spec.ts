import { expect, test } from '@playwright/test';

test.describe('ScrollTop', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/scrolltop');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic]')).toBeVisible();
    });

    test('вверху страницы кнопки окна нет, вложенная скрыта', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(2);
        await expect(page.locator('body button.p-scrolltop:not(.p-scrolltop-sticky)')).toHaveCount(0);
        await expect(page.locator('[data-demo=element] button.p-scrolltop-sticky')).toBeHidden();
    });

    test('окно: кнопка появляется после порога, fixed, клик возвращает наверх', async ({ page }) => {
        await page.evaluate(() => window.scrollTo(0, 800));
        const button = page.locator('button.p-scrolltop:not(.p-scrolltop-sticky)');
        await expect(button).toBeVisible();
        await expect(button).toHaveCSS('position', 'fixed');
        await expect(button).toHaveAttribute('aria-label', 'Scroll Top');
        await expect(button.locator('svg.p-scrolltop-icon')).toBeVisible();
        const box = (await button.boundingBox())!;
        const view = page.viewportSize()!;
        expect(box.x + box.width).toBeGreaterThan(view.width - 60);
        expect(box.y + box.height).toBeGreaterThan(view.height - 60);
        await button.click();
        await expect.poll(() => page.evaluate(() => window.scrollY)).toBe(0);
        await expect(button).toHaveCount(0);
    });

    test('окно: клавиши Enter и Space прокручивают наверх', async ({ page }) => {
        const button = page.locator('button.p-scrolltop:not(.p-scrolltop-sticky)');
        for (const key of ['Enter', 'Space']) {
            await page.evaluate(() => window.scrollTo(0, 800));
            await expect(button).toBeVisible();
            await button.focus();
            await page.keyboard.press(key);
            await expect.poll(() => page.evaluate(() => window.scrollY)).toBe(0);
            await expect(button).toHaveCount(0);
        }
    });

    test('окно: возврат ниже порога скрывает кнопку', async ({ page }) => {
        await page.evaluate(() => window.scrollTo(0, 800));
        const button = page.locator('button.p-scrolltop:not(.p-scrolltop-sticky)');
        await expect(button).toBeVisible();
        await page.evaluate(() => window.scrollTo(0, 0));
        await expect(button).toHaveCount(0);
    });

    test('element: кнопка внутри прокручиваемого блока, sticky, клик прокручивает блок', async ({ page }) => {
        const scroller = page.getByTestId('scroller');
        const button = scroller.locator('button.p-scrolltop-sticky');
        await scroller.scrollIntoViewIfNeeded();
        await scroller.evaluate((el) => (el.scrollTop = 250));
        await expect(button).toBeVisible();
        await expect(button).toHaveCSS('position', 'sticky');
        const inside = await button.evaluate((el) => {
            const b = el.getBoundingClientRect();
            const s = el.parentElement!.getBoundingClientRect();
            return b.top >= s.top && b.bottom <= s.bottom && b.left >= s.left && b.right <= s.right;
        });
        expect(inside).toBe(true);
        await button.click();
        await expect.poll(() => scroller.evaluate((el) => el.scrollTop)).toBe(0);
        await expect(button).toBeHidden();
    });
});
