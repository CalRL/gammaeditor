mod discord;

use crate::save::backup::create_backup;
use tauri::menu::{MenuEvent, SubmenuBuilder};
use tauri::{menu::{MenuBuilder, MenuItem, Submenu}, AppHandle, Manager};
use crate::save;

pub fn build_menu(app: &AppHandle) {
    let file = file_submenu(app);
    let help = help_submenu(app);
    let menu = MenuBuilder::new(app)
        .item(&file)
        .item(&help)
        .build()
        .expect("build main menu");

    app.set_menu(menu).expect("set menu");
}

fn file_submenu(app: &AppHandle) -> Submenu<tauri::Wry> {
    let open_item = MenuItem::with_id(app, "menu-open", "Open", true, None::<&str>).expect("open");
    let backup_item = MenuItem::with_id(app, "menu-backup", "Create Backup", true, None::<&str>).expect("createbackup");
    let save_item = MenuItem::with_id(app, "file-save", "Save To JSON", true, None::<&str>).expect("savejson");
    let quit_item = MenuItem::with_id(
        app,
        "file-quit",
        "Quit",
        true,
        None::<&str>
    ).expect("quit");


    return SubmenuBuilder::new(app, "File")
        .item(&open_item)
        .item(&backup_item)
        .item(&save_item)
        .separator()
        .item(&quit_item)
        .build()
        .expect("build File submenu");
}

fn help_submenu(app: &AppHandle) -> Submenu<tauri::Wry> {
    let discord = MenuItem::with_id(app, "help-discord", "Join the Discord", true, None::<&str>).expect("joindiscord");

    SubmenuBuilder::new(app, "Help")
        .item(&discord)
        .build()
        .expect("build Help submenu")
}

pub fn handle_menu_event(app: AppHandle, event: MenuEvent) {
    eprintln!("Received event: {:?}", &event.id());
    match event.id().0.as_str() {
        "menu-open" => {
            eprintln!("Opening");
            let window = app.clone().get_window("main").expect("Failed to get main window");
            crate::file::handle_open(app, window);
        }
        "menu-backup" => {
            create_backup().expect("Failed");
        }
        "file-quit" => {
            app.exit(0);
        }
        "file-save" => {
            save::save_to_json().expect("saved");

            eprintln!("Saved");
        }
        _ => {}
    }
}

pub trait MenuButton {
    fn get_identifier(&self) -> String;
    fn execute(&self);
}