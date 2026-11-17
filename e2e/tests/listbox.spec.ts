import { expect, test } from '@playwright/test';

test.describe('Listbox', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/listbox');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-listbox')).toBeAttached();
    });

    test('demos are rendered with theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(9);
        const root = page.locator('[data-demo=basic] .p-listbox');
        await expect(root.locator('.p-listbox-option')).toHaveCount(5);
        const radius = await root.evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
        const group = page.locator('[data-demo=group] .p-listbox-option-group').first();
        await expect(group).toHaveText('Germany');
        const weight = await group.evaluate((el) => getComputedStyle(el).fontWeight);
        expect(Number(weight)).toBeGreaterThanOrEqual(500);
    });

    test('a click selects an option and highlights it, a repeated click clears it', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-listbox');
        const london = root.locator('.p-listbox-option', { hasText: 'London' });
        await london.click();
        await expect(london).toHaveClass(/p-listbox-option-selected/);
        await expect(london).toHaveAttribute('aria-selected', 'true');
        await expect(page.getByTestId('echo')).toHaveText('LDN');
        const bg = await london.evaluate((el) => getComputedStyle(el).backgroundColor);
        const plain = await root.locator('.p-listbox-option').first().evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(bg).not.toBe(plain);
        await london.click();
        await expect(london).not.toHaveClass(/p-listbox-option-selected/);
        await expect(page.getByTestId('echo')).toHaveText('');
    });

    test('multiple selection: Ctrl adds, a plain click replaces', async ({ page }) => {
        const root = page.locator('[data-demo=multiple] .p-listbox');
        await root.locator('.p-listbox-option', { hasText: 'New York' }).click();
        await root.locator('.p-listbox-option', { hasText: 'Paris' }).click({ modifiers: ['Control'] });
        await expect(page.getByTestId('echo-multiple')).toHaveText('NY,PRS');
        await root.locator('.p-listbox-option', { hasText: 'London' }).click();
        await expect(page.getByTestId('echo-multiple')).toHaveText('LDN');
    });

    test('keyboard: arrows, Shift range, Enter, Ctrl+A', async ({ page }) => {
        const root = page.locator('[data-demo=multiple] .p-listbox');
        const list = root.locator('.p-listbox-list');
        await list.focus();
        await expect(root.locator('.p-listbox-option').first()).toHaveClass(/p-focus/);
        const id = await root.locator('.p-listbox-option').first().getAttribute('id');
        await expect(list).toHaveAttribute('aria-activedescendant', id!);
        await page.keyboard.press('ArrowDown');
        await expect(root.locator('.p-listbox-option').nth(1)).toHaveClass(/p-focus/);
        await page.keyboard.press('Enter');
        await expect(page.getByTestId('echo-multiple')).toHaveText('RM');
        // the anchor index is taken when Shift is pressed: hold it for the whole selection
        await page.keyboard.down('Shift');
        await page.keyboard.press('ArrowDown');
        await page.keyboard.press('ArrowDown');
        await page.keyboard.up('Shift');
        await expect(page.getByTestId('echo-multiple')).toHaveText('RM,LDN,IST');
        await page.keyboard.press('Control+a');
        await expect(page.getByTestId('echo-multiple')).toHaveText('NY,RM,LDN,IST,PRS');
    });

    test('search keeps matching options', async ({ page }) => {
        const root = page.locator('[data-demo=filter] .p-listbox');
        await root.locator('.p-listbox-filter').fill('lon');
        await expect(root.locator('.p-listbox-option')).toHaveCount(1);
        await expect(root.locator('.p-listbox-option')).toHaveText('London');
        await root.locator('.p-listbox-filter').fill('zzz');
        await expect(root.locator('.p-listbox-empty-message')).toBeVisible();
    });

    test('checkboxes: option and "select all" taking search into account', async ({ page }) => {
        const root = page.locator('[data-demo=checkbox] .p-listbox');
        await root.locator('.p-listbox-option', { hasText: 'Paris' }).locator('.p-checkbox').click();
        await expect(page.getByTestId('echo-checkbox')).toHaveText('PRS');
        await root.locator('.p-listbox-filter').fill('n');
        await root.locator('.p-listbox-header .p-checkbox').click();
        await expect(page.getByTestId('echo-checkbox')).toHaveText('NY,LDN,IST');
        await expect(root.locator('.p-listbox-header .p-checkbox')).toHaveClass(/p-checkbox-checked/);
        await root.locator('.p-listbox-header .p-checkbox').click();
        await expect(page.getByTestId('echo-checkbox')).toHaveText('');
    });

    test('checkmark shows a tick on the selected option', async ({ page }) => {
        const root = page.locator('[data-demo=checkmark] .p-listbox');
        await expect(root.locator('.p-listbox-option-blank-icon')).toHaveCount(5);
        await root.locator('.p-listbox-option', { hasText: 'Rome' }).click();
        await expect(root.locator('.p-listbox-option-check-icon')).toHaveCount(1);
        await expect(root.locator('.p-listbox-option-blank-icon')).toHaveCount(4);
    });

    test('a disabled list does not react, an invalid one gets a border', async ({ page }) => {
        const disabled = page.locator('[data-demo=disabled] .p-listbox');
        await expect(disabled).toHaveClass(/p-disabled/);
        await disabled.locator('.p-listbox-option').first().click({ force: true });
        await expect(disabled.locator('.p-listbox-option-selected')).toHaveCount(0);
        const invalid = page.locator('[data-demo=invalid] .p-listbox');
        await expect(invalid).toHaveClass(/p-invalid/);
        const border = await invalid.evaluate((el) => getComputedStyle(el).borderTopColor);
        const normal = await page.locator('[data-demo=basic] .p-listbox').evaluate((el) => getComputedStyle(el).borderTopColor);
        expect(border).not.toBe(normal);
        await invalid.locator('.p-listbox-option').first().click();
        await expect(invalid).not.toHaveClass(/p-invalid/);
    });

    test('option template shows the value in bold', async ({ page }) => {
        const first = page.locator('[data-demo=template] .p-listbox-option').first();
        await expect(first.locator('b')).toHaveText('AU');
        await first.click();
        await expect(first).toHaveClass(/p-listbox-option-selected/);
    });
});
