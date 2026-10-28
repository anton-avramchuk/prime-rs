import { expect, test } from '@playwright/test';

test('страница гидрируется, иконки и переключатель схемы работают', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (e) => errors.push(e.message));
    await page.goto('/');
    await expect(page.locator('[data-testid=icons] svg')).toHaveCount(54);

    const bg = () => page.locator('main').evaluate((el) => getComputedStyle(el).backgroundColor);
    const light = await bg();
    // клик срабатывает только после гидрации: ждём, пока класс появится
    await expect(async () => {
        await page.getByTestId('toggle-dark').click();
        await expect(page.locator('html')).toHaveClass(/p-dark/, { timeout: 500 });
    }).toPass();
    expect(await bg()).not.toBe(light);
    expect(errors).toEqual([]);
});
