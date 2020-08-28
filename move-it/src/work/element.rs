use std::path::PathBuf;
use tokio::fs::DirEntry;

pub struct Element {
    pub base: PathBuf,
    pub file: DirEntry,
}

impl Element {
    pub fn create(base: impl Into<PathBuf>, file: DirEntry) -> Self {
        Self {
            base: base.into(),
            file,
        }
    }

    pub fn get_file(&self) -> &DirEntry {
        &self.file
    }

    pub fn expand(&self, src: impl Into<String>) -> String {
        let file = self.file.path();
        let file = file.strip_prefix(&self.base).unwrap();
        String::from(PathBuf::from(src.into()).join(file).to_str().unwrap())
    }
}

//pub type ElementStream = impl futures::Stream<Item = Element> + Send + 'static;
