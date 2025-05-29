// src/main.rs
// Prevent console window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod decrypt;
mod path;
mod widget;
mod window;

fn main() {
    window::main_app();
}
