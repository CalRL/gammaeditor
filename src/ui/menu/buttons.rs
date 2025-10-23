use crate::ui::menu::MenuButton;

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