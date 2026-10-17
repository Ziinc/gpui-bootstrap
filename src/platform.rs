//! Built-in themes that match the platforms GPUI targets.
//!
//! Each [`Platform`] keeps Bootstrap's components and token structure but
//! swaps in that platform's system palette, type scale, corner radii and
//! control sizes, in light and dark:
//!
//! | Platform | Source |
//! | --- | --- |
//! | [`Platform::Web`] | Bootstrap 5.3 as shipped |
//! | [`Platform::MacOS`] | Apple Human Interface Guidelines, macOS system colors |
//! | [`Platform::Windows`] | Fluent 2 / WinUI 3 |
//! | [`Platform::Linux`] | GNOME libadwaita |
//! | [`Platform::IOS`] | Apple Human Interface Guidelines, iOS system colors, 44pt touch targets |
//! | [`Platform::Android`] | Material Design 3 baseline scheme, 40dp buttons |
//!
//! ```ignore
//! // The look of whatever OS the app runs on, following its appearance.
//! Theme::native(ColorMode::from(window.appearance())).apply(cx);
//! // Or a specific one, overridden like any other theme.
//! let mut theme = Theme::platform(Platform::Android, ColorMode::Dark);
//! theme.colors.primary = rgb(0x006a6a).into();
//! theme.apply(cx);
//! ```

use gpui_kit::{FontWeight, Hsla, px, rgb, rgba};

use crate::theme::{
    ColorMode, ControlSize, ControlSizes, Radii, Theme, ThemeColors, Typography, shade, tint,
};

/// A platform look. See the [module docs](self) for what each follows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Platform {
    /// Stock Bootstrap 5.3, the default.
    #[default]
    Web,
    MacOS,
    Windows,
    /// GNOME's libadwaita, the most common Linux desktop look.
    Linux,
    IOS,
    Android,
}

impl Platform {
    /// Every platform, in declaration order.
    pub const ALL: [Platform; 6] = [
        Platform::Web,
        Platform::MacOS,
        Platform::Windows,
        Platform::Linux,
        Platform::IOS,
        Platform::Android,
    ];

    /// The platform this binary was compiled for.
    pub fn current() -> Self {
        if cfg!(target_family = "wasm") {
            Platform::Web
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "ios") {
            Platform::IOS
        } else if cfg!(target_os = "android") {
            Platform::Android
        } else {
            Platform::Linux
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Platform::Web => "Web",
            Platform::MacOS => "macOS",
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
            Platform::IOS => "iOS",
            Platform::Android => "Android",
        }
    }

    /// Whether this is a touch-first platform with larger hit targets.
    pub fn is_mobile(self) -> bool {
        matches!(self, Platform::IOS | Platform::Android)
    }
}

impl From<gpui_kit::WindowAppearance> for ColorMode {
    fn from(appearance: gpui_kit::WindowAppearance) -> Self {
        match appearance {
            gpui_kit::WindowAppearance::Dark | gpui_kit::WindowAppearance::VibrantDark => {
                ColorMode::Dark
            }
            _ => ColorMode::Light,
        }
    }
}

fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

fn hexa(value: u32) -> Hsla {
    rgba(value).into()
}

/// The mode-dependent colors of a platform.
#[derive(Clone, Copy)]
pub(crate) struct Palette {
    pub primary: Hsla,
    pub secondary: Hsla,
    pub success: Hsla,
    pub info: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,
    pub light: Hsla,
    pub dark: Hsla,
    pub body_bg: Hsla,
    pub body_color: Hsla,
    pub emphasis_color: Hsla,
    pub secondary_color: Hsla,
    pub secondary_bg: Hsla,
    pub tertiary_color: Hsla,
    pub tertiary_bg: Hsla,
    pub border_color: Hsla,
    pub border_color_translucent: Hsla,
    pub backdrop: Hsla,
}

impl Palette {
    /// Theme colors in [`ThemeColors`] field order, for remapping on a mode change.
    pub fn theme_colors(&self) -> [Hsla; 8] {
        [
            self.primary,
            self.secondary,
            self.success,
            self.info,
            self.warning,
            self.danger,
            self.light,
            self.dark,
        ]
    }
}

/// Applies `palette`'s surface colors, and the link colors derived from the
/// theme's current primary, to `colors`.
pub(crate) fn apply_surfaces(
    colors: &mut ThemeColors,
    palette: &Palette,
    platform: Platform,
    mode: ColorMode,
) {
    colors.body_bg = palette.body_bg;
    colors.body_color = palette.body_color;
    colors.emphasis_color = palette.emphasis_color;
    colors.secondary_color = palette.secondary_color;
    colors.secondary_bg = palette.secondary_bg;
    colors.tertiary_color = palette.tertiary_color;
    colors.tertiary_bg = palette.tertiary_bg;
    colors.border_color = palette.border_color;
    colors.border_color_translucent = palette.border_color_translucent;
    colors.backdrop = palette.backdrop;
    let primary = colors.primary;
    (colors.link_color, colors.link_hover_color) = match (platform, mode) {
        (_, ColorMode::Light) => (primary, shade(primary, 0.2)),
        // Bootstrap lightens its mid-tone primary for dark links; the native
        // palettes already ship a lighter dark-mode accent.
        (Platform::Web, ColorMode::Dark) => (tint(primary, 0.4), tint(primary, 0.6)),
        (_, ColorMode::Dark) => (primary, tint(primary, 0.2)),
    };
}

pub(crate) fn palette(platform: Platform, mode: ColorMode) -> Palette {
    use ColorMode::{Dark, Light};
    match (platform, mode) {
        (Platform::Web, Light) => Palette {
            primary: hex(0x0d6efd),
            secondary: hex(0x6c757d),
            success: hex(0x198754),
            info: hex(0x0dcaf0),
            warning: hex(0xffc107),
            danger: hex(0xdc3545),
            light: hex(0xf8f9fa),
            dark: hex(0x212529),
            body_bg: hex(0xffffff),
            body_color: hex(0x212529),
            emphasis_color: hex(0x000000),
            secondary_color: hexa(0x212529bf),
            secondary_bg: hex(0xe9ecef),
            tertiary_color: hexa(0x21252980),
            tertiary_bg: hex(0xf8f9fa),
            border_color: hex(0xdee2e6),
            border_color_translucent: hexa(0x0000002d),
            backdrop: hexa(0x00000080),
        },
        (Platform::Web, Dark) => Palette {
            body_bg: hex(0x212529),
            body_color: hex(0xdee2e6),
            emphasis_color: hex(0xffffff),
            secondary_color: hexa(0xdee2e6bf),
            secondary_bg: hex(0x343a40),
            tertiary_color: hexa(0xdee2e680),
            tertiary_bg: hex(0x2b3035),
            border_color: hex(0x495057),
            border_color_translucent: hexa(0xffffff26),
            ..palette(Platform::Web, Light)
        },
        // macOS system colors (NSColor.system*), light and dark variants.
        (Platform::MacOS, Light) => Palette {
            primary: hex(0x007aff),
            secondary: hex(0x8e8e93),
            success: hex(0x28cd41),
            info: hex(0x55bef0),
            warning: hex(0xffcc00),
            danger: hex(0xff3b30),
            light: hex(0xf2f2f7),
            dark: hex(0x1c1c1e),
            body_bg: hex(0xffffff),
            body_color: hexa(0x000000d9),
            emphasis_color: hex(0x000000),
            secondary_color: hexa(0x00000080),
            secondary_bg: hex(0xe5e5ea),
            tertiary_color: hexa(0x00000040),
            tertiary_bg: hex(0xf5f5f7),
            border_color: hex(0xd1d1d6),
            border_color_translucent: hexa(0x0000001a),
            backdrop: hexa(0x00000033),
        },
        (Platform::MacOS, Dark) => Palette {
            primary: hex(0x0a84ff),
            secondary: hex(0x98989d),
            success: hex(0x32d74b),
            info: hex(0x5ac8f5),
            warning: hex(0xffd60a),
            danger: hex(0xff453a),
            light: hex(0x3a3a3c),
            dark: hex(0xe5e5ea),
            body_bg: hex(0x1e1e1e),
            body_color: hexa(0xffffffd9),
            emphasis_color: hex(0xffffff),
            secondary_color: hexa(0xffffff8c),
            secondary_bg: hex(0x3a3a3c),
            tertiary_color: hexa(0xffffff40),
            tertiary_bg: hex(0x2c2c2e),
            border_color: hex(0x48484a),
            border_color_translucent: hexa(0xffffff1a),
            backdrop: hexa(0x00000066),
        },
        // Fluent 2 / WinUI 3 theme resources.
        (Platform::Windows, Light) => Palette {
            primary: hex(0x005fb8),
            secondary: hex(0x616161),
            success: hex(0x0f7b0f),
            info: hex(0x0078d4),
            warning: hex(0xf7b500),
            danger: hex(0xc42b1c),
            light: hex(0xf3f3f3),
            dark: hex(0x1a1a1a),
            body_bg: hex(0xf9f9f9),
            body_color: hexa(0x000000e4),
            emphasis_color: hex(0x000000),
            secondary_color: hexa(0x0000009e),
            secondary_bg: hex(0xebebeb),
            tertiary_color: hexa(0x00000072),
            tertiary_bg: hex(0xf3f3f3),
            border_color: hex(0xe5e5e5),
            border_color_translucent: hexa(0x0000000f),
            backdrop: hexa(0x0000004d),
        },
        (Platform::Windows, Dark) => Palette {
            primary: hex(0x60cdff),
            secondary: hex(0x9e9e9e),
            success: hex(0x6ccb5f),
            info: hex(0x4cc2ff),
            warning: hex(0xfce100),
            danger: hex(0xff99a4),
            light: hex(0x3d3d3d),
            dark: hex(0xf3f3f3),
            body_bg: hex(0x202020),
            body_color: hex(0xffffff),
            emphasis_color: hex(0xffffff),
            secondary_color: hexa(0xffffffc8),
            secondary_bg: hex(0x2d2d2d),
            tertiary_color: hexa(0xffffff8b),
            tertiary_bg: hex(0x282828),
            border_color: hex(0x3a3a3a),
            border_color_translucent: hexa(0xffffff15),
            backdrop: hexa(0x0000004d),
        },
        // libadwaita named colors.
        (Platform::Linux, Light) => Palette {
            primary: hex(0x3584e4),
            secondary: hex(0x77767b),
            success: hex(0x2ec27e),
            info: hex(0x2190a4),
            warning: hex(0xe5a50a),
            danger: hex(0xe01b24),
            light: hex(0xf6f5f4),
            dark: hex(0x241f31),
            body_bg: hex(0xfafafb),
            body_color: hexa(0x000006cc),
            emphasis_color: hex(0x000000),
            secondary_color: hexa(0x0000068c),
            secondary_bg: hex(0xebebed),
            tertiary_color: hexa(0x00000659),
            tertiary_bg: hex(0xf4f4f6),
            border_color: hex(0xdedee0),
            border_color_translucent: hexa(0x00000626),
            backdrop: hexa(0x00000040),
        },
        (Platform::Linux, Dark) => Palette {
            primary: hex(0x3584e4),
            secondary: hex(0x9a9996),
            success: hex(0x26a269),
            info: hex(0x2190a4),
            warning: hex(0xcd9309),
            danger: hex(0xc01c28),
            light: hex(0x3a3a3e),
            dark: hex(0xf6f5f4),
            body_bg: hex(0x222226),
            body_color: hex(0xffffff),
            emphasis_color: hex(0xffffff),
            secondary_color: hexa(0xffffff8c),
            secondary_bg: hex(0x2e2e32),
            tertiary_color: hexa(0xffffff59),
            tertiary_bg: hex(0x29292d),
            border_color: hex(0x3a3a3e),
            border_color_translucent: hexa(0xffffff26),
            backdrop: hexa(0x00000066),
        },
        // UIKit system colors.
        (Platform::IOS, Light) => Palette {
            primary: hex(0x007aff),
            secondary: hex(0x8e8e93),
            success: hex(0x34c759),
            info: hex(0x32ade6),
            warning: hex(0xffcc00),
            danger: hex(0xff3b30),
            light: hex(0xf2f2f7),
            dark: hex(0x1c1c1e),
            body_bg: hex(0xffffff),
            body_color: hex(0x000000),
            emphasis_color: hex(0x000000),
            secondary_color: hexa(0x3c3c4399),
            secondary_bg: hex(0xe5e5ea),
            tertiary_color: hexa(0x3c3c434d),
            tertiary_bg: hex(0xf2f2f7),
            border_color: hex(0xc6c6c8),
            border_color_translucent: hexa(0x3c3c434a),
            backdrop: hexa(0x00000066),
        },
        (Platform::IOS, Dark) => Palette {
            primary: hex(0x0a84ff),
            secondary: hex(0x8e8e93),
            success: hex(0x30d158),
            info: hex(0x64d2ff),
            warning: hex(0xffd60a),
            danger: hex(0xff453a),
            light: hex(0x2c2c2e),
            dark: hex(0xf2f2f7),
            body_bg: hex(0x000000),
            body_color: hex(0xffffff),
            emphasis_color: hex(0xffffff),
            secondary_color: hexa(0xebebf599),
            secondary_bg: hex(0x2c2c2e),
            tertiary_color: hexa(0xebebf54d),
            tertiary_bg: hex(0x1c1c1e),
            border_color: hex(0x38383a),
            border_color_translucent: hexa(0x54545899),
            backdrop: hexa(0x00000099),
        },
        // Material 3 baseline scheme (primary, error, surfaces, outlines).
        (Platform::Android, Light) => Palette {
            primary: hex(0x6750a4),
            secondary: hex(0x625b71),
            success: hex(0x386a20),
            info: hex(0x0061a4),
            warning: hex(0xffb300),
            danger: hex(0xb3261e),
            light: hex(0xf3edf7),
            dark: hex(0x322f35),
            body_bg: hex(0xfef7ff),
            body_color: hex(0x1d1b20),
            emphasis_color: hex(0x1d1b20),
            secondary_color: hex(0x49454f),
            secondary_bg: hex(0xe6e0e9),
            tertiary_color: hex(0x79747e),
            tertiary_bg: hex(0xf3edf7),
            border_color: hex(0xcac4d0),
            border_color_translucent: hexa(0x79747e40),
            backdrop: hexa(0x00000052),
        },
        (Platform::Android, Dark) => Palette {
            primary: hex(0xd0bcff),
            secondary: hex(0xccc2dc),
            success: hex(0x9cd67d),
            info: hex(0x9ecaff),
            warning: hex(0xffca28),
            danger: hex(0xf2b8b5),
            light: hex(0x36343b),
            dark: hex(0xe6e0e9),
            body_bg: hex(0x141218),
            body_color: hex(0xe6e0e9),
            emphasis_color: hex(0xe6e0e9),
            secondary_color: hex(0xcac4d0),
            secondary_bg: hex(0x36343b),
            tertiary_color: hex(0x938f99),
            tertiary_bg: hex(0x211f26),
            border_color: hex(0x49454f),
            border_color_translucent: hexa(0x938f9940),
            backdrop: hexa(0x00000052),
        },
    }
}

fn controls(
    small: (f32, f32, f32),
    medium: (f32, f32, f32),
    large: (f32, f32, f32),
) -> ControlSizes {
    let size = |(x, y, min_height): (f32, f32, f32)| ControlSize {
        padding_x: px(x),
        padding_y: px(y),
        min_height: px(min_height),
    };
    ControlSizes {
        small: size(small),
        medium: size(medium),
        large: size(large),
        button_radius: None,
    }
}

fn radii(sm: f32, base: f32, lg: f32) -> Radii {
    Radii {
        sm: px(sm),
        base: px(base),
        lg: px(lg),
        xl: px(lg * 2.),
        xxl: px(lg * 4.),
        pill: px(800.),
    }
}

fn typography(base: f32, sm: f32, lg: f32, line_height: f32, headings: [f32; 6]) -> Typography {
    Typography {
        font_family: ".SystemUIFont".into(),
        font_family_mono: "monospace".into(),
        font_size_base: px(base),
        font_size_sm: px(sm),
        font_size_lg: px(lg),
        line_height,
        font_weight_normal: FontWeight::NORMAL,
        font_weight_medium: FontWeight::MEDIUM,
        font_weight_semibold: FontWeight::SEMIBOLD,
        font_weight_bold: FontWeight::BOLD,
        headings: headings.map(px),
        headings_font_weight: FontWeight::MEDIUM,
    }
}

impl Theme {
    /// The built-in theme for `platform` in `mode`.
    pub fn platform(platform: Platform, mode: ColorMode) -> Self {
        let colors = palette(platform, mode);
        let mut theme = Theme {
            platform,
            mode,
            colors: ThemeColors {
                primary: colors.primary,
                secondary: colors.secondary,
                success: colors.success,
                info: colors.info,
                warning: colors.warning,
                danger: colors.danger,
                light: colors.light,
                dark: colors.dark,
                white: hex(0xffffff),
                black: hex(0x000000),
                // Filled in by `apply_surfaces` below.
                body_bg: colors.body_bg,
                body_color: colors.body_color,
                emphasis_color: colors.emphasis_color,
                secondary_color: colors.secondary_color,
                secondary_bg: colors.secondary_bg,
                tertiary_color: colors.tertiary_color,
                tertiary_bg: colors.tertiary_bg,
                border_color: colors.border_color,
                border_color_translucent: colors.border_color_translucent,
                link_color: colors.primary,
                link_hover_color: colors.primary,
                backdrop: colors.backdrop,
            },
            radius: radii(4., 6., 8.),
            typography: typography(16., 14., 20., 1.5, [40., 32., 28., 24., 20., 16.]),
            controls: controls((8., 4., 0.), (12., 6., 0.), (16., 8., 0.)),
            spacer: px(16.),
            border_width: px(1.),
            focus_ring_width: px(4.),
            focus_ring_opacity: 0.25,
            disabled_opacity: 0.65,
            min_contrast_ratio: 4.5,
        };
        apply_surfaces(&mut theme.colors, &colors, platform, mode);

        match platform {
            Platform::Web => {}
            Platform::MacOS => {
                // Native filled controls put white text on the accent at 3:1.
                theme.min_contrast_ratio = 3.;
                // 13pt body, 22pt regular push buttons, 3pt accent focus ring.
                theme.radius = radii(4., 6., 10.);
                theme.typography = typography(13., 11., 15., 1.3, [26., 22., 17., 15., 13., 13.]);
                theme.typography.headings_font_weight = FontWeight::BOLD;
                theme.controls = controls((7., 2., 18.), (10., 3., 22.), (14., 6., 30.));
                theme.spacer = px(12.);
                theme.focus_ring_width = px(3.);
                theme.focus_ring_opacity = 0.5;
                theme.disabled_opacity = 0.4;
            }
            Platform::Windows => {
                // 14px body, 32px controls, 4px control and 8px overlay corners.
                theme.radius = radii(4., 4., 8.);
                theme.typography = typography(14., 12., 18., 1.43, [40., 28., 20., 18., 14., 14.]);
                theme.typography.headings_font_weight = FontWeight::SEMIBOLD;
                theme.controls = controls((8., 3., 24.), (11., 5., 32.), (14., 8., 40.));
                theme.spacer = px(12.);
                theme.focus_ring_width = px(2.);
                theme.focus_ring_opacity = 1.;
                theme.disabled_opacity = 0.4;
            }
            Platform::Linux => {
                // Native filled controls put white text on the accent at 3:1.
                theme.min_contrast_ratio = 3.;
                // 11pt Adwaita Sans, 34px buttons, 6px buttons and 12px cards.
                theme.radius = radii(6., 6., 12.);
                theme.typography = typography(15., 13., 18., 1.3, [30., 26., 20., 18., 15., 15.]);
                theme.typography.headings_font_weight = FontWeight::BOLD;
                theme.controls = controls((8., 3., 24.), (10., 6., 34.), (16., 10., 42.));
                theme.spacer = px(12.);
                theme.focus_ring_width = px(2.);
                theme.focus_ring_opacity = 0.5;
                theme.disabled_opacity = 0.5;
            }
            Platform::IOS => {
                // Native filled controls put white text on the accent at 3:1.
                theme.min_contrast_ratio = 3.;
                // 17pt body, 44pt minimum touch targets, continuous-ish corners.
                theme.radius = radii(8., 10., 14.);
                theme.typography = typography(17., 15., 20., 1.3, [34., 28., 22., 20., 17., 17.]);
                theme.typography.headings_font_weight = FontWeight::BOLD;
                theme.controls = controls((12., 6., 34.), (16., 11., 44.), (20., 14., 50.));
                theme.spacer = px(16.);
                theme.focus_ring_width = px(3.);
                theme.focus_ring_opacity = 0.5;
                theme.disabled_opacity = 0.35;
            }
            Platform::Android => {
                // 16sp body, 40dp pill buttons, 12dp cards, 28dp dialogs.
                theme.radius = radii(4., 12., 28.);
                theme.typography = typography(16., 14., 22., 1.5, [32., 28., 24., 22., 16., 14.]);
                theme.typography.headings_font_weight = FontWeight::NORMAL;
                theme.controls = controls((16., 6., 32.), (24., 8., 40.), (24., 12., 56.));
                theme.controls.button_radius = Some(px(800.));
                theme.spacer = px(16.);
                theme.focus_ring_width = px(3.);
                theme.focus_ring_opacity = 0.4;
                theme.disabled_opacity = 0.38;
            }
        }
        theme
    }

    /// The built-in theme of [`Platform::current`].
    pub fn native(mode: ColorMode) -> Self {
        Self::platform(Platform::current(), mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    #[test]
    fn every_platform_keeps_text_readable_on_its_fills() {
        for platform in Platform::ALL {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let theme = Theme::platform(platform, mode);
                for variant in Variant::ALL {
                    let base = theme.colors.variant(variant);
                    let text = theme.contrast(base);
                    assert!(
                        crate::theme::contrast_ratio(base, text) >= 1.9,
                        "{platform:?} {mode:?} {variant:?}"
                    );
                }
                assert!(
                    crate::theme::contrast_ratio(theme.colors.body_bg, theme.colors.body_color)
                        >= 4.5,
                    "{platform:?} {mode:?} body text"
                );
            }
        }
    }

    #[test]
    fn web_platform_is_stock_bootstrap() {
        assert_eq!(
            Theme::platform(Platform::Web, ColorMode::Light),
            Theme::light()
        );
        assert_eq!(
            Theme::platform(Platform::Web, ColorMode::Dark),
            Theme::dark()
        );
    }

    #[test]
    fn mode_switch_follows_the_platform_palette_but_keeps_overrides() {
        let mut theme = Theme::platform(Platform::MacOS, ColorMode::Light);
        theme.colors.danger = hex(0x123456);
        let dark = theme.with_mode(ColorMode::Dark);
        assert_eq!(dark, {
            let mut expected = Theme::platform(Platform::MacOS, ColorMode::Dark);
            expected.colors.danger = hex(0x123456);
            expected
        });
    }

    #[test]
    fn mobile_platforms_meet_touch_target_minimums() {
        assert!(
            Theme::platform(Platform::IOS, ColorMode::Light)
                .controls
                .medium
                .min_height
                >= px(44.)
        );
        assert!(
            Theme::platform(Platform::Android, ColorMode::Light)
                .controls
                .medium
                .min_height
                >= px(40.)
        );
        assert!(Platform::IOS.is_mobile() && !Platform::MacOS.is_mobile());
    }
}
