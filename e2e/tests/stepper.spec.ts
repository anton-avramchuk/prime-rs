import { expect, test } from '@playwright/test';

test.describe('Stepper', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/stepper');
        // клики до гидратации (загрузка wasm) теряются
        await page.waitForLoadState('networkidle');
        await expect(page.locator('[data-demo=basic] .p-stepper')).toBeAttached();
    });

    test('демонстрации отрисованы со стилями темы', async ({ page }) => {
        await expect(page.locator('[data-demo]')).toHaveCount(5);
        const root = page.locator('[data-demo=basic] .p-stepper');
        await expect(root.locator('.p-step')).toHaveCount(3);
        await expect(root.locator('.p-stepper-separator')).toHaveCount(2);
        const list = await root.locator('.p-steplist').evaluate((el) => getComputedStyle(el).display);
        expect(list).toBe('flex');
        const number = root.locator('.p-step-number').first();
        expect(await number.evaluate((el) => getComputedStyle(el).borderTopLeftRadius)).not.toBe('0px');
        const active = await number.evaluate((el) => getComputedStyle(el).color);
        const plain = await root.locator('.p-step-number').nth(1).evaluate((el) => getComputedStyle(el).color);
        expect(active).not.toBe(plain);
        const separator = await root.locator('.p-stepper-separator').first().evaluate((el) => getComputedStyle(el).height);
        expect(separator).not.toBe('0px');
    });

    test('клик по шагу и кнопки «Далее»/«Назад» переключают панели', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-stepper');
        await expect(root.locator('.p-steppanel-content')).toContainText('Content I');
        await root.locator('.p-step-header', { hasText: 'Header III' }).click();
        await expect(page.getByTestId('echo')).toHaveText('3');
        await expect(root.locator('.p-steppanel-content')).toContainText('Content III');
        await expect(root.locator('.p-steppanel-content')).toHaveCount(1);
        await root.getByRole('button', { name: 'Back' }).click();
        await expect(page.getByTestId('echo')).toHaveText('2');
        await root.getByRole('button', { name: 'Next' }).click();
        await expect(page.getByTestId('echo')).toHaveText('3');
        await expect(root.locator('.p-step-active .p-step-number')).toHaveText('3');
    });

    test('клавиатура: Tab и Enter по шагам', async ({ page }) => {
        const root = page.locator('[data-demo=basic] .p-stepper');
        await root.locator('.p-step-header').first().focus();
        await page.keyboard.press('Tab');
        await expect(root.locator('.p-step-header').nth(1)).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(page.getByTestId('echo')).toHaveText('2');
        await expect(root.locator('.p-step-header').nth(1)).toHaveAttribute('role', 'tab');
        await expect(root.locator('.p-step').nth(1)).toHaveAttribute('aria-current', 'step');
    });

    test('линейный режим блокирует клики по шагам, но не кнопки', async ({ page }) => {
        const root = page.locator('[data-demo=linear] .p-stepper');
        await expect(root).toHaveClass(/p-readonly/);
        await expect(root.locator('.p-step-header').nth(2)).toBeDisabled();
        await root.locator('.p-step-header').nth(2).click({ force: true });
        await expect(root.locator('.p-step-active .p-step-number')).toHaveText('1');
        await root.getByRole('button', { name: 'Next' }).click();
        await expect(root.locator('.p-step-active .p-step-number')).toHaveText('2');
        await expect(root.locator('.p-step-header').first()).toBeDisabled();
    });

    test('вертикальная раскладка раскрывает панель активного элемента', async ({ page }) => {
        const root = page.locator('[data-demo=vertical] .p-stepper');
        await expect(root.locator('.p-stepitem')).toHaveCount(3);
        await expect(root.locator('.p-stepitem-active')).toHaveCount(1);
        await expect(root.locator('.p-steppanel-active')).toContainText('Content I');
        await expect(root.locator('.p-stepper-separator')).toHaveCount(2);
        await root.locator('.p-step-header', { hasText: /Header II$/ }).click();
        await expect(root.locator('.p-stepitem-active .p-step-number')).toHaveText('2');
        await expect(root.locator('.p-steppanel-active')).toContainText('Content II');
        const second = root.locator('.p-steppanel').nth(1);
        await expect(second.locator('.p-motion')).toBeVisible();
        await expect(root.locator('.p-steppanel').first().locator('.p-motion')).toBeHidden();
    });

    test('шаблон: круглые шаги закрашиваются по ходу', async ({ page }) => {
        const root = page.locator('[data-demo=template] .p-stepper');
        const color = (n: number) => root.getByTestId(`round-${n}`).locator('span').evaluate((el) => getComputedStyle(el).backgroundColor);
        const first = await color(1);
        const third = await color(3);
        expect(first).not.toBe(third);
        await root.getByRole('button', { name: 'Next' }).click();
        await expect(root.locator('.p-steppanel-content')).toContainText('Choose your interests');
        expect(await color(2)).toBe(first);
        await root.getByRole('button', { name: 'Nature' }).click();
        await root.getByRole('button', { name: 'Next' }).click();
        await expect(root.locator('.p-steppanel-content')).toContainText('Account created successfully');
        expect(await color(3)).toBe(first);
    });

    test('только список шагов без панелей', async ({ page }) => {
        const root = page.locator('[data-demo=stepsonly] .p-stepper');
        await expect(root.locator('.p-step')).toHaveCount(3);
        await expect(root.locator('.p-steppanels')).toHaveCount(0);
        await root.locator('.p-step-header', { hasText: 'QA' }).click();
        await expect(root.locator('.p-step-active .p-step-title')).toHaveText('QA');
    });
});
