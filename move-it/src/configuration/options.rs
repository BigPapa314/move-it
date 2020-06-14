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
#[structopt(name = "mi", about = "Moves files. Will leave empty directories.")]
pub struct Options {
    /// The command to execute. Default value is move.
    #[structopt(subcommand)]
    pub command: Option<Command>,

    /// The source paths. If --target is not specified the last path is used as target.
    #[structopt(global = true, parse(from_os_str))]
    pub paths: Vec<PathBuf>,

    /// Target directory
    #[structopt(short = "t", long = "target", parse(from_os_str))]
    pub target: Option<PathBuf>,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Disable cration of missing directories
    #[structopt(long)]
    pub disable_dir_creation: Option<bool>,

    /// Create the given target directory.
    #[structopt(short, long)]
    pub create_target_dir: Option<bool>,

    /// Regex match on full source path. If matched the file will be included.
    #[structopt(short, long)]
    pub include: Option<Vec<String>>,

    /// Regex match on full source path. If matched the file will be excluded.
    #[structopt(short, long)]
    pub exclude: Option<Vec<String>>,
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
