mod action;
mod element;
mod filter;
mod producer;

use element::Element;
use futures::StreamExt; // 0.3.1

type Elements<'a> = futures::stream::BoxStream<'a, Element>;

pub struct Work<'a> {
    elements: Elements<'a>,
}

impl<'a> Work<'a> {
    pub fn new() -> Self {
        Self {
            elements: futures::stream::empty().boxed(),
        }
    }

    pub async fn execute(self) {
        self.elements
            .for_each(|x| async move {
                drop(x);
            })
            .await;
    }

    pub fn add_work<Adder>(&mut self, adder: Adder)
    where
        Adder: FnOnce(Elements<'a>) -> Elements<'a>,
    {
        let mut elements = futures::stream::empty().boxed();
        std::mem::swap(&mut self.elements, &mut elements);
        self.elements = adder(elements);
    }
}
