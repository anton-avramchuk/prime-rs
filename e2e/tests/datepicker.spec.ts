import { expect, test } from '@playwright/test';

test.describe('DatePicker', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1400 });
        await page.goto('/datepicker');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#basic_dp')).toBeAttached();
    });

    const POPUP = '.p-datepicker-panel:not(.p-datepicker-panel-inline)';
    const panel = (page) => page.locator(POPUP);
    const day = (page, d: number, month = 10) => page.locator(`${POPUP} [data-date="2026-${month}-${d}"]`);

    test('стили темы: поле и панель с сеткой 7 столбцов', async ({ page }) => {
        const field = page.locator('#basic_dp');
        expect(await field.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        await field.click();
        await expect(panel(page)).toBeVisible();
        expect(await panel(page).evaluate((el) => getComputedStyle(el).position)).toBe('absolute');
        const header = page.locator(`${POPUP} .p-datepicker-title`);
        await expect(header).toContainText('November');
        await expect(header).toContainText('2026');
        const cells = page.locator(`${POPUP} tbody tr`).first().locator('td');
        await expect(cells).toHaveCount(7);
        const a = (await cells.nth(0).boundingBox())!;
        const b = (await cells.nth(1).boundingBox())!;
        expect(b.x).toBeGreaterThan(a.x);
        expect(Math.abs(a.y - b.y)).toBeLessThan(1);
        await expect(page.locator(`${POPUP} .p-datepicker-today`)).toHaveCount(1);
    });

    test('выбор даты закрывает панель и заполняет поле', async ({ page }) => {
        await page.locator('#basic_dp').click();
        await day(page, 12).click();
        await expect(panel(page)).toHaveCount(0);
        await expect(page.locator('#basic_dp')).toHaveValue('11/12/2026');
        await expect(page.getByTestId('basic-echo')).toHaveText('2026-11-12');
        // Escape закрывает без выбора
        await page.locator('#basic_dp').click();
        await expect(panel(page)).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(panel(page)).toHaveCount(0);
    });

    test('ввод текста выбирает дату, а клик снаружи закрывает панель', async ({ page }) => {
        const field = page.locator('#basic_dp');
        await field.click();
        await field.fill('11/20/2026');
        await expect(page.getByTestId('basic-echo')).toHaveText('2026-11-20');
        await expect(day(page, 20)).toHaveClass(/p-datepicker-day-selected/);
        await page.mouse.click(5, 5);
        await expect(panel(page)).toHaveCount(0);
    });

    test('формат даты', async ({ page }) => {
        await expect(page.locator('#format_dp')).toHaveValue('Thursday, 5 November 2026');
    });

    test('inline: листание месяцев и виды', async ({ page }) => {
        const inline = page.locator('#inline_dp');
        await inline.scrollIntoViewIfNeeded();
        await expect(inline.locator('.p-datepicker-panel-inline')).toBeVisible();
        await inline.locator('.p-datepicker-next-button').click();
        await expect(inline.locator('.p-datepicker-title')).toContainText('December');
        await inline.locator('.p-datepicker-select-month').click();
        await expect(inline.locator('.p-datepicker-month-view')).toBeVisible();
        expect(await inline.locator('.p-datepicker-month').count()).toBe(12);
        await inline.locator('.p-datepicker-month').nth(0).click();
        await expect(inline.locator('.p-datepicker-title')).toContainText('January');
        await inline.locator('.p-datepicker-select-year').click();
        await expect(inline.locator('.p-datepicker-decade')).toContainText('2020 - 2029');
        await inline.locator('.p-datepicker-year').nth(9).click();
        await inline.locator('.p-datepicker-month').nth(5).click();
        await expect(inline.locator('.p-datepicker-title')).toContainText('June');
        await expect(inline.locator('.p-datepicker-title')).toContainText('2029');
        await inline.locator('[data-date="2029-5-15"]').click();
        await expect(page.getByTestId('inline-echo')).toHaveText('2029-6-15');
    });

    test('multiple и range', async ({ page }) => {
        const multiple = page.locator('#multiple_dp');
        await multiple.click();
        await day(page, 3).click();
        await day(page, 7).click();
        await expect(multiple).toHaveValue('11/03/2026, 11/07/2026');
        await day(page, 3).click();
        await expect(multiple).toHaveValue('11/07/2026');
        await page.mouse.click(5, 5);

        const range = page.locator('#range_dp');
        await range.scrollIntoViewIfNeeded();
        await range.click();
        await day(page, 5).click();
        await day(page, 9).click();
        await expect(range).toHaveValue('11/05/2026 - 11/09/2026');
        await expect(panel(page)).toHaveCount(0);
        await range.click();
        await expect(day(page, 7)).toHaveClass(/p-datepicker-day-selected-range/);
        expect(await day(page, 7).evaluate((el) => getComputedStyle(el).backgroundColor)).not.toBe('rgba(0, 0, 0, 0)');
    });

    test('min/max и выходные недоступны', async ({ page }) => {
        const field = page.locator('#minmax_dp');
        await field.scrollIntoViewIfNeeded();
        await field.click();
        await expect(day(page, 9)).toHaveClass(/p-disabled/);
        await expect(day(page, 26)).toHaveClass(/p-disabled/);
        await expect(day(page, 14)).toHaveClass(/p-disabled/); // суббота
        await expect(day(page, 12)).not.toHaveClass(/p-disabled/);
        await day(page, 9).click({ force: true });
        await expect(field).toHaveValue('');
        await day(page, 12).click();
        await expect(field).toHaveValue('11/12/2026');
    });

    test('кнопки «Сегодня» и «Очистить»', async ({ page }) => {
        const field = page.locator('#buttonbar_dp');
        await field.scrollIntoViewIfNeeded();
        await field.click();
        await page.locator(`${POPUP} .p-datepicker-today-button`).click();
        await expect(field).toHaveValue('11/05/2026');
        await field.click();
        await page.locator(`${POPUP} .p-datepicker-clear-button`).click();
        await expect(field).toHaveValue('');
    });

    test('выбор месяца и года целиком', async ({ page }) => {
        const month = page.locator('#month_dp');
        await month.scrollIntoViewIfNeeded();
        await month.click();
        await expect(page.locator(`${POPUP} .p-datepicker-month-view`)).toBeVisible();
        await page.locator(`${POPUP} .p-datepicker-month`).nth(3).click();
        await expect(month).toHaveValue('04/2026');
        const year = page.locator('#year_dp');
        await year.scrollIntoViewIfNeeded();
        await year.click();
        await page.locator(`${POPUP} .p-datepicker-year`).nth(4).click();
        await expect(year).toHaveValue('2024');
    });

    test('значки календаря, номера недель, два месяца, заблокированное поле', async ({ page }) => {
        const icon = page.locator('#icon_button_dp');
        await icon.scrollIntoViewIfNeeded();
        await icon.locator('xpath=following-sibling::button[contains(@class,"p-datepicker-dropdown")]').click();
        await expect(panel(page)).toBeVisible();
        await page.mouse.click(5, 5);
        const weeks = page.locator('#weeks_dp');
        await weeks.scrollIntoViewIfNeeded();
        await weeks.click();
        await expect(page.locator(`${POPUP} .p-datepicker-weeknumber`).first()).toBeVisible();
        await expect(page.locator(`${POPUP} .p-datepicker-weekheader`)).toBeVisible();
        await page.mouse.click(5, 5);
        const months = page.locator('#months_dp');
        await months.scrollIntoViewIfNeeded();
        await months.click();
        await expect(page.locator(`${POPUP} .p-datepicker-calendar`)).toHaveCount(2);
        await page.mouse.click(5, 5);
        await expect(page.locator('#disabled_dp')).toBeDisabled();
        await expect(page.locator('#invalid_dp')).toHaveClass(/p-invalid/);
    });
});
