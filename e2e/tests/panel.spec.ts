import { expect, test } from '@playwright/test';

test.describe('Panel', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/panel');
        await expect(page.locator('[data-demo=basic] .p-panel')).toBeVisible();
        // дожидаемся гидратации: первая кнопка получает обработчик
        await page.waitForFunction(() => document.querySelector('[data-demo=toggleable] .p-panel-toggle-button') !== null);
        await page.waitForTimeout(300);
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        const border = await page.locator('[data-demo=basic] .p-panel').evaluate((el) => getComputedStyle(el).borderTopWidth);
        expect(border).toBe('1px');
    });

    test('basic: шапка, заголовок и содержимое без переключателя', async ({ page }) => {
        const panel = page.locator('[data-demo=basic] .p-panel');
        await expect(panel.locator('.p-panel-title')).toHaveText('Header');
        await expect(panel.locator('.p-panel-content')).toContainText('Lorem ipsum');
        await expect(panel.locator('.p-panel-toggle-button, .p-panel-footer')).toHaveCount(0);
    });

    test('toggleable: кнопка сворачивает и разворачивает с aria-expanded и анимацией', async ({ page }) => {
        const panel = page.locator('[data-demo=toggleable] .p-panel');
        const button = panel.locator('.p-panel-toggle-button');
        const container = panel.locator('.p-panel-content-container');
        await expect(button).toHaveAttribute('aria-expanded', 'true');
        await expect(button).toHaveAttribute('aria-controls', (await container.getAttribute('id')) ?? '');
        await expect(container).toHaveAttribute('role', 'region');
        await expect(container).toBeVisible();

        await button.click();
        await expect(container).toHaveClass(/p-collapsible-leave-active/);
        await expect(container).not.toHaveClass(/p-collapsible-leave-active/);
        await expect(container).toBeHidden();
        await expect(button).toHaveAttribute('aria-expanded', 'false');
        await expect(container).toHaveAttribute('aria-hidden', 'true');
        await expect(panel).toHaveClass(/p-panel-collapsed/);
        await expect(page.getByTestId('collapsed-state')).toHaveText('collapsed');

        await button.click();
        await expect(container).toHaveClass(/p-collapsible-enter-active/);
        await expect(container).not.toHaveClass(/p-collapsible-enter-active/);
        await expect(container).toBeVisible();
        await expect(button).toHaveAttribute('aria-expanded', 'true');
        await expect(panel).toHaveClass(/p-panel-expanded/);
        await expect(page.getByTestId('collapsed-state')).toHaveText('expanded');
    });

    test('toggleable: Enter и Space на кнопке переключают панель', async ({ page }) => {
        const panel = page.locator('[data-demo=toggleable] .p-panel');
        const button = panel.locator('.p-panel-toggle-button');
        await button.focus();
        await page.keyboard.press('Enter');
        await expect(button).toHaveAttribute('aria-expanded', 'false');
        await page.keyboard.press('Space');
        await expect(button).toHaveAttribute('aria-expanded', 'true');
    });

    test('header-toggler: свёрнута при загрузке, переключается щелчком по шапке', async ({ page }) => {
        const panel = page.locator('[data-demo=header-toggler] .p-panel');
        const container = panel.locator('.p-panel-content-container');
        await expect(container).toBeHidden();
        await panel.locator('.p-panel-title').click();
        await expect(container).toBeVisible();
        await expect(panel.locator('.p-panel-toggle-button')).toHaveAttribute('aria-expanded', 'true');
        await panel.locator('.p-panel-toggle-button').click();
        await expect(container).toBeHidden();
    });

    test('template: слоты шапки, действий и подвала', async ({ page }) => {
        const panel = page.locator('[data-demo=template] .p-panel');
        await expect(panel.locator('.p-panel-header')).toContainText('Amy Elsner');
        const actions = await panel.locator('.p-panel-header-actions').evaluate((el) => [...el.children].map((c) => c.className.includes('p-panel-toggle-button')));
        expect(actions).toEqual([false, true]);
        await expect(panel.locator('.p-panel-footer')).toContainText('Updated 2 hours ago');
    });
});
