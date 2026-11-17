import { expect, test } from '@playwright/test';

test.describe('CascadeSelect', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1000 });
        await page.goto('/cascadeselect');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_select')).toBeAttached();
    });

    const field = (page) => page.locator('#basic_select');
    const option = (page, key: string) => page.locator(`li[id="basic_select_${key}"]`);
    const content = (page, key: string) => option(page, key).locator('> .p-cascadeselect-option-content');

    test('theme styles: field with a border, placeholder, button with an arrow', async ({ page }) => {
        const root = field(page);
        expect(await root.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        await expect(root.locator('.p-cascadeselect-label')).toHaveText('Select a City');
        await expect(root.locator('.p-cascadeselect-label')).toHaveClass(/p-placeholder/);
        await expect(root.locator('.p-cascadeselect-dropdown svg')).toHaveCount(1);
        await expect(root.locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('click opens the panel under the field; nested list to the right of the parent', async ({ page }) => {
        await field(page).click();
        const panel = field(page).locator('.p-cascadeselect-overlay').first();
        await expect(panel).toBeVisible();
        const fb = (await field(page).boundingBox())!;
        const pb = (await panel.boundingBox())!;
        expect(pb.y).toBeGreaterThanOrEqual(fb.y + fb.height - 2);
        // hover expands the group (as in PrimeNG; a following click on an expanded group would collapse it)
        await content(page, '0').hover();
        const sub = option(page, '0').locator('> ul.p-cascadeselect-option-list');
        await expect(sub).toBeVisible();
        const row = (await content(page, '0').boundingBox())!;
        const sb = (await sub.boundingBox())!;
        expect(sb.x).toBeGreaterThan(row.x + row.width / 2);
        await content(page, '0_0').hover();
        await expect(option(page, '0_0').locator('> ul.p-cascadeselect-option-list')).toBeVisible();
        await expect(content(page, '0')).toHaveClass(/p-cascadeselect-option-content/);
        await expect(option(page, '0')).toHaveClass(/p-cascadeselect-option-active/);
        await expect(option(page, '0').locator('.p-cascadeselect-group-icon svg').first()).toBeVisible();
    });

    test('selecting a leaf fills in the label and closes the panel', async ({ page }) => {
        await field(page).click();
        await content(page, '1').hover();
        await content(page, '1_1').hover();
        await expect(option(page, '1_1').locator('> ul.p-cascadeselect-option-list')).toBeVisible();
        await content(page, '1_1_1').click();
        await expect(field(page).locator('.p-cascadeselect-label')).toHaveText('Toronto');
        await expect(field(page).locator('.p-cascadeselect-overlay')).toHaveCount(0);
        await field(page).click();
        await expect(option(page, '1_1_1')).toHaveClass(/p-cascadeselect-option-selected/);
        await page.mouse.click(5, 5);
        await expect(field(page).locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('keyboard: ↓ opens, → enters a group, Enter selects a leaf', async ({ page }) => {
        const input = field(page).locator('input');
        await input.focus();
        await page.keyboard.press('ArrowDown');
        await expect(field(page).locator('.p-cascadeselect-overlay').first()).toBeVisible();
        await page.keyboard.press('ArrowDown');
        await expect(option(page, '0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(option(page, '0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('ArrowRight');
        await expect(option(page, '0_0_0')).toHaveClass(/p-focus/);
        await page.keyboard.press('Enter');
        await expect(field(page).locator('.p-cascadeselect-label')).toHaveText('Sydney');
        await expect(field(page).locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('clear icon resets the value', async ({ page }) => {
        const clear = page.locator('#clear_select');
        await clear.scrollIntoViewIfNeeded();
        await expect(clear.locator('.p-cascadeselect-label')).toHaveText('Sydney');
        await clear.locator('.p-cascadeselect-clear-icon').click();
        await expect(clear.locator('.p-cascadeselect-label')).toHaveText('Select a City');
        await expect(clear.locator('.p-cascadeselect-overlay')).toHaveCount(0);
    });

    test('disabled field, loading and invalid value', async ({ page }) => {
        const disabled = page.locator('#disabled_select');
        await disabled.scrollIntoViewIfNeeded();
        await expect(disabled).toHaveClass(/p-disabled/);
        await disabled.click({ force: true });
        await expect(disabled.locator('.p-cascadeselect-overlay')).toHaveCount(0);
        await expect(page.locator('#loading_select .p-cascadeselect-loading-icon')).toBeAttached();
        await expect(page.locator('#invalid_select')).toHaveClass(/p-invalid/);
    });

    test('template of an option', async ({ page }) => {
        const t = page.locator('#template_select');
        await t.scrollIntoViewIfNeeded();
        await t.click();
        await expect(t.locator('[data-testid="option-tpl"]').first()).toContainText('Australia');
    });
});
