use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mi", about = "mv on steroids.")]
pub struct Options {
    /// <source file> <target file>
    ///
    /// <source dir/file>... <target dir>
    ///
    /// -t <target dir> <source dir/file>...
    #[structopt(global = true, parse(from_os_str))]
    paths: Vec<PathBuf>,

    /// Target directory
    #[structopt(short = "t", long = "target", parse(from_os_str))]
    target: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_text() {
        let src = vec!["mi", "--help"];

        let opts = Options::from_iter(src.into_iter());
        println!("{:?}", opts);
    }
}
