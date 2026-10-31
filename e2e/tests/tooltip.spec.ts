import { expect, test, type Page } from '@playwright/test';

const tip = (page: Page) => page.locator('.p-tooltip');

test.describe('Tooltip', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/tooltip');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=position] input').first()).toBeVisible();
    });

    test('подсказок нет, пока не навели мышь', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        await expect(tip(page)).toHaveCount(0);
    });

    test('наведение показывает подсказку справа, уход скрывает', async ({ page }) => {
        const input = page.locator('[data-demo=position] input[placeholder=Right]');
        await input.hover();
        await expect(tip(page)).toBeVisible();
        await expect(tip(page)).toHaveClass(/p-tooltip-right/);
        await expect(tip(page)).toHaveAttribute('role', 'tooltip');
        await expect(tip(page).locator('.p-tooltip-text')).toHaveText('Enter your username');
        await expect(page.locator('body > .p-tooltip')).toHaveCount(1);
        const [i, t] = [(await input.boundingBox())!, (await tip(page).boundingBox())!];
        expect(t.x).toBeGreaterThanOrEqual(i.x + i.width - 1);
        await page.mouse.move(5, 5);
        await expect(tip(page)).toHaveCount(0);
    });

    test('верх, низ и лево расположены с нужной стороны', async ({ page }) => {
        const cases: [string, (i: any, t: any) => boolean][] = [
            ['Top', (i, t) => t.y + t.height <= i.y + 1],
            ['Bottom', (i, t) => t.y >= i.y + i.height - 1],
            ['Left', (i, t) => t.x + t.width <= i.x + 1],
        ];
        for (const [name, check] of cases) {
            const input = page.locator(`[data-demo=position] input[placeholder=${name}]`);
            await input.hover();
            await expect(tip(page)).toBeVisible();
            await expect(tip(page)).toHaveClass(new RegExp(`p-tooltip-${name.toLowerCase()}`));
            const [i, t] = [(await input.boundingBox())!, (await tip(page).boundingBox())!];
            expect(check(i, t), `${name}: ${JSON.stringify([i, t])}`).toBe(true);
            await page.mouse.move(5, 5);
            await expect(tip(page)).toHaveCount(0);
        }
    });

    test('event=focus: наведение не показывает, фокус показывает, blur скрывает', async ({ page }) => {
        const input = page.locator('[data-demo=event] input');
        await input.hover();
        await page.waitForTimeout(300);
        await expect(tip(page)).toHaveCount(0);
        await input.focus();
        await expect(tip(page)).toBeVisible();
        await input.blur();
        await expect(tip(page)).toHaveCount(0);
    });

    test('delay: показ не сразу', async ({ page }) => {
        const button = page.locator('[data-demo=delay]').getByRole('button', { name: 'Save' });
        await button.hover();
        await page.waitForTimeout(500);
        await expect(tip(page)).toHaveCount(0);
        await expect(tip(page)).toBeVisible({ timeout: 2000 });
        await expect(tip(page)).toContainText('Confirm to proceed');
    });

    test('autohide=false: подсказку можно навести мышью, autohide=true — нет', async ({ page }) => {
        const sticky = page.locator('[data-demo=autohide] input[placeholder="autoHide: false"]');
        await sticky.hover();
        await expect(tip(page)).toBeVisible();
        const t = (await tip(page).boundingBox())!;
        await page.mouse.move(t.x + t.width / 2, t.y + t.height / 2, { steps: 8 });
        await page.waitForTimeout(150);
        await expect(tip(page)).toBeVisible();
        await page.mouse.move(5, 5);
        await expect(tip(page)).toHaveCount(0);

        const normal = page.locator('[data-demo=autohide] input[placeholder="autoHide: true"]');
        await normal.hover();
        await expect(tip(page)).toBeVisible();
        await expect(tip(page)).toHaveCSS('pointer-events', 'none');
    });

    test('Escape скрывает подсказку', async ({ page }) => {
        await page.locator('[data-demo=position] input[placeholder=Top]').hover();
        await expect(tip(page)).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(tip(page)).toHaveCount(0);
    });

    test('custom: своё содержимое', async ({ page }) => {
        await page.locator('[data-demo=custom]').getByRole('button', { name: 'Button' }).hover();
        await expect(tip(page)).toBeVisible();
        await expect(tip(page)).toContainText('PrimeRS');
        await expect(tip(page)).toContainText('Custom tooltip content');
        await expect(tip(page)).toHaveClass(/p-tooltip-bottom/);
    });

    test('подсказка у правого края окна переносится влево', async ({ page }) => {
        const input = page.locator('[data-demo=position] input[placeholder=Right]');
        await input.evaluate((el) => {
            (el as HTMLElement).style.position = 'fixed';
            (el as HTMLElement).style.right = '0px';
            (el as HTMLElement).style.top = '300px';
        });
        await input.hover();
        await expect(tip(page)).toBeVisible();
        await expect(tip(page)).toHaveClass(/p-tooltip-left/);
    });

    test('подсказка видна поверх обычных элементов (слой подсказок)', async ({ page }) => {
        await page.locator('[data-demo=position] input[placeholder=Right]').hover();
        await expect(tip(page)).toBeVisible();
        const z = await tip(page).evaluate((el) => parseInt(getComputedStyle(el).zIndex || '0', 10));
        expect(z).toBeGreaterThan(0);
    });
});
