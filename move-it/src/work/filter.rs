use futures::{future, StreamExt}; // 0.3.1

use super::Work;
use crate::result::Result;

impl<'a> Work<'a> {
    pub fn include(self, re: impl Into<String>) -> Result<Work<'a>> {
        let re = regex::Regex::new(&re.into())?;

        self.add_work(move |elements| {
            elements
                .filter(move |element| {
                    future::ready(
                        re.is_match(element.get_file().path().as_path().to_str().unwrap()),
                    )
                })
                .boxed()
        })
    }

    pub fn exclude(self, re: impl Into<String>) -> Result<Work<'a>> {
        let re = regex::Regex::new(&re.into())?;

        self.add_work(move |elements| {
            elements
                .filter(move |element| {
                    future::ready(
                        !re.is_match(element.get_file().path().as_path().to_str().unwrap()),
                    )
                })
                .boxed()
        })
    }
}
