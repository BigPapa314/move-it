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
                        let to = std::path::PathBuf::from(element.expand(target).await.ok()?);

                        info!("COPY: {:?} -> {:?}", &from, &to);
                        if let Some(parent) = to.parent() {
                            if let Err(e) = fs::create_dir_all(parent).await {
                                error!("COPY: Could not create dir {:?}: {:?}", parent, e);
                            };
                        }

                        if let Err(e) = fs::copy(&from, &to).await {
                            error!("COPY: Could not move from {:?} to {:?}: {:?}", from, to, e);
                        };

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
                        let to = std::path::PathBuf::from(element.expand(target).await.ok()?);

                        info!("MOVE: {:?} -> {:?}", from, to);

                        if let Some(parent) = to.parent() {
                            if let Err(e) = fs::create_dir_all(parent).await {
                                error!("MOVE: Could not create dir {:?}: {:?}", parent, e);
                            };
                        }

                        if let Err(e) = fs::rename(&from, &to).await {
                            error!("MOVE: Could not move from {:?} to {:?}: {:?}", from, to, e);
                        };

                        Some(element)
                    }
                })
                .boxed()
        })
    }
}
