import { expect, test } from '@playwright/test';

const png = (name: string, size = 16) => ({ name, mimeType: 'image/png', buffer: Buffer.alloc(size, 1) });

test.describe('FileUpload', () => {
    test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 1200 });
        await page.goto('/fileupload');
        // events before hydration (wasm loading) are lost
        await page.waitForLoadState('networkidle');
        await expect(page.locator('#advanced_upload')).toBeAttached();
    });

    const adv = (page) => page.locator('#advanced_upload');

    test('theme styles: header with buttons, empty content', async ({ page }) => {
        const root = adv(page).locator('.p-fileupload');
        await expect(root).toHaveClass(/p-fileupload-advanced/);
        expect(await root.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');
        await expect(root.locator('input[type=file]')).toBeHidden();
        const buttons = root.locator('.p-fileupload-header .p-button');
        await expect(buttons).toHaveCount(3);
        await expect(buttons.nth(0)).toContainText('Choose');
        await expect(buttons.nth(1)).toBeDisabled();
        await expect(buttons.nth(2)).toBeDisabled();
        await expect(root.locator('.p-fileupload-content')).toContainText('Drag and drop files to here to upload.');
    });

    test('file selection: list, size, icon, preview and removal', async ({ page }) => {
        const root = adv(page);
        await root.locator('input[type=file]').setInputFiles([png('a.png', 2048), png('b.png', 10)]);
        const files = root.locator('.p-fileupload-file');
        await expect(files).toHaveCount(2);
        await expect(files.nth(0).locator('.p-fileupload-file-name')).toHaveText('a.png');
        await expect(files.nth(0).locator('.p-fileupload-file-size')).toHaveText('2.000 KB');
        await expect(files.nth(0).locator('.p-fileupload-file-badge')).toHaveText('Pending');
        await expect(files.nth(0).locator('img')).toHaveAttribute('src', /^blob:/);
        await expect(root.locator('.p-progressbar')).toBeVisible();
        await expect(root.locator('.p-fileupload-header .p-button').nth(1)).toBeEnabled();
        await files.nth(0).locator('.p-fileupload-file-remove-button').click();
        await expect(files).toHaveCount(1);
        await root.locator('.p-fileupload-cancel-button').click();
        await expect(root.locator('.p-fileupload-file')).toHaveCount(0);
    });

    test('invalid type and size give messages', async ({ page }) => {
        const root = adv(page);
        await root.locator('input[type=file]').setInputFiles({ name: 'a.txt', mimeType: 'text/plain', buffer: Buffer.from('hi') });
        await expect(root.locator('.p-message').first()).toContainText('a.txt: Invalid file type');
        await expect(root.locator('.p-fileupload-file')).toHaveCount(0);
        await root.locator('input[type=file]').setInputFiles(png('big.png', 1_200_000));
        await expect(root.locator('.p-message').first()).toContainText('Invalid file size');
        await expect(root.locator('.p-message').first()).toContainText('maximum upload size is 976.563 KB.');
    });

    test('successful upload: progress, "Completed" icon, event', async ({ page }) => {
        await page.route('**/api/upload', async (route) => {
            await route.fulfill({ status: 200, contentType: 'application/json', body: '{"ok":true}' });
        });
        const root = adv(page);
        await root.locator('input[type=file]').setInputFiles(png('a.png'));
        await root.locator('.p-fileupload-upload-button').click();
        await expect(page.getByTestId('advanced-status')).toHaveText('uploaded 1 (200)');
        await expect(root.locator('.p-fileupload-file')).toHaveCount(1);
        await expect(root.locator('.p-fileupload-file-badge')).toHaveText('Completed');
        // an uploaded file is removed by the same button
        await root.locator('.p-fileupload-file-remove-button').click();
        await expect(root.locator('.p-fileupload-file')).toHaveCount(0);
    });

    test('a server error keeps the file in the list', async ({ page }) => {
        await page.route('**/api/upload', (route) => route.fulfill({ status: 500, body: 'fail' }));
        const root = adv(page);
        await root.locator('input[type=file]').setInputFiles(png('a.png'));
        await root.locator('.p-fileupload-upload-button').click();
        await expect(page.getByTestId('advanced-status')).toHaveText('error 500');
        await expect(root.locator('.p-fileupload-file-badge')).toHaveText('Pending');
        await expect(root.locator('.p-fileupload-cancel-button')).toBeEnabled();
    });

    test('dragging highlights the content and adds a file', async ({ page }) => {
        const root = adv(page);
        const content = root.locator('.p-fileupload-content');
        const data = await page.evaluateHandle(() => {
            const dt = new DataTransfer();
            dt.items.add(new File(['x'], 'dropped.png', { type: 'image/png' }));
            return dt;
        });
        await content.dispatchEvent('dragover', { dataTransfer: data });
        await expect(content).toHaveClass(/p-fileupload-highlight/);
        await content.dispatchEvent('drop', { dataTransfer: data });
        await expect(content).not.toHaveClass(/p-fileupload-highlight/);
        await expect(root.locator('.p-fileupload-file-name')).toHaveText('dropped.png');
    });

    test('auto: upload right after selection', async ({ page }) => {
        await page.route('**/api/upload', (route) => route.fulfill({ status: 200, body: 'ok' }));
        const root = page.locator('#auto_upload');
        await expect(root.locator('.p-fileupload-upload-button')).toHaveCount(0);
        await root.locator('input[type=file]').setInputFiles([png('1.png'), png('2.png', 4)]);
        await expect(page.getByTestId('auto-status')).toHaveText('uploaded 2 (200)');
    });

    test('basic: label and auto upload', async ({ page }) => {
        await page.route('**/api/upload', (route) => route.fulfill({ status: 201, body: 'ok' }));
        const root = page.locator('#basic_upload .p-fileupload');
        await expect(root).toHaveClass(/p-fileupload-basic/);
        await expect(root.locator('.p-fileupload-choose-button')).toContainText('Browse');
        await root.locator('input[type=file]').setInputFiles(png('one.png'));
        await expect(page.getByTestId('basic-status')).toHaveText('uploaded 1 (201)');
    });

    test('custom: own handler and clear', async ({ page }) => {
        const root = page.locator('#custom_upload');
        await root.locator('input[type=file]').setInputFiles([png('1.png'), png('2.png', 4)]);
        await expect(root.locator('.p-fileupload-file')).toHaveCount(2);
        await root.locator('.p-fileupload-upload-button').click();
        await expect(page.getByTestId('custom-status')).toHaveText('custom 2');
        await expect(root.locator('.p-fileupload-file')).toHaveCount(0);
    });
});
