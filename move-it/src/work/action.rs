use futures::StreamExt; // 0.3.1

use crate::result::Result;
use crate::work::Work;
use log::*;
use tokio::fs;

impl<'a> Work<'a> {
    pub fn echo(self, target: impl Into<String>) -> Result<Work<'a>> {
        let target = target.into();

        self.add_work(move |elements| {
            elements
                .filter_map(move |element| {
                    let target = target.clone();
                    async move {
                        println!(
                            "{:?} -> {:?}",
                            element.get_file().path(),
                            element.expand(target).await.unwrap()
                        );

                        Some(element)
                    }
                })
                .boxed()
        })
    }

    pub fn copy(self, target: impl Into<String>) -> Result<Work<'a>> {
        let target = target.into();

        self.add_work(move |elements| {
            elements
                .filter_map(move |element| {
                    let target = target.clone();
                    async move {
                        let from = element.get_file().path();
                        let to = element.expand(target).await.unwrap();

                        info!("COPY: {:?} -> {:?}", from, to);
                        let _ = fs::copy(from, to).await.ok()?;

                        Some(element)
                    }
                })
                .boxed()
        })
    }

    pub fn r#move(self, target: impl Into<String>) -> Result<Work<'a>> {
        let target = target.into();

        self.add_work(move |elements| {
            elements
                .filter_map(move |element| {
                    let target = target.clone();
                    async move {
                        let from = element.get_file().path();
                        let to = element.expand(target).await.unwrap();

                        info!("MOVE: {:?} -> {:?}", from, to);
                        let _ = fs::rename(from, to).await.ok()?;

                        Some(element)
                    }
                })
                .boxed()
        })
    }
}
