use super::errors::*;
use std::fs::{copy, rename};
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

pub struct Copy();

impl Action for Copy {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        println!("copy: {:?} -> {:?}", src, dst);
        copy(&src, &dst).chain_err(|| "unable to copy file")?;
        Ok(())
    }
}
pub struct Move();

impl Action for Move {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        println!("move: {:?} -> {:?}", src, dst);
        rename(&src, &dst).chain_err(|| "unable to rename file")?;
        Ok(())
    }
}

pub struct Custom(dyn Fn(&PathBuf, &PathBuf) -> Result<()>);

impl Action for Custom {
    fn execute(&mut self, src: &PathBuf, dst: &PathBuf) -> Result<()> {
        (self.0)(src, dst)
    }
}
