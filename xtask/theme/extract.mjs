// Extraction of the Aura theme CSS the same way PrimeNG does it (basecomponent.ts / basestyle.ts):
// preset variables from `Theme.getCommon/getComponent`, component styles from `@primeuix/styles` with resolved `dt()`.
// Run by `cargo xtask gen-theme` from a directory with installed dependencies; prints JSON to stdout.
import { css as Css, dt, Theme } from '@primeuix/styled';
import { minifyCSS, resolve } from '@primeuix/utils';
import Aura from '@primeuix/themes/aura';
import { style as baseStyle } from '@primeuix/styles/base';

const { darkModeSelector, components, extraBaseCss, extraComponentCss = {} } = JSON.parse(process.argv[2]);

Theme.setTheme({ preset: Aura, options: { prefix: 'p', darkModeSelector, cssLayer: false } });

const common = Theme.getCommon('base', undefined);
// Like `BaseStyle.load` in PrimeNG: the style text goes through `resolve` and then through the `Css` tag, which substitutes the `dt('...')` values.
const resolved = (style) => Css`${resolve(style, { dt })}`;
const extra = resolved(extraBaseCss);
const globalStyle = Theme.transformCSS('global-style', `${resolved(baseStyle)}${Css`${common.style ?? ''}`}`);
const base = minifyCSS([common.primitive?.css, common.semantic?.css, common.global?.css, globalStyle, Css`${extra}`].filter(Boolean).join('\n'));

const out = { base, components: {} };
for (const name of [...components].sort()) {
    // Components without a module in `@primeuix/styles` (InputMask) have only the `For PrimeNG` block.
    let style = '';
    try {
        ({ style } = await import(`@primeuix/styles/${name}`));
    } catch (e) {
        if (e?.code !== 'ERR_MODULE_NOT_FOUND' || !extraComponentCss[name]) throw e;
    }
    const comp = Theme.getComponent(name, undefined);
    const extraComp = extraComponentCss[name] ? resolved(extraComponentCss[name]) : '';
    const body = Theme.transformCSS(name, `${resolved(style)}${extraComp}${Css`${comp.style ?? ''}`}`);
    out.components[name] = minifyCSS([comp.css, body].filter(Boolean).join('\n'));
}
process.stdout.write(JSON.stringify(out));
