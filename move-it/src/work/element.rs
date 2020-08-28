use std::path::PathBuf;
use tokio::fs::DirEntry;

use crate::result;
use crate::result::Result;
use regex::Regex;

pub struct Element {
    pub base: PathBuf,
    pub file: DirEntry,
}

impl Element {
    pub fn create(base: impl Into<PathBuf>, file: DirEntry) -> Self {
        Self {
            base: base.into(),
            file,
        }
    }

    pub fn get_file(&self) -> &DirEntry {
        &self.file
    }

    pub async fn expand(&self, src: impl Into<String>) -> Result<String> {
        lazy_static! {
            static ref SIMPLE: Regex = Regex::new(r"\{([^:\}]*):([^:\}]*)\}").unwrap();
            static ref COMPLEX: Regex =
                Regex::new(r"\{([^:\}]*):([^:\}]*):([^:\}]*):([^\}]*)\}").unwrap();
        }

        let src = src.into();
        let src = shellexpand::full(&src)?;
        let src = src.as_ref();

        let mut result = String::from(src);

        // cannot move captures_iter result to future
        // so the results are copied to a vector
        let caps = SIMPLE.captures_iter(&src).collect::<Vec<_>>();
        async {
            for cap in caps {
                if let Ok(var) = self
                    .get_variable(cap[1].to_string(), cap[2].to_string())
                    .await
                {
                    result = result.replace(&cap[0], &var);
                }
            }
        }
        .await;

        // cannot move captures_iter result to future
        // so the results are copied to a vector
        let caps = COMPLEX.captures_iter(&src).collect::<Vec<_>>();
        async {
            for cap in caps {
                if let Ok(var) = self
                    .get_variable(cap[1].to_string(), cap[2].to_string())
                    .await
                {
                    if let Ok(re) = Regex::new(&cap[3]) {
                        result = result.replace(&cap[0], &re.replace_all(&var, &cap[4]));
                    }
                }
            }
        }
        .await;

        Ok(result)
    }

    async fn get_variable(&self, provider: String, key: String) -> Result<String> {
        match provider.as_str() {
            "RAW" => self.get_raw_variable(key).await,
            "FILE" => self.get_file_variable(key).await,
            _ => Err(result::error(format!("Provider '{}' not found!", provider))),
        }
    }

    async fn get_raw_variable(&self, key: String) -> Result<String> {
        Ok(key.to_string())
    }
    async fn get_file_variable(&self, key: String) -> Result<String> {
        match key.as_str() {
            "NAME" => Ok(self
                .file
                .file_name()
                .to_str()
                .ok_or(result::error("convert error!"))?
                .to_string()),
            "STEM" => Ok(self
                .file
                .path()
                .file_stem()
                .ok_or(result::error("convert error!"))?
                .to_str()
                .ok_or(result::error("convert error!"))?
                .to_string()),
            "EXT" => Ok(self
                .file
                .path()
                .extension()
                .ok_or(result::error("convert error!"))?
                .to_str()
                .ok_or(result::error("convert error!"))?
                .to_string()),
            "SIZE" => {
                let meta_data = self.file.metadata().await?;
                Ok(meta_data.len().to_string())
            }
            "RELPATH" => {
                let path = self.file.path();
                let path = path.parent().unwrap();

                let mut result = path.strip_prefix(&self.base)?.to_string_lossy().to_string();
                if result.ends_with('/') || result.ends_with('\\') {
                    result.pop();
                }

                Ok(result)
            }
            _ => Err(result::error(format!("'FILE': Key '{}' not found!", key))),
        }
    }
}

//pub type ElementStream = impl futures::Stream<Item = Element> + Send + 'static;
