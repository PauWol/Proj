use os_info;

use crate::util::exec::{execute_tuple, run_with_indicator};
use crate::install::constants::{WindowsInstalls,LinuxFedoraInstalls,LinuxUbuntuInstalls};



#[cfg(target_os = "windows")]
pub fn i_python() {
    run_with_indicator("Installing Python Toolchain", vec![
        ("UV", Box::new(|| {
            if let Err(e) = execute_tuple(WindowsInstalls::UV) {
                eprintln!("Error installing UV: {}", e);
            }
        })),
        ("Python", Box::new(|| {
            if let Err(e) = execute_tuple(WindowsInstalls::PYTHON) {
                eprintln!("Error installing Python: {}", e);
            }
        })),
    ]);
}

#[cfg(target_os = "linux")]
pub fn i_python() {
    let info = os_info::get().os_type().to_string();

    if info.contains("Fedora") {
        run_with_indicator("Installing Python Toolchain", vec![
            ("UV", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxFedoraInstalls::UV) {
                    eprintln!("Error installing UV: {}", e);
                }
            })),
            ("Python", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxFedoraInstalls::PYTHON) {
                    eprintln!("Error installing Python: {}", e);
                }
            })),
        ]);
    } else if info.contains("Ubuntu") {
        run_with_indicator("Installing Python Toolchain", vec![
            ("UV", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxUbuntuInstalls::UV) {
                    eprintln!("Error installing UV: {}", e);
                }
            })),
            ("Python", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxUbuntuInstalls::PYTHON) {
                    eprintln!("Error installing Python: {}", e);
                }
            })),
        ]);
    } else {
        println!("Unsupported Linux distribution: {}", info);
    }
}