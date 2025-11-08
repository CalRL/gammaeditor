use std::io;
use std::process::{Child, Command};
use crate::logger::Logger;
use crate::ui::menu::{MenuButton, OnClick};

pub struct DiscordButton;

impl DiscordButton {
    pub fn new() -> Self {
        Self
    }
}

impl MenuButton for DiscordButton {
    fn get_identifier(&self) -> String {
        "menu-help-discord".to_string()
    }

    fn execute(&self) {
        todo!()
    }

    fn display(&self) -> String {
        "Join the Discord".to_string()
    }
}

impl OnClick for DiscordButton {
    fn on_click(&self) {
        Logger::info("Opening Discord URL in browser...");

        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("cmd")
                .args(["/C", "start", "https://discord.com/invite/tM5JVsGWnY"])
                .spawn();
        }

        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("open")
                .arg("https://discord.com/invite/tM5JVsGWnY")
                .spawn();
        }

        #[cfg(target_os = "linux")]
        {
            let _ = Command::new("xdg-open")
                .arg("https://discord.com/invite/tM5JVsGWnY")
                .spawn();
        }
    }
}