#![cfg_attr(not(test), warn(clippy::unwrap_used, clippy::expect_used))]

mod status;
mod ui_helpers;

use gpui::AppContext as _;
use okena_extensions::{ExtensionInstance, ExtensionManifest, ExtensionRegistration};
use std::sync::Arc;

pub fn register() -> ExtensionRegistration {
    ExtensionRegistration {
        manifest: ExtensionManifest {
            id: "github",
            name: "GitHub",
            default_enabled: false,
        },
        activate: Arc::new(|app| {
            let status = app.new(status::GitHubStatus::new);
            ExtensionInstance {
                status_bar_widgets: vec![status.into()],
                status_bar_right_widgets: vec![],
            }
        }),
        settings_view: None,
    }
}
