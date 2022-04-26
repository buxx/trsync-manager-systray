use std::process::Command;
use structopt::StructOpt;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    manager_bin_path: String,
    configure_bin_path: String,
}

fn main() {
    let opt = Opt::from_args();

    println!("Run {}", &opt.manager_bin_path);
    let mut manager_child = Command::new(&opt.manager_bin_path).spawn().unwrap();

    #[cfg(target_os = "linux")]
    {
        match linux::run_tray(opt.configure_bin_path.clone()) {
            Err(error) => {
                eprintln!("{}", error)
            }
            _ => {}
        }
    }

    #[cfg(target_os = "windows")]
    {
        match windows::run_tray(opt.configure_bin_path.clone()) {
            Err(error) => {
                eprintln!("{}", error)
            }
            _ => {}
        }
    }

    println!("Stop manager");
    manager_child.kill().unwrap();
    manager_child.wait().unwrap();
    println!("Finished")
}
