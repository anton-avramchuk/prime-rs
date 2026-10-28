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
const errors: Record<string, string> = {};
const save = () => {
    mkdirSync('results', { recursive: true });
    writeFileSync('results/visual.json', JSON.stringify({ metrics: report, errors }, null, 2));
};

test.skip(!UPSTREAM, 'UPSTREAM_URL не задан: нечего сравнивать');

for (const p of pages) {
    test(`визуальное сравнение: ${p.name}`, async ({ page, baseURL }) => {
        test.setTimeout(60_000);
        const shot = async (url: string, selector?: string) => {
            // networkidle не используем: у showcase PrimeNG есть фоновые запросы, страница не «затихает»
            const response = await page.goto(url, { waitUntil: 'load', timeout: 20_000 });
            if (!response?.ok()) throw new Error(`${url}: HTTP ${response?.status()}`);
            const target = selector ? page.locator(selector).first() : page;
            if (selector) await page.locator(selector).first().waitFor({ state: 'visible', timeout: 15_000 });
            await page.waitForTimeout(500);
            return PNG.sync.read(await target.screenshot({ animations: 'disabled', timeout: 10_000 }));
        };
        try {
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
            writeFileSync(`results/${p.name}-ours.png`, PNG.sync.write(ours));
            writeFileSync(`results/${p.name}-upstream.png`, PNG.sync.write(upstream));
            writeFileSync(`results/${p.name}-diff.png`, PNG.sync.write(diff));
        } catch (e) {
            // отчёт, а не блокировщик: причину сохраняем в visual.json, тест не падает
            errors[p.name] = String((e as Error).message).split('\n')[0];
        } finally {
            save();
        }
        expect(true).toBe(true);
    });
}
