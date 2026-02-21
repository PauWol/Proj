
pub struct WindowsInstalls;

impl WindowsInstalls {
    pub const UV: (&str, &[&str]) = (
        "powershell",
        &["-ExecutionPolicy", "ByPass", "-c", "irm https://astral.sh/uv/install.ps1 | iex"],
    );
    pub const PYTHON: (&str, &[&str]) = ("uv", &["python", "install"]);
}

pub struct LinuxFedoraInstalls;

impl LinuxFedoraInstalls {
    pub const UV: (&str, &[&str]) = (
    "bash",
    &["-c", "$(curl -fsSL https://astral.sh/uv/install.sh)"],
);
    pub const PYTHON: (&str, &[&str]) =  ("uv", &["python", "install"]);

}

pub struct LinuxUbuntuInstalls;

impl LinuxUbuntuInstalls {
    pub const UV: (&str, &[&str]) = (
    "bash",
    &["-c", "$(curl -fsSL https://astral.sh/uv/install.sh)"],
);
    pub const PYTHON: (&str, &[&str]) =  ("uv", &["python", "install"]);

}