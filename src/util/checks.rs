use crate::util::types::ProjectType;
use crate::util::exec::is_tool_installed;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use owo_colors::OwoColorize;

/// Checks if the required tool for a project type is installed
/// and displays a spinner while performing the check.
pub fn is_installed_with_ui(project_type: &ProjectType) -> bool {
    // Create a spinner
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message(format!("Checking {}...", project_type));

    // Perform the check (simulate a short delay for better UI)
    let installed = is_tool_installed(
        project_type.lang_command(),
        project_type.lang_version_arg(),
    );

    // Finish spinner with success/failure message
    if installed {
        pb.finish_with_message(format!("✔ {} found", project_type).green().to_string());
    } else {
        pb.finish_with_message(format!("✘ {} not found", project_type).red().to_string());
    }

    installed
}