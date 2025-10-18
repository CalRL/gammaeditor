use rfd::{MessageDialog, MessageLevel};

pub mod custom_struct;

pub fn fatal_error_dialog(error: String) -> MessageDialog {
    rfd::MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title("Fatal error")
        .set_description(error.to_string())
}