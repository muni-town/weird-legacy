use std::path::{PathBuf, Path};

use log::debug;
use site::{Site, SITE_CONTENT};

use crate::prelude::*;

pub fn create_new_site(
    root_dir: &Path,
    port: u16,
    output_dir: &PathBuf,
    base_url: &str,
    config_file: &Path,
    ws_port: Option<u16>,
) -> Result<(Site, String)> {
    SITE_CONTENT.write().unwrap().clear();

    let mut site = Site::new(root_dir, config_file)?;
    let address = format!("localhost:{}", port);

    let base_url = if base_url == "/" {
        String::from("/")
    } else {
        let base_address = format!("{}:{}", base_url, port);

        if site.config.base_url.ends_with('/') {
            format!("http://{}/", base_address)
        } else {
            format!("http://{}", base_address)
        }
    };

    site.enable_serve_mode();
    site.set_base_url(base_url);
    site.set_output_path(output_dir);
    site.load()?;
    if let Some(p) = ws_port {
        site.enable_live_reload_with_port(p);
    } else {
        site.enable_live_reload(port);
    }
    debug!(
        "Building zola site output path: {}",
        output_dir.to_str().unwrap_or_default()
    );
    site.build()?;
    Ok((site, address))
}

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
