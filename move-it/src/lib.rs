//! It moves files from one folder to an other.
//!
//! Example
//!
//! ```
//! let src = vec![SourceDescription::new("/tmp/in", "foo.txt")];
//! let engine = Engine::new(Box::new(src.iter()), OsString::from("/tmp/out"), OsString::from("{file_name}"));
//! asdf
//! ```

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

pub mod action;
pub mod configuration;
pub mod destination;
pub mod engine;
pub mod errors;
pub mod source;

pub use engine::Engine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let src: Vec<String> = vec![
            String::from("/tmp/in/foo.txt"),
            String::from("/tmp/in/foo2.txt"),
        ];
        let engine = Engine::from_args(Box::new(src.into_iter())).expect("could not create Engine");
        engine.run().expect("something failed");
    }
}
