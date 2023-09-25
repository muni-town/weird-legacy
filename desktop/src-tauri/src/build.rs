use std::path::{Path, PathBuf};

use log::debug;
use site::Site;

use crate::prelude::*;

pub fn build_site(
    root_dir: &Path,
    output_dir: &PathBuf,
    base_url: Option<&str>,
    config_file: &Path,
) -> Result<()> {
    // copy the css file
    let css_file = if output_dir.clone().join("css/style.css").is_file() {
        "css/style.css"
    } else {
        "style.css"
    };
    let css_path = output_dir.clone().join(css_file);
    let cache_path = config_file.parent().unwrap().join(css_file);
    if css_path.is_file() {
        std::fs::copy(&css_path, &cache_path).ok();
    }
    let mut site = Site::new(root_dir, config_file)?;

    if let Some(b) = base_url {
        site.set_base_url(b.to_string());
    }
    site.set_output_path(output_dir);
    site.load()?;
    debug!(
        "Building zola site output path: {}",
        output_dir.to_str().unwrap_or_default()
    );
    site.build()?;
    if !css_path.is_file() && cache_path.is_file() {
        std::fs::copy(cache_path, css_path).ok();
    }
    Ok(())
}
