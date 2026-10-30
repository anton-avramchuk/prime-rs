import { expect, test } from '@playwright/test';

test.describe('Timeline', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/timeline');
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-timeline')).toBeVisible();
    });

    test('демонстрации и структура событий', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(4);
        const timeline = page.locator('[data-demo=basic] .p-timeline');
        await expect(timeline).toHaveClass(/p-timeline-left/);
        await expect(timeline).toHaveClass(/p-timeline-vertical/);
        await expect(timeline.locator('.p-timeline-event')).toHaveCount(4);
        await expect(timeline.locator('.p-timeline-event-marker')).toHaveCount(4);
        await expect(timeline.locator('.p-timeline-event-connector')).toHaveCount(3);
        await expect(timeline.locator('.p-timeline-event-content').first()).toHaveText('Ordered');
    });

    test('вертикально события идут друг под другом, горизонтально — в строку', async ({ page }) => {
        const vertical = page.locator('[data-demo=basic] .p-timeline-event');
        const a = (await vertical.nth(0).boundingBox())!;
        const b = (await vertical.nth(1).boundingBox())!;
        expect(b.y).toBeGreaterThan(a.y + 1);
        expect(Math.abs(b.x - a.x)).toBeLessThan(2);

        const horizontal = page.locator('[data-demo=horizontal] .p-timeline-horizontal').first().locator('.p-timeline-event');
        const h0 = (await horizontal.nth(0).boundingBox())!;
        const h1 = (await horizontal.nth(1).boundingBox())!;
        expect(h1.x).toBeGreaterThan(h0.x + 1);
        expect(Math.abs(h1.y - h0.y)).toBeLessThan(2);
    });

    test('align: right и alternate', async ({ page }) => {
        const timelines = page.locator('[data-demo=alignment] .p-timeline');
        await expect(timelines.nth(1)).toHaveClass(/p-timeline-right/);
        await expect(timelines.nth(2)).toHaveClass(/p-timeline-alternate/);
        const content = (i: number, j: number) => timelines.nth(i).locator('.p-timeline-event-content').nth(j).boundingBox();
        const first = (await content(2, 0))!;
        const second = (await content(2, 1))!;
        expect(Math.abs(first.x - second.x)).toBeGreaterThan(5);
    });

    test('opposite: даты по другую сторону линии', async ({ page }) => {
        const timeline = page.locator('[data-demo=opposite] .p-timeline');
        await expect(timeline.locator('.p-timeline-event-opposite').first()).toContainText('15/10/2020 10:30');
        const opposite = (await timeline.locator('.p-timeline-event-opposite').first().boundingBox())!;
        const content = (await timeline.locator('.p-timeline-event-content').first().boundingBox())!;
        expect(opposite.x).toBeLessThan(content.x);
    });
});
