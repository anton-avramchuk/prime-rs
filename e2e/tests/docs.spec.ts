import { expect, test } from '@playwright/test';

test.describe('Документация компонентов', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1440, height: 1000 });
        await page.goto('/autocomplete');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
    });

    test('оболочка: заголовок, описание, импорт, примеры и оглавление', async ({ page }) => {
        await expect(page.locator('h2')).toHaveText('AutoComplete');
        await expect(page.locator('.doc-desc')).toContainText('AutoComplete');
        await expect(page.locator('.doc-import pre')).toContainText('use prime_autocomplete::');
        await expect(page.locator('[data-demo="basic"]')).toBeVisible();
        const toc = page.locator('.doc-toc a');
        await expect(toc.first()).toHaveText('Import');
        await expect(page.locator('.doc-toc')).toContainText('Basic');
        await expect(page.locator('.doc-toc')).toContainText('Props');
    });

    test('кнопка Code раскрывает исходник примера', async ({ page }) => {
        const code = page.getByTestId('code-basic');
        await expect(code.locator('pre')).toBeHidden();
        await code.locator('summary').click();
        await expect(code.locator('pre')).toBeVisible();
        await expect(code.locator('pre')).toContainText('fn Basic()');
        await expect(code.locator('pre')).toContainText('AutoComplete');
        await expect(code.locator('.tk-k').first()).toBeVisible();
    });

    test('оглавление ведёт к якорю примера', async ({ page }) => {
        await page.locator('.doc-toc a', { hasText: 'Multiple' }).click();
        await expect(page).toHaveURL(/#multiple$/);
        await expect(page.locator('h3#multiple')).toBeInViewport();
    });

    test('справочник API: свойства, события, тема', async ({ page }) => {
        const props = page.locator('h3#props');
        await props.scrollIntoViewIfNeeded();
        const table = page.locator('h3#props ~ h4 + table.doc-table').first();
        await expect(table).toContainText('suggestions');
        await expect(table).toContainText('Prop<bool>');
        await expect(page.locator('h3#events ~ h4 + table.doc-table').first()).toContainText('on_complete');
        await expect(page.locator('h3#theming ~ table').first()).toContainText('.p-autocomplete');
    });

    test('меню: разделы, поиск и переход', async ({ page }) => {
        const nav = page.getByTestId('nav');
        await expect(nav.locator('.doc-nav-title', { hasText: 'Data' })).toBeVisible();
        await page.getByTestId('nav-search').fill('tree');
        await expect(nav.getByRole('link', { name: 'TreeTable' })).toBeVisible();
        await expect(nav.getByRole('link', { name: 'Button', exact: true })).toBeHidden();
        await expect(nav.getByRole('link', { name: 'AutoComplete' })).toBeHidden();
        await nav.getByRole('link', { name: 'TreeTable' }).click();
        await expect(page).toHaveURL(/\/treetable$/);
        await expect(page.locator('h2')).toHaveText('TreeTable');
        await expect(nav.getByRole('link', { name: 'TreeTable' })).toHaveAttribute('aria-current', 'page');
    });

    test('переход к соседним компонентам внизу страницы', async ({ page }) => {
        await page.locator('.doc-pager a', { hasText: 'CascadeSelect' }).click();
        await expect(page).toHaveURL(/\/cascadeselect$/);
        await expect(page.locator('h2')).toHaveText('CascadeSelect');
    });

    test('заметки из docs/*.md и таблица свойств есть у Table', async ({ page }) => {
        await page.goto('/table');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('h3#notes')).toBeAttached();
        await expect(page.locator('.doc-notes').first()).toContainText('PrimeNG');
        await expect(page.locator('h3#props ~ h4 + table.doc-table').first()).toContainText('row_key');
        await page.getByTestId('code-filterrow').locator('summary').click();
        await expect(page.getByTestId('code-filterrow').locator('pre')).toContainText('filterable');
    });
});
