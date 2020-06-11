use super::SourceDescription;
use super::SourceIterator;

pub struct Each<'a> {
    sources: Box<dyn Iterator<Item = Box<SourceIterator>> + 'a>,
    current: Option<Box<SourceIterator>>,
}

impl<'a> Each<'a> {
    pub fn new(sources: Box<dyn Iterator<Item = Box<SourceIterator>>>) -> Self {
        Self {
            sources,
            current: None,
        }
    }

    fn next_item(&mut self) -> Option<SourceDescription> {
        self.current
            .as_deref_mut()
            .and_then(|current| current.next())
    }
}

impl<'a> Iterator for Each<'a> {
    type Item = SourceDescription;
    fn next(&mut self) -> Option<SourceDescription> {
        let mut result = self.next_item();

        while result.is_none() {
            self.current = self.sources.next();
            result = self.next_item();
        }

        result
    }
}
