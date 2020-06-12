use super::action::*;
use super::destination::*;
use super::errors::*;
use super::source::*;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub struct Engine<'a> {
    source: Box<SourceIterator<'a>>,
    destination: Box<dyn DestinationBuilder + 'a>,
    action: Box<ActionImpl<'a>>,
    create_target_dir: Option<PathBuf>,
}

impl<'a> Engine<'a> {
    pub fn new(
        source: Box<dyn Iterator<Item = SourceDescription> + 'a>,
        destination: Box<dyn DestinationBuilder + 'a>,
        action: Box<ActionImpl<'a>>,
        create_target_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            source,
            destination,
            action,
            create_target_dir,
        }
    }

    pub fn run(mut self) -> Result<()> {
        if let Some(target_dir) = self.create_target_dir {
            create_dir_all(target_dir).chain_err(|| "unable to create target directory")?;
        }

        for src_desc in self.source {
            let src_path = src_desc.source_path();
            let dst_path = self.destination.build(&src_desc);

            self.action.execute(&src_path, &dst_path)?;
        }

        Ok(())
    }
}
