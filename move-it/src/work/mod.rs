mod action;
mod element;
mod filter;
mod producer;

use element::Element;
use futures::StreamExt; // 0.3.1

use crate::result::Result;

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

    pub async fn execute(self) -> Result<()> {
        self.elements
            .for_each(|x| async move {
                drop(x);
            })
            .await;

        Ok(())
    }

    pub(crate) fn add_work<Adder>(self, adder: Adder) -> Result<Work<'a>>
    where
        Adder: FnOnce(Elements<'a>) -> Elements<'a>,
    {
        Ok(Self {
            elements: adder(self.elements),
        })
    }
}
