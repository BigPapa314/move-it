mod options;

use super::action::*;
use super::destination::*;
pub use super::engine::Engine;
use super::errors::*;
use super::source::*;
use std::ffi::OsString;
use std::path::PathBuf;

use options::*;

impl<'a> Engine<'a> {
    pub fn from_args(args: Box<dyn Iterator<Item = String> + 'a>) -> Result<Engine<'a>> {
        let opt = Options::from_iter(args);

        // source: Box<dyn Iterator<Item = SourceDescription> + 'a>,
        // destination: Box<dyn DestinationBuilder + 'a>,
        // action: Box<dyn Action + 'a>,

        let source = source_from_options(&opt);

        Ok(Engine::new(
            source,
            Box::new(SimpleDestinationBuilder::new("/tmp/out")),
            Box::new(Echo()),
        ))
    }
}

fn source_from_options<'a, 'b>(
    opt: &'a Options,
) -> Box<dyn Iterator<Item = SourceDescription> + 'b> {
    // Box::new(
    //     opt.paths
    //         .into_iter()
    //         .map(|arg| SourceDescription::new(PathBuf::from(""), PathBuf::from(&arg)))
    //         .into_iter(),
    // )
    todo!("implement source_from_options")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_text() {
        let src: Vec<String> = vec![String::from("mi"), String::from("--help")];

        let engine = Engine::from_args(Box::new(src.into_iter())).expect("could not create engine");
        engine.run().expect("engine run failed");
    }
}
