//! `.btn-close`.

use std::rc::Rc;

use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, Styled, Window, base,
    component::{Icon, IconName},
    prelude::FluentBuilder as _,
    px,
};

use crate::ActiveTheme as _;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A dismiss button: an `×` at half opacity, darker on hover.
#[cfg_attr(
    feature = "styleguide",
    gpui_styleguidist::preview("close-button/basic")
)]
#[derive(IntoElement)]
pub struct CloseButton {
    id: ElementId,
    white: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl CloseButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            white: false,
            disabled: false,
            on_click: None,
        }
    }

    /// `.btn-close-white`, for dark backgrounds.
    pub fn white(mut self, white: bool) -> Self {
        self.white = white;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for CloseButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = focus.is_focused(window);
        let on_click = self.on_click;
        close_button_frame(base::Button::new(self.id), self.white, focused, cx)
            .track_focus(&focus)
            .disabled(self.disabled)
            .when(self.disabled, |this| this.opacity(0.25))
            .when_some(on_click, |this, on_click| {
                this.on_click(move |event, window, cx| on_click(event, window, cx))
            })
    }
}

/// Styles a Base button as `.btn-close`; shared with [`crate::modal::Modal`],
/// whose close button comes from the dialog.
pub(crate) fn close_button_frame(
    button: base::Button,
    white: bool,
    focused: bool,
    cx: &App,
) -> base::Button {
    let theme = cx.theme();
    let color = if white {
        theme.colors.white
    } else {
        theme.colors.emphasis_color
    };
    button
        .accessibility_label("Close")
        .size(px(24.))
        .flex_shrink_0()
        .rounded(theme.radius.base)
        .text_color(color)
        .opacity(0.5)
        .hover(|style| style.opacity(0.75))
        .when(focused, |this| {
            this.opacity(1.)
                .shadow(vec![theme.focus_ring(theme.focus_ring_color())])
        })
        .child(Icon::new(IconName::Close).size(px(16.)))
}
