// src/window.rs
use fltk::{prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};
// use fltk_theme::{WidgetScheme, SchemeType,};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::decrypt::*;
use crate::path::*;
use crate::widget::*;

pub fn main_app() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    // let app = app::App::default();
    using_theme();
    show_window();
    app.run().unwrap();
}

fn using_theme() {
    let theme = ColorTheme::new(&color_themes::fleet::LIGHT);
    theme.apply();
    /*
    let widget_scheme = WidgetScheme::new(SchemeType::Fleet1);
    widget_scheme.apply();
    */
}

// We need to bind the callback function for the decrypt method that is independent of FLTK here,
// as well as to initialize and display the window.
fn show_window() {
    let w = Arc::new(Mutex::new(Widget::new()));
    let ui = w.lock().unwrap().window.clone();
    let w_clone = Arc::clone(&w);
    ui.clone().bn_execute.set_callback(move |_| {
        let mut w = w_clone.lock().unwrap();
        if !w.on_process {
            w.on_process = true;
            w.should_stop = false;
            let b_deal_file = ui.rb_deal_file.value();
            let b_recursive = ui.cb_recursive.value();
            let b_save_orig = ui.rb_save_orig.value();
            let b_backup = ui.cb_backup.value();
            let (_, vec_files) = split_string_into_vec_pathbuf(ui.en_deal_file.value(), ';');
            let path_deal_dir = PathBuf::from(ui.en_deal_dir.value());
            let path_save_other = PathBuf::from(ui.en_save_other.value());
            let w_thread = Arc::clone(&w_clone);
            /* let thread_join_handle = */
            thread::spawn(move || {
                let mut w = w_thread.lock().unwrap();
                execute_decrypt(
                    b_deal_file,
                    b_recursive,
                    b_save_orig,
                    b_backup,
                    vec_files,
                    path_deal_dir,
                    path_save_other,
                    &mut *w,
                );
            });
            // let _ = thread_join_handle.join();
        } else {
            w.should_stop = true;
            w.on_process = false;
        }
    });
}
