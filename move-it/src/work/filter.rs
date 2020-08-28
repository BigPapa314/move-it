use futures::{future, StreamExt}; // 0.3.1

use super::Work;
use regex::Regex;

impl<'a> Work<'a> {
    pub fn include(&mut self, re: Regex) {
        self.add_work(|elements| {
            elements
                .filter(move |element| {
                    future::ready(
                        re.is_match(element.get_file().path().as_path().to_str().unwrap()),
                    )
                })
                .boxed()
        });
    }

    pub fn exclude(&mut self, re: Regex) {
        self.add_work(|elements| {
            elements
                .filter(move |element| {
                    future::ready(
                        !re.is_match(element.get_file().path().as_path().to_str().unwrap()),
                    )
                })
                .boxed()
        });
    }
}
