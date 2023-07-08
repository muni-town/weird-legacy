use std::path::PathBuf;

use site::Site;

use crate::prelude::*;

pub fn build(
    root_dir: &PathBuf,
    config_file: &PathBuf,
    output_dir: &PathBuf,
    base_url: Option<&str>,
) -> Result<()> {
    let mut site = Site::new(root_dir, config_file)?;
    site.set_output_path(output_dir);
    if let Some(b) = base_url {
        site.set_base_url(b.to_string());
    }
    site.load()?;
    site.build()?;
    Ok(())
}
