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
pub mod destination;
pub mod engine;
pub mod errors;
pub mod helper;
pub mod source;

use destination::*;
pub use engine::Engine;
// use errors::*;
use action::*;
use source::*;

/// Placeholder for the move-it entry function
///
/// # Arguments
///
/// * `none` - Will be added soon.
///
/// *Note*: This is a placeholder function that will be replaced as some logic is added.
pub fn engine_from_args<'a>(args: &'a mut dyn Iterator<Item = impl AsRef<str>>) -> Engine<'a> {
    //let argv = args.collect::<Vec<String>>();

    let src = args.map(|arg| SourceDescription::new("", arg.as_ref()));

    // let src = vec![
    //     SourceDescription::new("", &argv[1]),
    //     SourceDescription::new("", &argv[2]),
    // ];
    Engine::new(
        Box::new(src.into_iter()),
        Box::new(SimpleDestinationBuilder::new("/tmp/out")),
        Box::new(Echo()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let src = vec!["/tmp/in/foo.txt", "/tmp/in/foo2.txt"];
        let mut src_iter = src.iter();
        let engine = engine_from_args(&mut src_iter);
        engine.run().expect("something failed");
    }
}
