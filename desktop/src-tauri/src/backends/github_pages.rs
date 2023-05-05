use std::path::Path;

pub struct GithubPagesBackend;

impl super::PublishingBackend for GithubPagesBackend {
    type Config = ();

    fn publish(_export_path: &Path, _config: Self::Config) -> crate::prelude::Result<()> {
        todo!("Implement GitHub Pages publishing");
    }
}
