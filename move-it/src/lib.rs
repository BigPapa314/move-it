//! It moves files from one folder to an other.

use log::*;
use std::io::Write;

mod mv;
mod result;

pub fn mv(from: String, to: String) {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(mv::mv(from, to)) {
        Ok(_) => info!("Done"),
        Err(e) => error!("An error ocurred: {}", e),
    };
}
