use super::super::SourceDescription;
use super::Filter;
use super::FilterType;

pub struct Or<'a> {
    filters: Vec<Box<FilterType<'a>>>,
}

impl<'a> Or<'a> {
    pub fn new(filters: Vec<Box<FilterType<'a>>>) -> Self {
        Self { filters }
    }
}

impl<'a> Filter for Or<'a> {
    fn matches(&mut self, src: &SourceDescription) -> bool {
        self.filters
            .iter_mut()
            .any(|ref mut filter| filter.matches(src))
    }
}
