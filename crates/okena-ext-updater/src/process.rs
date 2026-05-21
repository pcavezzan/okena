/// Create a [`std::process::Command`] that does **not** flash a console window
/// on Windows. Delegates to the shared helper in `okena-core`.
///
/// Note: the updater spawns *detached, long-lived* processes (the replacement
/// binary, `tar`/`unzip`) directly via `.spawn()`, so it deliberately does not
/// route through the command bus — these outlive the bus and must not hold a
/// worker permit.
pub fn command(program: &str) -> std::process::Command {
    okena_core::process::command(program)
}

/// Get the updates directory for the active profile.
pub fn get_config_dir() -> std::path::PathBuf {
    if let Some(p) = okena_core::profiles::try_current() {
        p.updates_dir()
    } else {
        dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("okena")
    }
}
