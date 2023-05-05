use std::path::Path;

use crate::prelude::*;

pub mod filesystem;
pub mod github_pages;

pub trait PublishingBackend {
    /// The configuration associated to this backend.
    type Config;

    /// Publish the zip file at `export_path` to the backend with the given config.
    fn publish(export_path: &Path, config: Self::Config) -> Result<()>;
}
