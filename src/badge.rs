//! `.badge`.

use gpui_kit::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, base::StyledExt as _, div, prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme as _, Variant};

/// A small count or label, `badge text-bg-<variant>`.
#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    variant: Variant,
    pill: bool,
    subtle: bool,
    style: StyleRefinement,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Primary,
            pill: false,
            subtle: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// `.rounded-pill`.
    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }

    /// The subtle palette: `bg-<variant>-subtle text-<variant>-emphasis`
    /// with a `border-<variant>-subtle` border.
    pub fn subtle(mut self, subtle: bool) -> Self {
        self.subtle = subtle;
        self
    }
}

impl Styled for Badge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Badge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.variant(self.variant);
        // `$badge-font-size: .75em`, `$badge-padding-y: .35em`, `$badge-padding-x: .65em`.
        let font_size = theme.typography.font_size_base * 0.75;
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .px(font_size * 0.65)
            .py(font_size * 0.35)
            .text_size(font_size)
            .font_weight(theme.typography.font_weight_bold)
            .line_height(px(font_size.as_f32()))
            .rounded(if self.pill {
                theme.radius.pill
            } else {
                theme.radius.base
            })
            .map(|this| {
                if self.subtle {
                    this.bg(colors.bg_subtle)
                        .text_color(colors.text_emphasis)
                        .border(theme.border_width)
                        .border_color(colors.border_subtle)
                } else {
                    this.bg(colors.base).text_color(colors.contrast)
                }
            })
            .child(self.label)
            .refine_style(&self.style)
    }
}
