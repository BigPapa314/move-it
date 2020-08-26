use futures::{stream, Stream, StreamExt}; // 0.3.1
use std::{io, path::PathBuf};
use tokio::fs::{self, DirEntry}; // 0.2.4

use crate::element::Element;

pub fn all_files_recursive(
    path: impl Into<PathBuf>,
) -> impl futures::Stream<Item = Element> + Send {
    let path = path.into();
    let dir_entries = visit(path.clone());
    dir_entries.map(move |entry| Element::create(path.clone(), entry.unwrap()))
}

fn visit(path: impl Into<PathBuf>) -> impl Stream<Item = io::Result<DirEntry>> + Send + 'static {
    async fn one_level(path: PathBuf, to_visit: &mut Vec<PathBuf>) -> io::Result<Vec<DirEntry>> {
        let mut dir = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            if child.metadata().await?.is_dir() {
                to_visit.push(child.path());
            }

            // println!("adding: {:?}", child);
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
