use super::source::*;
use std::path::Path;
use std::path::PathBuf;

pub trait DestinationBuilder {
    fn build(&self, source: &SourceDescription) -> PathBuf;
}

pub struct SimpleDestinationBuilder {
    destination: PathBuf,
}

impl SimpleDestinationBuilder {
    pub fn new(destination: PathBuf) -> Self {
        Self { destination }
    }
}

impl DestinationBuilder for SimpleDestinationBuilder {
    fn build(&self, source: &SourceDescription) -> PathBuf {
        Path::join(&self.destination, &source.offset)
    }
}
