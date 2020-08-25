use futures::{future, StreamExt}; // 0.3.1

use crate::element::Element;
use regex::Regex;

pub fn include<'a>(
    re: Regex,
    elements: impl futures::Stream<Item = Element> + Send + 'a,
) -> impl futures::Stream<Item = Element> + Send + 'a {
    elements
        .filter(move |element| future::ready(re.is_match(element.from.as_path().to_str().unwrap())))
}

pub fn exclude<'a>(
    re: Regex,
    elements: impl futures::Stream<Item = Element> + Send + 'a,
) -> impl futures::Stream<Item = Element> + Send + 'a {
    elements.filter(move |element| {
        future::ready(!re.is_match(element.from.as_path().to_str().unwrap()))
    })
}
