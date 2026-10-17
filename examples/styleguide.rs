//! The gpui-bootstrap styleguide, built with gpui-styleguidist.
//!
//! - `cargo run --example styleguide` opens it natively.
//! - `cargo styleguide serve` builds and serves the wasm version.
//! - `cargo run --example styleguide -- render` re-renders the snapshots under
//!   `styleguide/` that rustdoc embeds; `-- check` compares against them.

use gpui_bootstrap::{ColorMode, Platform, Size, Theme, prelude::*};
use gpui_kit::{
    base::{h_flex, v_flex},
    prelude::FluentBuilder as _,
    *,
};
use gpui_styleguidist::{Styleguide, example};

/// The hosted web styleguide that rustdoc previews link to.
const PLAYGROUND_URL: &str = "https://ziinc.github.io/gpui-bootstrap/";

fn main() {
    gpui_styleguidist::main(styleguide());
}

/// Theme names, `<platform>-<mode>`, in the order the theme picker lists them.
fn themes() -> Vec<String> {
    // Snapshots cover stock Bootstrap only: every extra theme multiplies the
    // committed PNGs. The live styleguide offers every platform.
    let snapshot = std::env::args()
        .nth(1)
        .is_some_and(|command| command == "render" || command == "check");
    let platforms: &[Platform] = if snapshot {
        &[Platform::Web]
    } else {
        &Platform::ALL
    };
    platforms
        .iter()
        .flat_map(|platform| {
            ["light", "dark"].map(|mode| format!("{}-{mode}", theme_slug(*platform)))
        })
        .collect()
}

fn theme_slug(platform: Platform) -> &'static str {
    match platform {
        Platform::Web => "bootstrap",
        Platform::MacOS => "macos",
        Platform::Windows => "windows",
        Platform::Linux => "linux",
        Platform::IOS => "ios",
        Platform::Android => "android",
    }
}

fn apply_theme(name: &str, cx: &mut App) {
    let (platform, mode) = name.rsplit_once('-').unwrap_or((name, "light"));
    let platform = Platform::ALL
        .into_iter()
        .find(|candidate| theme_slug(*candidate) == platform)
        .unwrap_or(Platform::Web);
    let mode = if mode == "dark" {
        ColorMode::Dark
    } else {
        ColorMode::Light
    };
    Theme::platform(platform, mode).apply(cx);
}

#[cfg(not(target_family = "wasm"))]
fn assets() -> gpui_kit::assets::Assets {
    gpui_kit::assets::Assets
}

/// On the web, gpui-kit fetches icons on demand; this uses the copies its own
/// gallery publishes.
#[cfg(target_family = "wasm")]
fn assets() -> gpui_kit::assets::Assets {
    gpui_kit::assets::Assets::new("https://gpui-kit.com/gallery/")
}

fn styleguide() -> Styleguide {
    Styleguide::new("gpui-bootstrap")
        .assets(assets())
        .setup(gpui_bootstrap::init)
        .open_window(|options, frame, cx| Ok(gpui_kit::open_window(options, cx, frame)?.0))
        .themes(themes(), apply_theme)
        .background(|cx| cx.theme().colors.body_bg)
        .playground_url(PLAYGROUND_URL)
        // Past this, snapshots cover the default plus each single-knob change,
        // which keeps the committed PNGs small.
        .variant_limit(8)
        .describe(
            "Button",
            "`.btn`. Solid, outline and link styles in every theme color and three sizes. \
             Use one primary button per surface.",
        )
        .describe(
            "ButtonGroup",
            "`.btn-group`. Joins buttons into one control with shared outer corners.",
        )
        .describe(
            "CloseButton",
            "`.btn-close`. Dismisses alerts, modals and toasts. Use `white` on dark fills.",
        )
        .describe(
            "Badge",
            "`.badge`. A count or label. `subtle` uses the theme color's tinted background.",
        )
        .describe(
            "Alert",
            "`.alert`. Contextual feedback with an optional heading. Passing `on_dismiss` adds \
             a close button; the application removes the alert.",
        )
        .describe(
            "Card",
            "`.card`. A bordered container with an optional header, title, subtitle and footer.",
        )
        .describe(
            "ListGroup",
            "`.list-group`. A list of items, optionally interactive, active or tinted.",
        )
        .describe(
            "Progress",
            "`.progress`. One bar, or several stacked bars that share the track.",
        )
        .describe("Spinner", "`.spinner-border` and `.spinner-grow`.")
        .describe(
            "Breadcrumb",
            "`.breadcrumb`. The path to the current page; the last item is active.",
        )
        .describe(
            "Nav",
            "`.nav`. Links, tabs, pills or underline. The application tracks the active item.",
        )
        .describe(
            "Pagination",
            "`.pagination`. Previous, pages with ellipses, next. The application stores the page.",
        )
        .describe(
            "Modal",
            "`.modal`. Focus trapping, Escape and the backdrop come from gpui-kit's Dialog. Keep \
             a DialogHandle in your view and render the modal every frame.",
        )
        .describe(
            "FormControl",
            "`.form-control`. gpui-kit's text input with Bootstrap's frame, sizes and validity.",
        )
        .describe(
            "Checks",
            "`.form-check`: Checkbox, Radio and Switch. The application stores the checked state \
             from `on_change`.",
        )
        .describe(
            "Typography",
            "Headings, `.lead` and `.text-body-secondary`, from the theme's type scale.",
        )
        .examples(buttons())
        .examples(indicators())
        .examples(surfaces())
        .examples(navigation())
        .examples(overlays_and_forms())
}

fn buttons() -> Vec<gpui_styleguidist::Example> {
    vec![
        example!("button/playground", |knobs, _, _| {
            Button::new("button")
                .label(knobs.text_samples("label", &["Save changes", "Delete"]))
                .variant(knobs.pick(
                    "variant",
                    &[
                        ("primary", Variant::Primary),
                        ("secondary", Variant::Secondary),
                        ("success", Variant::Success),
                        ("info", Variant::Info),
                        ("warning", Variant::Warning),
                        ("danger", Variant::Danger),
                        ("light", Variant::Light),
                        ("dark", Variant::Dark),
                    ],
                    "primary",
                ))
                .button_style(knobs.pick(
                    "style",
                    &[
                        ("solid", ButtonStyle::Solid),
                        ("outline", ButtonStyle::Outline),
                        ("link", ButtonStyle::Link),
                    ],
                    "solid",
                ))
                .with_size(knobs.pick(
                    "size",
                    &[
                        ("small", Size::Small),
                        ("medium", Size::Medium),
                        ("large", Size::Large),
                    ],
                    "medium",
                ))
                .active(knobs.bool("active", false))
                .disabled(knobs.bool("disabled", false))
        })
        .component("Button")
        .title("Playground")
        .description("Every builder as a knob."),
        example!("button/variants", |knobs, _, _| {
            let style = knobs.pick(
                "style",
                &[
                    ("solid", ButtonStyle::Solid),
                    ("outline", ButtonStyle::Outline),
                ],
                "solid",
            );
            h_flex()
                .flex_wrap()
                .gap_2()
                .children(Variant::ALL.map(|variant| {
                    Button::new(variant as usize)
                        .label(variant.name())
                        .variant(variant)
                        .button_style(style)
                }))
        })
        .component("Button")
        .title("Theme colors"),
        example!("button-group/basic", |knobs, _, _| {
            ButtonGroup::new("group")
                .with_size(knobs.pick(
                    "size",
                    &[
                        ("small", Size::Small),
                        ("medium", Size::Medium),
                        ("large", Size::Large),
                    ],
                    "medium",
                ))
                .button(Button::new("left").label("Left").outline())
                .button(Button::new("middle").label("Middle").outline().active(true))
                .button(Button::new("right").label("Right").outline())
        })
        .component("ButtonGroup")
        .title("Outline group"),
        example!("close-button/basic", |knobs, _, cx| {
            let white = knobs.bool("white", false);
            div()
                .p_3()
                .rounded_md()
                .when(white, |this| this.bg(cx.theme().colors.dark))
                .child(
                    CloseButton::new("close")
                        .white(white)
                        .disabled(knobs.bool("disabled", false)),
                )
        })
        .component("CloseButton")
        .title("Close button"),
    ]
}

fn indicators() -> Vec<gpui_styleguidist::Example> {
    vec![
        example!("badge/playground", |knobs, _, _| {
            Badge::new(knobs.text_samples("label", &["New", "99+"]))
                .variant(knobs.pick(
                    "variant",
                    &[
                        ("primary", Variant::Primary),
                        ("secondary", Variant::Secondary),
                        ("success", Variant::Success),
                        ("info", Variant::Info),
                        ("warning", Variant::Warning),
                        ("danger", Variant::Danger),
                        ("light", Variant::Light),
                        ("dark", Variant::Dark),
                    ],
                    "primary",
                ))
                .pill(knobs.bool("pill", false))
                .subtle(knobs.bool("subtle", false))
        })
        .component("Badge")
        .title("Playground"),
        example!("badge/variants", |knobs, _, _| {
            let subtle = knobs.bool("subtle", false);
            h_flex().gap_2().children(
                Variant::ALL
                    .map(|variant| Badge::new(variant.name()).variant(variant).subtle(subtle)),
            )
        })
        .component("Badge")
        .title("Theme colors"),
        example!("alert/playground", |knobs, _, _| {
            Alert::new("alert")
                .w(px(480.))
                .variant(knobs.pick(
                    "variant",
                    &[
                        ("primary", Variant::Primary),
                        ("secondary", Variant::Secondary),
                        ("success", Variant::Success),
                        ("info", Variant::Info),
                        ("warning", Variant::Warning),
                        ("danger", Variant::Danger),
                        ("light", Variant::Light),
                        ("dark", Variant::Dark),
                    ],
                    "success",
                ))
                .heading(knobs.text("heading", "Well done!"))
                .when(knobs.bool("dismissible", true), |this| {
                    this.on_dismiss(|_, _, _| {})
                })
                .child(knobs.text_samples(
                    "message",
                    &[
                        "You successfully read this important alert message.",
                        "Aww yeah, this longer message wraps onto a second line so you can see \
                         how the alert keeps its padding and close button.",
                    ],
                ))
        })
        .component("Alert")
        .title("Playground"),
        example!("alert/variants", |_, _, _| {
            v_flex()
                .w(px(420.))
                .gap_2()
                .children(Variant::ALL.map(|variant| {
                    Alert::new(variant as usize)
                        .variant(variant)
                        .child(format!("A simple {} alert.", variant.name()))
                }))
        })
        .component("Alert")
        .title("Theme colors"),
        example!("progress/playground", |knobs, _, _| {
            let value = knobs.number("value", 60., 0.0..=100., 5.);
            let bar = ProgressBar::new(value as f32).variant(knobs.pick(
                "variant",
                &[
                    ("primary", Variant::Primary),
                    ("success", Variant::Success),
                    ("info", Variant::Info),
                    ("warning", Variant::Warning),
                    ("danger", Variant::Danger),
                ],
                "primary",
            ));
            let bar = if knobs.bool("label", false) {
                bar.label(format!("{value}%"))
            } else {
                bar
            };
            Progress::new("progress").w(px(360.)).bar(bar)
        })
        .component("Progress")
        .title("Playground"),
        example!("progress/stacked", |_, _, _| {
            Progress::new("stacked")
                .w(px(360.))
                .bar(ProgressBar::new(15.))
                .bar(ProgressBar::new(30.).variant(Variant::Success))
                .bar(ProgressBar::new(20.).variant(Variant::Info))
        })
        .component("Progress")
        .title("Stacked bars"),
        example!("spinner/playground", |knobs, _, _| {
            Spinner::new()
                .spinner_style(knobs.pick(
                    "style",
                    &[
                        ("border", SpinnerStyle::Border),
                        ("grow", SpinnerStyle::Grow),
                    ],
                    "border",
                ))
                .variant(knobs.pick(
                    "variant",
                    &[
                        ("primary", Variant::Primary),
                        ("secondary", Variant::Secondary),
                        ("success", Variant::Success),
                        ("danger", Variant::Danger),
                    ],
                    "primary",
                ))
                .with_size(knobs.pick(
                    "size",
                    &[("small", Size::Small), ("medium", Size::Medium)],
                    "medium",
                ))
        })
        .component("Spinner")
        .title("Playground"),
    ]
}

fn surfaces() -> Vec<gpui_styleguidist::Example> {
    vec![
        example!("card/basic", |knobs, _, _| {
            Card::new()
                .w(px(288.))
                .when(knobs.bool("header", true), |card| card.header("Featured"))
                .title(knobs.text("title", "Card title"))
                .subtitle(knobs.text("subtitle", "Card subtitle"))
                .child(
                    "Some quick example text to build on the card title and make up the bulk of \
                     the card's content.",
                )
                .when(knobs.bool("footer", false), |card| {
                    card.footer(Muted::new("Last updated 3 mins ago"))
                })
                .when_some(
                    knobs.pick(
                        "border",
                        &[
                            ("none", None),
                            ("primary", Some(Variant::Primary)),
                            ("success", Some(Variant::Success)),
                            ("danger", Some(Variant::Danger)),
                        ],
                        "none",
                    ),
                    Card::border_variant,
                )
        })
        .component("Card")
        .title("Card"),
        example!("list-group/basic", |knobs, _, _| {
            let active = knobs.pick(
                "active",
                &[("none", None), ("first", Some(0)), ("second", Some(1))],
                "second",
            );
            ListGroup::new("list")
                .w(px(288.))
                .flush(knobs.bool("flush", false))
                .items(
                    ["An item", "A second item", "A third item"]
                        .into_iter()
                        .enumerate()
                        .map(|(index, label)| {
                            ListGroupItem::new(index)
                                .label(label)
                                .active(active == Some(index))
                                .on_click(|_, _, _| {})
                        }),
                )
                .item(
                    ListGroupItem::new("disabled")
                        .label("A disabled item")
                        .disabled(true),
                )
                .item(
                    ListGroupItem::new("success")
                        .label("A success item")
                        .variant(Variant::Success),
                )
        })
        .component("ListGroup")
        .title("Interactive list"),
        example!("typography/scale", |_, _, _| {
            v_flex()
                .w(px(420.))
                .gap_1()
                .children((1..=6).map(|level| Heading::new(level, format!("Heading {level}"))))
                .child(Lead::new(
                    "A lead paragraph stands out from the regular body text.",
                ))
                .child(Muted::new("Secondary body text for captions and help."))
        })
        .component("Typography")
        .title("Type scale"),
    ]
}

fn navigation() -> Vec<gpui_styleguidist::Example> {
    vec![
        example!("nav/playground", |knobs, _, _| {
            Nav::new("nav")
                .nav_style(knobs.pick(
                    "style",
                    &[
                        ("links", NavStyle::Links),
                        ("tabs", NavStyle::Tabs),
                        ("pills", NavStyle::Pills),
                        ("underline", NavStyle::Underline),
                    ],
                    "tabs",
                ))
                .fill(knobs.bool("fill", false))
                .w(px(480.))
                .item(NavItem::new("active", "Active").active(true))
                .item(NavItem::new("link", "Link"))
                .item(NavItem::new("another", "Another link"))
                .item(NavItem::new("disabled", "Disabled").disabled(true))
        })
        .component("Nav")
        .title("Playground"),
        example!("breadcrumb/basic", |knobs, _, _| {
            Breadcrumb::new("breadcrumb")
                .divider(knobs.text_samples("divider", &["/", ">"]))
                .item("Home", |_, _, _| {})
                .item("Library", |_, _, _| {})
                .active("Data")
        })
        .component("Breadcrumb")
        .title("Breadcrumb"),
        example!("pagination/playground", |knobs, _, _| {
            Pagination::new(
                "pagination",
                knobs.number("page", 4., 1.0..=20., 1.) as usize,
                knobs.number("pages", 20., 1.0..=50., 1.) as usize,
            )
            .with_size(knobs.pick(
                "size",
                &[
                    ("small", Size::Small),
                    ("medium", Size::Medium),
                    ("large", Size::Large),
                ],
                "medium",
            ))
            .disabled(knobs.bool("disabled", false))
        })
        .component("Pagination")
        .title("Playground"),
    ]
}

/// An open [`Modal`] over its trigger.
struct ModalDemo {
    handle: DialogHandle,
    size: ModalSize,
    centered: bool,
}

impl Render for ModalDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .p_4()
            .items_start()
            .child(
                Button::new("open")
                    .label("Launch demo modal")
                    .on_click(cx.listener(|this, _, window, cx| this.handle.open(window, cx))),
            )
            .child(
                Modal::new(&self.handle)
                    .title("Modal title")
                    .with_size(self.size)
                    .centered(self.centered)
                    .child("Woo-hoo, you're reading this text in a modal!")
                    .footer(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("close")
                                    .label("Close")
                                    .variant(Variant::Secondary)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.handle.close(window, cx)
                                    })),
                            )
                            .child(Button::new("save").label("Save changes")),
                    ),
            )
    }
}

/// A labelled [`FormControl`] that owns its [`InputState`].
struct FormControlDemo {
    input: Entity<InputState>,
    size: Size,
    disabled: bool,
    validity: Option<bool>,
}

impl Render for FormControlDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(px(360.))
            .gap_2()
            .child(FormLabel::new("Email address"))
            .child(
                FormControl::new(&self.input)
                    .with_size(self.size)
                    .disabled(self.disabled)
                    .when_some(self.validity, FormControl::validity),
            )
            .child(match self.validity {
                Some(false) => FormText::new("Please enter a valid email.").validity(false),
                Some(true) => FormText::new("Looks good!").validity(true),
                None => FormText::new("We'll never share your email with anyone else."),
            })
    }
}

fn overlays_and_forms() -> Vec<gpui_styleguidist::Example> {
    vec![
        example!(view "modal/basic", |knobs, _, cx| {
            cx.new(|_| ModalDemo {
                handle: DialogHandle::new(knobs.bool("open", true)),
                size: knobs.pick(
                    "size",
                    &[
                        ("small", ModalSize::Small),
                        ("medium", ModalSize::Medium),
                    ],
                    "small",
                ),
                centered: knobs.bool("centered", false),
            })
        })
        .component("Modal")
        .title("Modal")
        .description("Press Escape or the backdrop to close it, then the button to reopen it.")
        .size(640., 360.),
        example!(view "form-control/basic", |knobs, window, cx| {
            cx.new(|cx| FormControlDemo {
                input: cx.new(|cx| {
                    InputState::new(window, cx)
                        .placeholder("name@example.com")
                        .default_value(knobs.text_samples("value", &["", "mark@example.com"]))
                }),
                size: knobs.pick(
                    "size",
                    &[
                        ("small", Size::Small),
                        ("medium", Size::Medium),
                        ("large", Size::Large),
                    ],
                    "medium",
                ),
                disabled: knobs.bool("disabled", false),
                validity: knobs.pick(
                    "validity",
                    &[
                        ("none", None),
                        ("valid", Some(true)),
                        ("invalid", Some(false)),
                    ],
                    "none",
                ),
            })
        })
        .component("FormControl")
        .title("Labelled field")
        .description("Click it and type. Changing a knob rebuilds the view and its InputState."),
        example!("checks/playground", |knobs, _, _| {
            let checked = knobs.bool("checked", true);
            let disabled = knobs.bool("disabled", false);
            v_flex()
                .gap_2()
                .child(
                    Checkbox::new("checkbox")
                        .label("Checkbox")
                        .checked(checked)
                        .indeterminate(knobs.bool("indeterminate", false))
                        .disabled(disabled),
                )
                .child(
                    Radio::new("radio")
                        .label("Radio")
                        .checked(checked)
                        .disabled(disabled),
                )
                .child(
                    Switch::new("switch")
                        .label("Switch")
                        .checked(checked)
                        .disabled(disabled),
                )
        })
        .component("Checks")
        .title("Checkbox, radio and switch"),
    ]
}
