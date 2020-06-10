//! It moves files from one folder to an other.

use move_it;
use move_it::errors::Result;
use move_it::Engine;
use std::env;

fn main() -> Result<()> {
    let engine = Engine::from_args(Box::new(env::args()))?;
    engine.run()
}
