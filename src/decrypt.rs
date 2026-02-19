// src/decrypt.rs
// Prevent console window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use libloading::{Library, Symbol};
use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let exe_path = env::current_exe()?;
    println!("Executable path: {:?}", exe_path);
    #[cfg(windows)]
    let lib_filename = format!("{}.dll", env!("CARGO_PKG_NAME"));
    #[cfg(target_family = "unix")]
    let lib_filename = format!("lib{}.so", env!("CARGO_PKG_NAME"));
    let lib_path = exe_path.with_file_name(lib_filename);
    println!("Shared library path: {:?}", lib_path);
    unsafe {
        let lib = Library::new(lib_path)?;
        let func: Symbol<unsafe extern "C" fn()> = lib.get(b"main_window")?;
        func();
    }
    Ok(())
}
