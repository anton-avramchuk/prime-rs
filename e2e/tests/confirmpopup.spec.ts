import { expect, test } from '@playwright/test';

test.describe('ConfirmPopup', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/confirmpopup');
        // события до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] button').first()).toBeAttached();
    });

    const basic = (page) => page.locator('[data-demo=basic]');

    test('панель открывается под кнопкой со стрелкой и стилями темы', async ({ page }) => {
        const save = basic(page).getByRole('button', { name: 'Save' });
        await save.click();
        const panel = page.locator('.p-confirmpopup');
        await expect(panel).toBeVisible();
        await expect(panel).toHaveAttribute('role', 'alertdialog');
        await expect(panel.locator('.p-confirmpopup-message')).toHaveText('Are you sure you want to proceed?');
        expect(await panel.evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        expect(await panel.evaluate((el) => getComputedStyle(el).backgroundColor)).not.toBe('rgba(0, 0, 0, 0)');
        expect(await panel.evaluate((el) => getComputedStyle(el).boxShadow)).not.toBe('none');
        const p = (await panel.boundingBox())!;
        const b = (await save.boundingBox())!;
        expect(p.y).toBeGreaterThanOrEqual(b.y + b.height - 1);
        expect(await panel.evaluate((el) => el.style.getPropertyValue('--p-confirmpopup-arrow-left'))).not.toBe('');
        const arrow = await panel.evaluate((el) => getComputedStyle(el, '::before').content);
        expect(arrow).not.toBe('none');
        await expect(panel.locator('.p-confirmpopup-footer button')).toHaveText(['Cancel', 'Save']);
        await expect(panel.locator('.p-confirmpopup-accept-button')).toBeFocused();
    });

    test('согласие показывает сообщение, закрывает панель и возвращает фокус', async ({ page }) => {
        const save = basic(page).getByRole('button', { name: 'Save' });
        await save.click();
        await page.locator('.p-confirmpopup-accept-button').click();
        await expect(page.locator('.p-confirmpopup')).toHaveCount(0);
        await expect(page.locator('.p-toast-message').first()).toContainText('You have accepted');
        await expect(save).toBeFocused();
    });

    test('отказ и Escape показывают сообщение об отказе', async ({ page }) => {
        const save = basic(page).getByRole('button', { name: 'Save' });
        await save.click();
        await page.locator('.p-confirmpopup-reject-button').click();
        await expect(page.locator('.p-confirmpopup')).toHaveCount(0);
        await expect(page.locator('.p-toast-message').first()).toContainText('You have rejected');
        await save.click();
        await expect(page.locator('.p-confirmpopup')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-confirmpopup')).toHaveCount(0);
        await expect(page.locator('.p-toast-message')).toHaveCount(2);
    });

    test('клик снаружи закрывает без ответа, клик по цели и панели — нет', async ({ page }) => {
        const save = basic(page).getByRole('button', { name: 'Save' });
        await save.click();
        await page.locator('.p-confirmpopup-message').click();
        await expect(page.locator('.p-confirmpopup')).toBeVisible();
        await page.mouse.click(5, 5);
        await expect(page.locator('.p-confirmpopup')).toHaveCount(0);
        await expect(page.locator('.p-toast-message')).toHaveCount(0);
    });

    test('вторая кнопка переносит панель и меняет подписи', async ({ page }) => {
        const demo = basic(page);
        await demo.getByRole('button', { name: 'Save' }).click();
        await expect(page.locator('.p-confirmpopup')).toBeVisible();
        const del = demo.getByRole('button', { name: 'Delete' });
        await del.click();
        await expect(page.locator('.p-confirmpopup-message')).toHaveText('Do you want to delete this record?');
        const accept = page.locator('.p-confirmpopup-accept-button');
        await expect(accept).toHaveText('Delete');
        await expect(accept).toHaveClass(/p-button-danger/);
        await expect(page.locator('.p-confirmpopup')).toHaveCount(1);
        const p = (await page.locator('.p-confirmpopup').boundingBox())!;
        const b = (await del.boundingBox())!;
        expect(p.x + p.width).toBeGreaterThan(b.x);
    });

    test('у нижнего края окна панель переворачивается над кнопкой', async ({ page }) => {
        const save = basic(page).getByRole('button', { name: 'Save' });
        // окно ниже кнопки не вмещает панель: она должна оказаться над кнопкой
        const b0 = (await save.boundingBox())!;
        await page.setViewportSize({ width: 1280, height: Math.round(b0.y + b0.height + 40) });
        await save.click();
        const panel = page.locator('.p-confirmpopup');
        await expect(panel).toBeVisible();
        const p = (await panel.boundingBox())!;
        const b = (await save.boundingBox())!;
        expect(p.y + p.height).toBeLessThanOrEqual(b.y + 2);
        await expect(panel).toHaveAttribute('data-p-confirmpopup-flipped', 'true');
        await expect(panel).toHaveClass(/p-confirm-popup-flipped/);
    });

    test('шаблоны: своё содержимое и headless', async ({ page }) => {
        await page.locator('[data-demo=template]').getByRole('button', { name: 'Save' }).click();
        await expect(page.getByTestId('custom-content')).toBeVisible();
        await expect(page.locator('.p-confirmpopup-footer')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(page.locator('.p-confirmpopup')).toHaveCount(0);

        await page.locator('[data-demo=headless]').getByRole('button', { name: 'Save' }).click();
        const headless = page.getByTestId('headless');
        await expect(headless).toBeVisible();
        await expect(page.locator('.p-confirmpopup-footer')).toHaveCount(0);
        await headless.getByRole('button', { name: 'Save' }).click();
        await expect(page.locator('.p-confirmpopup')).toHaveCount(0);
        await expect(page.locator('.p-toast-message').first()).toContainText('You have accepted');
    });
});
