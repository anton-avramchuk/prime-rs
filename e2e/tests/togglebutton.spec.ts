import { expect, test } from '@playwright/test';

test.describe('ToggleButton', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/togglebutton');
        await expect(page.locator('[data-demo=basic] .p-togglebutton')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        const radius = await page.locator('[data-demo=basic] .p-togglebutton').evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
    });

    test('клик и клавиши переключают состояние', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-togglebutton');
        const label = root.locator('.p-togglebutton-label');
        await expect(label).toHaveText('Off');
        await expect(root).toHaveAttribute('aria-pressed', 'false');
        await root.click();
        await expect(root).toHaveClass(/p-togglebutton-checked/);
        await expect(root).toHaveAttribute('aria-pressed', 'true');
        await expect(label).toHaveText('On');
        await expect(page.getByTestId('echo')).toHaveText('true');
        await root.focus();
        await page.keyboard.press('Space');
        await expect(root).not.toHaveClass(/p-togglebutton-checked/);
        await page.keyboard.press('Enter');
        await expect(root).toHaveClass(/p-togglebutton-checked/);
        await expect(page.getByTestId('echo')).toHaveText('true');
    });

    test('иконка меняется вместе с состоянием', async ({ page }) => {
        const root = page.locator('[data-demo=customized] .p-togglebutton');
        await expect(root.locator('.p-togglebutton-icon svg')).toBeAttached();
        await expect(root.locator('.p-togglebutton-label')).toHaveText('Unlocked');
        const before = await root.locator('.p-togglebutton-icon').innerHTML();
        await root.click();
        await expect(root.locator('.p-togglebutton-label')).toHaveText('Locked');
        expect(await root.locator('.p-togglebutton-icon').innerHTML()).not.toBe(before);
        await expect(root).toHaveAttribute('aria-label', 'Do you confirm');
    });

    test('disabled, fluid, invalid и размеры', async ({ page }) => {
        const disabled = page.locator('[data-demo=disabled] .p-togglebutton');
        await expect(disabled).toHaveClass(/p-disabled/);
        await expect(disabled).toHaveAttribute('tabindex', '-1');
        await disabled.click({ force: true });
        await expect(disabled).not.toHaveClass(/p-togglebutton-checked/);
        await expect(page.locator('[data-demo=fluid] .p-togglebutton-fluid')).toHaveCount(1);
        const invalid = page.locator('[data-demo=invalid] .p-togglebutton');
        await expect(invalid).toHaveClass(/p-invalid/);
        await invalid.click();
        await expect(invalid).not.toHaveClass(/p-invalid/);
        await expect(page.locator('[data-demo=sizes] .p-togglebutton-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-togglebutton-lg')).toHaveCount(1);
    });
});
