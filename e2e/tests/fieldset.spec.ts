import { expect, test } from '@playwright/test';

test.describe('Fieldset', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/fieldset');
        await expect(page.locator('[data-demo=basic] .p-fieldset')).toBeVisible();
        await page.waitForFunction(() => document.querySelector('[data-demo=toggleable] .p-fieldset-toggle-button') !== null);
        await page.waitForTimeout(300);
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(3);
        const border = await page.locator('[data-demo=basic] .p-fieldset').evaluate((el) => getComputedStyle(el).borderTopWidth);
        expect(border).toBe('1px');
    });

    test('basic: легенда и содержимое без переключателя', async ({ page }) => {
        const fieldset = page.locator('[data-demo=basic] .p-fieldset');
        await expect(fieldset.locator('legend .p-fieldset-legend-label')).toHaveText('Header');
        await expect(fieldset.locator('.p-fieldset-content')).toContainText('Lorem ipsum');
        await expect(fieldset.locator('button')).toHaveCount(0);
    });

    test('toggleable: кнопка в легенде сворачивает и разворачивает с aria-expanded и анимацией', async ({ page }) => {
        const fieldset = page.locator('[data-demo=toggleable] .p-fieldset');
        const button = fieldset.locator('legend button.p-fieldset-toggle-button');
        const container = fieldset.locator('.p-fieldset-content-container');
        await expect(button).toHaveAttribute('aria-expanded', 'true');
        await expect(button).toHaveAttribute('aria-controls', (await container.getAttribute('id')) ?? '');
        await expect(container).toHaveAttribute('aria-labelledby', (await button.getAttribute('id')) ?? '');
        await expect(container).toBeVisible();

        await button.click();
        await expect(container).toHaveClass(/p-collapsible-leave-active/);
        await expect(container).not.toHaveClass(/p-collapsible-leave-active/);
        await expect(container).toBeHidden();
        await expect(button).toHaveAttribute('aria-expanded', 'false');
        await expect(container).toHaveAttribute('aria-hidden', 'true');
        await expect(fieldset).toHaveClass(/p-fieldset-collapsed/);
        await expect(page.getByTestId('collapsed-state')).toHaveText('collapsed');

        await button.click();
        await expect(container).toHaveClass(/p-collapsible-enter-active/);
        await expect(container).not.toHaveClass(/p-collapsible-enter-active/);
        await expect(container).toBeVisible();
        await expect(button).toHaveAttribute('aria-expanded', 'true');
        await expect(page.getByTestId('collapsed-state')).toHaveText('expanded');
    });

    test('toggleable: Enter и Space переключают Fieldset', async ({ page }) => {
        const button = page.locator('[data-demo=toggleable] .p-fieldset-toggle-button');
        await button.focus();
        await page.keyboard.press('Enter');
        await expect(button).toHaveAttribute('aria-expanded', 'false');
        await page.keyboard.press('Space');
        await expect(button).toHaveAttribute('aria-expanded', 'true');
    });

    test('template: слот легенды', async ({ page }) => {
        const fieldset = page.locator('[data-demo=template] .p-fieldset');
        await expect(fieldset.locator('legend')).toContainText('Amy Elsner');
        await expect(fieldset.locator('.p-fieldset-content')).toContainText('Lorem ipsum');
    });
});
