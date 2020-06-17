use super::super::source::SourceDescription;
use super::DestinationBuilder;
use std::path::Path;
use std::path::PathBuf;

pub struct Simple {
    destination: PathBuf,
}

impl Simple {
    pub fn new(destination: PathBuf) -> Self {
        Self { destination }
    }
}

impl DestinationBuilder for Simple {
    fn build(&self, source: &SourceDescription) -> PathBuf {
        Path::join(&self.destination, &source.offset)
    }
}
