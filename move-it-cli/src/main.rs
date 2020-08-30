//! It moves files from one folder to an other.

use move_it::Work;

use clap::{crate_description, crate_version, Clap, IntoApp};
use clap_generate::{
    generate, generators::Bash, generators::Elvish, generators::Fish, generators::PowerShell,
    generators::Zsh,
};
use log::*;
use snafu::Snafu;
use std::io::Write;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Command line parameter missing '{}'", name))]
    ClParameterMissing { name: String },
}

#[derive(Clap)]
#[clap(version=crate_version!(), author = "Thomas Kilian <Thomas-Kilian@gmx.net>", about(crate_description!()))]
struct Opts {
    /// If specified the files are copied and not moved
    #[clap(short, long)]
    copy: bool,

    /// Includes only files that full path matches the given regular expression
    #[clap(short, long)]
    include: Vec<String>,

    /// Excludes files that full path matches the given regular expression
    #[clap(short, long)]
    exclude: Vec<String>,

    /// Level of verbosity
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    /// Source folder/file
    #[clap(conflicts_with("generate-completion"), requires("destination"))]
    source: Option<String>,

    /// Destination folder
    #[clap(conflicts_with("generate-completion"))]
    destination: Option<String>,

    /// Description how the target names are built
    #[clap(short, long, default_value("{FILE:RELPATH}/{FILE:NAME}"))]
    name_builder: String,

    /// Generates completion scripts
    #[clap(short, long, possible_values(&["bash", "elvish", "fish", "powershell", "zsh"]))]
    generate_completion: Option<String>,
}

async fn run() -> Result<()> {
    let opts: Opts = Opts::parse();

    if let Some(generator) = opts.generate_completion {
        match generator.as_str() {
            "bash" => generate::<Bash, _>(&mut Opts::into_app(), "mi", &mut std::io::stdout()),
            "elvish" => generate::<Elvish, _>(&mut Opts::into_app(), "mi", &mut std::io::stdout()),
            "fish" => generate::<Fish, _>(&mut Opts::into_app(), "mi", &mut std::io::stdout()),
            "powershell" => {
                generate::<PowerShell, _>(&mut Opts::into_app(), "mi", &mut std::io::stdout())
            }
            "zsh" => generate::<Zsh, _>(&mut Opts::into_app(), "mi", &mut std::io::stdout()),
            _ => (),
        }

        return Ok(());
    }

    if opts.source.is_none() {
        ClParameterMissing { name: "source" }.fail()?
    }
    if opts.destination.is_none() {
        ClParameterMissing {
            name: "destination",
        }
        .fail()?
    }

    let mut work = Work::new();

    info!("SETUP: source: {}", opts.source.as_ref().unwrap());
    work = work.all_files_recursive(opts.source.as_ref().unwrap())?;

    for include in opts.include {
        info!("SETUP: include: {}", include);
        work = work.include(include)?;
    }

    for exclude in opts.exclude {
        info!("SETUP: exclude: {}", exclude);
        work = work.exclude(exclude)?;
    }

    let target_spec = format!(
        "{}/{}",
        opts.destination.as_ref().unwrap(),
        opts.name_builder
    );
    info!("SETUP: target_spec: {}", target_spec);

    info!("SETUP: verbose: {}", opts.verbose);
    if opts.verbose > 1 {
        work = work.echo(target_spec.clone())?;
    }

    if opts.copy {
        info!("SETUP: doing copy");
        work = work.copy(target_spec.clone())?;
    } else {
        info!("SETUP: doing move");
        work = work.r#move(target_spec.clone())?;
    }

    info!("Start");
    work.execute().await
}

fn main() {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(run()) {
        Ok(_) => info!("Done"),
        Err(e) => error!("An error ocurred: {}", e),
    };
}
