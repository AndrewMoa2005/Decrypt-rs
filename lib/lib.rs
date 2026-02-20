// lib/lib.rs

mod decrypt;
mod launcher;
mod loadicon;
mod path;
mod theme;
mod widget;
mod window;

#[unsafe(no_mangle)]
pub extern "C" fn main_app() {
    window::main_app();
}

#[unsafe(no_mangle)]
pub extern "C" fn launcher_app() {
    launcher::launcher_app();
}
