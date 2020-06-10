mod options;

use super::action::*;
use super::destination::*;
pub use super::engine::Engine;
use super::errors::*;
use super::source::*;

// use options::Options;

impl<'a> Engine<'a> {
    pub fn from_args(args: Box<dyn Iterator<Item = String> + 'a>) -> Result<Engine<'a>> {
        let src = args.map(|arg| SourceDescription::new("", &arg));
        Ok(Engine::new(
            Box::new(src.into_iter()),
            Box::new(SimpleDestinationBuilder::new("/tmp/out")),
            Box::new(Echo()),
        ))
    }

    // pub fn from_args(args: Box<dyn Iterator<Item = String> + 'a>) -> Result<Engine<'a>> {
    //     let options = Options::from_iter(args.into_iter());

    //     // let src = args.map(|arg| SourceDescription::new("", &arg));
    //     // Ok(Engine::new(
    //     //     Box::new(src.into_iter()),
    //     //     Box::new(SimpleDestinationBuilder::new("/tmp/out")),
    //     //     Box::new(Echo()),
    //     // ))
    // }
}
