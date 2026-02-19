// src/main.rs
// Prevent console window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use libloading::{Library, Symbol};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        #[cfg(windows)]
        let lib_filename = format!("{}.dll", env!("CARGO_PKG_NAME"));
        #[cfg(target_family = "unix")]
        let lib_filename = format!("lib{}.so", env!("CARGO_PKG_NAME"));
        let lib = Library::new(lib_filename)?;
        let func: Symbol<unsafe extern "C" fn()> = lib.get(b"main_window")?;
        func();
    }
    Ok(())
}
