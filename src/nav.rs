//! `.nav`, `.nav-tabs`, `.nav-pills` and `.nav-underline`.

use std::rc::Rc;

use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
    base::{self, StyledExt as _},
    prelude::FluentBuilder as _,
    px,
};

use crate::ActiveTheme as _;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// The look of a [`Nav`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavStyle {
    /// Plain `.nav` links.
    #[default]
    Links,
    /// `.nav-tabs`.
    Tabs,
    /// `.nav-pills`.
    Pills,
    /// `.nav-underline`.
    Underline,
}

/// A row of navigation links or tabs.
///
/// Tab semantics come from [`gpui_kit::base::Tabs`]; the application owns
/// which item is active and what each one shows.
#[derive(IntoElement)]
pub struct Nav {
    id: ElementId,
    nav_style: NavStyle,
    items: Vec<NavItem>,
    fill: bool,
    style: StyleRefinement,
}

impl Nav {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            nav_style: NavStyle::Links,
            items: Vec::new(),
            fill: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn nav_style(mut self, style: NavStyle) -> Self {
        self.nav_style = style;
        self
    }

    pub fn tabs(self) -> Self {
        self.nav_style(NavStyle::Tabs)
    }

    pub fn pills(self) -> Self {
        self.nav_style(NavStyle::Pills)
    }

    pub fn underline(self) -> Self {
        self.nav_style(NavStyle::Underline)
    }

    /// `.nav-fill`: items share the full width.
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn item(mut self, item: NavItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = NavItem>) -> Self {
        self.items.extend(items);
        self
    }
}

impl Styled for Nav {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// One `.nav-link`.
pub struct NavItem {
    id: ElementId,
    label: SharedString,
    active: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl NavItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            active: false,
            disabled: false,
            on_click: None,
        }
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

impl RenderOnce for Nav {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;
        let nav_style = self.nav_style;
        let count = self.items.len();
        let border_width = theme.border_width;
        let transparent = gpui_kit::transparent_black();

        base::Tabs::new(self.id)
            .flex()
            .flex_wrap()
            .when(nav_style == NavStyle::Tabs, |this| {
                this.border_b(border_width)
                    .border_color(colors.border_color)
            })
            .when(nav_style == NavStyle::Pills, |this| {
                this.gap(theme.spacing(1))
            })
            .when(nav_style == NavStyle::Underline, |this| {
                this.gap(theme.spacer)
            })
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let (link, link_hover) = (colors.link_color, colors.link_hover_color);
                let color = if item.disabled {
                    colors.secondary_color
                } else {
                    match (nav_style, item.active) {
                        (NavStyle::Tabs | NavStyle::Underline, true) => colors.emphasis_color,
                        (NavStyle::Pills, true) => theme.contrast(colors.primary),
                        _ => link,
                    }
                };
                let interactive = !item.disabled && !item.active;
                let hover_border = match nav_style {
                    NavStyle::Tabs => Some(colors.secondary_bg),
                    NavStyle::Underline => Some(link_hover),
                    _ => None,
                };
                base::Tab::new(item.id)
                    .set_position(index + 1, count)
                    .selected(item.active)
                    .disabled(item.disabled)
                    .when(self.fill, |this| this.flex_1())
                    .py(theme.spacing(2))
                    .text_color(color)
                    .map(|this| match nav_style {
                        NavStyle::Links => this.px(theme.spacer),
                        NavStyle::Tabs => this
                            .px(theme.spacer)
                            .mb(-border_width)
                            .border(border_width)
                            .rounded_t(theme.radius.base)
                            .border_color(transparent)
                            .when(item.active, |this| {
                                // The active tab covers the nav's bottom border.
                                this.bg(colors.body_bg)
                                    .border_color(colors.border_color)
                                    .border_b_0()
                                    .pb(theme.spacing(2) + border_width)
                            }),
                        NavStyle::Pills => this
                            .px(theme.spacer)
                            .rounded(theme.radius.base)
                            .when(item.active, |this| this.bg(colors.primary)),
                        NavStyle::Underline => this
                            .border_b(px(2.))
                            .border_color(transparent)
                            .when(item.active, |this| {
                                this.border_color(colors.emphasis_color)
                                    .font_weight(theme.typography.font_weight_semibold)
                            }),
                    })
                    .when(interactive, |this| {
                        this.cursor_pointer().hover(move |style| {
                            let style = style.text_color(link_hover);
                            match hover_border {
                                Some(border) => style.border_color(border),
                                None => style,
                            }
                        })
                    })
                    .when_some(
                        item.on_click.filter(|_| !item.disabled),
                        |this, on_click| {
                            this.on_click(move |event, window, cx| on_click(event, window, cx))
                        },
                    )
                    .child(item.label)
            }))
            .refine_style(&self.style)
    }
}
