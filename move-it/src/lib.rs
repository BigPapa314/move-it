//! It moves files from one folder to an other.

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
mod tools;

pub use engine::Engine;
