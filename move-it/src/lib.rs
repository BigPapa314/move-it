//! It moves files from one folder to an other.

#[macro_use]
extern crate lazy_static;

mod result;
mod work;

pub use work::Work;

// async fn test(from: impl Into<String>, to: impl Into<String>) -> Result<()> {
//     Work::new()
//         .all_files_recursive(from)?
//         .include(r"/test2")?
//         .exclude(r"/test3")?
//         .echo(to)?
//         .execute()
//         .await?;

//     Ok(())
// }

// pub fn mv(from: String, to: String) {
//     let start = std::time::Instant::now();
//     env_logger::Builder::from_default_env()
//         .format(move |buf, rec| {
//             let t = start.elapsed().as_secs_f32();
//             writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
//         })
//         .init();

//     let mut rt = tokio::runtime::Runtime::new().unwrap();

//     match rt.block_on(test(from, to)) {
//         Ok(_) => info!("Done"),
//         Err(e) => error!("An error ocurred: {}", e),
//     };
// }
