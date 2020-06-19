use super::super::super::source::SourceDescription;
use super::InfoProvider;
use std::env::vars;

pub struct Env();

impl Env {
    pub fn new() -> Self {
        Self()
    }
}

impl InfoProvider for Env {
    fn reset(&mut self) {}
    fn get(&mut self, _source: &SourceDescription, key: &String) -> Option<String> {
        vars().find_map(|(env_key, env_value)| {
            if &env_key == key {
                Some(env_value)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    use std::path::PathBuf;

    #[test]
    fn basic() {
        let mut env = Env::new();

        assert!(
            env.get(
                &SourceDescription::new(PathBuf::new(), PathBuf::new()),
                &String::from("HOME")
            ) == Some(String::from("/home/thomas"))
        );
    }
}
