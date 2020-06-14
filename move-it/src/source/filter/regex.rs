use super::super::SourceDescription;
use super::Filter;
use regex;

pub struct Regex {
    regex: regex::Regex,
}

impl Regex {
    pub fn new(regex: regex::Regex) -> Self {
        Self { regex }
    }
}

impl Filter for Regex {
    fn matches(&mut self, src: &SourceDescription) -> bool {
        self.regex.is_match(&src.source_path().to_string_lossy())
    }
}

#[cfg(test)]
mod tests {
    use super::super::SourceDescription;
    use super::*;
    use assert2::assert;
    use std::path::PathBuf;

    #[test]
    fn basic() {
        let mut r = Regex::new(regex::Regex::new(r"^\d{4}$").unwrap());
        assert!(r.matches(&SourceDescription::new(
            PathBuf::from(""),
            PathBuf::from("1234")
        )));
    }
}
