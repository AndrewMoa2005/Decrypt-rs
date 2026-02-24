// build.rs
// extern crate embed_resource;

fn main() {
    use std::env;
    use std::path::PathBuf;
    println!("cargo:rerun-if-changed=ui/widget.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("ui/widget.fl", out_path.join("widget.rs").to_str().unwrap())
        .expect("Failed to generate rust from fl file!");

    #[cfg(windows)]
    {
        let main_rc = String::from("resource/") + &env::var("CARGO_PKG_NAME").unwrap() + ".rc";
        embed_resource::compile(main_rc.as_str(), embed_resource::NONE)
            .manifest_optional()
            .unwrap();
    }
}
