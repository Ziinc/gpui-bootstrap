//! Reboot and typography: the page body, headings, `.lead` and muted text.

use gpui_kit::{
    App, Div, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled, Window, div,
    relative,
};

use crate::ActiveTheme as _;

/// A root element with Bootstrap's `body` styles: background, text color,
/// font family, base font size and line height. Wrap a window's content in it.
pub fn body(cx: &App) -> Div {
    let theme = cx.theme();
    div()
        .size_full()
        .bg(theme.colors.body_bg)
        .text_color(theme.colors.body_color)
        .font_family(theme.typography.font_family.clone())
        .text_size(theme.typography.font_size_base)
        .line_height(relative(theme.typography.line_height))
}

/// `<h1>` through `<h6>`, or the `.h1`–`.h6` classes.
#[derive(IntoElement)]
pub struct Heading {
    level: u8,
    text: SharedString,
}

impl Heading {
    /// A heading of `level` `1..=6`; out-of-range levels clamp.
    pub fn new(level: u8, text: impl Into<SharedString>) -> Self {
        Self {
            level: level.clamp(1, 6),
            text: text.into(),
        }
    }

    pub fn h1(text: impl Into<SharedString>) -> Self {
        Self::new(1, text)
    }
    pub fn h2(text: impl Into<SharedString>) -> Self {
        Self::new(2, text)
    }
    pub fn h3(text: impl Into<SharedString>) -> Self {
        Self::new(3, text)
    }
    pub fn h4(text: impl Into<SharedString>) -> Self {
        Self::new(4, text)
    }
    pub fn h5(text: impl Into<SharedString>) -> Self {
        Self::new(5, text)
    }
    pub fn h6(text: impl Into<SharedString>) -> Self {
        Self::new(6, text)
    }
}

impl RenderOnce for Heading {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let typography = &cx.theme().typography;
        div()
            .text_size(typography.headings[self.level as usize - 1])
            .font_weight(typography.headings_font_weight)
            .line_height(relative(1.2))
            .child(self.text)
    }
}

/// `.lead`: a larger, lighter paragraph.
#[derive(IntoElement)]
pub struct Lead {
    text: SharedString,
}

impl Lead {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for Lead {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .text_size(theme.typography.font_size_lg)
            .font_weight(gpui_kit::FontWeight::LIGHT)
            .child(self.text)
    }
}

/// `.text-body-secondary`: de-emphasized text.
#[derive(IntoElement)]
pub struct Muted {
    text: SharedString,
}

impl Muted {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for Muted {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_color(cx.theme().colors.secondary_color)
            .child(self.text)
    }
}
