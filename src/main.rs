#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

fn main() {
    #[cfg(target_os = "linux")]
    {
        match linux::run_tray() {
            Err(error) => {
                eprintln!("{}", error)
            }
            _ => {}
        }
    }

    #[cfg(target_os = "windows")]
    {
        match windows::run_tray() {
            Err(error) => {
                eprintln!("{}", error)
            }
            _ => {}
        }
    }
}
