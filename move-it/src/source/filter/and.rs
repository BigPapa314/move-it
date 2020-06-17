use super::super::SourceDescription;
use super::Filter;
use super::FilterType;

pub struct And<'a> {
    filters: Vec<Box<FilterType<'a>>>,
}

impl<'a> And<'a> {
    pub fn new(filters: Vec<Box<FilterType<'a>>>) -> Self {
        Self { filters }
    }
}

impl<'a> Filter for And<'a> {
    fn matches(&mut self, src: &SourceDescription) -> bool {
        self.filters
            .iter_mut()
            .all(|ref mut filter| filter.matches(src))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn basic() {}
}
