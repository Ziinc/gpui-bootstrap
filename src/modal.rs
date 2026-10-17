//! `.modal`.

use std::rc::Rc;

use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window,
    base::{
        DialogBackdrop, DialogChangeReason, DialogClose, DialogHandle, DialogPopup, DialogTitle,
        StyledExt as _, h_flex, v_flex,
    },
    div,
    prelude::FluentBuilder as _,
    px, relative,
};

use crate::{ActiveTheme as _, close_button::close_button_frame};

type OpenChangeHandler = Rc<dyn Fn(bool, DialogChangeReason, &mut Window, &mut App)>;

/// The width of a [`Modal`], `.modal-sm` through `.modal-xl`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModalSize {
    /// 300px.
    Small,
    /// 500px.
    #[default]
    Medium,
    /// 800px.
    Large,
    /// 1140px.
    ExtraLarge,
}

/// A dialog with Bootstrap's header, body and footer.
///
/// Focus trapping, Escape, the backdrop press and close callbacks come from
/// [`gpui_kit::base::Dialog`]. Keep a [`DialogHandle`] in your view, render
/// the modal every frame, and call `handle.open(window, cx)` to show it.
#[cfg_attr(feature = "styleguide", gpui_styleguidist::preview("modal/basic"))]
#[derive(IntoElement)]
pub struct Modal {
    handle: DialogHandle,
    title: Option<SharedString>,
    footer: Option<AnyElement>,
    children: Vec<AnyElement>,
    size: ModalSize,
    centered: bool,
    static_backdrop: bool,
    on_open_change: Option<OpenChangeHandler>,
    style: StyleRefinement,
}

impl Modal {
    pub fn new(handle: &DialogHandle) -> Self {
        Self {
            handle: handle.clone(),
            title: None,
            footer: None,
            children: Vec::new(),
            size: ModalSize::Medium,
            centered: false,
            static_backdrop: false,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    /// `.modal-title`, shown with a close button in `.modal-header`.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// `.modal-footer`, typically buttons.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn with_size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }

    /// `.modal-dialog-centered`.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// `data-bs-backdrop="static"`: a backdrop press does not close the modal.
    pub fn static_backdrop(mut self, static_backdrop: bool) -> Self {
        self.static_backdrop = static_backdrop;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, DialogChangeReason, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Modal {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Modal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Modal {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.handle.is_open() {
            return div().into_any_element();
        }
        let close_focus = window
            .use_keyed_state("modal-close", cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let close_focused = close_focus.is_focused(window);
        let theme = cx.theme().clone();
        let colors = &theme.colors;
        let width = match self.size {
            ModalSize::Small => px(300.),
            ModalSize::Medium => px(500.),
            ModalSize::Large => px(800.),
            ModalSize::ExtraLarge => px(1140.),
        };
        let padding = theme.spacer;
        let divider = colors.border_color;

        let popup = DialogPopup::new()
            .w(width)
            .max_w(relative(1.))
            .flex()
            .flex_col()
            .bg(colors.body_bg)
            .text_color(colors.body_color)
            .border(theme.border_width)
            .border_color(colors.border_color_translucent)
            .rounded(theme.radius.lg)
            .shadow(vec![theme.shadow_sm()])
            .when_some(self.title, |this, title| {
                this.child(
                    h_flex()
                        .justify_between()
                        .gap(theme.spacing(2))
                        .p(padding)
                        .border_b(theme.border_width)
                        .border_color(divider)
                        .child(
                            DialogTitle::new()
                                .text_size(theme.typography.headings[4])
                                .font_weight(theme.typography.headings_font_weight)
                                .line_height(relative(theme.typography.line_height))
                                .child(title),
                        )
                        .child(DialogClose::new().trigger(|button| {
                            close_button_frame(button, false, close_focused, cx)
                                .track_focus(&close_focus)
                        })),
                )
            })
            .child(
                v_flex()
                    .p(padding)
                    .gap(theme.spacing(2))
                    .children(self.children),
            )
            .when_some(self.footer, |this, footer| {
                this.child(
                    h_flex()
                        .justify_end()
                        .flex_wrap()
                        .gap(theme.spacing(2))
                        .p(padding - theme.spacing(1))
                        .border_t(theme.border_width)
                        .border_color(divider)
                        .child(footer),
                )
            });

        gpui_kit::base::Dialog::new(cx)
            .handle(self.handle)
            .when_some(self.on_open_change, |this, on_open_change| {
                this.on_open_change(move |open, reason, window, cx| {
                    on_open_change(open, reason, window, cx)
                })
            })
            .close_on_backdrop_press(!self.static_backdrop)
            .backdrop(DialogBackdrop::new().size_full().bg(colors.backdrop))
            .popup(popup)
            .when(!self.centered, |this| {
                // `$modal-dialog-margin-y-sm-up: 1.75rem`.
                this.items_start().pt(px(28.))
            })
            .px(theme.spacing(2))
            .refine_style(&self.style)
            .into_any_element()
    }
}
