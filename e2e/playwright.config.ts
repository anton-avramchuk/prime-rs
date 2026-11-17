import { defineConfig } from '@playwright/test';

// Servers are started externally: our showcase (cargo leptos serve) and, for comparison, the PrimeNG showcase.
// PW_CHANNEL=chrome allows using an installed Chrome instead of the downloaded Chromium.
export default defineConfig({
    testDir: './tests',
    outputDir: './results/artifacts',
    reporter: [['list'], ['json', { outputFile: './results/report.json' }]],
    retries: 0,
    workers: 1,
    use: {
        baseURL: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:3000',
        channel: process.env.PW_CHANNEL || undefined,
        viewport: { width: 1280, height: 800 },
        trace: 'off'
    }
});
