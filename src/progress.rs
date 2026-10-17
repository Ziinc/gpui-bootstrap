//! `.progress` and `.progress-stacked`.

use gpui_kit::{
    App, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString, StyleRefinement,
    Styled, Window,
    base::{self, StyledExt as _},
    div,
    prelude::FluentBuilder as _,
    relative,
};

use crate::{ActiveTheme as _, Variant};

/// A progress track holding one or more [`ProgressBar`]s side by side.
///
/// Value semantics and accessibility come from [`gpui_kit::base::Progress`].
#[cfg_attr(
    feature = "styleguide",
    gpui_styleguidist::preview("progress/playground")
)]
#[derive(IntoElement)]
pub struct Progress {
    id: ElementId,
    bars: Vec<ProgressBar>,
    accessibility_label: Option<SharedString>,
    style: StyleRefinement,
}

impl Progress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            bars: Vec::new(),
            accessibility_label: None,
            style: StyleRefinement::default(),
        }
    }

    /// Shorthand for a single primary bar at `value` percent.
    pub fn value(self, value: f32) -> Self {
        self.bar(ProgressBar::new(value))
    }

    /// Adds a bar; several bars stack like `.progress-stacked`.
    pub fn bar(mut self, bar: ProgressBar) -> Self {
        self.bars.push(bar);
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }
}

impl Styled for Progress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// One segment of a [`Progress`], `.progress-bar`.
pub struct ProgressBar {
    value: f32,
    variant: Variant,
    label: Option<SharedString>,
}

impl ProgressBar {
    /// A bar `value` percent wide, clamped to `0..=100`. NaN counts as 0.
    pub fn new(value: f32) -> Self {
        Self {
            value: if value.is_nan() {
                0.
            } else {
                value.clamp(0., 100.)
            },
            variant: Variant::Primary,
            label: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Text centered on the bar, such as `"25%"`.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for Progress {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let total = self.bars.iter().map(|bar| bar.value).sum::<f32>().min(100.);
        // `$progress-height: 1rem`, `$progress-font-size: .75rem`.
        let height = theme.typography.font_size_base;
        base::Progress::new(self.id)
            .value(total)
            .when_some(self.accessibility_label, |this, label| {
                this.accessibility_label(label)
            })
            .flex()
            .h(height)
            .w_full()
            .overflow_hidden()
            .rounded(theme.radius.base)
            .bg(theme.colors.secondary_bg)
            .text_size(height * 0.75)
            .children(self.bars.into_iter().map(|bar| {
                let colors = theme.variant(bar.variant);
                // Bars past 100% in total are clipped by the track, not
                // squeezed to fit.
                div()
                    .h_full()
                    .w(relative(bar.value / 100.))
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .overflow_hidden()
                    .bg(colors.base)
                    .text_color(colors.contrast)
                    .line_height(relative(1.))
                    .children(bar.label)
            }))
            .refine_style(&self.style)
    }
}
