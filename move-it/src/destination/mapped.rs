use super::super::source::SourceDescription;
use super::super::tools::info_provider::InfoProvider;
use super::DestinationBuilder;
use regex;
use std::path::Path;
use std::path::PathBuf;

pub type InfoProviders<'a> = Vec<Box<dyn InfoProvider + 'a>>;

pub struct Mapped<'a> {
    simple_regex: regex::Regex,
    complex_regex: regex::Regex,
    target: PathBuf,
    destination_pattern: String,
    info_providers: InfoProviders<'a>,
}

impl<'a> Mapped<'a> {
    pub fn new(target: PathBuf, destination_pattern: String) -> Self {
        Self {
            simple_regex: regex::Regex::new(r"(\$\{(\w+)\})").unwrap(),
            complex_regex: regex::Regex::new(r"(\$(\w+))").unwrap(),
            target,
            destination_pattern,
            info_providers: InfoProviders::<'a>::new(),
        }
    }

    fn add(&mut self, info_provider: Box<dyn InfoProvider + 'a>) {
        self.info_providers.push(info_provider);
    }

    fn reset(&mut self) {
        self.info_providers
            .iter_mut()
            .for_each(|provider| provider.reset());
    }

    fn get(
        _info_providers: &mut InfoProviders<'a>,
        _source: &SourceDescription,
        _key: &str,
    ) -> Option<String> {
        None
    }
}

impl<'a> DestinationBuilder for Mapped<'a> {
    fn build(&mut self, source: &SourceDescription) -> PathBuf {
        self.reset();

        let pattern =
            String::from(Path::join(&self.target, &self.destination_pattern).to_string_lossy());
        // let mut target = pattern.clone();

        let mut info_providers = &mut self.info_providers;

        let mut target = self
            .simple_regex
            .replace_all(&pattern, |caps: &regex::Captures| {
                Mapped::<'a>::get(&mut info_providers, source, &caps[2])
                    .expect(&format!("Key '{}' not found!", &caps[2]))
            });

        let target2 = self
            .complex_regex
            .replace_all(&target, |caps: &regex::Captures| {
                Mapped::<'a>::get(&mut info_providers, source, &caps[2])
                    .expect(&format!("Key '{}' not found!", &caps[2]))
            });

        PathBuf::from(String::from(target2.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tools::info_provider::env::Env;
    use super::*;
    use assert2::assert;
    //use std::path::PathBuf;

    #[test]
    fn basic() {
        let re1 = regex::Regex::new(r"(\$\{(\w+)\})").unwrap();
        let re2 = regex::Regex::new(r"(\$(\w+))").unwrap();

        let source_string = String::from("Test ${strange} test $strange2.");
        let mut result_string = source_string.clone();

        for cap in re1.captures_iter(&source_string) {
            println!("'{}'", &cap[2]);
            result_string = result_string.replace(&cap[1], "home");
        }

        for cap in re2.captures_iter(&source_string) {
            println!("'{}'", &cap[2]);
            result_string = result_string.replace(&cap[1], "home");
        }

        println!("'{}'", &result_string);
    }

    #[test]
    fn mapped() {
        let mut mapped = Mapped::new(
            PathBuf::from("/$HOME/hallo"),
            String::from("$HOME/value.txt"),
        );

        mapped.add(Box::new(Env::new()));

        let target = mapped.build(&SourceDescription::new(
            PathBuf::from("/home/test"),
            PathBuf::from("some/other.txt"),
        ));

        println!("'{}'", target.to_string_lossy());
    }
}
