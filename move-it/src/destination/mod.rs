mod mapped;
mod simple;

pub use mapped::Mapped;
pub use simple::Simple;

use super::source::*;
use std::path::PathBuf;

pub type DestinationBuilderImpl<'a> = dyn DestinationBuilder + 'a;

pub trait DestinationBuilder {
    fn build(&self, source: &SourceDescription) -> PathBuf;
}
