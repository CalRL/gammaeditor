use crate::menu::MenuButton;

struct DiscordButton;

impl MenuButton for DiscordButton {
    fn get_identifier(&self) -> String {
        "menu-help-discord".to_string()
    }

    fn execute(&self) {
        todo!()
    }
}