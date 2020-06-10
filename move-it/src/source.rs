use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SourceDescription {
    pub base: PathBuf,
    pub offset: PathBuf,
}

impl SourceDescription {
    pub fn new(base: &str, offset: &str) -> Self {
        Self {
            base: PathBuf::from(base),
            offset: PathBuf::from(offset),
        }
    }

    pub fn source_path(&self) -> PathBuf {
        Path::join(&self.base, &self.offset)
    }
}
