extern crate pretty_env_logger;

mod config;
mod git;

use config::*;
use failure::Error;
use git::{git_root_path, git_staged_file_paths};
use lint::*;
use log::{info, trace, warn};
use std::path::PathBuf;
use std::str::{FromStr, ParseBoolError};
use std::time::Instant;
use structopt::StructOpt;

fn parse_bool(v: &str) -> Result<bool, ParseBoolError> {
    match v {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Ok(<bool as FromStr>::from_str(v)?),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "git-hooked", about = "Explanation of git-hooked usage.")]
struct Cli {
    #[structopt(long, help = "specify custom config path", parse(from_os_str))]
    config: Option<PathBuf>,

    #[structopt(long, env = "GIT_HOOKED", parse(try_from_str = parse_bool))]
    hook: bool,
}

fn main() -> Result<(), Error> {
    let start = Instant::now();

    pretty_env_logger::init_custom_env("LOG");

    let args = Cli::from_args();

    // NOTE: stop execution if GIT_HOOKED is not set
    if args.staged == false {
        trace!("GIT_HOOKED not set or invalid value, skipping...");
        return Ok(());
    }

    let root_path: String = git_root_path()?;

    let config = get_config(args.config, &root_path)?;

    let linters = config.linters.unwrap();

    trace!("linters: {:?}", linters);

    // DO STUFF
    //
    //
    //

    // NOTE: we are done log total execution time
    let duration = start.elapsed();
    info!("'FINISHED after {:?}", duration);

    return Ok(());
}
