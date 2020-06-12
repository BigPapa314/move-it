use super::action::*;
use super::destination::*;
use super::errors::*;
use super::source::*;
//use std::fs::copy;

pub struct Engine<'a> {
    source: Box<SourceIterator<'a>>,
    destination: Box<dyn DestinationBuilder + 'a>,
    action: Box<ActionImpl<'a>>,
}

impl<'a> Engine<'a> {
    pub fn new(
        source: Box<dyn Iterator<Item = SourceDescription> + 'a>,
        destination: Box<dyn DestinationBuilder + 'a>,
        action: Box<ActionImpl<'a>>,
    ) -> Self {
        Self {
            source,
            destination,
            action,
        }
    }

    pub fn run(mut self) -> Result<()> {
        for src_desc in self.source {
            let src_path = src_desc.source_path();
            let dst_path = self.destination.build(&src_desc);

            self.action
                .execute(&src_path, &dst_path)
                .chain_err(|| "could not call action")?;
        }
        Ok(())
    }
}
