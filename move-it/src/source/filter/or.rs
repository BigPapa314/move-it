use super::super::SourceDescription;
use super::Filter;
use super::FilterType;

pub struct Or<'a> {
    filters: Box<dyn Iterator<Item = Box<FilterType<'a>>> + 'a>,
}

impl<'a> Or<'a> {
    pub fn new(filters: Box<dyn Iterator<Item = Box<FilterType<'a>>> + 'a>) -> Self {
        Self { filters }
    }
}

impl<'a> Filter for Or<'a> {
    fn matches(&mut self, src: &SourceDescription) -> bool {
        self.filters.any(|ref mut filter| filter.matches(src))
    }
}
