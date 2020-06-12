use super::errors::*;
use std::fs::{copy, create_dir_all, rename};
use std::path::PathBuf;

pub trait Action {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()>;
}

pub type ActionImpl<'a> = dyn Action + 'a;

pub struct Echo();

impl Action for Echo {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        println!("echo: {:?} -> {:?}", src, dst);
        Ok(())
    }
}

pub struct Copy {
    verbose: bool,
    create_missing_dirs: bool,
}

impl Copy {
    pub fn new(verbose: bool, create_missing_dirs: bool) -> Self {
        Self {
            verbose,
            create_missing_dirs,
        }
    }
}

impl Action for Copy {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        if self.verbose {
            println!("copy: {:?} -> {:?}", src, dst);
        }
        if self.create_missing_dirs {
            create_dir_all(dst.parent().unwrap()).chain_err(|| "unable to create directory")?;
        }
        copy(&src, &dst)
            .chain_err(|| "unable to copy file")
            .and(Ok(()))
    }
}
pub struct Move {
    verbose: bool,
    create_missing_dirs: bool,
}

impl Move {
    pub fn new(verbose: bool, create_missing_dirs: bool) -> Self {
        Self {
            verbose,
            create_missing_dirs,
        }
    }
}

impl Action for Move {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        if self.verbose {
            println!("move: {:?} -> {:?}", src, dst);
        }
        if self.create_missing_dirs {
            create_dir_all(dst.parent().unwrap()).chain_err(|| "unable to create directory")?;
        }
        rename(&src, &dst).chain_err(|| "unable to rename file")
    }
}

pub struct Custom(dyn Fn(&PathBuf, &PathBuf) -> Result<()>);

impl Action for Custom {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        (self.0)(src, dst)
    }
}
