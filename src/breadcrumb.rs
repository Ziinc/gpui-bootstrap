//! `.breadcrumb`.

use std::rc::Rc;

use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
    base::{self, StyledExt as _, h_flex},
    div,
    prelude::FluentBuilder as _,
};

use crate::ActiveTheme as _;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

struct Crumb {
    label: SharedString,
    on_click: Option<ClickHandler>,
}

/// A navigation trail. The last item is the current page.
#[cfg_attr(feature = "styleguide", gpui_styleguidist::preview("breadcrumb/basic"))]
#[derive(IntoElement)]
pub struct Breadcrumb {
    id: ElementId,
    items: Vec<Crumb>,
    divider: SharedString,
    style: StyleRefinement,
}

impl Breadcrumb {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            divider: "/".into(),
            style: StyleRefinement::default(),
        }
    }

    /// An item that navigates when clicked.
    pub fn item(
        mut self,
        label: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(Crumb {
            label: label.into(),
            on_click: Some(Rc::new(on_click)),
        });
        self
    }

    /// The current page, which does not navigate.
    pub fn active(mut self, label: impl Into<SharedString>) -> Self {
        self.items.push(Crumb {
            label: label.into(),
            on_click: None,
        });
        self
    }

    /// `$breadcrumb-divider`, `/` by default.
    pub fn divider(mut self, divider: impl Into<SharedString>) -> Self {
        self.divider = divider.into();
        self
    }
}

impl Styled for Breadcrumb {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let count = self.items.len();
        let divider = self.divider;
        let (link, link_hover, muted) = (
            theme.colors.link_color,
            theme.colors.link_hover_color,
            theme.colors.secondary_color,
        );
        let gap = theme.spacing(2);
        h_flex()
            .id(self.id)
            .role(Role::Navigation)
            .aria_label("Breadcrumb")
            .flex_wrap()
            .children(
                self.items
                    .into_iter()
                    .enumerate()
                    .map(move |(index, crumb)| {
                        let last = index + 1 == count;
                        h_flex()
                            .when(index > 0, |this| {
                                this.child(div().px(gap).text_color(muted).child(divider.clone()))
                            })
                            .map(|this| match crumb.on_click.filter(|_| !last) {
                                Some(on_click) => this.child(
                                    base::Button::new(("crumb", index))
                                        .role(Role::Link)
                                        .text_color(link)
                                        .underline()
                                        .cursor_pointer()
                                        .hover(move |style| style.text_color(link_hover))
                                        .on_click(move |event, window, cx| {
                                            on_click(event, window, cx)
                                        })
                                        .child(crumb.label),
                                ),
                                None => this.child(div().text_color(muted).child(crumb.label)),
                            })
                    }),
            )
            .refine_style(&self.style)
    }
}
