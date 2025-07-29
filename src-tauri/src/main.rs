// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

#[derive(Parser)]
#[command(name = "galaxy")]
#[command(about = "Galaxy Window Manager")]
struct Args {
    /// Enable debug mode with console output
    #[arg(long, help = "Enable debug logging and show console window")]
    debug: bool,
}

#[cfg(target_os = "windows")]
fn allocate_console() {
    use winapi::um::consoleapi::AllocConsole;
    use winapi::um::wincon::SetConsoleTitleA;
    use std::ffi::CString;
    
    unsafe {
        if AllocConsole() != 0 {
            let title = CString::new("Galaxy Window Manager - Debug Console").unwrap();
            SetConsoleTitleA(title.as_ptr());
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn allocate_console() {
    // No-op on non-Windows platforms
}

fn main() {
    let args = Args::parse();
    
    if args.debug {
        allocate_console();
        println!("Debug mode enabled - console output will be visible");
    }
    
    galaxy_lib::run(args.debug)
}
