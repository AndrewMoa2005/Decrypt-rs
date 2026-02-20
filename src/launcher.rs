// src/launcher.rs
// Prevent console window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod loadslib;
use crate::loadslib::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_shared_lib(env!("CARGO_PKG_NAME"), b"launcher_app")?;
    Ok(())
}
