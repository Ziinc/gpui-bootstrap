//! Bootstrap's design tokens as a GPUI global.
//!
//! [`Theme`] mirrors Bootstrap 5.3's Sass variables and CSS custom properties:
//! the eight theme colors, the body and border colors of each color mode, the
//! spacer, the radius scale and the type scale. Every component reads it at
//! render time, and every derived color (hover and active shades, subtle
//! backgrounds, emphasis text, focus rings) is computed from it the way
//! Bootstrap's Sass computes them. Changing `colors.primary` therefore
//! re-themes every primary button, alert, badge, link, checkbox and focus ring
//! at once.
//!
//! ```ignore
//! use gpui_bootstrap::{Theme, ThemeColors};
//!
//! let mut theme = Theme::light();
//! theme.colors.primary = gpui_kit::rgb(0x6f42c1).into(); // Bootstrap purple
//! theme.radius.base = gpui_kit::px(0.);
//! theme.apply(cx);
//! ```

use gpui_kit::{
    App, BoxShadow, FontWeight, Global, Hsla, Pixels, Rgba, SharedString, point, px, rgb, rgba,
};

/// Bootstrap's two color modes (`data-bs-theme`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorMode {
    #[default]
    Light,
    Dark,
}

/// One of Bootstrap's eight theme colors, the `primary` in `btn-primary`,
/// `alert-primary`, `text-bg-primary` and the rest.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
}

impl Variant {
    /// Every variant, in Bootstrap's `$theme-colors` order.
    pub const ALL: [Variant; 8] = [
        Variant::Primary,
        Variant::Secondary,
        Variant::Success,
        Variant::Info,
        Variant::Warning,
        Variant::Danger,
        Variant::Light,
        Variant::Dark,
    ];

    /// The Bootstrap class suffix, such as `"primary"`.
    pub fn name(self) -> &'static str {
        match self {
            Variant::Primary => "primary",
            Variant::Secondary => "secondary",
            Variant::Success => "success",
            Variant::Info => "info",
            Variant::Warning => "warning",
            Variant::Danger => "danger",
            Variant::Light => "light",
            Variant::Dark => "dark",
        }
    }
}

/// Control sizes, Bootstrap's `-sm` and `-lg` modifiers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

/// Color tokens. Fields are public so a theme reads like Bootstrap's Sass
/// variables: set the ones you want and keep the rest.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeColors {
    pub primary: Hsla,
    pub secondary: Hsla,
    pub success: Hsla,
    pub info: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,
    pub light: Hsla,
    pub dark: Hsla,
    /// `$white`, the light foreground `color-contrast` picks.
    pub white: Hsla,
    /// `$black`, the dark foreground `color-contrast` picks.
    pub black: Hsla,
    /// `--bs-body-bg`.
    pub body_bg: Hsla,
    /// `--bs-body-color`.
    pub body_color: Hsla,
    /// `--bs-emphasis-color`.
    pub emphasis_color: Hsla,
    /// `--bs-secondary-color`, muted text.
    pub secondary_color: Hsla,
    /// `--bs-secondary-bg`, such as a progress track.
    pub secondary_bg: Hsla,
    /// `--bs-tertiary-color`.
    pub tertiary_color: Hsla,
    /// `--bs-tertiary-bg`, such as a hovered list item.
    pub tertiary_bg: Hsla,
    /// `--bs-border-color`.
    pub border_color: Hsla,
    /// `--bs-border-color-translucent`, used by cards, modals and list groups.
    pub border_color_translucent: Hsla,
    /// `--bs-link-color`.
    pub link_color: Hsla,
    /// `--bs-link-hover-color`.
    pub link_hover_color: Hsla,
    /// `$modal-backdrop-bg` at `$modal-backdrop-opacity`.
    pub backdrop: Hsla,
}

impl ThemeColors {
    /// The base color of a variant.
    pub fn variant(&self, variant: Variant) -> Hsla {
        match variant {
            Variant::Primary => self.primary,
            Variant::Secondary => self.secondary,
            Variant::Success => self.success,
            Variant::Info => self.info,
            Variant::Warning => self.warning,
            Variant::Danger => self.danger,
            Variant::Light => self.light,
            Variant::Dark => self.dark,
        }
    }
}

/// The radius scale, `--bs-border-radius-*`.
#[derive(Clone, Debug, PartialEq)]
pub struct Radii {
    pub sm: Pixels,
    pub base: Pixels,
    pub lg: Pixels,
    pub xl: Pixels,
    pub xxl: Pixels,
    pub pill: Pixels,
}

impl Radii {
    /// The radius a control of `size` uses.
    pub fn for_size(&self, size: Size) -> Pixels {
        match size {
            Size::Small => self.sm,
            Size::Medium => self.base,
            Size::Large => self.lg,
        }
    }
}

/// The type scale.
#[derive(Clone, Debug, PartialEq)]
pub struct Typography {
    /// `$font-family-sans-serif`. `.SystemUIFont` is GPUI's platform UI font.
    pub font_family: SharedString,
    /// `$font-family-monospace`.
    pub font_family_mono: SharedString,
    /// `$font-size-base`.
    pub font_size_base: Pixels,
    /// `$font-size-sm`.
    pub font_size_sm: Pixels,
    /// `$font-size-lg`.
    pub font_size_lg: Pixels,
    /// `$line-height-base`, as a multiple of the font size.
    pub line_height: f32,
    pub font_weight_normal: FontWeight,
    pub font_weight_medium: FontWeight,
    pub font_weight_semibold: FontWeight,
    pub font_weight_bold: FontWeight,
    /// `$h1-font-size` through `$h6-font-size`.
    pub headings: [Pixels; 6],
    /// `$headings-font-weight`.
    pub headings_font_weight: FontWeight,
}

impl Typography {
    /// The font size a control of `size` uses.
    pub fn for_size(&self, size: Size) -> Pixels {
        match size {
            Size::Small => self.font_size_sm,
            Size::Medium => self.font_size_base,
            Size::Large => self.font_size_lg,
        }
    }
}

/// Bootstrap's full token set, installed as a GPUI global by [`crate::init`].
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub mode: ColorMode,
    pub colors: ThemeColors,
    pub radius: Radii,
    pub typography: Typography,
    /// `$spacer`; the spacing scale `0..=5` is `0, .25, .5, 1, 1.5, 3` times it.
    pub spacer: Pixels,
    /// `$border-width`.
    pub border_width: Pixels,
    /// `$focus-ring-width`.
    pub focus_ring_width: Pixels,
    /// `$focus-ring-opacity`.
    pub focus_ring_opacity: f32,
    /// `$btn-disabled-opacity`.
    pub disabled_opacity: f32,
    /// `$min-contrast-ratio`, used to pick text on a filled background.
    pub min_contrast_ratio: f32,
}

impl Global for Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

/// Reads the active [`Theme`].
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    #[inline]
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

fn hexa(value: u32) -> Hsla {
    rgba(value).into()
}

impl Theme {
    /// Bootstrap 5.3's default light theme.
    pub fn light() -> Self {
        let primary = hex(0x0d6efd);
        Self {
            mode: ColorMode::Light,
            colors: ThemeColors {
                primary,
                secondary: hex(0x6c757d),
                success: hex(0x198754),
                info: hex(0x0dcaf0),
                warning: hex(0xffc107),
                danger: hex(0xdc3545),
                light: hex(0xf8f9fa),
                dark: hex(0x212529),
                white: hex(0xffffff),
                black: hex(0x000000),
                body_bg: hex(0xffffff),
                body_color: hex(0x212529),
                emphasis_color: hex(0x000000),
                secondary_color: hexa(0x212529bf),
                secondary_bg: hex(0xe9ecef),
                tertiary_color: hexa(0x21252980),
                tertiary_bg: hex(0xf8f9fa),
                border_color: hex(0xdee2e6),
                border_color_translucent: hexa(0x0000002d),
                link_color: primary,
                link_hover_color: shade(primary, 0.2),
                backdrop: hexa(0x00000080),
            },
            radius: Radii {
                sm: px(4.),
                base: px(6.),
                lg: px(8.),
                xl: px(16.),
                xxl: px(32.),
                pill: px(800.),
            },
            typography: Typography {
                font_family: ".SystemUIFont".into(),
                font_family_mono: "monospace".into(),
                font_size_base: px(16.),
                font_size_sm: px(14.),
                font_size_lg: px(20.),
                line_height: 1.5,
                font_weight_normal: FontWeight::NORMAL,
                font_weight_medium: FontWeight::MEDIUM,
                font_weight_semibold: FontWeight::SEMIBOLD,
                font_weight_bold: FontWeight::BOLD,
                headings: [px(40.), px(32.), px(28.), px(24.), px(20.), px(16.)],
                headings_font_weight: FontWeight::MEDIUM,
            },
            spacer: px(16.),
            border_width: px(1.),
            focus_ring_width: px(4.),
            focus_ring_opacity: 0.25,
            disabled_opacity: 0.65,
            min_contrast_ratio: 4.5,
        }
    }

    /// Bootstrap 5.3's dark color mode.
    pub fn dark() -> Self {
        Self::light().with_mode(ColorMode::Dark)
    }

    /// Keeps the theme colors, radii and type scale, and replaces the surface
    /// colors (body, borders, links, backdrop) with the defaults of `mode`.
    pub fn with_mode(mut self, mode: ColorMode) -> Self {
        let primary = self.colors.primary;
        let colors = &mut self.colors;
        match mode {
            ColorMode::Light => {
                colors.body_bg = hex(0xffffff);
                colors.body_color = hex(0x212529);
                colors.emphasis_color = hex(0x000000);
                colors.secondary_color = hexa(0x212529bf);
                colors.secondary_bg = hex(0xe9ecef);
                colors.tertiary_color = hexa(0x21252980);
                colors.tertiary_bg = hex(0xf8f9fa);
                colors.border_color = hex(0xdee2e6);
                colors.border_color_translucent = hexa(0x0000002d);
                colors.link_color = primary;
                colors.link_hover_color = shade(primary, 0.2);
            }
            ColorMode::Dark => {
                colors.body_bg = hex(0x212529);
                colors.body_color = hex(0xdee2e6);
                colors.emphasis_color = hex(0xffffff);
                colors.secondary_color = hexa(0xdee2e6bf);
                colors.secondary_bg = hex(0x343a40);
                colors.tertiary_color = hexa(0xdee2e680);
                colors.tertiary_bg = hex(0x2b3035);
                colors.border_color = hex(0x495057);
                colors.border_color_translucent = hexa(0xffffff26);
                colors.link_color = tint(primary, 0.4);
                colors.link_hover_color = tint(primary, 0.6);
            }
        }
        self.mode = mode;
        self
    }

    /// The installed theme. Panics if [`crate::init`] has not run.
    #[inline]
    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    /// Installs this theme and repaints every window.
    ///
    /// It also maps the tokens onto gpui-kit's own component theme, so the
    /// text editor inside [`crate::form::FormControl`], scrollbars, and any
    /// gpui-kit component you mix in follow the same colors and mode.
    pub fn apply(self, cx: &mut App) {
        use gpui_kit::component::{Theme as KitTheme, ThemeMode};

        let mode = match self.mode {
            ColorMode::Light => ThemeMode::Light,
            ColorMode::Dark => ThemeMode::Dark,
        };
        if KitTheme::global(cx).mode != mode {
            KitTheme::change(mode, None, cx);
        }
        let colors = &self.colors;
        let primary_foreground = self.contrast(colors.primary);
        KitTheme::update(cx, |kit| {
            kit.background = colors.body_bg;
            kit.foreground = colors.body_color;
            kit.muted_foreground = colors.secondary_color;
            kit.border = colors.border_color;
            kit.input = colors.border_color;
            kit.primary = colors.primary;
            kit.primary_foreground = primary_foreground;
            kit.ring = colors.primary;
            kit.caret = colors.body_color;
            kit.selection = colors.primary.opacity(0.25);
            kit.link = colors.link_color;
            kit.danger = colors.danger;
            kit.success = colors.success;
            kit.warning = colors.warning;
            kit.info = colors.info;
            kit.popover = colors.body_bg;
            kit.radius = self.radius.base;
            kit.radius_lg = self.radius.lg;
        });
        cx.set_global(self);
        cx.refresh_windows();
    }

    /// Edits the installed theme in place and re-applies it.
    ///
    /// ```ignore
    /// Theme::update(cx, |theme| theme.colors.primary = rgb(0xd63384).into());
    /// ```
    pub fn update(cx: &mut App, edit: impl FnOnce(&mut Theme)) {
        let mut theme = Theme::global(cx).clone();
        edit(&mut theme);
        theme.apply(cx);
    }

    /// Switches the installed theme's color mode, keeping its theme colors.
    pub fn change_mode(mode: ColorMode, cx: &mut App) {
        let theme = Theme::global(cx).clone().with_mode(mode);
        theme.apply(cx);
    }

    /// Step `level` of the spacing scale (`0..=5`), like `p-3` or `gap-2`.
    pub fn spacing(&self, level: u8) -> Pixels {
        let factor = match level {
            0 => 0.,
            1 => 0.25,
            2 => 0.5,
            3 => 1.,
            4 => 1.5,
            _ => 3.,
        };
        self.spacer * factor
    }

    /// The colors derived from a variant for the current color mode.
    pub fn variant(&self, variant: Variant) -> VariantColors {
        let base = self.colors.variant(variant);
        let (text_emphasis, bg_subtle, border_subtle) = match self.mode {
            ColorMode::Light => (shade(base, 0.6), tint(base, 0.8), tint(base, 0.6)),
            ColorMode::Dark => (tint(base, 0.4), shade(base, 0.8), shade(base, 0.4)),
        };
        // Bootstrap special-cases the neutral ends of the scale.
        let (text_emphasis, bg_subtle, border_subtle) = match (self.mode, variant) {
            (ColorMode::Light, Variant::Light) => (
                self.colors.body_color.opacity(0.75),
                mix(base, self.colors.body_bg, 0.5),
                self.colors.border_color,
            ),
            (ColorMode::Light, Variant::Dark) => {
                (base, self.colors.secondary_bg, self.colors.border_color)
            }
            (ColorMode::Dark, Variant::Light) => (
                self.colors.body_color,
                self.colors.secondary_bg,
                self.colors.border_color,
            ),
            (ColorMode::Dark, Variant::Dark) => (
                self.colors.body_color,
                self.colors.tertiary_bg,
                self.colors.border_color,
            ),
            _ => (text_emphasis, bg_subtle, border_subtle),
        };
        VariantColors {
            base,
            contrast: self.contrast(base),
            text_emphasis,
            bg_subtle,
            border_subtle,
        }
    }

    /// Bootstrap's `color-contrast()`: white when it reaches
    /// [`min_contrast_ratio`](Self::min_contrast_ratio) on `background`,
    /// otherwise whichever of white and black contrasts more.
    pub fn contrast(&self, background: Hsla) -> Hsla {
        let (white, black) = (self.colors.white, self.colors.black);
        let on_white = contrast_ratio(background, white);
        if on_white >= self.min_contrast_ratio {
            return white;
        }
        let on_black = contrast_ratio(background, black);
        if on_black >= self.min_contrast_ratio || on_black > on_white {
            black
        } else {
            white
        }
    }

    /// The colors of a solid `btn-<variant>` button in each interaction state.
    pub fn button(&self, variant: Variant) -> ButtonColors {
        let base = self.colors.variant(variant);
        let color = self.contrast(base);
        // `button-variant` shades on light text and tints on dark text; the
        // `light` and `dark` buttons are fixed to shade and tint.
        let shades = match variant {
            Variant::Light => true,
            Variant::Dark => false,
            _ => color == self.colors.white,
        };
        let (hover_bg, hover_border, active_bg, active_border) = if shades {
            (
                shade(base, 0.15),
                shade(base, 0.2),
                shade(base, 0.2),
                shade(base, 0.25),
            )
        } else {
            (
                tint(base, 0.15),
                tint(base, 0.1),
                tint(base, 0.2),
                tint(base, 0.1),
            )
        };
        ButtonColors {
            bg: base,
            border: base,
            color,
            hover_bg,
            hover_border,
            hover_color: self.contrast(hover_bg),
            active_bg,
            active_border,
            active_color: self.contrast(active_bg),
            focus_ring: mix(color, base, 0.15).opacity(0.5),
        }
    }

    /// The colors of a `btn-outline-<variant>` button in each interaction state.
    pub fn outline_button(&self, variant: Variant) -> ButtonColors {
        let base = self.colors.variant(variant);
        let contrast = self.contrast(base);
        ButtonColors {
            bg: gpui_kit::transparent_black(),
            border: base,
            color: base,
            hover_bg: base,
            hover_border: base,
            hover_color: contrast,
            active_bg: base,
            active_border: base,
            active_color: contrast,
            focus_ring: base.opacity(0.5),
        }
    }

    /// The `$input-btn-focus-box-shadow` ring around a focused control.
    pub fn focus_ring(&self, color: Hsla) -> BoxShadow {
        BoxShadow {
            color,
            offset: point(px(0.), px(0.)),
            blur_radius: px(0.),
            spread_radius: self.focus_ring_width,
            inset: false,
        }
    }

    /// The default focus ring color, `$focus-ring-color`.
    pub fn focus_ring_color(&self) -> Hsla {
        self.colors.primary.opacity(self.focus_ring_opacity)
    }

    /// `$input-focus-border-color`.
    pub fn input_focus_border_color(&self) -> Hsla {
        tint(self.colors.primary, 0.5)
    }

    /// The `box-shadow` utility, `$box-shadow`.
    pub fn shadow(&self) -> BoxShadow {
        BoxShadow {
            color: hexa(0x00000026),
            offset: point(px(0.), px(8.)),
            blur_radius: px(16.),
            spread_radius: px(0.),
            inset: false,
        }
    }

    /// `$box-shadow-sm`.
    pub fn shadow_sm(&self) -> BoxShadow {
        BoxShadow {
            color: hexa(0x00000013),
            offset: point(px(0.), px(2.)),
            blur_radius: px(4.),
            spread_radius: px(0.),
            inset: false,
        }
    }
}

/// The colors derived from one [`Variant`], the `--bs-<variant>-*` custom
/// properties.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariantColors {
    /// `--bs-<variant>`.
    pub base: Hsla,
    /// Text that reads on `base`, as in `text-bg-<variant>`.
    pub contrast: Hsla,
    /// `--bs-<variant>-text-emphasis`.
    pub text_emphasis: Hsla,
    /// `--bs-<variant>-bg-subtle`.
    pub bg_subtle: Hsla,
    /// `--bs-<variant>-border-subtle`.
    pub border_subtle: Hsla,
}

/// The `--bs-btn-*` colors of one button style.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonColors {
    pub bg: Hsla,
    pub border: Hsla,
    pub color: Hsla,
    pub hover_bg: Hsla,
    pub hover_border: Hsla,
    pub hover_color: Hsla,
    pub active_bg: Hsla,
    pub active_border: Hsla,
    pub active_color: Hsla,
    pub focus_ring: Hsla,
}

/// Sass's `mix($a, $b, $weight)`: `weight` of `a`, the rest of `b`, in sRGB.
pub fn mix(a: Hsla, b: Hsla, weight: f32) -> Hsla {
    let (a, b): (Rgba, Rgba) = (a.into(), b.into());
    let w = weight.clamp(0., 1.);
    Rgba {
        r: a.r * w + b.r * (1. - w),
        g: a.g * w + b.g * (1. - w),
        b: a.b * w + b.b * (1. - w),
        a: a.a * w + b.a * (1. - w),
    }
    .into()
}

/// Bootstrap's `tint-color()`: mixes `amount` of white into `color`.
pub fn tint(color: Hsla, amount: f32) -> Hsla {
    mix(hex(0xffffff), color, amount)
}

/// Bootstrap's `shade-color()`: mixes `amount` of black into `color`.
pub fn shade(color: Hsla, amount: f32) -> Hsla {
    mix(hex(0x000000), color, amount)
}

/// WCAG relative luminance.
fn luminance(color: Hsla) -> f32 {
    let color: Rgba = color.into();
    let channel = |c: f32| {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(color.r) + 0.7152 * channel(color.g) + 0.0722 * channel(color.b)
}

/// WCAG contrast ratio between two colors.
pub fn contrast_ratio(a: Hsla, b: Hsla) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    let (light, dark) = if la > lb { (la, lb) } else { (lb, la) };
    (light + 0.05) / (dark + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Hsla, b: u32) -> bool {
        let (a, b): (Rgba, Rgba) = (a.into(), rgb(b));
        [(a.r, b.r), (a.g, b.g), (a.b, b.b)]
            .iter()
            .all(|(x, y)| (x - y).abs() < 1.5 / 255.)
    }

    #[test]
    fn contrast_matches_bootstraps_text_bg_choices() {
        let theme = Theme::light();
        let white = theme.colors.white;
        let black = theme.colors.black;
        for (variant, expected) in [
            (Variant::Primary, white),
            (Variant::Secondary, white),
            (Variant::Success, white),
            (Variant::Info, black),
            (Variant::Warning, black),
            (Variant::Danger, white),
            (Variant::Light, black),
            (Variant::Dark, white),
        ] {
            assert_eq!(theme.variant(variant).contrast, expected, "{variant:?}");
        }
    }

    #[test]
    fn derived_colors_match_bootstraps_compiled_css() {
        let theme = Theme::light();
        let primary = theme.variant(Variant::Primary);
        assert!(close(primary.text_emphasis, 0x052c65));
        assert!(close(primary.bg_subtle, 0xcfe2ff));
        assert!(close(primary.border_subtle, 0x9ec5fe));
        let button = theme.button(Variant::Primary);
        assert!(close(button.hover_bg, 0x0b5ed7));
        assert!(close(button.active_bg, 0x0a58ca));
        let warning = theme.button(Variant::Warning);
        assert!(close(warning.hover_bg, 0xffca2c));
        let dark = Theme::dark();
        assert!(close(
            dark.variant(Variant::Primary).text_emphasis,
            0x6ea8fe
        ));
        assert!(close(dark.variant(Variant::Primary).bg_subtle, 0x031633));
    }

    #[test]
    fn overriding_a_theme_color_rederives_its_states() {
        let mut theme = Theme::light();
        theme.colors.primary = hex(0x6f42c1);
        assert!(close(theme.button(Variant::Primary).hover_bg, 0x5e38a4));
        assert!(close(theme.variant(Variant::Primary).bg_subtle, 0xe2d9f3));
    }

    #[test]
    fn with_mode_keeps_theme_colors() {
        let mut theme = Theme::light();
        theme.colors.danger = hex(0xff0000);
        let dark = theme.with_mode(ColorMode::Dark);
        assert_eq!(dark.mode, ColorMode::Dark);
        assert_eq!(dark.colors.danger, hex(0xff0000));
        assert_eq!(dark.colors.body_bg, hex(0x212529));
    }
}
