use super::destination::*;
use super::errors::*;
use super::source::*;
use std::fs::copy;

pub trait Action {
    fn execute(&mut self, src: &SourceDescription, dst: &mut dyn DestinationBuilder) -> Result<()>;
}

pub struct Echo();

impl Action for Echo {
    fn execute(&mut self, src: &SourceDescription, dst: &mut dyn DestinationBuilder) -> Result<()> {
        let src_path = src.source_path();
        let dst_path = dst.build(&src);
        println!("echo: {:?} -> {:?}", src_path, dst_path);
        Ok(())
    }
    // = dyn Fn(&SourceDescription, &mut dyn DestinationBuilder) -> Result<()>;
}

pub struct Copy();

impl Action for Copy {
    fn execute(&mut self, src: &SourceDescription, dst: &mut dyn DestinationBuilder) -> Result<()> {
        let src_path = src.source_path();
        let dst_path = dst.build(&src);
        println!("copy: {:?} -> {:?}", src_path, dst_path);
        copy(&src_path, &dst_path).chain_err(|| "unable to copy file")?;
        Ok(())
    }
    // = dyn Fn(&SourceDescription, &mut dyn DestinationBuilder) -> Result<()>;
}

pub struct Custom(dyn Fn(&SourceDescription, &mut dyn DestinationBuilder) -> Result<()>);

impl Action for Custom {
    fn execute(&mut self, src: &SourceDescription, dst: &mut dyn DestinationBuilder) -> Result<()> {
        (self.0)(src, dst)
    }
    // = dyn Fn(&SourceDescription, &mut dyn DestinationBuilder) -> Result<()>;
}
