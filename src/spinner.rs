//! `.spinner-border` and `.spinner-grow`.

use std::time::Duration;

use gpui_kit::{
    Animation, AnimationExt as _, App, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, Role, StatefulInteractiveElement as _, StyleRefinement, Styled, Transformation,
    Window,
    base::StyledExt as _,
    component::{Icon, IconName},
    div, ease_in_out, percentage,
    prelude::FluentBuilder as _,
    px,
};

use crate::{ActiveTheme as _, Size, Variant};

/// How a [`Spinner`] animates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerStyle {
    /// `.spinner-border`: a rotating ring.
    #[default]
    Border,
    /// `.spinner-grow`: a pulsing dot.
    Grow,
}

/// An indeterminate loading indicator. Without a variant it takes the
/// surrounding text color, like Bootstrap's `currentcolor`.
#[cfg_attr(
    feature = "styleguide",
    gpui_styleguidist::preview("spinner/playground")
)]
#[derive(IntoElement)]
pub struct Spinner {
    spinner_style: SpinnerStyle,
    variant: Option<Variant>,
    size: Size,
    style: StyleRefinement,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            spinner_style: SpinnerStyle::Border,
            variant: None,
            size: Size::Medium,
            style: StyleRefinement::default(),
        }
    }

    pub fn spinner_style(mut self, style: SpinnerStyle) -> Self {
        self.spinner_style = style;
        self
    }

    /// Shorthand for [`SpinnerStyle::Grow`].
    pub fn grow(self) -> Self {
        self.spinner_style(SpinnerStyle::Grow)
    }

    /// `.text-<variant>`.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    /// [`Size::Small`] is `.spinner-border-sm`; [`Size::Large`] is a 3rem spinner.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn small(self) -> Self {
        self.with_size(Size::Small)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Spinner {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        // `$spinner-width: 2rem`, `$spinner-width-sm: 1rem`.
        let size = match self.size {
            Size::Small => px(16.),
            Size::Medium => px(32.),
            Size::Large => px(48.),
        };
        let color = self.variant.map(|variant| theme.colors.variant(variant));
        div()
            .id("spinner")
            .role(Role::ProgressIndicator)
            .aria_label("Loading")
            .size(size)
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .when_some(color, |this, color| this.text_color(color))
            .map(|this| match self.spinner_style {
                SpinnerStyle::Border => {
                    this.child(Icon::new(IconName::LoaderCircle).size(size).with_animation(
                        "spinner-border",
                        Animation::new(Duration::from_millis(750)).repeat(),
                        |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                    ))
                }
                SpinnerStyle::Grow => {
                    let dot_color = color.unwrap_or(theme.colors.body_color);
                    this.child(
                        div().rounded_full().bg(dot_color).with_animation(
                            "spinner-grow",
                            Animation::new(Duration::from_millis(750))
                                .repeat()
                                .with_easing(ease_in_out),
                            move |dot, delta| {
                                // `spinner-grow`: scale 0 → 1 while fading out.
                                dot.size(size * delta).opacity(1. - delta * 0.9)
                            },
                        ),
                    )
                }
            })
            .refine_style(&self.style)
    }
}
