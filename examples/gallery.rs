//! Every gpui-bootstrap component, with live theme switching.
//!
//! - `cargo run --example gallery` opens the gallery.
//! - `cargo run --example gallery -- --screenshot out.png [--dark] [--theme brand|flat]`
//!   renders it headlessly to a PNG.

use gpui_bootstrap::{
    prelude::*,
    theme::{ColorMode, Theme},
};
use gpui_kit::{base::h_flex, base::v_flex, prelude::FluentBuilder as _, *};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Preset {
    Bootstrap,
    Brand,
    Flat,
}

impl Preset {
    fn name(self) -> &'static str {
        match self {
            Preset::Bootstrap => "Bootstrap",
            Preset::Brand => "Brand",
            Preset::Flat => "Flat",
        }
    }

    /// How an application re-themes: start from a mode and override tokens.
    fn theme(self, mode: ColorMode) -> Theme {
        let mut theme = Theme::light().with_mode(mode);
        match self {
            Preset::Bootstrap => {}
            Preset::Brand => {
                theme.colors.primary = rgb(0x6f42c1).into();
                theme.colors.success = rgb(0x20c997).into();
                theme.colors.danger = rgb(0xd63384).into();
                theme.radius.sm = px(6.);
                theme.radius.base = px(10.);
                theme.radius.lg = px(14.);
                theme = theme.with_mode(mode); // re-derive link colors
            }
            Preset::Flat => {
                theme.radius.sm = px(0.);
                theme.radius.base = px(0.);
                theme.radius.lg = px(0.);
                theme.typography.font_size_base = px(14.);
                theme.typography.font_size_sm = px(12.);
                theme.typography.font_size_lg = px(17.);
            }
        }
        theme
    }
}

struct Gallery {
    preset: Preset,
    mode: ColorMode,
    tab: usize,
    page: usize,
    remember: bool,
    terms: bool,
    plan: usize,
    notifications: bool,
    alert_open: bool,
    selected_item: usize,
    email: Entity<InputState>,
    modal: DialogHandle,
}

impl Gallery {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            preset: Preset::Bootstrap,
            mode: ColorMode::Light,
            tab: 0,
            page: 3,
            remember: true,
            terms: false,
            plan: 1,
            notifications: true,
            alert_open: true,
            selected_item: 1,
            email: cx.new(|cx| InputState::new(window, cx).placeholder("name@example.com")),
            modal: DialogHandle::new(false),
        }
    }

    fn set_theme(&mut self, preset: Preset, mode: ColorMode, cx: &mut Context<Self>) {
        self.preset = preset;
        self.mode = mode;
        preset.theme(mode).apply(cx);
        cx.notify();
    }
}

fn section(title: &'static str, content: impl IntoElement) -> impl IntoElement {
    v_flex().gap_3().child(Heading::h4(title)).child(content)
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        let toggle = |update: fn(&mut Gallery, bool)| {
            let view = view.clone();
            move |checked: bool, _: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    update(this, checked);
                    cx.notify();
                })
            }
        };

        let header = h_flex()
            .justify_between()
            .flex_wrap()
            .gap_4()
            .child(
                v_flex()
                    .gap_1()
                    .child(Heading::h1("gpui-bootstrap"))
                    .child(Lead::new(
                        "Bootstrap 5 components for GPUI, built on gpui-kit.",
                    )),
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(Nav::new("presets").pills().items(
                        [Preset::Bootstrap, Preset::Brand, Preset::Flat].map(|preset| {
                            NavItem::new(preset.name(), preset.name())
                                .active(self.preset == preset)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.set_theme(preset, this.mode, cx)
                                }))
                        }),
                    ))
                    .child(
                        Switch::new("dark")
                            .label("Dark mode")
                            .checked(self.mode == ColorMode::Dark)
                            .on_change({
                                let view = view.clone();
                                move |dark, _, cx| {
                                    view.update(cx, |this, cx| {
                                        let mode = if dark {
                                            ColorMode::Dark
                                        } else {
                                            ColorMode::Light
                                        };
                                        this.set_theme(this.preset, mode, cx)
                                    })
                                }
                            }),
                    ),
            );

        let buttons =
            v_flex()
                .gap_3()
                .child(
                    h_flex()
                        .flex_wrap()
                        .gap_2()
                        .children(Variant::ALL.map(|variant| {
                            Button::new(("solid", variant as usize))
                                .label(capitalize(variant.name()))
                                .variant(variant)
                        }))
                        .child(Button::new("link").label("Link").link()),
                )
                .child(
                    h_flex()
                        .flex_wrap()
                        .gap_2()
                        .children(Variant::ALL.map(|variant| {
                            Button::new(("outline", variant as usize))
                                .label(capitalize(variant.name()))
                                .variant(variant)
                                .outline()
                        })),
                )
                .child(
                    h_flex()
                        .flex_wrap()
                        .gap_2()
                        .child(Button::new("large").label("Large").large())
                        .child(
                            Button::new("small")
                                .label("Small")
                                .small()
                                .variant(Variant::Secondary),
                        )
                        .child(Button::new("active").label("Active").active(true))
                        .child(Button::new("disabled").label("Disabled").disabled(true))
                        .child(
                            Button::new("loading")
                                .variant(Variant::Primary)
                                .disabled(true)
                                .child(Spinner::new().small())
                                .child("Loading…"),
                        )
                        .child(
                            ButtonGroup::new("group").buttons(["Left", "Middle", "Right"].map(
                                |label| {
                                    Button::new(label)
                                        .label(label)
                                        .variant(Variant::Primary)
                                        .outline()
                                        .active(label == "Middle")
                                },
                            )),
                        )
                        .child(Button::new("open-modal").label("Launch modal").on_click(
                            cx.listener(|this, _, window, cx| this.modal.open(window, cx)),
                        )),
                );

        let badges = v_flex()
            .gap_2()
            .child(h_flex().flex_wrap().gap_2().children(
                Variant::ALL.map(|variant| Badge::new(capitalize(variant.name())).variant(variant)),
            ))
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_2()
                    .children(Variant::ALL.map(|variant| {
                        Badge::new(capitalize(variant.name()))
                            .variant(variant)
                            .pill(true)
                            .subtle(true)
                    })),
            );

        let alerts = v_flex()
            .gap_2()
            .when(self.alert_open, |this| {
                this.child(
                    Alert::new("dismissible")
                        .variant(Variant::Warning)
                        .heading("Holy guacamole!")
                        .child("You should check in on some of those fields below.")
                        .on_dismiss(cx.listener(|this, _, _, cx| {
                            this.alert_open = false;
                            cx.notify();
                        })),
                )
            })
            .children(
                [
                    Variant::Primary,
                    Variant::Success,
                    Variant::Danger,
                    Variant::Dark,
                ]
                .map(|variant| {
                    Alert::new(("alert", variant as usize))
                        .variant(variant)
                        .child(format!("A simple {} alert—check it out!", variant.name()))
                }),
            );

        let tabs = ["Active", "Link", "Another link"];
        let navs = v_flex()
            .gap_4()
            .child(
                Breadcrumb::new("crumbs")
                    .item("Home", |_, _, _| {})
                    .item("Library", |_, _, _| {})
                    .active("Data"),
            )
            .children(
                [NavStyle::Tabs, NavStyle::Pills, NavStyle::Underline].map(|style| {
                    Nav::new(("nav", style as usize)).nav_style(style).items(
                        tabs.iter()
                            .enumerate()
                            .map(|(index, label)| {
                                NavItem::new(*label, *label)
                                    .active(self.tab == index)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.tab = index;
                                        cx.notify();
                                    }))
                            })
                            .chain([NavItem::new("disabled", "Disabled").disabled(true)]),
                    )
                }),
            )
            .child(Pagination::new("pages", self.page, 12).on_change({
                let view = view.clone();
                move |page, _, cx| {
                    view.update(cx, |this, cx| {
                        this.page = page;
                        cx.notify();
                    })
                }
            }));

        let form = v_flex()
            .gap_3()
            .max_w(px(480.))
            .child(
                v_flex()
                    .child(FormLabel::new("Email address"))
                    .child(FormControl::new(&self.email))
                    .child(FormText::new(
                        "We'll never share your email with anyone else.",
                    )),
            )
            .child(
                Checkbox::new("remember")
                    .label("Remember me")
                    .checked(self.remember)
                    .on_change(toggle(|this, checked| this.remember = checked)),
            )
            .child(
                Checkbox::new("terms")
                    .label("I agree to the terms")
                    .checked(self.terms)
                    .on_change(toggle(|this, checked| this.terms = checked)),
            )
            .child(
                Checkbox::new("disabled-check")
                    .label("Disabled")
                    .disabled(true),
            )
            .child(
                v_flex()
                    .gap_1()
                    .children(["Free", "Pro", "Enterprise"].iter().enumerate().map(
                        |(index, label)| {
                            let view = view.clone();
                            Radio::new(("plan", index))
                                .label(*label)
                                .checked(self.plan == index)
                                .on_change(move |_, _, cx| {
                                    view.update(cx, |this, cx| {
                                        this.plan = index;
                                        cx.notify();
                                    })
                                })
                        },
                    )),
            )
            .child(
                Switch::new("notifications")
                    .label("Email notifications")
                    .checked(self.notifications)
                    .on_change(toggle(|this, checked| this.notifications = checked)),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("submit").label("Submit"))
                    .child(
                        Button::new("cancel")
                            .label("Cancel")
                            .variant(Variant::Secondary)
                            .outline(),
                    ),
            );

        let surfaces = h_flex()
            .items_start()
            .flex_wrap()
            .gap_4()
            .child(
                Card::new()
                    .w(px(300.))
                    .header("Featured")
                    .title("Card title")
                    .subtitle("Card subtitle")
                    .child("Some quick example text to build on the card title.")
                    .child(h_flex().child(Button::new("card-go").label("Go somewhere")))
                    .footer(Muted::new("2 days ago")),
            )
            .child(
                ListGroup::new("list").w(px(300.)).items(
                    ["An item", "A second item", "A third item", "A fourth item"]
                        .iter()
                        .enumerate()
                        .map(|(index, label)| {
                            ListGroupItem::new(("item", index))
                                .label(*label)
                                .active(self.selected_item == index)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.selected_item = index;
                                    cx.notify();
                                }))
                        })
                        .chain([ListGroupItem::new("item-disabled")
                            .label("A disabled item")
                            .disabled(true)]),
                ),
            )
            .child(
                v_flex()
                    .w(px(300.))
                    .gap_3()
                    .child(Progress::new("p1").value(25.))
                    .child(
                        Progress::new("p2")
                            .bar(ProgressBar::new(60.).variant(Variant::Success).label("60%")),
                    )
                    .child(
                        Progress::new("p3")
                            .bar(ProgressBar::new(15.))
                            .bar(ProgressBar::new(30.).variant(Variant::Success))
                            .bar(ProgressBar::new(20.).variant(Variant::Info)),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .children(
                                [Variant::Primary, Variant::Success, Variant::Danger]
                                    .map(|variant| Spinner::new().variant(variant)),
                            )
                            .child(Spinner::new().grow().variant(Variant::Warning)),
                    ),
            );

        body(cx)
            .child(
                div().id("scroll").size_full().overflow_y_scroll().child(
                    v_flex()
                        .p_8()
                        .gap_8()
                        .child(header)
                        .child(section("Buttons", buttons))
                        .child(section("Badges", badges))
                        .child(section("Alerts", alerts))
                        .child(section("Navigation", navs))
                        .child(section("Cards, list groups, progress", surfaces))
                        .child(section("Forms", form)),
                ),
            )
            .child(
                Modal::new(&self.modal)
                    .title("Modal title")
                    .child("Woo-hoo, you're reading this text in a modal!")
                    .footer(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("modal-close")
                                    .label("Close")
                                    .variant(Variant::Secondary)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.modal.close(window, cx)
                                    })),
                            )
                            .child(Button::new("modal-save").label("Save changes")),
                    ),
            )
    }
}

fn capitalize(name: &str) -> String {
    let mut chars = name.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = if args.iter().any(|arg| arg == "--dark") {
        ColorMode::Dark
    } else {
        ColorMode::Light
    };
    let preset = match args
        .iter()
        .position(|arg| arg == "--theme")
        .and_then(|index| args.get(index + 1))
        .map(String::as_str)
    {
        Some("brand") => Preset::Brand,
        Some("flat") => Preset::Flat,
        _ => Preset::Bootstrap,
    };
    if let Some(path) = args
        .iter()
        .position(|arg| arg == "--screenshot")
        .and_then(|index| args.get(index + 1))
    {
        screenshot(path, preset, mode).expect("screenshot");
        return;
    }

    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx| {
            gpui_bootstrap::init(cx);
            preset.theme(mode).apply(cx);
            let bounds = Bounds::centered(None, size(px(1200.), px(900.)), cx);
            gpui_kit::open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                cx,
                |window, cx| {
                    cx.new(|cx| {
                        let mut gallery = Gallery::new(window, cx);
                        gallery.preset = preset;
                        gallery.mode = mode;
                        gallery
                    })
                },
            )
            .expect("open window");
            cx.activate(true);
        });
}

/// Renders the gallery once, headlessly, to a PNG.
fn screenshot(path: &str, preset: Preset, mode: ColorMode) -> anyhow::Result<()> {
    use std::sync::Arc;

    let mut cx = HeadlessAppContext::with_platform(
        gpui_kit::platform::current_platform(true).text_system(),
        Arc::new(gpui_kit::assets::Assets),
        gpui_kit::platform::current_headless_renderer,
    );
    cx.update(|cx| {
        gpui_bootstrap::init(cx);
        preset.theme(mode).apply(cx);
    });
    let window = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(1200.), px(2150.)),
                })),
                focus: false,
                show: false,
                ..Default::default()
            },
            cx,
            |window, cx| {
                cx.new(|cx| {
                    let mut gallery = Gallery::new(window, cx);
                    gallery.preset = preset;
                    gallery.mode = mode;
                    gallery
                })
            },
        )
    })?;
    let (handle, _) = window;
    for _ in 0..3 {
        cx.run_until_parked();
        cx.update_window(handle, |_, window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
        })?;
    }
    cx.capture_screenshot(handle)?.save(path)?;
    Ok(())
}
