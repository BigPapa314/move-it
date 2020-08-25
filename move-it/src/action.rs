use futures::StreamExt; // 0.3.1

use crate::element::Element;

pub async fn echo(elements: impl futures::Stream<Item = Element> + Send) {
    elements
        .for_each(|element| async move {
            println!("{:?} -> {:?}", element.from, element.to);
        })
        .await;
}
