use std::process::Command;
use std::io;

pub fn exec(cmd: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()?; // ? propagates errors

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).trim(),
        ))
    }
}

pub fn is_tool_installed(tool: &str, version_arg: &str) -> bool {
    match exec(tool, &[version_arg]) {
        Ok(output) => {
            println!("{} found: {}", tool, output.lines().next().unwrap_or(""));
            true
        }
        Err(_) => false,
    }
}