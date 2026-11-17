import { expect, test, type Page } from '@playwright/test';

const tip = (page: Page) => page.locator('.p-tooltip');

test.describe('Tooltip', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/tooltip');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=position] input').first()).toBeVisible();
    });

    test('no tooltips until the mouse hovers', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(6);
        await expect(tip(page)).toHaveCount(0);
    });

    test('hover shows the tooltip on the right, leaving hides it', async ({ page }) => {
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

    test('top, bottom and left are placed on the correct side', async ({ page }) => {
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

    test('event=focus: hover does not show, focus shows, blur hides', async ({ page }) => {
        const input = page.locator('[data-demo=event] input');
        await input.hover();
        await page.waitForTimeout(300);
        await expect(tip(page)).toHaveCount(0);
        await input.focus();
        await expect(tip(page)).toBeVisible();
        await input.blur();
        await expect(tip(page)).toHaveCount(0);
    });

    test('delay: not shown immediately', async ({ page }) => {
        const button = page.locator('[data-demo=delay]').getByRole('button', { name: 'Save' });
        await button.hover();
        await page.waitForTimeout(500);
        await expect(tip(page)).toHaveCount(0);
        await expect(tip(page)).toBeVisible({ timeout: 2000 });
        await expect(tip(page)).toContainText('Confirm to proceed');
    });

    test('autohide=false: the tooltip can be hovered with the mouse, autohide=true: it cannot', async ({ page }) => {
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

    test('Escape hides the tooltip', async ({ page }) => {
        await page.locator('[data-demo=position] input[placeholder=Top]').hover();
        await expect(tip(page)).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(tip(page)).toHaveCount(0);
    });

    test('custom: custom content', async ({ page }) => {
        await page.locator('[data-demo=custom]').getByRole('button', { name: 'Button' }).hover();
        await expect(tip(page)).toBeVisible();
        await expect(tip(page)).toContainText('PrimeRS');
        await expect(tip(page)).toContainText('Custom tooltip content');
        await expect(tip(page)).toHaveClass(/p-tooltip-bottom/);
    });

    test('a tooltip at the right edge of the window moves to the left', async ({ page }) => {
        const input = page.locator('[data-demo=position] input[placeholder=Right]');
        await input.evaluate((el) => {
            (el as HTMLElement).style.position = 'fixed';
            (el as HTMLElement).style.right = '0px';
            (el as HTMLElement).style.top = '300px';
            (el as HTMLElement).style.zIndex = '1000'; // above the page side table of contents
        });
        await input.hover();
        await expect(tip(page)).toBeVisible();
        await expect(tip(page)).toHaveClass(/p-tooltip-left/);
    });

    test('the tooltip is visible on top of regular elements (tooltip layer)', async ({ page }) => {
        await page.locator('[data-demo=position] input[placeholder=Right]').hover();
        await expect(tip(page)).toBeVisible();
        const z = await tip(page).evaluate((el) => parseInt(getComputedStyle(el).zIndex || '0', 10));
        expect(z).toBeGreaterThan(0);
    });
});
