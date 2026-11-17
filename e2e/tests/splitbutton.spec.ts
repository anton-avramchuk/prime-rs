import { expect, test } from '@playwright/test';

test.describe('SplitButton', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/splitbutton');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-splitbutton')).toBeAttached();
    });

    test('demos are rendered with theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(11);
        const root = page.locator('[data-demo=basic] .p-splitbutton');
        await expect(root.locator('.p-splitbutton-button')).toHaveText('Save');
        // the flex item of the demo card gets a computed flex value instead of inline-flex
        expect(['flex', 'inline-flex']).toContain(await root.evaluate((el) => getComputedStyle(el).display));
        // the buttons are joined: the main one has no right rounding, the arrow has no left one
        const main = root.locator('.p-splitbutton-button');
        const dropdown = root.locator('.p-splitbutton-dropdown');
        expect(await main.evaluate((el) => getComputedStyle(el).borderTopRightRadius)).toBe('0px');
        expect(await dropdown.evaluate((el) => getComputedStyle(el).borderTopLeftRadius)).toBe('0px');
        const a = (await main.boundingBox())!;
        const b = (await dropdown.boundingBox())!;
        expect(Math.abs(a.x + a.width - b.x)).toBeLessThan(2);
    });

    test('main button triggers the action', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-splitbutton');
        await root.locator('.p-splitbutton-button').click();
        await expect(page.getByTestId('echo')).toHaveText('1');
        await expect(page.locator('.p-toast-message')).toContainText('Data Saved');
    });

    test('arrow opens the menu under the component, a repeated click closes it', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-splitbutton');
        const dropdown = root.locator('.p-splitbutton-dropdown');
        await dropdown.click();
        const menu = root.locator('.p-menu-overlay');
        await expect(menu).toBeVisible();
        await expect(dropdown).toHaveAttribute('aria-expanded', 'true');
        const field = (await root.boundingBox())!;
        const panel = (await menu.boundingBox())!;
        expect(panel.y).toBeGreaterThanOrEqual(field.y + field.height - 2);
        expect(panel.width).toBeGreaterThanOrEqual(field.width - 2);
        await expect(menu.locator('.p-menu-item')).toHaveCount(4);
        await dropdown.click();
        await expect(menu).toHaveCount(0);
        await expect(dropdown).toHaveAttribute('aria-expanded', 'false');
    });

    test('menu item runs the command and closes the menu', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-splitbutton');
        await root.locator('.p-splitbutton-dropdown').click();
        await root.locator('.p-menu-item', { hasText: 'Update' }).click();
        await expect(page.locator('.p-toast-message')).toContainText('Data Updated');
        await expect(root.locator('.p-menu-overlay')).toHaveCount(0);
    });

    test('keys ↓ and Escape, outside click closes the menu', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-splitbutton');
        const dropdown = root.locator('.p-splitbutton-dropdown');
        await dropdown.focus();
        await page.keyboard.press('ArrowDown');
        await expect(root.locator('.p-menu-overlay')).toBeVisible();
        await page.keyboard.press('Escape');
        await page.locator('h2').click();
        await expect(root.locator('.p-menu-overlay')).toHaveCount(0);
        await dropdown.click();
        await expect(root.locator('.p-menu-overlay')).toBeVisible();
        await page.locator('h2').click();
        await expect(root.locator('.p-menu-overlay')).toHaveCount(0);
    });

    test('modifiers and severity are visible on the buttons', async ({ page }) => {
        const raised = page.locator('[data-demo=raised] .p-splitbutton').first();
        await expect(raised).toHaveClass(/p-splitbutton-raised/);
        expect(await raised.evaluate((el) => getComputedStyle(el).boxShadow)).not.toBe('none');
        const rounded = page.locator('[data-demo=rounded] .p-splitbutton').first();
        await expect(rounded).toHaveClass(/p-splitbutton-rounded/);
        const success = page.locator('[data-demo=severity] .p-splitbutton', { hasText: 'Success' });
        await expect(success.locator('.p-splitbutton-button')).toHaveClass(/p-button-success/);
        await expect(success.locator('.p-splitbutton-dropdown')).toHaveClass(/p-button-success/);
        const small = page.locator('[data-demo=sizes] .p-splitbutton-sm');
        const large = page.locator('[data-demo=sizes] .p-splitbutton-lg');
        expect((await small.boundingBox())!.height).toBeLessThan((await large.boundingBox())!.height);
    });

    test('disabled component does not open; template replaces the label', async ({ page }) => {
        const disabled = page.locator('[data-demo=disabled] .p-splitbutton');
        await expect(disabled.locator('.p-splitbutton-button')).toBeDisabled();
        await expect(disabled.locator('.p-splitbutton-dropdown')).toBeDisabled();
        const template = page.locator('[data-demo=template] .p-splitbutton');
        await expect(template.locator('.p-splitbutton-button')).toContainText('PrimeNG');
        await expect(template.locator('.p-button-label')).toHaveCount(0);
    });
});
