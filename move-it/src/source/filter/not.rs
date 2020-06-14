use super::super::SourceDescription;
use super::Filter;
use super::FilterType;

pub struct Not<'a> {
    filter: Box<FilterType<'a>>,
}

impl<'a> Not<'a> {
    pub fn new(filter: Box<FilterType<'a>>) -> Self {
        Self { filter }
    }
}

impl<'a> Filter for Not<'a> {
    fn matches(&mut self, src: &SourceDescription) -> bool {
        !self.filter.matches(src)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn basic() {}
}
