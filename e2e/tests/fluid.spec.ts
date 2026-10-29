import { expect, test } from '@playwright/test';

test.describe('Fluid', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/fluid');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-fluid')).toBeVisible();
    });

    test('все потребители получают класс растягивания', async ({ page }) => {
        const fluid = page.locator('[data-demo=basic] .p-fluid');
        await expect(fluid.locator('.p-button-fluid')).toHaveCount(1);
        await expect(fluid.locator('input.p-inputtext-fluid:not(.p-inputnumber-input)')).toHaveCount(1);
        await expect(fluid.locator('input.p-inputnumber-input.p-inputtext-fluid')).toHaveCount(1);
        await expect(fluid.locator('.p-textarea-fluid')).toHaveCount(1);
        await expect(fluid.locator('.p-inputnumber-fluid')).toHaveCount(1);
        await expect(fluid.locator('.p-select-fluid')).toHaveCount(1);
    });

    test('поля растянуты на ширину контейнера, явное fluid=false и поля вне Fluid нет', async ({ page }) => {
        const width = (loc: ReturnType<typeof page.locator>) => loc.evaluate((el) => el.getBoundingClientRect().width);
        const container = await width(page.getByTestId('fluid'));
        expect(Math.abs((await width(page.locator('[data-demo=basic] .p-fluid input.p-inputtext-fluid:not(.p-inputnumber-input)'))) - container)).toBeLessThan(2);
        expect(Math.abs((await width(page.locator('[data-demo=basic] .p-fluid button.p-button-fluid'))) - container)).toBeLessThan(2);
        const optedOut = page.locator('[data-demo=basic] .p-fluid input[placeholder="Opted out"]');
        await expect(optedOut).not.toHaveClass(/p-inputtext-fluid/);
        expect(await width(optedOut)).toBeLessThan(container - 20);
        const plain = page.getByTestId('plain');
        await expect(plain.locator('.p-inputtext-fluid, .p-button-fluid')).toHaveCount(0);
    });
});
