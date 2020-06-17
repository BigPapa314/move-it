mod options;

use super::action::*;
use super::destination;
pub use super::engine::Engine;
use super::errors::*;
use super::source::*;
use shellexpand;
use std::path::PathBuf;
use std::process::exit;

use options::*;

impl<'a> Engine<'a> {
    pub fn from_args(args: Box<dyn Iterator<Item = String> + 'a>) -> Result<Engine<'a>> {
        let opt = Options::from_iter(args);

        let command = opt.command.unwrap_or(Command::Move);

        // check options
        if opt.paths.len() < 1 || (opt.target.is_none() && opt.paths.len() < 2) {
            let _ = Options::clap().print_help();
            return Err(format!("opt.paths.len: {}", opt.paths.len()).into());
        }

        let mut paths = opt.paths.clone().into_iter();

        let target = match &opt.target {
            Some(target) => target.clone(),
            None => paths.next_back().unwrap(),
        };

        let source = Box::new(each::Each::new(Box::new(
            paths
                .map(move |path| -> Box<SourceIterator> {
                    let path = PathBuf::from(String::from(
                        shellexpand::full(&path.to_string_lossy()).expect("could not parse string"),
                    ));
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

        let verbose = opt.verbose > 0;
        let create_target_dir = opt.create_target_dir.unwrap_or(false);
        let disable_dir_creation = opt.disable_dir_creation.unwrap_or(false);

        let action: Box<ActionImpl<'a>> = match command {
            Command::Echo => Box::new(Echo()),
            Command::Copy => Box::new(Copy::new(verbose, !disable_dir_creation)),
            Command::Move => Box::new(Move::new(verbose, !disable_dir_creation)),
        };

        let create_target_dir = if create_target_dir {
            Some(target.clone())
        } else {
            None
        };

        let include_filter = opt.include.as_deref().map(|rgx| {
            Box::new(filter::Or::new(
                rgx.split(&opt.include_delimiter)
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .into_iter()
                    .map(|rgx| {
                        let res: Box<filter::FilterType<'a>> = Box::new(filter::Regex::new(
                            regex::Regex::new(&rgx).expect("could not parse regex"),
                        ));
                        res
                    })
                    .collect(),
            ))
        });

        let exclude_filter = opt.exclude.as_deref().map(|rgx| {
            Box::new(filter::Not::new(Box::new(filter::Or::new(
                rgx.split(&opt.exclude_delimiter)
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .into_iter()
                    .map(|rgx| {
                        let res: Box<filter::FilterType<'a>> = Box::new(filter::Regex::new(
                            regex::Regex::new(&rgx).expect("could not parse regex"),
                        ));
                        res
                    })
                    .collect(),
            ))))
        });

        let filter: Box<filter::FilterType<'_>> = match (include_filter, exclude_filter) {
            (Some(include), Some(exclude)) => Box::new(filter::And::new(vec![include, exclude])),
            (_, Some(exclude)) => exclude,
            (Some(include), _) => include,
            _ => Box::new(filter::AlwaysTrue()),
        };

        let source_iterator = Box::new(filter::SourceFilter::new(source, filter));

        let destination_builder: Box<destination::DestinationBuilderImpl<'_>> =
            match opt.destination {
                Some(destination_pattern) => {
                    Box::new(destination::Mapped::new(target, destination_pattern))
                }
                None => Box::new(destination::Simple::new(target)),
            };

        Ok(Engine::new(
            source_iterator,
            destination_builder,
            action,
            create_target_dir,
        ))
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
    fn copy() {
        let src: Vec<String> = vec![
            String::from("mi"),
            String::from("-c"),
            String::from("echo"),
            String::from("-i"),
            String::from("/2."),
            String::from("~/new"),
            String::from("~/new_copy"),
        ];

        let engine = Engine::from_args(Box::new(src.into_iter())).expect("could not create engine");
        engine.run().expect("engine run failed");
    }
}
