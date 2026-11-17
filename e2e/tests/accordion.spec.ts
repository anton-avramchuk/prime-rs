import { expect, test } from '@playwright/test';

test.describe('Accordion', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/accordion');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-accordion')).toBeVisible();
        await page.waitForFunction(() => document.querySelector('[data-demo=controlled] .p-accordionheader') !== null);
        await page.waitForTimeout(300);
    });

    test('demos are rendered with theme styles', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        const header = page.locator('[data-demo=basic] .p-accordionheader').first();
        const padding = await header.evaluate((el) => getComputedStyle(el).paddingTop);
        expect(padding).not.toBe('0px');
        await expect(header.locator('.p-accordionheader-toggle-icon')).toBeVisible();
    });

    test('basic: the first panel is open, the rest are hidden', async ({ page }) => {
        const panels = page.locator('[data-demo=basic] .p-accordionpanel');
        await expect(panels).toHaveCount(4);
        await expect(panels.nth(0)).toHaveClass(/p-accordionpanel-active/);
        await expect(panels.nth(0).locator('.p-accordioncontent')).toContainText('Lorem ipsum');
        await expect(panels.nth(0).locator('.p-accordioncontent-content')).toBeVisible();
        await expect(panels.nth(1).locator('.p-accordioncontent-content')).toBeHidden();
    });

    test('basic: single mode with animation and aria', async ({ page }) => {
        const demo = page.locator('[data-demo=basic]');
        const headers = demo.locator('.p-accordionheader');
        const second = demo.locator('.p-motion').nth(1);
        await expect(headers.nth(0)).toHaveAttribute('aria-expanded', 'true');
        await expect(headers.nth(1)).toHaveAttribute('aria-controls', (await demo.locator('.p-accordioncontent').nth(1).getAttribute('id')) ?? '');

        await headers.nth(1).click();
        await expect(second).toHaveClass(/p-collapsible-enter-active/);
        await expect(second).not.toHaveClass(/p-collapsible-enter-active/);
        await expect(headers.nth(1)).toHaveAttribute('aria-expanded', 'true');
        await expect(headers.nth(0)).toHaveAttribute('aria-expanded', 'false');
        await expect(demo.locator('.p-accordioncontent-content').nth(1)).toBeVisible();
        await expect(demo.locator('.p-accordioncontent-content').nth(0)).toBeHidden();

        await headers.nth(1).click();
        await expect(headers.nth(1)).toHaveAttribute('aria-expanded', 'false');
        await expect(demo.locator('.p-accordioncontent-content').nth(1)).toBeHidden();
    });

    test('multiple: several panels can be kept open', async ({ page }) => {
        const demo = page.locator('[data-demo=multiple]');
        const headers = demo.locator('.p-accordionheader');
        await headers.nth(1).click();
        await expect(headers.nth(0)).toHaveAttribute('aria-expanded', 'true');
        await expect(headers.nth(1)).toHaveAttribute('aria-expanded', 'true');
        await expect(demo.locator('.p-accordioncontent-content').nth(1)).toBeVisible();
    });

    test('disabled: a disabled panel does not open, arrows skip it', async ({ page }) => {
        const demo = page.locator('[data-demo=disabled]');
        const headers = demo.locator('.p-accordionheader');
        await expect(headers.nth(3)).toHaveAttribute('aria-disabled', 'true');
        await expect(demo.locator('.p-accordionpanel').nth(3)).toHaveClass(/p-disabled/);
        await headers.nth(3).click({ force: true });
        await expect(headers.nth(3)).toHaveAttribute('aria-expanded', 'false');

        await headers.nth(2).focus();
        await page.keyboard.press('ArrowDown');
        await expect(headers.nth(0)).toBeFocused();
        await page.keyboard.press('ArrowUp');
        await expect(headers.nth(2)).toBeFocused();
        await page.keyboard.press('Home');
        await expect(headers.nth(0)).toBeFocused();
        await page.keyboard.press('End');
        await expect(headers.nth(2)).toBeFocused();
    });

    test('keyboard: Enter and Space toggle the panel', async ({ page }) => {
        const headers = page.locator('[data-demo=basic] .p-accordionheader');
        await headers.nth(1).focus();
        await page.keyboard.press('Enter');
        await expect(headers.nth(1)).toHaveAttribute('aria-expanded', 'true');
        await page.keyboard.press('Space');
        await expect(headers.nth(1)).toHaveAttribute('aria-expanded', 'false');
    });

    test('controlled: buttons control the active panel', async ({ page }) => {
        const demo = page.locator('[data-demo=controlled]');
        const headers = demo.locator('.p-accordionheader');
        await demo.getByRole('button', { name: '3', exact: true }).click();
        await expect(headers.nth(2)).toHaveAttribute('aria-expanded', 'true');
        await expect(headers.nth(0)).toHaveAttribute('aria-expanded', 'false');
        await headers.nth(1).click();
        await expect(headers.nth(1)).toHaveAttribute('aria-expanded', 'true');
        await expect(headers.nth(2)).toHaveAttribute('aria-expanded', 'false');
    });

    test('dynamic: panels from a list', async ({ page }) => {
        const demo = page.locator('[data-demo=dynamic]');
        await expect(demo.locator('.p-accordionpanel')).toHaveCount(3);
        await expect(demo.locator('.p-accordionheader').first()).toContainText('Title 1');
        await expect(demo.locator('.p-accordioncontent-content').first()).toHaveText('Content 1');
    });

    test('template: the icon slot changes with the state', async ({ page }) => {
        const demo = page.locator('[data-demo=template]');
        const headers = demo.locator('.p-accordionheader');
        await expect(headers.first()).toContainText('Amy Elsner');
        const svg = async (i: number) => headers.nth(i).locator('svg').last().innerHTML();
        const open = await svg(0);
        const closed = await svg(1);
        expect(open).not.toBe(closed);
        await headers.nth(1).click();
        await expect.poll(() => svg(1)).toBe(open);
    });
});
