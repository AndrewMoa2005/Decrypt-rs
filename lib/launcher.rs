// lib/launcher.rs
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::needless_update)]
#![allow(unused_assignments)]

use crate::loadicon::*;
use crate::theme::*;
use fltk::{prelude::*, *};
use std::env;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

include!(concat!(env!("OUT_DIR"), "/widget.rs"));

pub fn launcher_app() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    // let app = app::App::default();
    using_theme();
    make_window();
    app.run().unwrap();
}

fn make_window() {
    let mut ui = LauncherWindow::make_window();
    load_icon_from_resource(ui.launcher_win.clone());
    ui.en_ps_name.set_value(env!("CARGO_PKG_NAME"));
    ui.bn_ps_new.set_callback(move |_| {
        if ui.en_ps_name.value().is_empty() {
            fltk::dialog::alert_default("进程名不能为空!");
            println!("Process name is empty!");
            return;
        }
        let ps_name = ui.en_ps_name.value();
        println!("Process name: {}", ps_name);
        let launcher_path = env::current_exe().unwrap();
        let cp_new_ps = if ps_name == env!("CARGO_PKG_NAME") {
            false
        } else {
            true
        };
        let mut exe_path: PathBuf;
        if cp_new_ps {
            if cfg!(windows) {
                exe_path = launcher_path.with_file_name(format!("{}.exe", ps_name));
            } else {
                exe_path = launcher_path.with_file_name(format!("{}", ps_name));
            }
            #[cfg(windows)]
            let src_path = launcher_path.with_file_name(format!("{}.exe", env!("CARGO_PKG_NAME")));
            #[cfg(target_family = "unix")]
            let src_path = launcher_path.with_file_name(format!("{}", env!("CARGO_PKG_NAME")));
            std::fs::copy(&src_path, &exe_path).unwrap();
            println!("Copied: {:?} to {:?}", &src_path, &exe_path);
        } else {
            if cfg!(windows) {
                exe_path = launcher_path.with_file_name(format!("{}.exe", env!("CARGO_PKG_NAME")));
            } else {
                exe_path = launcher_path.with_file_name(format!("{}", env!("CARGO_PKG_NAME")));
            }
        }
        #[cfg(windows)]
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        #[cfg(windows)]
        let status = Command::new(&exe_path)
            .creation_flags(CREATE_NO_WINDOW)
            .status();
        #[cfg(target_family = "unix")]
        let status = Command::new(&exe_path).status();
        match status {
            Ok(exit_status) if !exit_status.success() => {
                eprintln!("Failed to launcher: {:?}", exe_path);
            }
            Err(e) => {
                eprintln!("Failed to launcher: {:?}, error: {}", exe_path, e);
            }
            _ => {}
        }
        if cp_new_ps {
            std::fs::remove_file(&exe_path).unwrap();
            println!("Deleted: {:?}", &exe_path);
        }
        ui.launcher_win.hide();
    });
}
