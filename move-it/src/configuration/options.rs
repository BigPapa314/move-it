use std::path::PathBuf;
pub use structopt::clap::arg_enum;
pub use structopt::*;

arg_enum! {
    #[derive(Debug)]
    pub enum Command {
        Echo,
        Copy,
        Move,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mi", about = "Moves files. Will leave empty directories.")]
pub struct Options {
    /// The command to execute. Default value is move.
    #[structopt(short, long)]
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
    #[structopt(long)]
    pub create_target_dir: Option<bool>,

    /// Delimiter to devide multiple provided include values.
    #[structopt(long, default_value = ";")]
    pub include_delimiter: String,

    /// Regex match on full source path. If matched the file will be included.
    #[structopt(short, long)]
    pub include: Option<String>,

    /// Delimiter to devide multiple provided exclude values.
    #[structopt(long, default_value = ";")]
    pub exclude_delimiter: String,

    /// Regex match on full source path. If matched the file will be excluded.
    #[structopt(short, long)]
    pub exclude: Option<String>,
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
