use std::path::PathBuf;

pub struct Element {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl Element {
    pub fn Create(from: impl Into<PathBuf>) -> Self {
        Self {
            from: from.into(),
            to: PathBuf::default(),
        }
    }
}

//pub type ElementStream = impl futures::Stream<Item = Element> + Send + 'static;
