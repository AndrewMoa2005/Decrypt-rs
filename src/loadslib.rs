// src/loadslib.rs
use libloading::{Library, Symbol};
use std::env;
use std::path::PathBuf;
fn load_func(lib_path: &PathBuf, sym_name: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = Library::new(lib_path)?;
        let func: Symbol<unsafe extern "C" fn()> = lib.get(sym_name)?;
        func();
    }
    Ok(())
}
pub fn load_shared_lib(lib_name: &str, sym_name: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let exe_path = env::current_exe()?;
    println!("Executable path: {:?}", exe_path);
    #[cfg(windows)]
    let lib_filename = format!("{}.dll", lib_name);
    #[cfg(target_family = "unix")]
    let lib_filename = format!("lib{}.so", lib_name);
    let lib_path = exe_path.with_file_name(lib_filename);
    println!("Shared library path: {:?}", lib_path);
    load_func(&lib_path, sym_name)?;
    Ok(())
}
