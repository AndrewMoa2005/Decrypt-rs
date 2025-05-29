// src/widget.rs
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::needless_update)]
#![allow(unused_assignments)]

use fltk::{prelude::*, *};
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/widget.rs"));

use crate::path::*;

#[derive(Debug, Clone)]
pub struct Widget {
    pub window: MainWindow,
    pub should_stop: bool,
    pub on_process: bool,
}

impl Widget {
    // initialize a new window and show it.
    pub fn new() -> Self {
        let mut w = Widget {
            window: Self::make_window(),
            should_stop: false,
            on_process: false,
        };
        w
    }

    fn load_icon_from_resource(mut w: fltk::window::DoubleWindow) {
        // Directly compile the icon file into the binary.
        let icon_data = include_bytes!(concat!("../resource/", env!("CARGO_PKG_NAME"), ".png"));
        /*
        The loading of the ICO file may occasionally encounter errors that prevent the program from starting;
        therefore, PNG images are used here, which also helps to reduce the size of the program package.
        */
        if let Ok(icon) = fltk::image::PngImage::from_data(icon_data) {
            w.set_icon(Some(icon));
        }
    }

    fn make_window() -> MainWindow {
        let mut ui = MainWindow::make_window();
        Self::load_icon_from_resource(ui.main_window.clone());
        //initialize MainWindow widgets
        Self::reflush_active(ui.clone());
        ui.p_bar.set_maximum(1.0);
        ui.p_bar.set_minimum(0.0);
        ui.p_bar.set_value(0.0);

        let mut ui_weak = ui.clone();
        ui.rb_deal_file.set_callback(move |_| {
            ui_weak.rb_deal_dir.set_value(!ui_weak.rb_deal_file.value());
            Self::reflush_active(ui_weak.clone());
        });

        let mut entry = ui.en_deal_file.clone();
        Self::handle_dnd_filenames(entry);

        let mut ui_weak = ui.clone();
        ui.bn_deal_file.set_callback(move |_| {
            let (i, str) = Self::choose_files();
            if i {
                ui_weak.en_deal_file.set_value(&str);
            }
        });

        let mut ui_weak = ui.clone();
        ui.rb_deal_dir.set_callback(move |_| {
            ui_weak.rb_deal_file.set_value(!ui_weak.rb_deal_dir.value());
            Self::reflush_active(ui_weak.clone());
        });

        let mut entry = ui.en_deal_dir.clone();
        Self::handle_dnd_dir(entry);

        let mut ui_weak = ui.clone();
        ui.bn_deal_dir.set_callback(move |_| {
            let (i, str) = Self::choose_dir();
            if i {
                ui_weak.en_deal_dir.set_value(&str);
            }
        });

        let mut ui_weak = ui.clone();
        ui.rb_save_orig.set_callback(move |_| {
            ui_weak
                .rb_save_other
                .set_value(!ui_weak.rb_save_orig.value());
            Self::reflush_active(ui_weak.clone());
        });

        let mut ui_weak = ui.clone();
        ui.rb_save_other.set_callback(move |_| {
            ui_weak
                .rb_save_orig
                .set_value(!ui_weak.rb_save_other.value());
            Self::reflush_active(ui_weak.clone());
        });

        let mut entry = ui.en_save_other.clone();
        Self::handle_dnd_dir(entry);

        let mut ui_weak = ui.clone();
        ui.bn_save_other.set_callback(move |_| {
            let (i, str) = Self::choose_dir();
            if i {
                ui_weak.en_save_other.set_value(&str);
            }
        });

        ui.bn_link.set_callback(|_| {
            let str = include_str!("../resource/statement.txt");
            fltk::dialog::message_title("免责声明");
            // fltk::dialog::message_set_hotspot(true);
            fltk::dialog::message_icon_label("!");
            fltk::dialog::message_default(str);
        });

        ui.bn_about.set_callback(|_| {
            Self::show_about_dialog();
        });

        ui
    }

    fn show_about_dialog() {
        let mut ui = AboutDialog::make_window();

        let mut buf = fltk::text::TextBuffer::default();
        let log = include_str!("../resource/updatelog.txt");
        buf.set_text(log);
        ui.td_text.set_buffer(buf);

        let version = env!("CARGO_PKG_VERSION");
        let mut ver_str = ui.td_version.label();
        ver_str.push_str(version);
        ui.td_version.set_label(&ver_str);

        // Close this dialog
        ui.bn_about_enter.set_callback(move |_| {
            <DoubleWindow as fltk::prelude::WidgetBase>::delete(ui.about_dialog.clone());
        });
    }

    fn reflush_active(mut ui: MainWindow) {
        if ui.rb_deal_file.value() {
            ui.en_deal_file.activate();
            ui.bn_deal_file.activate();
            ui.en_deal_dir.deactivate();
            ui.bn_deal_dir.deactivate();
            ui.cb_recursive.deactivate();
        } else {
            ui.en_deal_file.deactivate();
            ui.bn_deal_file.deactivate();
            ui.en_deal_dir.activate();
            ui.bn_deal_dir.activate();
            ui.cb_recursive.activate();
        }
        if ui.rb_save_orig.value() {
            ui.cb_backup.activate();
            ui.en_save_other.deactivate();
            ui.bn_save_other.deactivate();
        } else {
            ui.cb_backup.deactivate();
            ui.en_save_other.activate();
            ui.bn_save_other.activate();
        }
    }

    fn choose_files() -> (bool, String) {
        let mut success: bool = false;
        let mut str = String::new();
        let mut fnfc =
            dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseMultiFile);
        match fnfc.try_show() {
            Ok(FileDialogAction::Success) => {
                let filenames = fnfc.filenames();
                (success, str) = vec_pathbuf_to_string(&filenames, ';');
            }
            _ => (),
        }
        (success, str)
    }

    fn choose_dir() -> (bool, String) {
        let mut success: bool = false;
        let mut str = String::new();
        let mut fnfc = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
        match fnfc.try_show() {
            Ok(FileDialogAction::Success) => {
                let dir = fnfc.filename();
                (success, str) = pathbuf_to_string(dir);
            }
            _ => (),
        }
        (success, str)
    }

    fn handle_dnd_filenames(mut entry: fltk::input::Input) {
        entry.handle({
            let mut dnd = false;
            let mut released = false;
            let mut entry_weak = entry.clone();
            move |_, ev| match ev {
                Event::DndEnter => {
                    dnd = true;
                    true
                }
                Event::DndRelease => {
                    released = true;
                    true
                }
                Event::DndLeave => {
                    dnd = false;
                    released = false;
                    true
                }
                Event::DndDrag => true,
                Event::Paste => {
                    if dnd && released {
                        let text = app::event_text();
                        let (b1, v) = split_string_into_vec_pathbuf(text, '\n');
                        if !b1 {
                            return false;
                        }
                        match verify_path_vec(&v) {
                            PathVecType::OnlyFiles => {
                                // we use a timeout to avoid pasting the path into the input widget
                                let mut en = entry_weak.clone();
                                let mut r = false;
                                app::add_timeout3(0.01, {
                                    move |_| {
                                        let (b2, str) = vec_pathbuf_to_string(&v, ';');
                                        if !b2 {
                                            return;
                                        }
                                        en.set_value(&str);
                                        dnd = false;
                                        released = false;
                                        r = true;
                                    }
                                });
                                r
                            }
                            _ => {
                                let mut en = entry_weak.clone();
                                app::add_timeout3(0.01, {
                                    move |_| {
                                        en.set_value("");
                                    }
                                });
                                false
                            }
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        });
    }

    fn handle_dnd_dir(mut entry: fltk::input::Input) {
        entry.handle({
            let mut dnd = false;
            let mut released = false;
            let mut entry_weak = entry.clone();
            move |_, ev| match ev {
                Event::DndEnter => {
                    dnd = true;
                    true
                }
                Event::DndRelease => {
                    released = true;
                    true
                }
                Event::DndLeave => {
                    dnd = false;
                    released = false;
                    true
                }
                Event::DndDrag => true,
                Event::Paste => {
                    if dnd && released {
                        let text = app::event_text();
                        let (b1, v) = split_string_into_vec_pathbuf(text, '\n');
                        if !b1 {
                            return false;
                        }
                        match verify_path_vec(&v) {
                            PathVecType::OnlyDir => {
                                // we use a timeout to avoid pasting the path into the input widget
                                let mut en = entry_weak.clone();
                                let mut r = false;
                                app::add_timeout3(0.01, {
                                    move |_| {
                                        let (b2, str) = vec_pathbuf_to_string(&v, ';');
                                        if !b2 {
                                            return;
                                        }
                                        en.set_value(&str);
                                        dnd = false;
                                        released = false;
                                        r = true;
                                    }
                                });
                                r
                            }
                            _ => {
                                let mut en = entry_weak.clone();
                                app::add_timeout3(0.01, {
                                    move |_| {
                                        en.set_value("");
                                    }
                                });
                                false
                            }
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        });
    }
}
