pub mod env;
use super::super::source::SourceDescription;

pub trait InfoProvider {
    fn reset(&mut self);
    fn get(&mut self, source: &SourceDescription, key: &String) -> Option<String>;
}
