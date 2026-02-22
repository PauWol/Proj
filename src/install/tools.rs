use std::vec;

use os_info;

use crate::util::exec::{execute_tuple, run_with_indicator,run_checks_with_indicator,get_tool_version_tuple};
use crate::install::constants::{LinuxFedoraInstalls, LinuxUbuntuInstalls, WindowsInstalls,CHECKS};
use crate::util::types::ProjectType;


#[cfg(target_os = "windows")]
/// Installs Python using the appropriate package manager for Windows.
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

#[cfg(target_os = "windows")]
/// Installs Node.js using the appropriate package manager for Windows.
pub fn i_node() {
    run_with_indicator("Installing Node.js Toolchain", vec![
        ("Volta", Box::new(|| {
            if let Err(e) = execute_tuple(WindowsInstalls::VOLTA) {
                eprintln!("Error installing Volta: {}", e);
            }
        })),
        ("Node.js", Box::new(|| {
            if let Err(e) = execute_tuple(WindowsInstalls::NODE) {
                eprintln!("Error installing Node.js: {}", e);
            }
        })),
    ]);
}

#[cfg(target_os = "linux")]
/// Installs Python using the appropriate package manager based on the Linux distribution.
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

#[cfg(target_os = "linux")]
/// Installs Node.js using the appropriate package manager based on the Linux distribution.
pub fn i_node() {
    let info = os_info::get().os_type().to_string();

    if info.contains("Fedora") {
        run_with_indicator("Installing Node.js Toolchain", vec![
            ("Volta", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxFedoraInstalls::VOLTA) {
                    eprintln!("Error installing Volta: {}", e);
                }
            })),
            ("Node.js", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxFedoraInstalls::NODE) {
                    eprintln!("Error installing Node.js: {}", e);
                }
            })),
        ]);
    } else if info.contains("Ubuntu") {
        run_with_indicator("Installing Node.js Toolchain", vec![
            ("Volta", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxUbuntuInstalls::VOLTA) {
                    eprintln!("Error installing Volta: {}", e);
                }
            })),
            ("Node.js", Box::new(|| {
                if let Err(e) = execute_tuple(LinuxUbuntuInstalls::NODE) {
                    eprintln!("Error installing Node.js: {}", e);
                }
            })),
        ]);
    } else {
        println!("Unsupported Linux distribution: {}", info);
    }
}   


/// Installs the necessary tools for the given project type.
pub fn install_toolchain(project_type: &ProjectType) {
    match project_type {
        ProjectType::Python => i_python(),
        ProjectType::Node => i_node(),
        _ => println!("Installation for {:?} is not implemented yet.", project_type),
    }
}



fn is_python() ->  Vec<(String, bool)>  {
    return run_checks_with_indicator(
        "Checking Python Toolchain",
        vec![
            (
                "Python",
                Box::new(|| {
                    match get_tool_version_tuple(CHECKS::IS_PYTHON) {
                        Some(version) => (true, Some(version)),
                        None => (false, None),
                    }
                }),
            ),
            (
                "UV",
                Box::new(|| {
                    match get_tool_version_tuple(CHECKS::IS_UV){
                        Some(version) => (true, Some(version)),
                        None => (false, None),
                    }
                }),
            ),
        ],
    );
}


fn is_node() -> Vec<(String, bool)> {
    return run_checks_with_indicator(
        "Checking Node.js Toolchain",
        vec![
            ("Volta",
                Box::new(|| {
                    match get_tool_version_tuple(CHECKS::IS_VOLTA) {
                        Some(version) => (true, Some(version)),
                        None => (false, None),
                    }
                }),
            ),
            (
                "Node.js",
                Box::new(|| {
                    match get_tool_version_tuple(CHECKS::IS_NODE) {
                        Some(version) => (true, Some(version)),
                        None => (false, None),
                    }
                }),
            ),
        ],
    );
}



pub fn check_toolchain(project_type: &ProjectType) -> Vec<(String, bool)> {
    match project_type {
        ProjectType::Python => {
            return is_python()
        },
        ProjectType::Node => {
            return is_node()
        },
        _ => {
            println!("Toolchain check for {:?} is not implemented yet.", project_type);
            return vec![];
        },
    }

}