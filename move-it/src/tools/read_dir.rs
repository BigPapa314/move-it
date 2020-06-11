use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ReadDirRecursive {
    dir: ReadDir,
    next: Option<Box<ReadDirRecursive>>,
}

impl ReadDirRecursive {
    fn next_path(&mut self) -> Option<DirEntry> {
        match self.dir.next() {
            Some(next_path) => match next_path {
                Ok(next_path) => Some(next_path),
                _ => return None,
            },
            None => return None,
        }
    }
}

impl Iterator for ReadDirRecursive {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        let mut result = self.next.as_deref_mut().and_then(|next| next.next());

        while result.is_none() {
            let next_path = self.next_path()?;
            let next_path = next_path.path();

            if next_path.is_file() {
                result = Some(next_path);
            } else if next_path.is_dir() {
                let next_dir = match fs::read_dir(next_path) {
                    Ok(next_dir) => next_dir,
                    _ => continue,
                };

                self.next = Some(Box::new(ReadDirRecursive::from(next_dir)));

                result = self.next.as_deref_mut().and_then(|next| next.next());
            }
        }

        return result;
    }
}

impl From<ReadDir> for ReadDirRecursive {
    fn from(dir: ReadDir) -> Self {
        Self { dir, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_text() {
        let dir = fs::read_dir("/home/thomas/Downloads").expect("could not read directory");

        for path in ReadDirRecursive::from(dir) {
            println!("{:?}", path);
        }
    }
}
