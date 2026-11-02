import { expect, test } from '@playwright/test';

test.describe('Paginator', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/paginator');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-paginator')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        const root = page.locator('[data-demo=basic] .p-paginator');
        await expect(root.locator('.p-paginator-page')).toHaveCount(5);
        await expect(root.locator('.p-paginator-page-selected')).toHaveText('1');
        const display = await root.evaluate((el) => getComputedStyle(el).display);
        expect(display).toBe('flex');
        const radius = await root.locator('.p-paginator-page').first().evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
        expect(radius).not.toBe('0px');
        const selected = await root.locator('.p-paginator-page-selected').evaluate((el) => getComputedStyle(el).backgroundColor);
        const plain = await root.locator('.p-paginator-page').nth(1).evaluate((el) => getComputedStyle(el).backgroundColor);
        expect(selected).not.toBe(plain);
    });

    test('кнопки и ссылки переключают страницы', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-paginator');
        await expect(root.locator('.p-paginator-prev')).toBeDisabled();
        await root.locator('.p-paginator-next').click();
        await expect(page.getByTestId('echo')).toHaveText('first=10 rows=10');
        await expect(root.locator('.p-paginator-page-selected')).toHaveText('2');
        await root.locator('.p-paginator-page', { hasText: /^5$/ }).click();
        await expect(page.getByTestId('echo')).toHaveText('first=40 rows=10');
        await expect(root.locator('.p-paginator-page').first()).toHaveText('3');
        await root.locator('.p-paginator-last').click();
        await expect(page.getByTestId('echo')).toHaveText('first=110 rows=10');
        await expect(root.locator('.p-paginator-next')).toBeDisabled();
        await root.locator('.p-paginator-first').click();
        await expect(page.getByTestId('echo')).toHaveText('first=0 rows=10');
    });

    test('клавиатура: Tab и Enter по кнопкам', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-paginator');
        await root.locator('.p-paginator-page', { hasText: /^2$/ }).focus();
        await page.keyboard.press('Enter');
        await expect(page.getByTestId('echo')).toHaveText('first=10 rows=10');
        await page.keyboard.press('Tab');
        await expect(root.locator('.p-paginator-page', { hasText: /^3$/ })).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(page.getByTestId('echo')).toHaveText('first=20 rows=10');
    });

    test('список числа строк пересчитывает страницы', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-paginator');
        await root.locator('.p-paginator-page', { hasText: /^3$/ }).click();
        await expect(page.getByTestId('echo')).toHaveText('first=20 rows=10');
        const rpp = root.locator('.p-paginator-rpp-dropdown');
        await expect(rpp.locator('.p-select-label')).toHaveText('10');
        await rpp.click();
        await rpp.locator('.p-select-option', { hasText: '30' }).click();
        await expect(page.getByTestId('echo')).toHaveText('first=0 rows=30');
        await expect(root.locator('.p-paginator-page')).toHaveCount(4);
        await expect(rpp.locator('.p-select-label')).toHaveText('30');
    });

    test('отчёт о странице показывает диапазон записей', async ({ page }) => {
        const root = page.locator('[data-demo=currentpagereport] .p-paginator');
        await expect(root.locator('.p-paginator-current')).toHaveText('Showing 1 to 10 of 120');
        await expect(root.locator('.p-paginator-page')).toHaveCount(0);
        await root.locator('.p-paginator-next').click();
        await expect(root.locator('.p-paginator-current')).toHaveText('Showing 11 to 20 of 120');
        await root.locator('.p-paginator-last').click();
        await expect(root.locator('.p-paginator-current')).toHaveText('Showing 111 to 120 of 120');
    });

    test('список перехода к странице меняет картинку', async ({ page }) => {
        const root = page.locator('[data-demo=images] .p-paginator');
        await expect(page.getByTestId('image')).toHaveText('Image 1');
        const jtp = root.locator('.p-paginator-jtp-dropdown');
        await expect(jtp.locator('.p-select-label')).toHaveText('1 of 12');
        await jtp.click();
        await jtp.locator('.p-select-option').nth(6).click();
        await expect(page.getByTestId('image')).toHaveText('Image 7');
        await expect(jtp.locator('.p-select-label')).toHaveText('7 of 12');
        await root.locator('.p-paginator-next').click();
        await expect(page.getByTestId('image')).toHaveText('Image 8');
    });

    test('шаблонные области и ползунок', async ({ page }) => {
        const demo = page.locator('[data-demo=template]');
        await expect(demo.locator('.p-button')).toHaveCount(2);
        const second = demo.locator('.p-paginator').nth(1);
        await expect(second.locator('.p-paginator-current')).toHaveText('1 - 10 of 120');
        await second.locator('.p-paginator-next').click();
        await expect(second.locator('.p-paginator-current')).toHaveText('11 - 20 of 120');
        const items = demo.locator('.p-select').nth(1);
        await items.click();
        await items.locator('.p-select-option', { hasText: '30' }).click();
        await expect(second.locator('.p-paginator-current')).toHaveText('1 - 30 of 120');
    });
});
