use super::super::source::SourceDescription;
use super::DestinationBuilder;
use std::path::Path;
use std::path::PathBuf;

pub struct Mapped {
    target: PathBuf,
    destination_pattern: String,
}

impl Mapped {
    pub fn new(target: PathBuf, destination_pattern: String) -> Self {
        Self {
            target,
            destination_pattern,
        }
    }
}

impl DestinationBuilder for Mapped {
    fn build(&self, source: &SourceDescription) -> PathBuf {
        Path::join(&self.target, &source.offset)
    }
}
