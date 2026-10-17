//! `.pagination`.

use std::rc::Rc;

use gpui_kit::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
    base::{self, PaginationItem, PaginationState, StyledExt as _},
    div,
    prelude::FluentBuilder as _,
    relative,
};

use crate::{ActiveTheme as _, Size};

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Page links with previous and next controls.
///
/// Which pages show and where ellipses go comes from
/// [`gpui_kit::base::PaginationState`]. Pages are 1-based.
#[cfg_attr(
    feature = "styleguide",
    gpui_styleguidist::preview("pagination/playground")
)]
#[derive(IntoElement)]
pub struct Pagination {
    id: ElementId,
    current_page: usize,
    total_pages: usize,
    visible_pages: usize,
    size: Size,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    style: StyleRefinement,
}

impl Pagination {
    pub fn new(id: impl Into<ElementId>, current_page: usize, total_pages: usize) -> Self {
        Self {
            id: id.into(),
            current_page,
            total_pages,
            visible_pages: 5,
            size: Size::Medium,
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    /// How many page numbers show before ellipses take over. Default 5.
    pub fn visible_pages(mut self, visible_pages: usize) -> Self {
        self.visible_pages = visible_pages;
        self
    }

    /// `.pagination-sm` / `.pagination-lg`.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Called with the requested page.
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Pagination {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

enum Link {
    Previous(Option<usize>),
    Page(usize),
    Ellipsis,
    Next(Option<usize>),
}

impl RenderOnce for Pagination {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;
        let mut state = PaginationState::new(self.current_page, self.total_pages)
            .visible_pages(self.visible_pages)
            .disabled(self.disabled);
        if let Some(on_change) = self.on_change.clone() {
            state = state.on_change(move |page, window, cx| on_change(page, window, cx));
        }
        let current = state.current_page();
        let mut links = vec![Link::Previous(state.previous_page())];
        links.extend(state.items().into_iter().map(|item| match item {
            PaginationItem::Page(page) => Link::Page(page),
            PaginationItem::Ellipsis(_) => Link::Ellipsis,
        }));
        links.push(Link::Next(state.next_page()));

        let count = links.len();
        let control = theme.controls.get(self.size);
        let radius = theme.radius.for_size(self.size);
        let font_size = theme.typography.for_size(self.size);
        let on_change = self.on_change;
        let disabled = self.disabled;

        base::Pagination::new(self.id, state)
            .flex()
            .children(links.into_iter().enumerate().map(|(index, link)| {
                let (label, target, active): (SharedString, Option<usize>, bool) = match link {
                    Link::Previous(page) => ("«".into(), page, false),
                    Link::Next(page) => ("»".into(), page, false),
                    Link::Page(page) => (page.to_string().into(), Some(page), page == current),
                    Link::Ellipsis => ("…".into(), None, false),
                };
                let enabled = !disabled && target.is_some() && !active;
                let (bg, color, border) = if active {
                    (
                        colors.primary,
                        theme.contrast(colors.primary),
                        colors.primary,
                    )
                } else if enabled {
                    (colors.body_bg, colors.link_color, colors.border_color)
                } else {
                    (
                        colors.secondary_bg,
                        colors.secondary_color,
                        colors.border_color,
                    )
                };
                let (hover_bg, hover_color) = (colors.tertiary_bg, colors.link_hover_color);
                let on_change = on_change.clone();
                base::Button::new(("page", index))
                    .disabled(!enabled)
                    .selected(active)
                    .when(matches!(index, 0), |this| {
                        this.accessibility_label("Previous")
                    })
                    .when(index + 1 == count, |this| this.accessibility_label("Next"))
                    .px(control.padding_x)
                    .py(control.padding_y)
                    .min_h(control.min_height)
                    .min_w(control.min_height)
                    .text_size(font_size)
                    .line_height(relative(theme.typography.line_height))
                    .bg(bg)
                    .text_color(color)
                    .border(theme.border_width)
                    .border_color(border)
                    .when(index > 0, |this| this.border_l_0())
                    .when(index == 0, |this| this.rounded_l(radius))
                    .when(index + 1 == count, |this| this.rounded_r(radius))
                    .when(enabled, |this| {
                        this.hover(move |style| style.bg(hover_bg).text_color(hover_color))
                    })
                    .when_some(
                        target.filter(|_| enabled).zip(on_change),
                        |this, (page, on_change)| {
                            this.on_click(move |_, window, cx| on_change(page, window, cx))
                        },
                    )
                    .child(div().child(label))
            }))
            .refine_style(&self.style)
    }
}
