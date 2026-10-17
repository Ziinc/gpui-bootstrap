//! `.list-group`.

use std::rc::Rc;

use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    base::{self, StyledExt as _},
    div,
    prelude::FluentBuilder as _,
};

use crate::{ActiveTheme as _, Variant};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A bordered list of items.
#[derive(IntoElement)]
pub struct ListGroup {
    id: ElementId,
    items: Vec<ListGroupItem>,
    flush: bool,
    style: StyleRefinement,
}

impl ListGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            flush: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: ListGroupItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = ListGroupItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// `.list-group-flush`: no outer border or rounding, for use inside a card.
    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = flush;
        self
    }
}

impl Styled for ListGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ListGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let count = self.items.len();
        let flush = self.flush;
        div()
            .id(self.id)
            .role(gpui_kit::Role::List)
            .flex()
            .flex_col()
            .when(!flush, |this| {
                this.border(theme.border_width)
                    .border_color(theme.colors.border_color_translucent)
                    .rounded(theme.radius.base)
                    .overflow_hidden()
            })
            .children(self.items.into_iter().enumerate().map(|(index, mut item)| {
                item.separated = index > 0;
                item.flush_last = flush && index + 1 == count;
                item
            }))
            .refine_style(&self.style)
    }
}

/// One `.list-group-item`. Giving it a click handler makes it an
/// `.list-group-item-action`: focusable, hoverable, activatable.
#[derive(IntoElement)]
pub struct ListGroupItem {
    id: ElementId,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    variant: Option<Variant>,
    active: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
    separated: bool,
    flush_last: bool,
    style: StyleRefinement,
}

impl ListGroupItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            children: Vec::new(),
            variant: None,
            active: false,
            disabled: false,
            on_click: None,
            separated: false,
            flush_last: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// `.list-group-item-<variant>`.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
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

impl Styled for ListGroupItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ListGroupItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ListGroupItem {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = focus.is_focused(window);
        let theme = cx.theme();
        let (mut bg, mut color, mut hover_bg) = (
            theme.colors.body_bg,
            theme.colors.body_color,
            theme.colors.tertiary_bg,
        );
        if let Some(variant) = self.variant {
            let colors = theme.variant(variant);
            bg = colors.bg_subtle;
            color = colors.text_emphasis;
            hover_bg = colors.border_subtle;
        }
        let mut border = theme.colors.border_color_translucent;
        if self.active {
            bg = theme.colors.primary;
            color = theme.contrast(bg);
            border = bg;
        }
        if self.disabled {
            color = theme.colors.secondary_color;
        }
        let action = self.on_click.is_some() && !self.disabled && !self.active;

        base::Button::new(self.id)
            .track_focus(&focus)
            .focusable(self.on_click.is_some())
            .tab_stop(self.on_click.is_some())
            .role(gpui_kit::Role::ListItem)
            .disabled(self.disabled)
            .selected(self.active)
            .justify_start()
            .gap(theme.spacing(2))
            .w_full()
            .px(theme.spacer)
            .py(theme.spacing(2))
            .bg(bg)
            .text_color(color)
            .border_color(border)
            .when(self.separated, |this| this.border_t(theme.border_width))
            .when(self.flush_last, |this| this.border_b(theme.border_width))
            .when(action, |this| {
                let active_bg = theme.colors.secondary_bg;
                this.hover(move |style| style.bg(hover_bg))
                    .active(move |style| style.bg(active_bg))
            })
            .when(focused, |this| {
                this.shadow(vec![theme.focus_ring(theme.focus_ring_color())])
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |event, window, cx| on_click(event, window, cx))
                },
            )
            .when_some(self.label, |this, label| this.child(label))
            .children(self.children)
            .refine_style(&self.style)
    }
}
