import { expect, test } from '@playwright/test';

test.describe('Avatar', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/avatar');
        await expect(page.locator('[data-demo=label] .p-avatar').first()).toBeVisible();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        const radius = await page.locator('[data-demo=shape] .p-avatar-circle').evaluate((el) => getComputedStyle(el).borderRadius);
        expect(radius).not.toBe('0px');
    });

    test('форма и размер: классы и data-p', async ({ page }) => {
        const circle = page.locator('[data-demo=shape] .p-avatar-circle');
        await expect(circle).toHaveCount(1);
        await expect(circle).toHaveAttribute('data-p', 'circle normal');
        const square = page.locator('[data-demo=shape] .p-avatar:not(.p-avatar-circle)');
        await expect(square).toHaveAttribute('data-p', 'square normal');
        await expect(page.locator('[data-demo=size] .p-avatar-lg')).toHaveAttribute('data-p', 'square large');
        await expect(page.locator('[data-demo=size] .p-avatar-xl')).toHaveAttribute('data-p', 'square xlarge');
    });

    test('приоритет блоков: метка, иконка, изображение', async ({ page }) => {
        await expect(page.locator('[data-demo=label] .p-avatar-label')).toHaveCount(6);
        await expect(page.locator('[data-demo=icon] .p-avatar-icon')).toHaveCount(6);
        await expect(page.locator('[data-demo=image] .p-avatar.p-avatar-image img')).toHaveCount(3);
        await expect(page.locator('[data-demo=label] img')).toHaveCount(0);
    });

    test('template: пользовательское содержимое', async ({ page }) => {
        await expect(page.locator('[data-demo=template] .p-avatar')).toContainText('AB');
    });
});
