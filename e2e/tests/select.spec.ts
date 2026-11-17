import { expect, test } from '@playwright/test';

test.describe('Select', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/select');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-select')).toBeAttached();
    });

    test('demos are rendered with theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(14);
        const label = page.locator('[data-demo=basic] .p-select-label');
        await expect(label).toHaveText('Select a City');
        await expect(label).toHaveClass(/p-placeholder/);
        const radius = await page.locator('[data-demo=basic] .p-select').evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
        const color = await label.evaluate((el) => getComputedStyle(el).color);
        expect(color).not.toBe(await page.locator('[data-demo=basic]').evaluate((el) => getComputedStyle(el).color));
    });

    test('click opens the panel under the field, selection closes it', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-select');
        await root.click();
        const overlay = root.locator('.p-select-overlay');
        await expect(overlay).toBeVisible();
        await expect(root.locator('.p-select-label')).toHaveAttribute('aria-expanded', 'true');
        const field = (await root.boundingBox())!;
        const panel = (await overlay.boundingBox())!;
        expect(panel.y).toBeGreaterThanOrEqual(field.y + field.height - 2);
        expect(await overlay.evaluate((el) => Number(getComputedStyle(el).zIndex))).toBeGreaterThanOrEqual(1000);
        await expect(overlay.locator('.p-select-option')).toHaveCount(5);
        await overlay.locator('.p-select-option', { hasText: 'London' }).click();
        await expect(overlay).toHaveCount(0);
        await expect(root.locator('.p-select-label')).toHaveText('London');
        await expect(page.getByTestId('echo')).toHaveText('LDN');
        await expect(root.locator('.p-select-label')).toBeFocused();
    });

    test('keyboard: arrows, Enter, Escape, type-ahead', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-select');
        const label = root.locator('.p-select-label');
        await label.focus();
        await page.keyboard.press('ArrowDown');
        await expect(root.locator('.p-select-overlay')).toBeVisible();
        await page.keyboard.press('ArrowDown');
        await page.keyboard.press('ArrowDown');
        await expect(root.locator('.p-select-option').nth(1)).toHaveClass(/p-focus/);
        const id = await root.locator('.p-select-option').nth(1).getAttribute('id');
        await expect(label).toHaveAttribute('aria-activedescendant', id!);
        await page.keyboard.press('Enter');
        await expect(root.locator('.p-select-overlay')).toHaveCount(0);
        await expect(page.getByTestId('echo')).toHaveText('RM');
        await page.keyboard.press('Space');
        await expect(root.locator('.p-select-overlay')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(root.locator('.p-select-overlay')).toHaveCount(0);
        await expect(label).toBeFocused();
        await page.keyboard.type('Par');
        await expect(root.locator('.p-select-overlay')).toBeVisible();
        await expect(root.locator('.p-select-option').nth(4)).toHaveClass(/p-focus/);
        await page.keyboard.press('Enter');
        await expect(page.getByTestId('echo')).toHaveText('PRS');
    });

    test('click outside closes the panel', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-select');
        await root.click();
        await expect(root.locator('.p-select-overlay')).toBeVisible();
        await page.locator('h2').first().click();
        await expect(root.locator('.p-select-overlay')).toHaveCount(0);
    });

    test('search, groups, checkmark and clear', async ({ page }) => {
        const filter = page.locator('[data-demo=filter] .p-select');
        await filter.scrollIntoViewIfNeeded();
        await filter.click();
        const input = filter.locator('.p-select-filter');
        await expect(input).toBeFocused();
        await page.keyboard.type('or');
        await expect(filter.locator('.p-select-option')).toHaveCount(1);
        await page.keyboard.press('Control+A');
        await page.keyboard.type('zzz');
        await expect(filter.locator('.p-select-empty-message')).toHaveText('No results found');
        await page.keyboard.press('Escape');
        await expect(filter.locator('.p-select-overlay')).toHaveCount(0);

        const group = page.locator('[data-demo=group] .p-select');
        await group.scrollIntoViewIfNeeded();
        await group.click();
        await expect(group.locator('.p-select-option-group')).toHaveCount(3);
        await group.locator('.p-select-option', { hasText: 'Kyoto' }).click();
        await expect(group.locator('.p-select-label')).toHaveText('Kyoto');

        const check = page.locator('[data-demo=checkmark] .p-select');
        await check.scrollIntoViewIfNeeded();
        await check.click();
        await check.locator('.p-select-option', { hasText: 'Rome' }).click();
        await check.click();
        await expect(check.locator('.p-select-option-check-icon')).toHaveCount(1);
        await page.keyboard.press('Escape');

        const clearable = page.locator('[data-demo=clearicon] .p-select');
        await clearable.scrollIntoViewIfNeeded();
        await expect(clearable.locator('.p-select-label')).toHaveText('Paris');
        await clearable.locator('.p-select-clear-icon').click();
        await expect(clearable.locator('.p-select-label')).toHaveText('Select a City');
        await expect(clearable.locator('.p-select-overlay')).toHaveCount(0);
    });

    test('disabled, loading, invalid, sizes, template', async ({ page }) => {
        for (const demo of ['disabled', 'loadingstate']) {
            const root = page.locator(`[data-demo=${demo}] .p-select`);
            await root.scrollIntoViewIfNeeded();
            await root.click({ force: true });
            await expect(root.locator('.p-select-overlay')).toHaveCount(0);
        }
        await expect(page.locator('[data-demo=disabled] .p-select')).toHaveClass(/p-disabled/);
        await expect(page.locator('[data-demo=loadingstate] .p-select-loading-icon')).toBeAttached();
        await expect(page.locator('[data-demo=sizes] .p-select-sm')).toHaveCount(1);
        await expect(page.locator('[data-demo=sizes] .p-select-lg')).toHaveCount(1);
        await expect(page.locator('[data-demo=filled] .p-variant-filled')).toHaveCount(1);
        await expect(page.locator('[data-demo=fluid] .p-select-fluid')).toHaveCount(1);

        const invalid = page.locator('[data-demo=invalid] .p-select').first();
        await invalid.scrollIntoViewIfNeeded();
        await expect(invalid).toHaveClass(/p-invalid/);
        await invalid.click();
        await invalid.locator('.p-select-option').first().click();
        await expect(invalid).not.toHaveClass(/p-invalid/);

        const template = page.locator('[data-demo=template] .p-select');
        await template.scrollIntoViewIfNeeded();
        await template.click();
        await template.locator('.p-select-option', { hasText: 'Brazil' }).click();
        await expect(template.locator('.p-select-label b')).toHaveText('BR');
    });

    test('floatlabel: label in a wrapper and rises on input', async ({ page }) => {
        await page.waitForLoadState('networkidle');
        const demo = page.locator('[data-demo=floatlabel]');
        await expect(demo.locator('.p-floatlabel.p-floatlabel-over .p-select')).toHaveCount(1);
        const label = demo.locator('.p-floatlabel > label');
        await expect(label).toHaveCount(1);
        const before = (await label.boundingBox())!.y;
        await demo.locator('.p-floatlabel [role=combobox]').first().focus();
        await expect.poll(async () => (await label.boundingBox())!.y).toBeLessThan(before);
    });

    test('iftalabel: label in a wrapper above the value', async ({ page }) => {
        const demo = page.locator('[data-demo=iftalabel]');
        await expect(demo.locator('.p-iftalabel .p-select')).toHaveCount(1);
        const label = demo.locator('.p-iftalabel > label');
        const box = (await label.boundingBox())!;
        const root = (await demo.locator('.p-iftalabel').boundingBox())!;
        expect(box.y).toBeGreaterThanOrEqual(root.y);
        expect(box.y + box.height).toBeLessThanOrEqual(root.y + root.height);
    });
});
