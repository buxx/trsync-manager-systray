use gtk;
use tray_item::TrayItem;

fn run_tray() -> Result<(), String> {
    match gtk::init() {
        Err(error) => return Err(format!("Unable to initialize gtk : '{}'", error)),
        _ => {}
    };

    let mut tray = match TrayItem::new("Tracim", "emblem-shared") {
        Ok(tray_) => tray_,
        Err(error) => return Err(format!("Unable to create tray item : '{}'", error)),
    };

    match tray.add_menu_item("Configurer", || {
        println!("Hello!");
    }) {
        Err(error) => return Err(format!("Unable to add menu item : '{:?}'", error)),
        _ => {}
    };

    match tray.add_menu_item("Quitter", || {
        gtk::main_quit();
    }) {
        Err(error) => return Err(format!("Unable to add menu item : '{:?}'", error)),
        _ => {}
    };

    gtk::main();
    Ok(())
}

fn main() {
    match run_tray() {
        Err(error) => {
            eprintln!("{}", error)
        }
        _ => {}
    }
}
