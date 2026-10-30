import { expect, test } from '@playwright/test';

test.describe('Tabs', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/tabs');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-tabs')).toBeVisible();
    });

    test('структура: семь демонстраций, роли и связи', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(7);
        const demo = page.locator('[data-demo=basic]');
        await expect(demo.locator('[role=tablist]')).toHaveCount(1);
        await expect(demo.locator('[role=tab]')).toHaveCount(3);
        await expect(demo.locator('[role=tabpanel]')).toHaveCount(3);
        const tab = demo.locator('[role=tab]').first();
        const panel = demo.locator('[role=tabpanel]').first();
        await expect(tab).toHaveAttribute('aria-controls', (await panel.getAttribute('id'))!);
        await expect(panel).toHaveAttribute('aria-labelledby', (await tab.getAttribute('id'))!);
        await expect(tab).toHaveAttribute('aria-selected', 'true');
        await expect(demo.locator('[role=tabpanel]:not([hidden])')).toHaveCount(1);
    });

    test('клик переключает вкладку и панель', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        await demo.getByRole('tab', { name: 'Header II', exact: true }).click();
        await expect(demo.getByRole('tab', { name: 'Header II', exact: true })).toHaveAttribute('aria-selected', 'true');
        await expect(demo.locator('[role=tabpanel]:not([hidden])')).toContainText('Sed ut perspiciatis');
        await expect(demo.getByRole('tab', { name: 'Header I', exact: true })).toHaveAttribute('tabindex', '-1');
    });

    test('индикатор перемещается под активную вкладку', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const bar = demo.locator('.p-tablist-active-bar');
        const second = demo.getByRole('tab', { name: 'Header II', exact: true });
        await second.click();
        await expect
            .poll(async () => {
                const [b, t] = [(await bar.boundingBox())!, (await second.boundingBox())!];
                return Math.abs(b.x - t.x) < 2 && Math.abs(b.width - t.width) < 2;
            })
            .toBe(true);
    });

    test('клавиатура: стрелки, Home/End, Enter', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const tabs = demo.getByRole('tab');
        await tabs.first().focus();
        await page.keyboard.press('ArrowRight');
        await expect(tabs.nth(1)).toBeFocused();
        await page.keyboard.press('End');
        await expect(tabs.nth(2)).toBeFocused();
        await page.keyboard.press('ArrowRight');
        await expect(tabs.nth(0)).toBeFocused();
        await page.keyboard.press('ArrowLeft');
        await expect(tabs.nth(2)).toBeFocused();
        await page.keyboard.press('Home');
        await expect(tabs.nth(0)).toBeFocused();
        await expect(tabs.nth(0)).toHaveAttribute('aria-selected', 'true');
        await page.keyboard.press('ArrowRight');
        await page.keyboard.press('Enter');
        await expect(tabs.nth(1)).toHaveAttribute('aria-selected', 'true');
        await page.keyboard.press('ArrowRight');
        await page.keyboard.press('Space');
        await expect(tabs.nth(2)).toHaveAttribute('aria-selected', 'true');
    });

    test('Tab из вкладки уходит дальше (roving tabindex)', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        await demo.getByRole('tab').first().focus();
        await page.keyboard.press('Tab');
        await expect(page.locator('[data-demo=basic] [role=tab]:focus')).toHaveCount(0);
    });

    test('disabled: клик не переключает, стрелки пропускают', async ({ page }) => {
        const demo = page.locator('[data-demo=disabled]');
        const disabled = demo.getByRole('tab', { name: 'Header IV', exact: true });
        await expect(disabled).toHaveAttribute('aria-disabled', 'true');
        await disabled.click({ force: true });
        await expect(demo.getByRole('tab', { name: 'Header I', exact: true })).toHaveAttribute('aria-selected', 'true');
        await demo.getByRole('tab', { name: 'Header III', exact: true }).focus();
        await page.keyboard.press('ArrowRight');
        await expect(demo.getByRole('tab', { name: 'Header I', exact: true })).toBeFocused();
    });

    test('controlled: кнопки управляют вкладками', async ({ page }) => {
        const demo = page.locator('[data-demo=controlled]');
        await demo.getByRole('button', { name: '2', exact: true }).click();
        await expect(demo.getByRole('tab', { name: 'Header II', exact: true })).toHaveAttribute('aria-selected', 'true');
        await demo.getByRole('tab', { name: 'Header III', exact: true }).click();
        await expect(demo.locator('[role=tabpanel]:not([hidden])')).toContainText('At vero eos');
    });

    test('lazy: содержимое неактивных панелей не выводится до первой активации', async ({ page }) => {
        const demo = page.locator('[data-demo=lazy]');
        await expect(demo.locator('[role=tabpanel]').nth(1)).toBeEmpty();
        await demo.getByRole('tab', { name: 'Header II', exact: true }).click();
        await expect(demo.locator('[role=tabpanel]').nth(1)).toContainText('Sed ut perspiciatis');
        await demo.getByRole('tab', { name: 'Header I', exact: true }).click();
        await expect(demo.locator('[role=tabpanel]').nth(1)).toContainText('Sed ut perspiciatis');
        await expect(demo.locator('[role=tabpanel]').nth(1)).toBeHidden();
    });

    test('scrollable: кнопки прокручивают список', async ({ page }) => {
        const demo = page.locator('[data-demo=scrollable]');
        await demo.scrollIntoViewIfNeeded();
        const next = demo.locator('.p-tablist-next-button');
        await expect(next).toBeVisible();
        await expect(demo.locator('.p-tablist-prev-button')).toHaveCount(0);
        const content = demo.locator('.p-tablist-content');
        await next.click();
        await expect.poll(() => content.evaluate((el) => el.scrollLeft)).toBeGreaterThan(50);
        await expect(demo.locator('.p-tablist-prev-button')).toBeVisible();
        await demo.locator('.p-tablist-prev-button').click();
        await expect.poll(() => content.evaluate((el) => el.scrollLeft)).toBe(0);
    });

    test('customtemplate: аватары, бейдж и свои значки кнопок', async ({ page }) => {
        const demo = page.locator('[data-demo=customtemplate]');
        await expect(demo.locator('[role=tab] .p-avatar')).toHaveCount(3);
        await expect(demo.locator('[role=tab] .p-badge')).toHaveText('2');
    });
});
