pub mod directory;
pub mod each;

use std::path::Path;
use std::path::PathBuf;

pub type SourceIterator = dyn Iterator<Item = SourceDescription>;

#[derive(Debug)]
pub struct SourceDescription {
    pub base: PathBuf,
    pub offset: PathBuf,
}

impl SourceDescription {
    pub fn new(base: PathBuf, offset: PathBuf) -> Self {
        Self { base, offset }
    }

    pub fn source_path(&self) -> PathBuf {
        Path::join(&self.base, &self.offset)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn help_text() {}
}
