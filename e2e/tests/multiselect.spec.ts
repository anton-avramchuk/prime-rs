import { expect, test } from '@playwright/test';

test.describe('MultiSelect', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/multiselect');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-multiselect')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(14);
        const label = page.locator('[data-demo=basic] .p-multiselect-label');
        await expect(label).toHaveText('Select Cities');
        await expect(label).toHaveClass(/p-placeholder/);
        const radius = await page.locator('[data-demo=basic] .p-multiselect').evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
        const color = await label.evaluate((el) => getComputedStyle(el).color);
        expect(color).not.toBe(await page.locator('[data-demo=basic]').evaluate((el) => getComputedStyle(el).color));
    });

    test('клик открывает панель под полем, выбор не закрывает её', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-multiselect');
        await root.click();
        const overlay = root.locator('.p-multiselect-overlay');
        await expect(overlay).toBeVisible();
        const field = (await root.boundingBox())!;
        const panel = (await overlay.boundingBox())!;
        expect(panel.y).toBeGreaterThanOrEqual(field.y + field.height - 2);
        expect(await overlay.evaluate((el) => Number(getComputedStyle(el).zIndex))).toBeGreaterThanOrEqual(1000);
        await expect(overlay.locator('.p-multiselect-option')).toHaveCount(5);
        await overlay.locator('.p-multiselect-option', { hasText: 'London' }).click();
        await overlay.locator('.p-multiselect-option', { hasText: 'Paris' }).click();
        await expect(overlay).toBeVisible();
        await expect(root.locator('.p-multiselect-label')).toHaveText('London, Paris');
        await expect(page.getByTestId('echo')).toHaveText('LDN,PRS');
        const selected = overlay.locator('.p-multiselect-option', { hasText: 'London' });
        await expect(selected).toHaveClass(/p-multiselect-option-selected/);
        await expect(selected.locator('.p-checkbox')).toHaveClass(/p-checkbox-checked/);
        await page.locator('h2').click();
        await expect(overlay).toHaveCount(0);
    });

    test('больше трёх значений превращаются в сводку', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-multiselect');
        await root.click();
        for (const city of ['New York', 'Rome', 'London', 'Istanbul']) {
            await root.locator('.p-multiselect-option', { hasText: city }).click();
        }
        await expect(root.locator('.p-multiselect-label')).toHaveText('4 items selected');
    });

    test('чипы удаляются кликом по значку и не открывают панель', async ({ page }) => {
        const root = page.locator('[data-demo=chips] .p-multiselect');
        await root.click();
        await root.locator('.p-multiselect-option', { hasText: 'New York' }).click();
        await root.locator('.p-multiselect-option', { hasText: 'London' }).click();
        await page.locator('h2').click();
        await expect(root.locator('.p-multiselect-chip-item')).toHaveCount(2);
        await expect(page.getByTestId('echo-chips')).toHaveText('NY,LDN');
        await root.locator('.p-multiselect-chip-item').first().locator('.p-chip-remove-icon').click();
        await expect(root.locator('.p-multiselect-chip-item')).toHaveCount(1);
        await expect(page.getByTestId('echo-chips')).toHaveText('LDN');
        await expect(root.locator('.p-multiselect-overlay')).toHaveCount(0);
    });

    test('«выбрать все» и поиск', async ({ page }) => {
        const root = page.locator('[data-demo=filter] .p-multiselect');
        await root.click();
        await root.locator('.p-multiselect-filter').fill('on');
        await expect(root.locator('.p-multiselect-option')).toHaveCount(1);
        await root.locator('.p-multiselect-filter').fill('');
        await root.locator('.p-multiselect-header .p-checkbox').click();
        await expect(root.locator('.p-multiselect-header .p-checkbox')).toHaveClass(/p-checkbox-checked/);
        await expect(root.locator('.p-multiselect-option .p-checkbox-checked')).toHaveCount(5);
        await expect(root.locator('.p-multiselect-label')).toHaveText('5 items selected');
        await root.locator('.p-multiselect-header .p-checkbox').click();
        await expect(root.locator('.p-multiselect-option .p-checkbox-checked')).toHaveCount(0);
    });

    test('клавиатура: стрелки, Enter, Shift-диапазон, Escape', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-multiselect');
        const input = root.locator('.p-hidden-accessible input');
        await input.focus();
        await page.keyboard.press('ArrowDown');
        await expect(root.locator('.p-multiselect-overlay')).toBeVisible();
        await expect(root.locator('.p-multiselect-option').first()).toHaveClass(/p-focus/);
        const id = await root.locator('.p-multiselect-option').first().getAttribute('id');
        await expect(input).toHaveAttribute('aria-activedescendant', id!);
        await page.keyboard.press('Enter');
        await expect(page.getByTestId('echo')).toHaveText('NY');
        await page.keyboard.down('Shift');
        await page.keyboard.press('ArrowDown');
        await page.keyboard.press('ArrowDown');
        await page.keyboard.up('Shift');
        await expect(page.getByTestId('echo')).toHaveText('NY,RM,LDN');
        await page.keyboard.press('Escape');
        await expect(root.locator('.p-multiselect-overlay')).toHaveCount(0);
        await expect(input).toBeFocused();
    });

    test('значок очистки сбрасывает выбор', async ({ page }) => {
        const root = page.locator('[data-demo=clearicon] .p-multiselect');
        await expect(root.locator('.p-multiselect-label')).toHaveText('Paris');
        await root.locator('.p-multiselect-clear-icon').click();
        await expect(root.locator('.p-multiselect-label')).toHaveText('Select Cities');
        await expect(root.locator('.p-multiselect-overlay')).toHaveCount(0);
    });

    test('недоступное поле не открывается, загрузка показывает спиннер', async ({ page }) => {
        const disabled = page.locator('[data-demo=disabled] .p-multiselect');
        await expect(disabled).toHaveClass(/p-disabled/);
        await disabled.click({ force: true });
        await expect(disabled.locator('.p-multiselect-overlay')).toHaveCount(0);
        await expect(page.locator('[data-demo=loadingstate] .p-multiselect-loading-icon')).toBeVisible();
    });

    test('группы и шаблон варианта', async ({ page }) => {
        const group = page.locator('[data-demo=group] .p-multiselect');
        await group.click();
        await expect(group.locator('.p-multiselect-option-group')).toHaveCount(3);
        await expect(group.locator('.p-multiselect-option-group').first()).toHaveText('Germany');
        await page.locator('h2').click();
        const template = page.locator('[data-demo=template] .p-multiselect');
        await template.click();
        await expect(template.locator('.p-multiselect-option').first().locator('b')).toHaveText('AU');
    });
});
