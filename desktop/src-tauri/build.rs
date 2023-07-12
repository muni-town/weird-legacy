use std::path::Path;

#[path = "./src/utils/zip.rs"]
mod utils;

fn main() {
    generate_zip();
    tauri_build::build()
}

fn generate_zip() {
    let path = Path::new(file!()).parent().unwrap();
    let src_dir = path.join("../template").to_str().unwrap().to_owned();
    let dst_file = path
        .join("./assets/template.zip")
        .to_str()
        .unwrap()
        .to_owned();
    if let Err(e) = utils::zip_dir(&src_dir, &dst_file, zip::CompressionMethod::Deflated) {
        println!("cargo:warning=Could not create template.zip: {e}")
    }
}
