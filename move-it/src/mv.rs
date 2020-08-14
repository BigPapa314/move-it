use crate::result::Result;

use futures::{stream, Stream, StreamExt}; // 0.3.1
use std::{io, path::PathBuf};
use tokio::fs::{self, DirEntry}; // 0.2.4

pub async fn mv_file(from: impl Into<PathBuf>, to: impl Into<PathBuf>) -> Result<()> {
    println!(
        "{} -> {}",
        from.into().to_str().unwrap(),
        to.into().to_str().unwrap()
    );
    // tokio::fs::rename(from.into(), to.into()).await?;
    Ok(())
}

pub async fn mv(from: impl Into<String>, to: impl Into<String>) -> Result<()> {
    let _from = std::path::PathBuf::from(shellexpand::full(&from.into())?.as_ref());
    let _to = std::path::PathBuf::from(shellexpand::full(&to.into())?.as_ref());

    let sources = visit(&_from);

    sources
        .for_each(|entry| async {
            match entry {
                Ok(entry) => {
                    let source = entry.path();
                    let offest = source.strip_prefix(&_from).unwrap();
                    let destionation = _to.join(offest);

                    if entry.metadata().await.unwrap().is_dir() {
                        println!("creating: {}", destionation.to_str().unwrap());
                        tokio::fs::create_dir_all(destionation).await.unwrap();
                    } else {
                        mv_file(source, destionation).await.unwrap();
                    }
                }
                Err(e) => eprintln!("encountered an error: {}", e),
            }
        })
        .await;

    Ok(())
}

fn visit(path: impl Into<PathBuf>) -> impl Stream<Item = io::Result<DirEntry>> + Send + 'static {
    async fn one_level(path: PathBuf, to_visit: &mut Vec<PathBuf>) -> io::Result<Vec<DirEntry>> {
        let mut dir = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            if child.metadata().await?.is_dir() {
                to_visit.push(child.path());
            }

            println!("adding: {:?}", child);
            files.push(child);
        }

        Ok(files)
    }

    stream::unfold(vec![path.into()], |mut to_visit| async {
        let path = to_visit.pop()?;
        let file_stream = match one_level(path, &mut to_visit).await {
            Ok(files) => stream::iter(files).map(Ok).left_stream(),
            Err(e) => stream::once(async { Err(e) }).right_stream(),
        };

        Some((file_stream, to_visit))
    })
    .flatten()
}
