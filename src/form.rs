//! Forms: `.form-control`, `.form-check` (checkboxes, radios, switches),
//! `.form-label` and `.form-text`.

use std::rc::Rc;

use gpui_kit::{
    App, ClickEvent, ElementId, Entity, Focusable as _, Hsla, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
    base::{self, CheckboxState, StyledExt as _},
    component::{
        Icon, IconName, Sizable as _,
        input::{Input, InputState},
    },
    div,
    prelude::FluentBuilder as _,
    relative,
};

use crate::{ActiveTheme as _, Size, Theme};

type ToggleHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// `.form-label`.
#[derive(IntoElement)]
pub struct FormLabel {
    text: SharedString,
}

impl FormLabel {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for FormLabel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div().mb(cx.theme().spacing(2)).child(self.text)
    }
}

/// `.form-text`: help text under a control. Variants of validity use
/// `.valid-feedback` / `.invalid-feedback` colors.
#[derive(IntoElement)]
pub struct FormText {
    text: SharedString,
    validity: Option<bool>,
}

impl FormText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            validity: None,
        }
    }

    /// `.valid-feedback` (`true`) or `.invalid-feedback` (`false`).
    pub fn validity(mut self, valid: bool) -> Self {
        self.validity = Some(valid);
        self
    }
}

impl RenderOnce for FormText {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let color = match self.validity {
            Some(true) => theme.colors.success,
            Some(false) => theme.colors.danger,
            None => theme.colors.secondary_color,
        };
        div()
            .mt(theme.spacing(1))
            .text_size(theme.typography.font_size_base * 0.875)
            .text_color(color)
            .child(self.text)
    }
}

/// A text field, `.form-control`, over gpui-kit's [`Input`] editor.
///
/// Create the state once and keep it in your view:
/// `cx.new(|cx| InputState::new(window, cx).placeholder("name@example.com"))`.
#[cfg_attr(
    feature = "styleguide",
    gpui_styleguidist::preview("form-control/basic")
)]
#[derive(IntoElement)]
pub struct FormControl {
    state: Entity<InputState>,
    size: Size,
    disabled: bool,
    validity: Option<bool>,
    style: StyleRefinement,
}

impl FormControl {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            size: Size::Medium,
            disabled: false,
            validity: None,
            style: StyleRefinement::default(),
        }
    }

    /// `.form-control-sm` / `.form-control-lg`.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// `.is-valid` (`true`) or `.is-invalid` (`false`).
    pub fn validity(mut self, valid: bool) -> Self {
        self.validity = Some(valid);
        self
    }
}

impl Styled for FormControl {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FormControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focused = self.state.read(cx).focus_handle(cx).is_focused(window);
        let theme = cx.theme();
        let colors = &theme.colors;
        let control = theme.controls.get(self.size);
        let font_size = theme.typography.for_size(self.size);
        let validity_color = self
            .validity
            .map(|valid| if valid { colors.success } else { colors.danger });
        let border = match (validity_color, focused) {
            (Some(color), _) => color,
            (None, true) => theme.input_focus_border_color(),
            (None, false) => colors.border_color,
        };
        let ring = validity_color.map_or(theme.focus_ring_color(), |color| {
            color.opacity(theme.focus_ring_opacity)
        });
        div()
            .w_full()
            .flex()
            .items_center()
            .px(control.padding_x)
            .py(control.padding_y)
            .min_h(control.min_height)
            .bg(if self.disabled {
                colors.secondary_bg
            } else {
                colors.body_bg
            })
            .text_color(colors.body_color)
            .text_size(font_size)
            .border(theme.border_width)
            .border_color(border)
            .rounded(theme.radius.for_size(self.size))
            .when(focused, |this| this.shadow(vec![theme.focus_ring(ring)]))
            .child(
                Input::new(&self.state)
                    .appearance(false)
                    .disabled(self.disabled)
                    .map(|input| match self.size {
                        Size::Small => input.small(),
                        Size::Medium => input,
                        Size::Large => input.large(),
                    })
                    .w_full()
                    .px_0()
                    .py_0()
                    .h(font_size * theme.typography.line_height)
                    .text_size(font_size),
            )
            .refine_style(&self.style)
    }
}

/// The colors a check, radio or switch indicator paints with.
struct CheckColors {
    bg: Hsla,
    border: Hsla,
}

fn check_colors(theme: &Theme, checked: bool, focused: bool) -> CheckColors {
    match (checked, focused) {
        (true, _) => CheckColors {
            bg: theme.colors.primary,
            border: theme.colors.primary,
        },
        (false, true) => CheckColors {
            bg: theme.colors.body_bg,
            border: theme.input_focus_border_color(),
        },
        (false, false) => CheckColors {
            bg: theme.colors.body_bg,
            border: theme.colors.border_color,
        },
    }
}

/// A `.form-check` row: indicator, then label.
fn check_row<E: Styled + gpui_kit::ParentElement + IntoElement>(
    root: E,
    indicator: impl IntoElement,
    label: Option<SharedString>,
    disabled: bool,
    theme: &Theme,
) -> E {
    // Hug the content so blank space beside the label doesn't toggle it, but
    // stay within the container so long labels wrap.
    root.h_flex()
        .self_start()
        .max_w_full()
        .items_start()
        .gap(theme.spacing(2))
        .line_height(relative(theme.typography.line_height))
        .when(disabled, |this| this.opacity(0.5))
        .child(indicator)
        .when_some(label, |this, label| {
            this.child(div().flex_1().min_w_0().child(label))
        })
}

/// The vertical offset that centers a 1em indicator on the first line,
/// Bootstrap's `$form-check-input-margin-top`.
fn indicator_offset(theme: &Theme) -> gpui_kit::Pixels {
    theme.typography.font_size_base * (theme.typography.line_height - 1.) / 2.
}

macro_rules! check_builders {
    () => {
        pub fn label(mut self, label: impl Into<SharedString>) -> Self {
            self.label = Some(label.into());
            self
        }

        pub fn checked(mut self, checked: bool) -> Self {
            self.checked = checked;
            self
        }

        pub fn disabled(mut self, disabled: bool) -> Self {
            self.disabled = disabled;
            self
        }

        /// Called with the requested checked state. The application stores it.
        pub fn on_change(
            mut self,
            handler: impl Fn(bool, &mut Window, &mut App) + 'static,
        ) -> Self {
            self.on_change = Some(Rc::new(handler));
            self
        }
    };
}

/// A checkbox, `.form-check-input[type=checkbox]`, over
/// [`gpui_kit::base::Checkbox`].
#[cfg_attr(
    feature = "styleguide",
    gpui_styleguidist::preview("checks/playground")
)]
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
    on_change: Option<ToggleHandler>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            checked: false,
            indeterminate: false,
            disabled: false,
            on_change: None,
        }
    }

    check_builders!();

    /// Shows a dash for a partly-selected group; overrides `checked` visually.
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = crate::use_focus(self.id.clone(), window, cx);
        let focused = focus.is_focused(window);
        let theme = cx.theme();
        let filled = self.checked || self.indeterminate;
        let colors = check_colors(theme, filled, focused);
        let size = theme.typography.font_size_base;
        let state = if self.indeterminate {
            CheckboxState::Indeterminate
        } else if self.checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };
        let on_change = self.on_change;
        let indicator = div()
            .flex_shrink_0()
            .mt(indicator_offset(theme))
            .size(size)
            .flex()
            .items_center()
            .justify_center()
            .bg(colors.bg)
            .border(theme.border_width)
            .border_color(colors.border)
            .rounded(theme.radius.sm)
            .text_color(theme.contrast(theme.colors.primary))
            .when(focused, |this| {
                this.shadow(vec![theme.focus_ring(theme.focus_ring_color())])
            })
            .when(filled, |this| {
                this.child(
                    Icon::new(if self.indeterminate {
                        IconName::Minus
                    } else {
                        IconName::Check
                    })
                    .size(size * 0.75),
                )
            });
        check_row(
            base::Checkbox::new(self.id)
                .track_focus(&focus)
                .state(state)
                .disabled(self.disabled)
                .when_some(self.label.clone(), |this, label| {
                    this.accessibility_label(label)
                })
                .when_some(on_change, |this, on_change| {
                    this.on_change(move |state, _, window, cx| {
                        on_change(state == CheckboxState::Checked, window, cx)
                    })
                }),
            indicator,
            self.label,
            self.disabled,
            theme,
        )
    }
}

/// A radio button, `.form-check-input[type=radio]`, over
/// [`gpui_kit::base::Radio`]. Group radios in a
/// [`gpui_kit::base::RadioGroup`] or any container; the application keeps
/// which one is checked.
#[derive(IntoElement)]
pub struct Radio {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    disabled: bool,
    on_change: Option<ToggleHandler>,
}

impl Radio {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            checked: false,
            disabled: false,
            on_change: None,
        }
    }

    check_builders!();
}

impl RenderOnce for Radio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = crate::use_focus(self.id.clone(), window, cx);
        let focused = focus.is_focused(window);
        let theme = cx.theme();
        let colors = check_colors(theme, self.checked, focused);
        let size = theme.typography.font_size_base;
        let on_change = self.on_change;
        let indicator = div()
            .flex_shrink_0()
            .mt(indicator_offset(theme))
            .size(size)
            .flex()
            .items_center()
            .justify_center()
            .bg(colors.bg)
            .border(theme.border_width)
            .border_color(colors.border)
            .rounded_full()
            .when(focused, |this| {
                this.shadow(vec![theme.focus_ring(theme.focus_ring_color())])
            })
            .when(self.checked, |this| {
                this.child(
                    div()
                        .size(size * 0.5)
                        .rounded_full()
                        .bg(theme.contrast(theme.colors.primary)),
                )
            });
        check_row(
            base::Radio::new(self.id)
                .track_focus(&focus)
                .checked(self.checked)
                .disabled(self.disabled)
                .when_some(self.label.clone(), |this, label| {
                    this.accessibility_label(label)
                })
                .when_some(on_change, |this, on_change| {
                    this.on_change(move |checked, _: &ClickEvent, window, cx| {
                        on_change(checked, window, cx)
                    })
                }),
            indicator,
            self.label,
            self.disabled,
            theme,
        )
    }
}

/// A toggle switch, `.form-switch`, over [`gpui_kit::base::Switch`].
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    disabled: bool,
    on_change: Option<ToggleHandler>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            checked: false,
            disabled: false,
            on_change: None,
        }
    }

    check_builders!();
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = crate::use_focus(self.id.clone(), window, cx);
        let focused = focus.is_focused(window);
        let theme = cx.theme();
        let colors = check_colors(theme, self.checked, focused);
        let height = theme.typography.font_size_base;
        // The knob is drawn at 3/4 of the track height, as in Bootstrap's SVG,
        // and never larger than the space inside the border.
        let inner = (height - theme.border_width * 2.).max(gpui_kit::px(0.));
        let knob = (height * 0.75).min(inner);
        let knob_color = match (self.checked, focused) {
            (true, _) => theme.contrast(theme.colors.primary),
            (false, true) => theme.input_focus_border_color(),
            (false, false) => theme.colors.body_color.opacity(0.25),
        };
        let on_change = self.on_change;
        let indicator = div()
            .flex_shrink_0()
            .mt(indicator_offset(theme))
            .w(height * 2.)
            .h(height)
            .flex()
            .items_center()
            .px(((inner - knob) / 2.).max(gpui_kit::px(0.)))
            .when(self.checked, |this| this.justify_end())
            .bg(colors.bg)
            .border(theme.border_width)
            .border_color(colors.border)
            .rounded(theme.radius.pill)
            .when(focused, |this| {
                this.shadow(vec![theme.focus_ring(theme.focus_ring_color())])
            })
            .child(div().size(knob).rounded_full().bg(knob_color));
        check_row(
            base::Switch::new(self.id)
                .track_focus(&focus)
                .checked(self.checked)
                .disabled(self.disabled)
                .when_some(self.label.clone(), |this, label| {
                    this.accessibility_label(label)
                })
                .when_some(on_change, |this, on_change| {
                    this.on_change(move |checked, _, window, cx| on_change(checked, window, cx))
                }),
            indicator,
            self.label,
            self.disabled,
            theme,
        )
    }
}
