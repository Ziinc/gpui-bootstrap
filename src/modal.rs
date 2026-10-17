//! `.modal`.

use std::rc::Rc;

use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
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
    id: ElementId,
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
            id: "modal".into(),
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
    /// Distinguishes modals that are open at the same time, such as a
    /// confirmation over a form. Defaults to `"modal"`.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

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
            .use_keyed_state(
                ElementId::NamedChild(self.id.clone().into(), "close".into()),
                cx,
                |_, cx| cx.focus_handle(),
            )
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
        // `$modal-dialog-margin-y-sm-up: 1.75rem`, above and below.
        let margin = px(28.);
        let divider = colors.border_color;

        let popup = DialogPopup::new()
            .w(width)
            .max_w(relative(1.))
            // `.modal-dialog-scrollable`: the body scrolls so the header and
            // footer stay reachable.
            .max_h(window.viewport_size().height - margin * 2.)
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
                        .flex_shrink_0()
                        .gap(theme.spacing(2))
                        .p(padding)
                        .border_b(theme.border_width)
                        .border_color(divider)
                        .child(
                            DialogTitle::new()
                                .flex_1()
                                .min_w_0()
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
                    .id(ElementId::NamedChild(self.id.clone().into(), "body".into()))
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .p(padding)
                    .gap(theme.spacing(2))
                    .children(self.children),
            )
            .when_some(self.footer, |this, footer| {
                this.child(
                    h_flex()
                        .justify_end()
                        .flex_shrink_0()
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
                this.items_start().pt(margin)
            })
            .px(theme.spacing(2))
            .refine_style(&self.style)
            .into_any_element()
    }
}
