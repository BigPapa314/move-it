use super::super::SourceDescription;
use super::Filter;
use super::FilterType;

pub struct And<'a> {
    filters: Box<dyn Iterator<Item = Box<FilterType<'a>>> + 'a>,
}

impl<'a> And<'a> {
    pub fn new(filters: Box<dyn Iterator<Item = Box<FilterType<'a>>> + 'a>) -> Self {
        Self { filters }
    }
}

impl<'a> Filter for And<'a> {
    fn matches(&mut self, src: &SourceDescription) -> bool {
        self.filters.all(|ref mut filter| filter.matches(src))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn basic() {}
}
