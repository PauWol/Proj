
pub struct WindowsInstalls;

impl WindowsInstalls {

    pub const UV: (&str, &[&str]) = ("powershell",&["-ExecutionPolicy", "ByPass", "-c", "irm https://astral.sh/uv/install.ps1 | iex"],);

    pub const PYTHON: (&str, &[&str]) = ("uv", &["python", "install"]);

    pub const VOLTA: (&str, &[&str]) = ("winget",&["install", "Volta.Volta"],);

    pub const NODE: (&str, &[&str]) = ("volta",&["install", "node"],);
}


// I know this setup is a bit redundant, but it allows for easy extension to other distros in the future, and keeps the commands organized.

pub struct LinuxFedoraInstalls;

impl LinuxFedoraInstalls {

        pub const UV: (&str, &[&str]) = ("bash",&["-c", "curl -fsSL https://astral.sh/uv/install.sh | bash"]);

    pub const PYTHON: (&str, &[&str]) =  ("uv", &["python", "install"]);

    pub const VOLTA: (&str, &[&str]) = ("bash", &["-c", "curl -sSfL https://get.volta.sh | bash"]);

    pub const NODE: (&str, &[&str]) = ("volta",&["install", "node"],);

}

pub struct LinuxUbuntuInstalls;

impl LinuxUbuntuInstalls {

    pub const UV: (&str, &[&str]) = ("bash",&["-c", "curl -fsSL https://astral.sh/uv/install.sh | bash"]);

    pub const PYTHON: (&str, &[&str]) =  ("uv", &["python", "install"]);

    pub const VOLTA: (&str, &[&str]) = ("bash", &["-c", "curl -sSfL https://get.volta.sh | bash"]);

    pub const NODE: (&str, &[&str]) = ("volta",&["install", "node"],);

}

pub struct CHECKS;


impl CHECKS {

    pub const IS_UV: (&str, &str) = ("uv", "--version");
    pub const IS_PYTHON: (&str, &str) = ("python", "--version");
    pub const IS_VOLTA: (&str, &str) = ("volta", "--version");
    pub const IS_NODE: (&str, &str) = ("node", "--version");

}