# gpui-bootstrap

Bootstrap 5 components for [GPUI](https://gpui.rs), built on
[gpui-kit](https://gpui-kit.com).

Behavior (focus, keyboard activation, accessibility, modal focus trapping)
comes from gpui-kit's unstyled `gpui_kit::base` controls. This crate supplies
Bootstrap 5.3's look through one token `Theme` that every component reads at
render time, so re-theming is a matter of changing a few values.

| Light | Dark |
| --- | --- |
| ![Light](docs/light.png) | ![Dark](docs/dark.png) |
| **Re-themed: brand colors and radii** | **Re-themed: flat, compact, dark** |
| ![Brand](docs/brand.png) | ![Flat](docs/flat.png) |

## Components

| Bootstrap | gpui-bootstrap |
| --- | --- |
| `.btn`, `.btn-outline-*`, `.btn-link`, `.btn-group` | `Button`, `ButtonGroup` |
| `.btn-close` | `CloseButton` |
| `.badge` | `Badge` |
| `.alert` (dismissible) | `Alert` |
| `.card` | `Card` |
| `.list-group` | `ListGroup`, `ListGroupItem` |
| `.progress`, `.progress-stacked` | `Progress`, `ProgressBar` |
| `.spinner-border`, `.spinner-grow` | `Spinner` |
| `.breadcrumb` | `Breadcrumb` |
| `.nav`, `.nav-tabs`, `.nav-pills`, `.nav-underline` | `Nav`, `NavItem` |
| `.pagination` | `Pagination` |
| `.modal` | `Modal` |
| `.form-control`, `.form-label`, `.form-text` | `FormControl`, `FormLabel`, `FormText` |
| `.form-check` (checkbox, radio, switch) | `Checkbox`, `Radio`, `Switch` |
| Headings, `.lead`, `.text-body-secondary` | `Heading`, `Lead`, `Muted`, `body` |

Every component takes a `Variant` (`Primary`, `Secondary`, `Success`, `Info`,
`Warning`, `Danger`, `Light`, `Dark`) where Bootstrap does, and controls take
`Size::Small` / `Size::Large` for `-sm` / `-lg`.

## Usage

```toml
[dependencies]
gpui-kit = "=0.7.3"
gpui-bootstrap = { git = "https://github.com/Ziinc/gpui-bootstrap" }
```

gpui-kit pins its `gpui-pre` snapshot and re-exports GPUI, so you never list
GPUI yourself. gpui-bootstrap tracks the same gpui-kit version.

```rust
use gpui_bootstrap::prelude::*;
use gpui_kit::*;

struct App;

impl Render for App {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        body(cx).p_4().gap_2().flex().child(
            Button::new("save")
                .label("Save")
                .on_click(|_, _, _| println!("saved")),
        )
        .child(Badge::new("New").variant(Variant::Success).pill(true))
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets) // icons for close, check, spinner
        .run(|cx| {
            gpui_bootstrap::init(cx);
            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| App))
                .unwrap();
        });
}
```

## Theming

`Theme` mirrors Bootstrap's Sass variables. Its fields are public: change the
ones you want and keep the rest. Hover, active and focus colors, the subtle
backgrounds of alerts and badges, emphasis text, and the text color that reads
on each fill are derived from the eight theme colors exactly as Bootstrap's
`tint-color()`, `shade-color()` and `color-contrast()` do, so overriding
`primary` re-themes every primary button, alert, link, checkbox and focus ring.

```rust
use gpui_bootstrap::{ColorMode, Theme};

// Start from a preset and override tokens.
let mut theme = Theme::light();
theme.colors.primary = rgb(0x6f42c1).into();
theme.radius.base = px(10.);
theme.typography.font_size_base = px(14.);
theme.apply(cx);

// Or edit the installed theme in place.
Theme::update(cx, |theme| theme.colors.danger = rgb(0xd63384).into());

// Switch color mode, keeping the theme colors.
Theme::change_mode(ColorMode::Dark, cx);
```

| Token group | Fields |
| --- | --- |
| `colors` | the eight theme colors, `white`, `black`, `body_bg`, `body_color`, `emphasis_color`, `secondary_*`, `tertiary_*`, `border_color`, `border_color_translucent`, `link_color`, `link_hover_color`, `backdrop` |
| `radius` | `sm`, `base`, `lg`, `xl`, `xxl`, `pill` |
| `typography` | font families, `font_size_base/sm/lg`, `line_height`, weights, `headings` |
| scalars | `spacer` (spacing scale via `theme.spacing(0..=5)`), `border_width`, `focus_ring_width`, `focus_ring_opacity`, `disabled_opacity`, `min_contrast_ratio` |

`Theme::apply` also maps the tokens onto gpui-kit's own component theme, so
the text editor inside `FormControl`, scrollbars, and any gpui-kit component
you mix in follow the same colors and mode.

Read the active theme in your own components with `cx.theme()` (trait
`gpui_bootstrap::ActiveTheme`), and use `theme.variant(v)` /
`theme.button(v)` for the derived colors.

## Gallery

```sh
cargo run --example gallery
# Headless PNG (Linux needs Vulkan, e.g. mesa-vulkan-drivers):
cargo run --example gallery -- --screenshot out.png --dark --theme brand
```

## License

MIT
