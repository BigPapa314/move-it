use futures::StreamExt; // 0.3.1
use std::path::PathBuf;

use crate::element::Element;

pub async fn echo(
    elements: impl futures::Stream<Item = Element> + Send,
    target: impl Into<PathBuf>,
) {
    let target = target.into();
    elements
        .for_each(|element| {
            let target = target.clone();
            async move {
                println!("{:?} -> {:?}", element.get_from(), element.get_to(target));
            }
        })
        .await;
}
