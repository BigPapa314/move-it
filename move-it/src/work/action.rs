use futures::StreamExt; // 0.3.1
use std::path::PathBuf;

use crate::work::Work;

impl<'a> Work<'a> {
    pub fn echo(&mut self, target: impl Into<PathBuf>) {
        let target = target.into();

        self.add_work(|elements| {
            elements
                .map(move |element| {
                    println!(
                        "{:?} -> {:?}",
                        element.get_file().path(),
                        element.expand(target.clone().as_path().to_str().unwrap())
                    );
                    element
                })
                .boxed()
        });
    }
}
