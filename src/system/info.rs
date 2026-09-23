use nix::unistd::{self, User};
use std::io::stdin;

pub struct Info;

impl Info {
    pub fn new() -> Self {
        Self
    }

    pub fn username(&self) -> String {
        User::from_uid(unistd::getuid())
            .ok()
            .flatten()
            .map(|user| user.name)
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn hostname(&self) -> String {
        unistd::gethostname()
            .map(|hostname| hostname.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "unknown".to_string())
    }

    pub fn ttyname(&self) -> String {
        unistd::ttyname(stdin())
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".to_string())
    }
}
