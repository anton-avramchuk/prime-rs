import { expect, test } from '@playwright/test';

test.describe('ConfirmDialog', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/confirmdialog');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] button').first()).toBeAttached();
    });

    const basic = (page) => page.locator('[data-demo=basic]');
    const dialog = (page) => page.locator('.p-confirmdialog');

    test('a centered window with the mask, header, text and theme buttons', async ({ page }) => {
        await basic(page).getByRole('button', { name: 'Save' }).click();
        const d = dialog(page);
        await expect(d).toBeVisible();
        await expect(d).toHaveAttribute('role', 'alertdialog');
        await expect(d.locator('.p-dialog-title')).toHaveText('Confirmation');
        await expect(d.locator('.p-confirmdialog-message')).toHaveText('Please confirm to proceed.');
        await expect(d.locator('.p-dialog-footer button')).toHaveText(['Cancel', 'Save']);
        const mask = page.locator('.p-dialog-mask');
        expect(await mask.evaluate((el) => getComputedStyle(el).position)).toBe('fixed');
        const box = (await d.boundingBox())!;
        const view = page.viewportSize()!;
        expect(Math.abs(box.x + box.width / 2 - view.width / 2)).toBeLessThan(3);
        expect(Math.abs(box.y + box.height / 2 - view.height / 2)).toBeLessThan(3);
        expect(await d.evaluate((el) => getComputedStyle(el).backgroundColor)).not.toBe('rgba(0, 0, 0, 0)');
        expect(await page.evaluate(() => document.body.classList.contains('p-overflow-hidden'))).toBe(true);
        await expect(d.locator('.p-confirmdialog-accept-button')).toBeFocused();
    });

    test('icon next to the text, content laid out in a row', async ({ page }) => {
        await basic(page).getByRole('button', { name: 'Delete' }).click();
        const content = dialog(page).locator('.p-dialog-content');
        await expect(content.locator('.p-confirmdialog-icon')).toBeVisible();
        expect(await content.evaluate((el) => getComputedStyle(el).display)).toBe('flex');
        const icon = (await content.locator('.p-confirmdialog-icon').boundingBox())!;
        const text = (await content.locator('.p-confirmdialog-message').boundingBox())!;
        expect(text.x).toBeGreaterThanOrEqual(icon.x + icon.width - 1);
        await expect(dialog(page).locator('.p-confirmdialog-accept-button')).toHaveClass(/p-button-danger/);
    });

    test('accept and reject show messages and close the window', async ({ page }) => {
        await basic(page).getByRole('button', { name: 'Save' }).click();
        await dialog(page).locator('.p-confirmdialog-accept-button').click();
        await expect(dialog(page)).toHaveCount(0);
        await expect(page.locator('.p-toast-message').first()).toContainText('You have accepted');
        expect(await page.evaluate(() => document.body.classList.contains('p-overflow-hidden'))).toBe(false);
        await basic(page).getByRole('button', { name: 'Save' }).click();
        await dialog(page).locator('.p-confirmdialog-reject-button').click();
        await expect(dialog(page)).toHaveCount(0);
        await expect(page.locator('.p-toast-message')).toHaveCount(2);
    });

    test('close icon and Escape count as a rejection', async ({ page }) => {
        await basic(page).getByRole('button', { name: 'Save' }).click();
        await dialog(page).locator('.p-dialog-close-button').click();
        await expect(dialog(page)).toHaveCount(0);
        await expect(page.locator('.p-toast-message').first()).toContainText('You have rejected');
        await basic(page).getByRole('button', { name: 'Save' }).click();
        await expect(dialog(page)).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(dialog(page)).toHaveCount(0);
        await expect(page.locator('.p-toast-message')).toHaveCount(2);
    });

    test('positions: the window moves to edges and corners', async ({ page }) => {
        const demo = page.locator('[data-demo=position]');
        const view = page.viewportSize()!;
        // the mask has an inner padding (about 1.5rem): the window is pressed to the edge within it
        const edge = 30;
        const cases: [string, (b: { x: number; y: number; width: number; height: number }) => void][] = [
            ['Left', (b) => expect(b.x).toBeLessThan(edge)],
            ['Right', (b) => expect(view.width - (b.x + b.width)).toBeLessThan(edge)],
            ['TopLeft', (b) => (expect(b.x).toBeLessThan(edge), expect(b.y).toBeLessThan(edge))],
            ['BottomRight', (b) => (expect(view.width - (b.x + b.width)).toBeLessThan(edge), expect(view.height - (b.y + b.height)).toBeLessThan(edge))],
        ];
        for (const [name, check] of cases) {
            await demo.getByRole('button', { name, exact: true }).click();
            await expect(dialog(page)).toBeVisible();
            check((await dialog(page).boundingBox())!);
            await page.keyboard.press('Escape');
            await expect(dialog(page)).toHaveCount(0);
        }
    });

    test('templates: a custom message and headless', async ({ page }) => {
        await page.locator('[data-demo=template]').getByRole('button', { name: 'Save' }).click();
        await expect(page.getByTestId('custom-message')).toBeVisible();
        await expect(dialog(page).locator('.p-dialog-footer')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(dialog(page)).toHaveCount(0);

        await page.locator('[data-demo=headless]').getByRole('button', { name: 'Save' }).click();
        const headless = page.getByTestId('headless');
        await expect(headless).toBeVisible();
        await expect(dialog(page).locator('.p-dialog-footer')).toHaveCount(0);
        await headless.getByRole('button', { name: 'Save' }).click();
        await expect(dialog(page)).toHaveCount(0);
        await expect(page.locator('.p-toast-message').first()).toContainText('You have accepted');
    });
});
