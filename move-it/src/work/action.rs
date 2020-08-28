use futures::StreamExt; // 0.3.1

use crate::work::Work;

impl<'a> Work<'a> {
    pub fn echo(&mut self, target: impl Into<String>) {
        let target = target.into();

        self.add_work(|elements| {
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
        });
    }
}
