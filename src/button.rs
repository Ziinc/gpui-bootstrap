//! Buttons (`.btn`) and button groups (`.btn-group`).

use std::rc::Rc;

use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    base::{self, StyledExt as _},
    div,
    prelude::FluentBuilder as _,
    relative,
};

use crate::{ActiveTheme as _, ButtonColors, Size, Theme, Variant};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// How a [`Button`] paints its [`Variant`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonStyle {
    /// `btn-<variant>`.
    #[default]
    Solid,
    /// `btn-outline-<variant>`.
    Outline,
    /// `btn-link`: a button that looks like a link. Ignores the variant.
    Link,
}

/// Where a button sits in a [`ButtonGroup`], which decides its rounded corners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GroupPosition {
    First,
    Middle,
    Last,
}

/// A Bootstrap button.
///
/// Focus, Space/Enter activation and accessibility come from
/// [`gpui_kit::base::Button`].
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    variant: Variant,
    button_style: ButtonStyle,
    size: Size,
    disabled: bool,
    active: bool,
    on_click: Option<ClickHandler>,
    accessibility_label: Option<SharedString>,
    group_position: Option<GroupPosition>,
    style: StyleRefinement,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            children: Vec::new(),
            variant: Variant::Primary,
            button_style: ButtonStyle::Solid,
            size: Size::Medium,
            disabled: false,
            active: false,
            on_click: None,
            accessibility_label: None,
            group_position: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn button_style(mut self, style: ButtonStyle) -> Self {
        self.button_style = style;
        self
    }

    /// Shorthand for [`ButtonStyle::Outline`].
    pub fn outline(self) -> Self {
        self.button_style(ButtonStyle::Outline)
    }

    /// Shorthand for [`ButtonStyle::Link`].
    pub fn link(self) -> Self {
        self.button_style(ButtonStyle::Link)
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// `btn-sm`.
    pub fn small(self) -> Self {
        self.with_size(Size::Small)
    }

    /// `btn-lg`.
    pub fn large(self) -> Self {
        self.with_size(Size::Large)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Paints the pressed state, Bootstrap's `.active`, as for a toggle.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    fn group_position(mut self, position: GroupPosition) -> Self {
        self.group_position = Some(position);
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

fn link_colors(theme: &Theme) -> ButtonColors {
    let transparent = gpui_kit::transparent_black();
    ButtonColors {
        bg: transparent,
        border: transparent,
        color: theme.colors.link_color,
        hover_bg: transparent,
        hover_border: transparent,
        hover_color: theme.colors.link_hover_color,
        active_bg: transparent,
        active_border: transparent,
        active_color: theme.colors.link_hover_color,
        focus_ring: theme.focus_ring_color(),
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = focus.is_focused(window);
        let theme = cx.theme();
        let colors = match self.button_style {
            ButtonStyle::Solid => theme.button(self.variant),
            ButtonStyle::Outline => theme.outline_button(self.variant),
            ButtonStyle::Link => link_colors(theme),
        };
        let is_link = self.button_style == ButtonStyle::Link;
        let control = theme.controls.get(self.size);
        let radius = theme
            .controls
            .button_radius
            .unwrap_or(theme.radius.for_size(self.size));
        let interactive = !self.disabled && !self.active;

        base::Button::new(self.id)
            .track_focus(&focus)
            .disabled(self.disabled)
            .selected(self.active)
            .when_some(self.accessibility_label, |this, label| {
                this.accessibility_label(label)
            })
            .h_flex()
            .flex_shrink_0()
            .gap(theme.spacing(2))
            .px(control.padding_x)
            .py(control.padding_y)
            .min_h(control.min_height)
            .text_size(theme.typography.for_size(self.size))
            .line_height(relative(theme.typography.line_height))
            .font_weight(theme.typography.font_weight_normal)
            .border(theme.border_width)
            .border_color(colors.border)
            .bg(colors.bg)
            .text_color(colors.color)
            .map(|this| match self.group_position {
                None => this.rounded(radius),
                Some(GroupPosition::First) => this.rounded_l(radius),
                // Joined buttons share one border instead of doubling it.
                Some(GroupPosition::Middle) => this.border_l_0(),
                Some(GroupPosition::Last) => this.rounded_r(radius).border_l_0(),
            })
            .when(is_link, |this| this.underline())
            .when(interactive, |this| {
                this.hover(move |style| {
                    style
                        .bg(colors.hover_bg)
                        .border_color(colors.hover_border)
                        .text_color(colors.hover_color)
                })
                .active(move |style| {
                    style
                        .bg(colors.active_bg)
                        .border_color(colors.active_border)
                        .text_color(colors.active_color)
                })
            })
            .when(self.active, |this| {
                this.bg(colors.active_bg)
                    .border_color(colors.active_border)
                    .text_color(colors.active_color)
            })
            .when(self.disabled, |this| this.opacity(theme.disabled_opacity))
            .when(focused, |this| {
                this.shadow(vec![theme.focus_ring(colors.focus_ring)])
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |event, window, cx| on_click(event, window, cx))
            })
            .when_some(self.label, |this, label| this.child(label))
            .children(self.children)
            .refine_style(&self.style)
    }
}

/// `.btn-group`: buttons joined into one control.
#[derive(IntoElement)]
pub struct ButtonGroup {
    id: ElementId,
    buttons: Vec<Button>,
    size: Option<Size>,
    style: StyleRefinement,
}

impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            buttons: Vec::new(),
            size: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn buttons(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.buttons.extend(buttons);
        self
    }

    /// `btn-group-sm` / `btn-group-lg`: sizes every button in the group.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
}

impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let count = self.buttons.len();
        let size = self.size;
        div()
            .id(self.id)
            .role(gpui_kit::Role::Group)
            .h_flex()
            .flex_shrink_0()
            .children(self.buttons.into_iter().enumerate().map(|(index, button)| {
                let button = match size {
                    Some(size) => button.with_size(size),
                    None => button,
                };
                match (index, count) {
                    (_, 1) => button,
                    (0, _) => button.group_position(GroupPosition::First),
                    (index, count) if index + 1 == count => {
                        button.group_position(GroupPosition::Last)
                    }
                    _ => button.group_position(GroupPosition::Middle),
                }
            }))
            .refine_style(&self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_builder() {
        let button = Button::new("save")
            .label("Save")
            .variant(Variant::Danger)
            .outline()
            .small()
            .disabled(true)
            .active(true);
        assert_eq!(button.variant, Variant::Danger);
        assert_eq!(button.button_style, ButtonStyle::Outline);
        assert_eq!(button.size, Size::Small);
        assert!(button.disabled && button.active);
    }
}
