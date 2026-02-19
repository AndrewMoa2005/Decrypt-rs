// lib/decrypt.rs
use fltk::{prelude::*, *};
use std::fs;
use std::io::{Read, Write};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
// Prevent the interface from refreshing too quickly.
use std::time::Instant;
// Sleeps threads for debugging
// use std::{thread, time::Duration};

use crate::widget::*;

pub fn execute_decrypt(
    deal_file: bool,
    recursive: bool,
    save_orig: bool,
    backup_orig: bool,
    ext_name: &str,
    load_file_path: Vec<PathBuf>,
    load_dir_path: PathBuf,
    save_dir_path: PathBuf,
    widget: &mut Widget,
) {
    let should_stop = &mut widget.should_stop;
    let on_process = &mut widget.on_process;
    if *should_stop {
        let pbar = widget.window.p_bar.clone();
        let bn = widget.window.bn_execute.clone();
        set_status(DecryptStatus::Interrupted, 0.0, pbar, bn, on_process);
        return;
    }
    let pbar = widget.window.p_bar.clone();
    let bn = widget.window.bn_execute.clone();
    set_status(DecryptStatus::Beginning, 0.0, pbar, bn, on_process);
    // Create the subfolder.

    if !deal_file && !save_orig && recursive {
        if !copy_directories(
            &load_dir_path,
            &save_dir_path,
            false,
            recursive,
            should_stop,
        ) {
            let pbar = widget.window.p_bar.clone();
            let bn = widget.window.bn_execute.clone();
            set_status(DecryptStatus::Error, 0.0, pbar, bn, on_process);
            return;
        }
    }
    let pbar = widget.window.p_bar.clone();
    let bn = widget.window.bn_execute.clone();
    set_status(DecryptStatus::Preparing, 0.0, pbar, bn, on_process);
    let mut files = load_file_path;
    if !deal_file {
        if let Some(children_files) = get_children_files(&load_dir_path, recursive, should_stop) {
            files = children_files;
        } else {
            eprintln!(
                "Failed to get children files from directory: {:?}",
                load_dir_path
            );
            let pbar = widget.window.p_bar.clone();
            let bn = widget.window.bn_execute.clone();
            set_status(DecryptStatus::Error, 0.0, pbar, bn, on_process);
            return;
        }
    }
    let mut processed = 0 as f64;
    let total_files = files.len() as f64;
    for iter in files.iter() {
        if *should_stop {
            let pbar = widget.window.p_bar.clone();
            let bn = widget.window.bn_execute.clone();
            set_status(
                DecryptStatus::Interrupted,
                processed / total_files,
                pbar,
                bn,
                on_process,
            );
            return;
        }
        if save_orig {
            print!("Processing file: {:?}\n", iter);
            if write_file(
                iter,
                &PathBuf::from(""),
                ext_name,
                backup_orig,
                save_orig,
                should_stop,
            ) {
                processed += 1.0;
            } else {
                let pbar = widget.window.p_bar.clone();
                let bn = widget.window.bn_execute.clone();
                set_status(
                    DecryptStatus::Error,
                    processed / total_files,
                    pbar,
                    bn,
                    on_process,
                );
                return;
            }
        } else {
            if deal_file {
                let filename = iter.file_name().unwrap();
                let dst = save_dir_path.join(filename);
                print!("Processing file: {:?} -> {:?}\n", iter, dst);
                if write_file(iter, &dst, ext_name,backup_orig, false, should_stop) {
                    processed += 1.0;
                } else {
                    let pbar = widget.window.p_bar.clone();
                    let bn = widget.window.bn_execute.clone();
                    set_status(
                        DecryptStatus::Error,
                        processed / total_files,
                        pbar,
                        bn,
                        on_process,
                    );
                    return;
                }
            } else {
                let mut filepath = iter.to_str().unwrap().to_string();
                let in_dirpath = load_dir_path.to_str().unwrap();
                let out_dirpath = save_dir_path.to_str().unwrap();
                filepath = filepath.replace(in_dirpath, out_dirpath);
                let dst = PathBuf::from(filepath);
                print!("Processing file: {:?} -> {:?}\n", iter, dst);
                if write_file(iter, &dst, ext_name, backup_orig, false, should_stop) {
                    processed += 1.0;
                } else {
                    let pbar = widget.window.p_bar.clone();
                    let bn = widget.window.bn_execute.clone();
                    set_status(
                        DecryptStatus::Error,
                        processed / total_files,
                        pbar,
                        bn,
                        on_process,
                    );
                    return;
                }
            }
        }
        let pbar = widget.window.p_bar.clone();
        let bn = widget.window.bn_execute.clone();
        set_status(
            DecryptStatus::Dealing,
            processed / total_files,
            pbar,
            bn,
            on_process,
        );
    }
    let pbar = widget.window.p_bar.clone();
    let bn = widget.window.bn_execute.clone();
    set_status(
        DecryptStatus::Success,
        processed / total_files,
        pbar,
        bn,
        on_process,
    );
}

enum DecryptStatus {
    Beginning,
    Preparing,
    Dealing,
    Interrupted,
    Error,
    Success,
}

fn set_status(
    status: DecryptStatus,
    pbar_value: f64,
    mut pbar: fltk::misc::Progress,
    mut bn: fltk::button::Button,
    process: &mut bool,
) {
    let start = Instant::now();
    let _ = app::lock();
    // Sleeps threads for debugging
    // thread::sleep(Duration::from_millis(1000));
    pbar.set_value(pbar_value);
    match status {
        DecryptStatus::Beginning => {
            pbar.set_label("开始处理");
            bn.set_label("停止处理(&S)");
        }
        DecryptStatus::Preparing => {
            pbar.set_label("准备文件");
            bn.set_label("停止处理(&S)");
        }
        DecryptStatus::Dealing => {
            pbar.set_label(format!("处理进度：{:.2}%", pbar_value * 100.0).as_str());
            bn.set_label("停止处理(&S)");
        }
        DecryptStatus::Interrupted => {
            pbar.set_label("处理中断");
            bn.set_label("保存明文(&S)");
            *process = false;
        }
        DecryptStatus::Error => {
            pbar.set_label("处理出错");
            bn.set_label("保存明文(&S)");
            *process = false;
        }
        DecryptStatus::Success => {
            pbar.set_label("处理完成");
            bn.set_label("保存明文(&S)");
            *process = false;
        }
    }
    let duration = start.elapsed();
    if duration.as_millis() > 100 {
        app::flush();
    }
    app::unlock();
    app::awake(); // Ensure the UI updates
}

fn write_file(
    src: &PathBuf,
    dst: &PathBuf,
    ext_name: &str,
    backup: bool,
    save_original: bool,
    should_stop: &bool,
) -> bool {
    if *should_stop {
        return false;
    }

    if !src.exists() || !src.is_file() {
        eprintln!("Source file does not exist or is not a file: {:?}", src);
        return false;
    }

    if let Some(parent) = dst.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!(
                    "Failed to create parent directory: {:?}, error: {}",
                    parent, e
                );
                return false;
            }
        }
    }

    let tmp = PathBuf::from(format!("{}.{}", dst.to_string_lossy(), ext_name));

    // Using an alternative to buffered I/O.
    let mut buffer = [0u8; 1024 * 1024]; // 1 MiB Buffer
    let mut source = match fs::File::open(&src) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open source file: {:?}, error: {}", src, e);
            return false;
        }
    };
    let mut temp = match fs::File::create(&tmp) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create temporary file: {:?}, error: {}", tmp, e);
            return false;
        }
    };

    loop {
        let n = match source.read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from source file: error: {}", e);
                return false;
            }
        };
        if n == 0 {
            break;
        }
        match temp.write_all(&buffer[..n]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to write to temporary file: error: {}", e);
                return false;
            }
        };
    }

    #[cfg(windows)]
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    if backup && save_original {
        let bak = PathBuf::from(format!("{}.bak", src.to_string_lossy()));
        #[cfg(windows)]
        let status = Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "/c",
                "move",
                "/y",
                &src.to_string_lossy(),
                &bak.to_string_lossy(),
                "&&",
                "move",
                "/y",
                &tmp.to_string_lossy(),
                &src.to_string_lossy(),
            ])
            .status();
        #[cfg(target_family = "unix")]
        let status = Command::new("sh")
            .args([
                "-c",
                format!("mv -f {:?} {:?} && mv -f {:?} {:?}", src, bak, tmp, src,).as_str(),
            ])
            .status();

        match status {
            Ok(exit_status) if !exit_status.success() => {
                eprintln!("Failed to backup original file: {:?}", src);
                return false;
            }
            Err(e) => {
                eprintln!("Failed to backup original file: {:?}, error: {}", src, e);
                return false;
            }
            _ => {}
        }
    } else if !save_original && (dst.to_string_lossy().len() > 0) {
        #[cfg(windows)]
        let status = Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "/c",
                "move",
                "/y",
                &tmp.to_string_lossy(),
                &dst.to_string_lossy(),
            ])
            .status();
        #[cfg(target_family = "unix")]
        let status = Command::new("sh")
            .args([
                "-c",
                format!("mv -f {:?} {:?}", tmp, dst,).as_str(),
            ])
            .status();

        match status {
            Ok(exit_status) if !exit_status.success() => {
                eprintln!("Failed to move temporary file to destination: {:?}", dst);
                return false;
            }
            Err(e) => {
                eprintln!(
                    "Failed to move temporary file to destination: {:?}, error: {}",
                    dst, e
                );
                return false;
            }
            _ => {}
        }
    } else {
        #[cfg(windows)]
        let status = Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .args([
                "/c",
                "move",
                "/y",
                &tmp.to_string_lossy(),
                &src.to_string_lossy(),
            ])
            .status();
        #[cfg(target_family = "unix")]
        let status = Command::new("sh")
            .args([
                "-c",
                format!("mv -f {:?} {:?}", tmp, src,).as_str(),
            ])
            .status();

        match status {
            Ok(exit_status) if !exit_status.success() => {
                eprintln!("Failed to move temporary file to source: {:?}", src);
                return false;
            }
            Err(e) => {
                eprintln!(
                    "Failed to move temporary file to source: {:?}, error: {}",
                    src, e
                );
                return false;
            }
            _ => {}
        }
    }

    true
}

fn get_children_files(dir: &PathBuf, recursive: bool, should_stop: &bool) -> Option<Vec<PathBuf>> {
    if *should_stop {
        return None;
    }

    if !dir.exists() || !dir.is_dir() {
        eprintln!("Directory does not exist or is not a directory: {:?}", dir);
        return None;
    }

    let mut files = vec![];

    for entry in match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory: {:?}, error: {}", dir, e);
            return None;
        }
    } {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading entry in directory: {:?}", e);
                continue;
            }
        };

        let path = entry.path();
        if path.is_dir() && recursive {
            if let Some(mut sub_files) = get_children_files(&path, recursive, should_stop) {
                files.append(&mut sub_files);
            }
        } else if path.is_file() {
            files.push(path);
        }
    }

    Some(files)
}

fn copy_directories(
    src: &PathBuf,
    dst: &PathBuf,
    copy_files: bool,
    recursive: bool,
    should_stop: &bool,
) -> bool {
    if *should_stop {
        return false;
    }

    if !src.exists() || !src.is_dir() {
        eprintln!(
            "Source directory does not exist or is not a directory: {:?}",
            src
        );
        return false;
    }

    if !dst.exists() {
        if let Err(e) = std::fs::create_dir_all(dst) {
            eprintln!(
                "Failed to create destination directory: {:?}, error: {}",
                dst, e
            );
            return false;
        }
    }

    for entry in match std::fs::read_dir(src) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read source directory: {:?}, error: {}", src, e);
            return false;
        }
    } {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading entry in source directory: {:?}", e);
                continue;
            }
        };

        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            if recursive
                && !copy_directories(&src_path, &dst_path, copy_files, recursive, should_stop)
            {
                return false;
            }
        } else if copy_files && src_path.is_file() {
            if let Err(e) = std::fs::copy(&src_path, &dst_path) {
                eprintln!(
                    "Failed to copy file from {:?} to {:?}, error: {}",
                    src_path, dst_path, e
                );
                return false;
            }
        }
    }

    true
}
