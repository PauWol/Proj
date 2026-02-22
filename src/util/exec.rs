use std::process::Command;
use std::io;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle,HumanDuration};
use std::thread;
use std::time::{Duration, Instant};
use console::{style, Emoji};




/// Executes a command and returns its output as a String.
pub fn execute(cmd: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()?; // ? propagates errors

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        eprintln!("Error executing {}: {}", cmd, String::from_utf8_lossy(&output.stderr).trim());

        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).trim(),
        ))
    }
}

/// Helper function to execute a command given as a tuple (command, args)
pub fn execute_tuple(cmd: (&str, &[&str])) -> io::Result<String> {
    execute(cmd.0, cmd.1)
}


pub fn is_tool_installed(tool: &str, version_arg: &str) -> bool {
    match execute(tool, &[version_arg]) {
        Ok(output) => {
            println!("{} found: {}", tool, output.lines().next().unwrap_or(""));
            true
        }
        Err(_) => false,
    }
}



static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
pub fn run_with_indicator(
    title: &str,
    tasks: Vec<(&str, Box<dyn FnOnce() + Send>)>,
) {
    let started = Instant::now();
    let total = tasks.len(); // ← real total count

    println!("\n{}\n", style(title).bold().underlined());

    let multi = MultiProgress::new();

    let spinner_style = ProgressStyle::with_template(
        "{prefix:.bold.dim} {spinner} {wide_msg}",
    )
    .unwrap()
    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let mut handles = Vec::new();

    for (i, (name, task)) in tasks.into_iter().enumerate() {
        let pb = multi.add(ProgressBar::new_spinner());
        pb.set_style(spinner_style.clone());

        // 👇 Real 1/N counter
        pb.set_prefix(format!("[{}/{}]", i + 1, total));

        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_message(format!("Installing {}", name));

        let name = name.to_string();

        let handle = thread::spawn(move || {
            task();
            pb.finish_with_message(format!("Installed {}", name));
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    multi.clear().unwrap();

    println!(
        "\n{} Done in {}\n",
        SPARKLE,
        HumanDuration(started.elapsed())
    );
}

pub fn run_checks_with_indicator(
    title: &str,
    tasks: Vec<(&str, Box<dyn FnOnce() -> (bool, Option<String>) + Send>)>,
) -> Vec<(String, bool)> {
    let started = Instant::now();
    let total = tasks.len();

    println!();
    println!("{}", style(title).bold().underlined());
    println!();

    let mut results = Vec::new();

    // Determine longest name for alignment
    let longest = tasks
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);

    for (name, task) in tasks {
        let (installed, version) = task();

        let padded_name = format!("{:<width$}", name, width = longest);

        if installed {
            let version_text = version.unwrap_or_else(|| "unknown".into());

            println!(
                "  {}  {}  {}",
                style(padded_name).bold(),
                style("✓").green(),
                style(version_text).dim()
            );
        } else {
            println!(
                "  {}  {}",
                style(padded_name).bold(),
                style("✗ not found").red()
            );
        }

        results.push((name.to_string(), installed));
    }

    let passed = results.iter().filter(|(_, ok)| *ok).count();

    println!();

    if passed == total {
        println!(
            "{} All tools ready ({}/{}) in {}\n",
            style("✓").green(),
            passed,
            total,
            HumanDuration(started.elapsed())
        );
    } else {
        println!(
            "{} {}/{} tools available in {}\n",
            style("⚠").yellow(),
            passed,
            total,
            HumanDuration(started.elapsed())
        );
    }

    results
}

pub fn get_tool_version(cmd: &str, arg: &str) -> Option<String> {
    use std::process::Command;

    Command::new(cmd)
        .arg(arg)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}

pub fn get_tool_version_tuple(cmd: (&str, &str)) -> Option<String> {
    get_tool_version(cmd.0, cmd.1)
}