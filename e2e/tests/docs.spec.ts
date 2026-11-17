import { expect, test } from '@playwright/test';

test.describe('Component documentation', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1440, height: 1000 });
        await page.goto('/autocomplete');
        // clicks before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
    });

    test('shell: title, description, import, examples and table of contents', async ({ page }) => {
        await expect(page.locator('h2')).toHaveText('AutoComplete');
        await expect(page.locator('.doc-desc')).toContainText('AutoComplete');
        await expect(page.locator('.doc-import pre')).toContainText('use prime_autocomplete::');
        await expect(page.locator('[data-demo="basic"]')).toBeVisible();
        const toc = page.locator('.doc-toc a');
        await expect(toc.first()).toHaveText('Import');
        await expect(page.locator('.doc-toc')).toContainText('Basic');
        await expect(page.locator('.doc-toc')).toContainText('Props');
    });

    test('the Code button expands the example source', async ({ page }) => {
        const code = page.getByTestId('code-basic');
        await expect(code.locator('pre')).toBeHidden();
        await code.locator('summary').click();
        await expect(code.locator('pre')).toBeVisible();
        await expect(code.locator('pre')).toContainText('fn Basic()');
        await expect(code.locator('pre')).toContainText('AutoComplete');
        await expect(code.locator('.tk-k').first()).toBeVisible();
    });

    test('the table of contents leads to the example anchor', async ({ page }) => {
        await page.locator('.doc-toc a', { hasText: 'Multiple' }).click();
        await expect(page).toHaveURL(/#multiple$/);
        await expect(page.locator('h3#multiple')).toBeInViewport();
    });

    test('API reference: properties, events, theme', async ({ page }) => {
        const props = page.locator('h3#props');
        await props.scrollIntoViewIfNeeded();
        const table = page.locator('h3#props ~ h4 + table.doc-table').first();
        await expect(table).toContainText('suggestions');
        await expect(table).toContainText('Prop<bool>');
        await expect(page.locator('h3#events ~ h4 + table.doc-table').first()).toContainText('on_complete');
        await expect(page.locator('h3#theming ~ table').first()).toContainText('.p-autocomplete');
    });

    test('menu: sections, search and navigation', async ({ page }) => {
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

    test('navigation to neighbouring components at the bottom of the page', async ({ page }) => {
        await page.locator('.doc-pager a', { hasText: 'CascadeSelect' }).click();
        await expect(page).toHaveURL(/\/cascadeselect$/);
        await expect(page.locator('h2')).toHaveText('CascadeSelect');
    });

    test('notes from docs/*.md and the properties table are present in Table', async ({ page }) => {
        await page.goto('/table');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('h3#notes')).toBeAttached();
        await expect(page.locator('.doc-notes').first()).toContainText('PrimeNG');
        await expect(page.locator('h3#props ~ h4 + table.doc-table').first()).toContainText('row_key');
        await page.getByTestId('code-filterrow').locator('summary').click();
        await expect(page.getByTestId('code-filterrow').locator('pre')).toContainText('filterable');
    });
});
