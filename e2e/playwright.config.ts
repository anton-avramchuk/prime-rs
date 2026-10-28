import { defineConfig } from '@playwright/test';

// Серверы запускаются снаружи: наш showcase (cargo leptos serve) и, для сравнения, showcase PrimeNG.
// PW_CHANNEL=chrome позволяет использовать установленный Chrome вместо скачанного Chromium.
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
