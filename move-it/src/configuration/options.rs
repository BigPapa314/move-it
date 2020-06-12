use std::path::PathBuf;
pub use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt()]
pub enum Command {
    /// Echos the source and the target file names.
    Echo,
    /// Copies the source files to the target.
    Copy,
    /// Moves the source files to the target.
    Move,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mi", about = "mv on steroids.")]
pub struct Options {
    /// The source paths. If --target is not specified the last path is used as target.
    #[structopt(global = true, parse(from_os_str))]
    pub paths: Vec<PathBuf>,

    /// Target directory
    #[structopt(short = "t", long = "target", parse(from_os_str))]
    pub target: Option<PathBuf>,

    /// The command to execute. Default value is move.
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_help_text() {
        let src = vec!["mi", "--help"];

        let opts = Options::from_iter(src.into_iter());
        println!("{:?}", opts);
    }

    #[test]
    fn short_help_text() {
        let src = vec!["mi", "-h"];

        let opts = Options::from_iter(src.into_iter());
        println!("{:?}", opts);
    }
}
