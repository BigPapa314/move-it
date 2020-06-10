//! It moves files from one folder to an other.

use move_it;
use std::env;

use move_it::errors::*;

fn main() -> Result<()> {
    let mut args = env::args();
    let engine = move_it::engine_from_args(&mut args);
    engine.run()
}
