// lib/lib.rs

mod decrypt;
mod path;
mod widget;
mod window;

#[unsafe(no_mangle)]
pub extern "C" fn main_window() {
    window::main_window();
}
