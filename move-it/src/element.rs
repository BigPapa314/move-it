use std::path::PathBuf;
use tokio::fs::DirEntry;

pub struct Element {
    source: PathBuf,
    from: DirEntry,
}

impl Element {
    pub fn create(source: impl Into<PathBuf>, from: DirEntry) -> Self {
        Self {
            source: source.into(),
            from,
        }
    }

    pub fn get_from(&self) -> PathBuf {
        self.from.path()
    }

    pub fn get_to(&self, target: impl Into<PathBuf>) -> PathBuf {
        let from = self.get_from();
        let from = from.strip_prefix(&self.source).unwrap();
        target.into().join(from)
    }
}

//pub type ElementStream = impl futures::Stream<Item = Element> + Send + 'static;
