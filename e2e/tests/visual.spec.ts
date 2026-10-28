import { expect, test } from '@playwright/test';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import pixelmatch from 'pixelmatch';
import { PNG } from 'pngjs';

// Сравнение страниц компонентов с оригиналом PrimeNG. Это отчёт о соответствии, а не блокировщик PR:
// расхождение не валит тест, а попадает в results/visual.json. Эталон снимается заново с запущенного PrimeNG showcase.
//
// pages.json: [{ "name": "button-basic", "ours": "/button", "upstream": "/button", "oursSelector": "[data-demo=\"basic\"]", "upstreamSelector": "basic-doc .card" }]
const UPSTREAM = process.env.UPSTREAM_URL;
const pages: { name: string; ours: string; upstream: string; oursSelector?: string; upstreamSelector?: string }[] = JSON.parse(readFileSync(new URL('./pages.json', import.meta.url), 'utf8'));
const report: Record<string, number> = {};

test.skip(!UPSTREAM, 'UPSTREAM_URL не задан: нечего сравнивать');

for (const p of pages) {
    test(`визуальное сравнение: ${p.name}`, async ({ page, baseURL }) => {
        const shot = async (url: string, selector?: string) => {
            await page.goto(url);
            await page.waitForLoadState('networkidle');
            const target = selector ? page.locator(selector).first() : page;
            return PNG.sync.read(await target.screenshot({ animations: 'disabled' }));
        };
        const ours = await shot(new URL(p.ours, baseURL).toString(), p.oursSelector);
        const upstream = await shot(new URL(p.upstream, UPSTREAM).toString(), p.upstreamSelector);
        const width = Math.min(ours.width, upstream.width);
        const height = Math.min(ours.height, upstream.height);
        const crop = (img: PNG) => {
            const out = new PNG({ width, height });
            PNG.bitblt(img, out, 0, 0, width, height, 0, 0);
            return out;
        };
        const diff = new PNG({ width, height });
        const mismatched = pixelmatch(crop(ours).data, crop(upstream).data, diff.data, width, height, { threshold: 0.1 });
        report[`visual.${p.name}.mismatch_pct`] = Math.round((mismatched / (width * height)) * 10000) / 100;
        mkdirSync('results', { recursive: true });
        writeFileSync(`results/${p.name}-diff.png`, PNG.sync.write(diff));
        writeFileSync('results/visual.json', JSON.stringify({ metrics: report }, null, 2));
        expect(true).toBe(true);
    });
}
