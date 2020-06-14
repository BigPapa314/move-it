pub mod always_true;
pub mod and;
pub mod not;
pub mod or;
pub mod regex;

use super::SourceDescription;
use super::SourceIterator;

pub type FilterType<'a> = dyn Filter + 'a;

pub trait Filter {
    fn matches(&mut self, src: &SourceDescription) -> bool;
}

pub struct SourceFilter<'a> {
    source: Box<SourceIterator<'a>>,
    filter: Box<FilterType<'a>>,
}

impl<'a> SourceFilter<'a> {
    pub fn new(source: Box<SourceIterator<'a>>, filter: Box<FilterType<'a>>) -> Self {
        Self { source, filter }
    }
}

impl<'a> Iterator for SourceFilter<'a> {
    type Item = SourceDescription;
    fn next(&mut self) -> Option<SourceDescription> {
        let mut result;
        loop {
            result = self.source.next()?;
            let check = self.filter.matches(&result);
            println!("{} -> {}", result.source_path().to_string_lossy(), check);
            if check {
                break;
            }
        }

        Some(result)
    }
}
