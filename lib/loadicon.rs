// lib/loadicon.rs
use fltk::prelude::*;
pub fn load_icon_from_resource(mut w: fltk::window::DoubleWindow) {
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
