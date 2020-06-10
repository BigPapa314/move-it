//use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SourceDescription {
    pub base: PathBuf,
    pub offset: PathBuf,
}

impl SourceDescription {
    pub fn new(base: &str, offset: &str) -> Self {
        Self {
            base: PathBuf::from(base),
            offset: PathBuf::from(offset),
        }
    }

    pub fn source_path(&self) -> PathBuf {
        Path::join(&self.base, &self.offset)
    }
}

// struct FilesystemImpl<'a> {
//     base_path: &'a Path,
//     source: Option<Box<dyn Iterator<Item = &'a OsStr> + 'a>>,
//     next: Option<Box<FilesystemImpl<'a>>>,
// }

// impl<'a> FilesystemImpl<'a> {
//     pub fn new(base_path: &'a Path) -> Self {
//         let source: Option<Box<dyn Iterator<Item = &'a OsStr> + 'a>>;

//         if base_path.is_dir() {
//             source = Some(Box::new(base_path.iter()));
//         } else {
//             source = None
//         }

//         Self {
//             base_path,
//             source,
//             next: None,
//         }
//     }
// }

// pub struct Filesystem<'a> {
//     base_path: PathBuf,
//     filesystem_impl: Option<Box<FilesystemImpl<'a>>>,
// }

// impl<'a> Filesystem<'a> {
//     pub fn new(base_path: PathBuf) -> Self {
//         Self {
//             base_path,
//             filesystem_impl: Some(Box::new(FilesystemImpl::new(base_path.as_path()))),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn help_text() {
//         let path = PathBuf::from("/tmp");
//         let _filesystem = Filesystem::new(path);

//         //println!("{:?}", filesystem);
//     }
// }
