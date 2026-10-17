//! Bootstrap 5 components for [GPUI](https://gpui.rs), built on
//! [gpui-kit](https://gpui-kit.com).
//!
//! Behavior (focus, keyboard activation, accessibility, dialogs) comes from
//! `gpui_kit::base`'s unstyled controls; this crate supplies Bootstrap's look
//! through one token [`Theme`] that every component reads at render time.
//!
//! ```ignore
//! use gpui_bootstrap::prelude::*;
//! use gpui_kit::*;
//!
//! gpui_kit::application().with_assets(gpui_kit::assets::Assets).run(|cx| {
//!     gpui_bootstrap::init(cx);
//!     // Re-theme: any token, or start from `Theme::dark()`.
//!     let mut theme = Theme::light();
//!     theme.colors.primary = rgb(0x6f42c1).into();
//!     theme.apply(cx);
//!     // ...open a window and render `Button::new("save").label("Save")`.
//! });
//! ```

pub mod alert;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod close_button;
pub mod form;
pub mod list_group;
pub mod modal;
pub mod nav;
pub mod pagination;
pub mod platform;
pub mod progress;
pub mod spinner;
pub mod text;
pub mod theme;

pub use platform::Platform;
pub use theme::{
    ActiveTheme, ButtonColors, ColorMode, ControlSize, ControlSizes, Radii, Size, Theme,
    ThemeColors, Typography, Variant, VariantColors,
};

use gpui_kit::App;

/// Initializes gpui-kit and installs the light [`Theme`] unless one is
/// already set. Call it before opening windows.
pub fn init(cx: &mut App) {
    gpui_kit::init(cx);
    if !cx.has_global::<Theme>() {
        Theme::light().apply(cx);
    }
}

/// Every component and the theme types.
pub mod prelude {
    pub use crate::alert::Alert;
    pub use crate::badge::Badge;
    pub use crate::breadcrumb::Breadcrumb;
    pub use crate::button::{Button, ButtonGroup, ButtonStyle};
    pub use crate::card::Card;
    pub use crate::close_button::CloseButton;
    pub use crate::form::{Checkbox, FormControl, FormLabel, FormText, Radio, Switch};
    pub use crate::list_group::{ListGroup, ListGroupItem};
    pub use crate::modal::{Modal, ModalSize};
    pub use crate::nav::{Nav, NavItem, NavStyle};
    pub use crate::pagination::Pagination;
    pub use crate::platform::Platform;
    pub use crate::progress::{Progress, ProgressBar};
    pub use crate::spinner::{Spinner, SpinnerStyle};
    pub use crate::text::{Heading, Lead, Muted, body};
    pub use crate::theme::{ActiveTheme as _, ColorMode, Size, Theme, Variant};
    pub use gpui_kit::base::DialogHandle;
    pub use gpui_kit::component::input::InputState;
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{TestAppContext, rgb};

    #[test]
    fn update_rethemes_and_syncs_gpui_kit() {
        let cx = TestAppContext::single();
        cx.update(|cx| {
            init(cx);
            Theme::update(cx, |theme| {
                *theme = Theme::dark();
                theme.colors.primary = rgb(0x6f42c1).into();
            });
            let theme = Theme::global(cx);
            assert_eq!(theme.mode, ColorMode::Dark);
            let kit = gpui_kit::component::Theme::global(cx);
            assert!(kit.mode.is_dark());
            assert_eq!(kit.primary, theme.colors.primary);
            assert_eq!(kit.background, theme.colors.body_bg);
        });
    }
}
