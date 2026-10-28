import { test } from '@playwright/test';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';

// Браузерные замеры: время создания 1000 компонентов и память. Результаты — results/memory.json (метрики «меньше — лучше»).
// JS-куча (`JSHeapUsedSize`) не включает память Rust, поэтому для wasm дополнительно читается `window.wasmMemoryBytes()` (размер линейной памяти).
const ITERATIONS = 20;
const OUT = 'results/memory.json';

const save = (metrics: Record<string, number>) => {
    mkdirSync('results', { recursive: true });
    let all: Record<string, number> = {};
    try {
        all = JSON.parse(readFileSync(OUT, 'utf8')).metrics ?? {};
    } catch {
        /* первый сценарий */
    }
    writeFileSync(OUT, JSON.stringify({ metrics: { ...all, ...metrics } }, null, 2));
};

for (const scenario of [
    { name: 'icons', mount: 'bench-mount', clear: 'bench-clear', area: 'bench-area', selector: '[data-testid=bench-area] svg', label: '1000 иконок' },
    { name: 'buttons-live', mount: 'bench-buttons-live-mount', clear: 'bench-buttons-live-clear', area: 'bench-buttons-live-area', selector: '[data-testid=bench-buttons-live-area] button', label: '1000 кнопок (сигнал)' },
    { name: 'badges', mount: 'bench-badges-mount', clear: 'bench-badges-clear', area: 'bench-badges-area', selector: '[data-testid=bench-badges-area] .p-badge', label: '1000 бейджей' },
    { name: 'badges-live', mount: 'bench-badges-live-mount', clear: 'bench-badges-live-clear', area: 'bench-badges-live-area', selector: '[data-testid=bench-badges-live-area] .p-badge', label: '1000 бейджей (сигнал)' },
    { name: 'buttons', mount: 'bench-buttons-mount', clear: 'bench-buttons-clear', area: 'bench-buttons-area', selector: '[data-testid=bench-buttons-area] button', label: '1000 кнопок' }
]) {
    test(`память и время: ${scenario.label}, создание и удаление в цикле`, async ({ page }) => {
        await page.goto('/');
        const cdp = await page.context().newCDPSession(page);
        await cdp.send('Performance.enable');

        const metrics = async () => {
            await cdp.send('HeapProfiler.collectGarbage');
            const { metrics } = await cdp.send('Performance.getMetrics');
            const get = (name: string) => metrics.find((m) => m.name === name)?.value ?? 0;
            const wasm = await page.evaluate(() => (window as unknown as { wasmMemoryBytes?: () => number }).wasmMemoryBytes?.() ?? 0);
            return { heap: get('JSHeapUsedSize'), nodes: get('Nodes'), wasm };
        };
        // ждём гидрацию
        await page.waitForFunction((id) => document.querySelector(`[data-testid=${id}]`) !== null, scenario.mount);
        await page.waitForFunction(() => typeof (window as unknown as { wasmMemoryBytes?: unknown }).wasmMemoryBytes === 'function');
        await page.waitForTimeout(500);

        const before = await metrics();
        const mountMs = await page.evaluate(async (id) => {
            const btn = document.querySelector<HTMLButtonElement>(`[data-testid=${id}]`)!;
            const t = performance.now();
            btn.click();
            await new Promise((r) => requestAnimationFrame(() => r(null)));
            return performance.now() - t;
        }, scenario.mount);
        await page.waitForFunction(([sel, n]) => document.querySelectorAll(sel as string).length === n, [scenario.selector, 1000] as const);
        const mounted = await metrics();

        for (let i = 0; i < ITERATIONS; i++) {
            await page.getByTestId(scenario.clear).click();
            await page.getByTestId(scenario.mount).click();
        }
        await page.getByTestId(scenario.clear).click();
        await page.waitForFunction((sel) => document.querySelectorAll(sel).length === 0, scenario.selector);
        const after = await metrics();

        const key = (name: string) => `browser.${scenario.name}.${name}`;
        const result = {
            [key('mount-1000.ms')]: Math.round(mountMs * 10) / 10,
            [key('mount-1000.heap_bytes')]: mounted.heap - before.heap,
            [key('mount-1000.wasm_bytes')]: mounted.wasm - before.wasm,
            [key('mount-1000.dom_nodes')]: mounted.nodes - before.nodes,
            // утечка: рост после 20 циклов создания и удаления
            [key('loop.heap_growth_bytes')]: Math.max(0, after.heap - before.heap),
            [key('loop.wasm_growth_bytes')]: Math.max(0, after.wasm - mounted.wasm),
            [key('loop.dom_nodes_growth')]: after.nodes - before.nodes
        };
        save(result);
        console.log(JSON.stringify(result));
    });
}
