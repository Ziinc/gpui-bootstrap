//! `.card`.

use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window,
    base::{StyledExt as _, v_flex},
    div,
    prelude::FluentBuilder as _,
    relative,
};

use crate::{ActiveTheme as _, Variant};

/// A bordered content container with an optional header and footer.
///
/// Children go in `.card-body`, after the title and subtitle.
#[derive(IntoElement)]
pub struct Card {
    header: Option<AnyElement>,
    title: Option<SharedString>,
    subtitle: Option<SharedString>,
    footer: Option<AnyElement>,
    border_variant: Option<Variant>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Card {
    pub fn new() -> Self {
        Self {
            header: None,
            title: None,
            subtitle: None,
            footer: None,
            border_variant: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    /// `.card-header`.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// `.card-title`.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// `.card-subtitle`.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// `.card-footer`.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// `.border-<variant>`.
    pub fn border_variant(mut self, variant: Variant) -> Self {
        self.border_variant = Some(variant);
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let border = self
            .border_variant
            .map_or(theme.colors.border_color_translucent, |variant| {
                theme.colors.variant(variant)
            });
        // `$card-cap-bg: rgba(var(--bs-body-color-rgb), .03)`.
        let cap_bg = theme.colors.body_color.opacity(0.03);
        let cap = |content: AnyElement| {
            div()
                .px(theme.spacer)
                .py(theme.spacing(2))
                .bg(cap_bg)
                .border_color(border)
                .child(content)
        };
        let has_body = self.title.is_some() || self.subtitle.is_some() || !self.children.is_empty();
        v_flex()
            .min_w_0()
            .bg(theme.colors.body_bg)
            .border(theme.border_width)
            .border_color(border)
            .rounded(theme.radius.base)
            .overflow_hidden()
            .when_some(self.header, |this, header| {
                this.child(cap(header).border_b(theme.border_width))
            })
            .when(has_body, |this| {
                this.child(
                    v_flex()
                        .flex_1()
                        .p(theme.spacer)
                        .gap(theme.spacing(2))
                        .when_some(self.title, |this, title| {
                            this.child(
                                div()
                                    .text_size(theme.typography.headings[4])
                                    .font_weight(theme.typography.headings_font_weight)
                                    .line_height(relative(1.2))
                                    .child(title),
                            )
                        })
                        .when_some(self.subtitle, |this, subtitle| {
                            this.child(
                                div()
                                    .text_color(theme.colors.secondary_color)
                                    .font_weight(theme.typography.headings_font_weight)
                                    .child(subtitle),
                            )
                        })
                        .children(self.children),
                )
            })
            .when_some(self.footer, |this, footer| {
                this.child(cap(footer).border_t(theme.border_width))
            })
            .refine_style(&self.style)
    }
}
