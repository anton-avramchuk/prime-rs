//! Преобразования цветов HSB / RGB / HEX — чистый порт формул `colorpicker.ts` без DOM.
//!
//! Округления и порядок операций совпадают с PrimeNG (там те же числа `f64`), поэтому результаты побитово те же.

/// Размер поля насыщенности/яркости и шкалы оттенка в пикселях.
pub const AREA: f64 = 150.0;

/// Цвет в модели HSB: оттенок `h` 0–360, насыщенность `s` и яркость `b` 0–100.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsb {
    /// Оттенок, градусы.
    pub h: f64,
    /// Насыщенность, проценты.
    pub s: f64,
    /// Яркость, проценты.
    pub b: f64,
}

/// Цвет в модели RGB, каналы 0–255.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
    /// Красный.
    pub r: u8,
    /// Зелёный.
    pub g: u8,
    /// Синий.
    pub b: u8,
}

/// Ограничивает HSB допустимыми диапазонами (`validateHSB`).
pub fn validate_hsb(hsb: Hsb) -> Hsb {
    Hsb { h: hsb.h.clamp(0.0, 360.0), s: hsb.s.clamp(0.0, 100.0), b: hsb.b.clamp(0.0, 100.0) }
}

/// Разбирает HEX с `#` или без; всё неразбираемое — чёрный (`HEXtoRGB`).
pub fn hex_to_rgb(hex: &str) -> Rgb {
    let digits = hex.strip_prefix('#').unwrap_or(hex);
    let value = u32::from_str_radix(digits, 16).unwrap_or(0);
    Rgb { r: ((value >> 16) & 0xff) as u8, g: ((value >> 8) & 0xff) as u8, b: (value & 0xff) as u8 }
}

/// RGB в HSB (`RGBtoHSB`).
pub fn rgb_to_hsb(rgb: Rgb) -> Hsb {
    let (r, g, b) = (f64::from(rgb.r), f64::from(rgb.g), f64::from(rgb.b));
    let min = r.min(g).min(b);
    let max = r.max(g).max(b);
    let delta = max - min;
    let mut s = if max != 0.0 { 255.0 * delta / max } else { 0.0 };
    let mut h;
    if s != 0.0 {
        h = if r == max {
            (g - b) / delta
        } else if g == max {
            2.0 + (b - r) / delta
        } else {
            4.0 + (r - g) / delta
        };
    } else {
        h = -1.0;
    }
    h *= 60.0;
    if h < 0.0 {
        h += 360.0;
    }
    s *= 100.0 / 255.0;
    Hsb { h, s, b: max * (100.0 / 255.0) }
}

/// HEX (с `#` или без) в HSB (`HEXtoHSB`).
pub fn hex_to_hsb(hex: &str) -> Hsb {
    rgb_to_hsb(hex_to_rgb(hex))
}

/// HSB в RGB с округлением до целых (`HSBtoRGB`).
pub fn hsb_to_rgb(hsb: Hsb) -> Rgb {
    let mut h = hsb.h;
    let s = hsb.s * 255.0 / 100.0;
    let v = hsb.b * 255.0 / 100.0;
    let (r, g, b);
    if s == 0.0 {
        (r, g, b) = (v, v, v);
    } else {
        let t1 = v;
        let t2 = (255.0 - s) * v / 255.0;
        let t3 = (t1 - t2) * (h % 60.0) / 60.0;
        if h == 360.0 {
            h = 0.0;
        }
        (r, g, b) = if h < 60.0 {
            (t1, t2 + t3, t2)
        } else if h < 120.0 {
            (t1 - t3, t1, t2)
        } else if h < 180.0 {
            (t2, t1, t2 + t3)
        } else if h < 240.0 {
            (t2, t1 - t3, t1)
        } else if h < 300.0 {
            (t2 + t3, t2, t1)
        } else if h < 360.0 {
            (t1, t2, t1 - t3)
        } else {
            (0.0, 0.0, 0.0)
        };
    }
    let channel = |v: f64| v.round().clamp(0.0, 255.0) as u8;
    Rgb { r: channel(r), g: channel(g), b: channel(b) }
}

/// RGB в шесть шестнадцатеричных цифр без `#` (`RGBtoHEX`).
pub fn rgb_to_hex(rgb: Rgb) -> String {
    format!("{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
}

/// HSB в шесть шестнадцатеричных цифр без `#` (`HSBtoHEX`).
pub fn hsb_to_hex(hsb: Hsb) -> String {
    rgb_to_hex(hsb_to_rgb(hsb))
}

/// Насыщенность и яркость по точке `(x, y)` внутри поля (от его левого верхнего угла).
pub fn pick_field(x: f64, y: f64) -> (f64, f64) {
    let s = (100.0 * x.clamp(0.0, AREA) / AREA).floor();
    let b = (100.0 * (AREA - y.clamp(0.0, AREA)) / AREA).floor();
    (s, b)
}

/// Оттенок по вертикальной координате `y` внутри шкалы.
pub fn pick_hue(y: f64) -> f64 {
    (360.0 * (AREA - y.clamp(0.0, AREA)) / AREA).floor()
}

/// Положение указателя поля в пикселях: `(left, top)`.
pub fn field_handle(hsb: Hsb) -> (f64, f64) {
    ((AREA * hsb.s / 100.0).floor(), (AREA * (100.0 - hsb.b) / 100.0).floor())
}

/// Положение указателя шкалы оттенка (`top`, пиксели).
pub fn hue_handle(hsb: Hsb) -> f64 {
    (AREA - AREA * hsb.h / 360.0).floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hsb(h: f64, s: f64, b: f64) -> Hsb {
        Hsb { h, s, b }
    }

    #[test]
    fn hsb_to_hex_table() {
        for (input, expected) in [
            (hsb(0.0, 100.0, 100.0), "ff0000"),
            (hsb(120.0, 100.0, 100.0), "00ff00"),
            (hsb(240.0, 100.0, 100.0), "0000ff"),
            (hsb(60.0, 100.0, 100.0), "ffff00"),
            (hsb(180.0, 100.0, 100.0), "00ffff"),
            (hsb(300.0, 100.0, 100.0), "ff00ff"),
            (hsb(360.0, 100.0, 100.0), "ff0000"),
            (hsb(0.0, 0.0, 100.0), "ffffff"),
            (hsb(0.0, 0.0, 0.0), "000000"),
            (hsb(200.0, 50.0, 50.0), "406a80"),
            (hsb(0.0, 50.0, 100.0), "ff8080"),
        ] {
            assert_eq!(hsb_to_hex(input), expected, "{input:?}");
        }
    }

    #[test]
    fn hex_to_hsb_table() {
        let green = hex_to_hsb("#00ff00");
        assert_eq!((green.h, green.b), (120.0, 100.0));
        assert!((green.s - 100.0).abs() < 1e-9);
        let red = hex_to_hsb("ff0000");
        assert_eq!((red.h, red.b), (0.0, 100.0));
        let white = hex_to_hsb("ffffff");
        // у серых h = -1·60 + 360, как в PrimeNG
        assert_eq!((white.h, white.s), (300.0, 0.0));
        assert!((white.b - 100.0).abs() < 1e-9);
        assert_eq!(hex_to_hsb("000000"), hsb(300.0, 0.0, 0.0));
    }

    #[test]
    fn hex_parsing() {
        assert_eq!(hex_to_rgb("#ff8000"), Rgb { r: 255, g: 128, b: 0 });
        assert_eq!(hex_to_rgb("0000ff"), Rgb { r: 0, g: 0, b: 255 });
        // короткие значения читаются числом, как parseInt
        assert_eq!(hex_to_rgb("ff"), Rgb { r: 0, g: 0, b: 255 });
        for bad in ["", "#", "zzz", "not a color"] {
            assert_eq!(hex_to_rgb(bad), Rgb { r: 0, g: 0, b: 0 }, "{bad:?}");
        }
    }

    #[test]
    fn round_trip_stays_within_one_step() {
        for hex in ["ff0000", "00ff00", "123456", "abcdef", "808080", "fedcba"] {
            assert_eq!(hsb_to_hex(hex_to_hsb(hex)), hex);
        }
    }

    #[test]
    fn validate_clamps() {
        assert_eq!(validate_hsb(hsb(400.0, -5.0, 120.0)), hsb(360.0, 0.0, 100.0));
        assert_eq!(validate_hsb(hsb(-1.0, 50.0, 50.0)), hsb(0.0, 50.0, 50.0));
    }

    #[test]
    fn picking() {
        assert_eq!(pick_field(150.0, 0.0), (100.0, 100.0));
        assert_eq!(pick_field(0.0, 150.0), (0.0, 0.0));
        assert_eq!(pick_field(-30.0, 400.0), (0.0, 0.0));
        assert_eq!(pick_field(75.0, 75.0), (50.0, 50.0));
        assert_eq!(pick_field(999.0, -10.0), (100.0, 100.0));
        assert_eq!(pick_hue(0.0), 360.0);
        assert_eq!(pick_hue(150.0), 0.0);
        assert_eq!(pick_hue(400.0), 0.0);
        assert_eq!(pick_hue(75.0), 180.0);
    }

    #[test]
    fn handles() {
        assert_eq!(field_handle(hsb(0.0, 100.0, 100.0)), (150.0, 0.0));
        assert_eq!(field_handle(hsb(0.0, 0.0, 0.0)), (0.0, 150.0));
        assert_eq!(hue_handle(hsb(0.0, 100.0, 100.0)), 150.0);
        assert_eq!(hue_handle(hsb(360.0, 100.0, 100.0)), 0.0);
        assert_eq!(hue_handle(hsb(180.0, 100.0, 100.0)), 75.0);
    }
}
