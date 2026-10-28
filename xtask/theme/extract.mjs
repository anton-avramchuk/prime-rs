// Извлечение CSS темы Aura так же, как это делает PrimeNG (basecomponent.ts / basestyle.ts):
// переменные пресета из `Theme.getCommon/getComponent`, стили компонентов из `@primeuix/styles` с разрешёнными `dt()`.
// Запускается `cargo xtask gen-theme` из каталога с установленными зависимостями; печатает JSON в stdout.
import { css as Css, dt, Theme } from '@primeuix/styled';
import { minifyCSS, resolve } from '@primeuix/utils';
import Aura from '@primeuix/themes/aura';
import { style as baseStyle } from '@primeuix/styles/base';

const { darkModeSelector, components, extraBaseCss } = JSON.parse(process.argv[2]);

Theme.setTheme({ preset: Aura, options: { prefix: 'p', darkModeSelector, cssLayer: false } });

const common = Theme.getCommon('base', undefined);
const extra = resolve(extraBaseCss, { dt });
const globalStyle = Theme.transformCSS('global-style', `${resolve(baseStyle, { dt })}${Css`${common.style ?? ''}`}`);
const base = minifyCSS([common.primitive?.css, common.semantic?.css, common.global?.css, globalStyle, Css`${extra}`].filter(Boolean).join('\n'));

const out = { base, components: {} };
for (const name of [...components].sort()) {
    const { style } = await import(`@primeuix/styles/${name}`);
    const comp = Theme.getComponent(name, undefined);
    const body = Theme.transformCSS(name, `${resolve(style, { dt })}${Css`${comp.style ?? ''}`}`);
    out.components[name] = minifyCSS([comp.css, body].filter(Boolean).join('\n'));
}
process.stdout.write(JSON.stringify(out));
