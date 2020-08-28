//! It moves files from one folder to an other.

#[macro_use]
extern crate lazy_static;

mod result;
mod work;

use log::*;
use std::io::Write;

use crate::result::Result;

use work::*;

async fn test(from: impl Into<String>, to: impl Into<String>) -> Result<()> {
    let from0 = from.into();
    let from1 = shellexpand::full(&from0)?;
    let from = from1.as_ref();

    println!("{:?}", from);

    let from = std::path::PathBuf::from(from);
    //let to = std::path::PathBuf::from(shellexpand::full(&to.into())?.as_ref());

    let mut work = Work::new();
    work.all_files_recursive(from);

    //work.include(regex::Regex::new(r"/test2").unwrap());
    //work.exclude(regex::Regex::new(r"/test3").unwrap());

    work.echo(to.into());

    work.execute().await;

    // let source = producer::all_files_recursive(from);

    // let source = filter::include(regex::Regex::new(r"/test2").unwrap(), source);
    // let source = filter::exclude(regex::Regex::new(r"/test3").unwrap(), source);

    // action::echo(source, to).await;

    Ok(())
}

pub fn mv(from: String, to: String) {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(test(from, to)) {
        Ok(_) => info!("Done"),
        Err(e) => error!("An error ocurred: {}", e),
    };
}
