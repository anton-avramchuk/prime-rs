import { test } from '@playwright/test';
import { mkdirSync, writeFileSync } from 'node:fs';

// Браузерные замеры: время создания 1000 компонентов и память кучи. Результаты — results/memory.json (метрики "меньше — лучше").
const ITERATIONS = 20;

test('память и время: 1000 компонентов, создание и удаление в цикле', async ({ page }) => {
    await page.goto('/');
    const cdp = await page.context().newCDPSession(page);
    await cdp.send('Performance.enable');

    const metrics = async () => {
        await cdp.send('HeapProfiler.collectGarbage');
        const { metrics } = await cdp.send('Performance.getMetrics');
        const get = (name: string) => metrics.find((m) => m.name === name)?.value ?? 0;
        return { heap: get('JSHeapUsedSize'), nodes: get('Nodes') };
    };
    // ждём гидрацию
    await page.waitForFunction(() => document.querySelector('[data-testid=bench-mount]') !== null);
    await page.waitForTimeout(500);

    const before = await metrics();
    const mountMs = await page.evaluate(async () => {
        const btn = document.querySelector<HTMLButtonElement>('[data-testid=bench-mount]')!;
        const t = performance.now();
        btn.click();
        await new Promise((r) => requestAnimationFrame(() => r(null)));
        return performance.now() - t;
    });
    await page.waitForFunction(() => document.querySelectorAll('[data-testid=bench-area] svg').length === 1000);
    const mounted = await metrics();

    for (let i = 0; i < ITERATIONS; i++) {
        await page.getByTestId('bench-clear').click();
        await page.getByTestId('bench-mount').click();
    }
    await page.getByTestId('bench-clear').click();
    await page.waitForFunction(() => document.querySelectorAll('[data-testid=bench-area] svg').length === 0);
    const after = await metrics();

    const result = {
        metrics: {
            'browser.mount-1000.ms': Math.round(mountMs * 10) / 10,
            'browser.mount-1000.heap_bytes': mounted.heap - before.heap,
            'browser.mount-1000.dom_nodes': mounted.nodes - before.nodes,
            // утечка: рост кучи и DOM после 20 циклов создания и удаления
            'browser.loop.heap_growth_bytes': Math.max(0, after.heap - before.heap),
            'browser.loop.dom_nodes_growth': after.nodes - before.nodes
        }
    };
    mkdirSync('results', { recursive: true });
    writeFileSync('results/memory.json', JSON.stringify(result, null, 2));
    console.log(JSON.stringify(result.metrics));
});
