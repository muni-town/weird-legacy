use std::path::{Path, PathBuf};

use crate::prelude::*;

pub struct FilesystemBackend;

impl super::PublishingBackend for FilesystemBackend {
    type Config = PathBuf;

    fn publish(export_path: &Path, target_path: PathBuf) -> Result<()> {
        // Simply move the export file to the path the user selected.
        std::fs::rename(export_path, target_path)?;

        Ok(())
    }
}
