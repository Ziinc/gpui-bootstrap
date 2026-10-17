//! `.alert`.

use std::rc::Rc;

use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
    base::{StyledExt as _, h_flex, v_flex},
    div,
    prelude::FluentBuilder as _,
};

use crate::{ActiveTheme as _, Variant, close_button::CloseButton};

type DismissHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A contextual message, `alert alert-<variant>`.
#[derive(IntoElement)]
pub struct Alert {
    id: ElementId,
    variant: Variant,
    heading: Option<SharedString>,
    children: Vec<AnyElement>,
    on_dismiss: Option<DismissHandler>,
    style: StyleRefinement,
}

impl Alert {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: Variant::Primary,
            heading: None,
            children: Vec::new(),
            on_dismiss: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// `.alert-heading`.
    pub fn heading(mut self, heading: impl Into<SharedString>) -> Self {
        self.heading = Some(heading.into());
        self
    }

    /// `.alert-dismissible`: shows a close button that calls `handler`. The
    /// application removes the alert.
    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }
}

impl Styled for Alert {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Alert {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Alert {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.variant(self.variant);
        let id = self.id.clone();
        h_flex()
            .id(self.id)
            .role(Role::Alert)
            .items_start()
            .gap(theme.spacing(3))
            .p(theme.spacer)
            .border(theme.border_width)
            .border_color(colors.border_subtle)
            .rounded(theme.radius.base)
            .bg(colors.bg_subtle)
            .text_color(colors.text_emphasis)
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap(theme.spacing(2))
                    .when_some(self.heading, |this, heading| {
                        this.child(
                            div()
                                .text_size(theme.typography.headings[3])
                                .font_weight(theme.typography.headings_font_weight)
                                .line_height(gpui_kit::relative(1.2))
                                .child(heading),
                        )
                    })
                    .children(self.children),
            )
            .when_some(self.on_dismiss, |this, on_dismiss| {
                this.child(
                    CloseButton::new((id, "dismiss"))
                        .on_click(move |event, window, cx| on_dismiss(event, window, cx)),
                )
            })
            .refine_style(&self.style)
    }
}
