use super::super::SourceDescription;
use super::Filter;

pub struct AlwaysTrue();

impl<'a> Filter for AlwaysTrue {
    fn matches(&mut self, _src: &SourceDescription) -> bool {
        true
    }
}
