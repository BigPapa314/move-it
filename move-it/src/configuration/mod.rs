mod options;

use super::action::*;
use super::destination::*;
pub use super::engine::Engine;
use super::errors::*;
use super::source::*;
use std::path::PathBuf;
use std::process::exit;

use options::*;

impl<'a> Engine<'a> {
    pub fn from_args(args: Box<dyn Iterator<Item = String> + 'a>) -> Result<Engine<'a>> {
        let opt = Options::from_iter(args);

        // source: Box<dyn Iterator<Item = SourceDescription> + 'a>,
        // destination: Box<dyn DestinationBuilder + 'a>,
        // action: Box<dyn Action + 'a>,

        check_options(&opt);

        let mut paths = opt.paths.clone().into_iter();
        let target = match &opt.target {
            Some(target) => target.clone(),
            None => paths.next_back().unwrap(),
        };

        let source = Box::new(each::Each::new(Box::new(
            paths
                .map(|path| -> Box<SourceIterator> {
                    if path.is_dir() {
                        match directory::Directory::new(path) {
                            Ok(dir) => Box::new(dir),
                            Err(_error) => {
                                let _ = Options::clap().print_help();
                                exit(-1);
                            }
                        }
                    } else if path.is_file() {
                        Box::new(vec![SourceDescription::new(PathBuf::from(""), path)].into_iter())
                    } else {
                        Box::new(Vec::<SourceDescription>::new().into_iter())
                    }
                })
                .into_iter(),
        )));

        let action: Box<ActionImpl<'a>> = match opt.command {
            Some(Command::Echo) => Box::new(Echo()),
            Some(Command::Copy) => Box::new(Copy()),
            Some(Command::Move) | None => Box::new(Move()),
        };

        let destination = Box::new(SimpleDestinationBuilder::new(target));

        Ok(Engine::new(source, destination, action))
    }
}

fn check_options(opt: &Options) {
    if opt.paths.len() < 1 || (opt.target.is_none() && opt.paths.len() < 2) {
        let _ = Options::clap().print_help();
        exit(-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_help_text() {
        let src: Vec<String> = vec![String::from("mi"), String::from("-h")];

        let engine = Engine::from_args(Box::new(src.into_iter())).expect("could not create engine");
        engine.run().expect("engine run failed");
    }

    #[test]
    fn long_help_text() {
        let src: Vec<String> = vec![String::from("mi"), String::from("--help")];

        let engine = Engine::from_args(Box::new(src.into_iter())).expect("could not create engine");
        engine.run().expect("engine run failed");
    }

    #[test]
    fn simple_move() {
        let src: Vec<String> = vec![
            String::from("mi"),
            String::from("/home/thomas/Downloads"),
            String::from("/home/thomas/new"),
        ];

        let engine = Engine::from_args(Box::new(src.into_iter())).expect("could not create engine");
        engine.run().expect("engine run failed");
    }
}
