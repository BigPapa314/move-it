use super::super::errors::*;
use super::super::tools::read_dir::ReadDirRecursive;
use super::SourceDescription;
use std::fs;
use std::path::PathBuf;

pub struct Directory {
    base: PathBuf,
    iter: ReadDirRecursive,
}

impl Directory {
    pub fn new(dir: PathBuf) -> Result<Self> {
        let rd = fs::read_dir(&dir).chain_err(|| "could not open directory")?;

        Ok(Self {
            base: dir,
            iter: ReadDirRecursive::from(rd),
        })
    }
}

impl Iterator for Directory {
    type Item = SourceDescription;
    fn next(&mut self) -> Option<SourceDescription> {
        let mut next = self.iter.next()?;
        next = match next.strip_prefix(&self.base) {
            Ok(next) => next.to_path_buf(),
            _ => return None,
        };

        Some(SourceDescription::new(self.base.clone(), next))
    }
}
