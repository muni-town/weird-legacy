use std::path::{PathBuf, Path};

use log::debug;
use site::{Site, SITE_CONTENT};

use crate::prelude::*;

pub fn build_site(
    root_dir: &Path,
    output_dir: &PathBuf,
    base_url: Option<&str>,
    config_file: &Path,
) -> Result<()> {
    SITE_CONTENT.write().unwrap().clear();

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
    Ok(())
}
