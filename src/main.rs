extern crate pretty_env_logger;

mod config;
mod git;
mod hook;
mod path_utils;

use failure::Error;

use crate::config::{get_config, Config};
use crate::git::git_root_path;
use crate::hook::HookType;

use log::{info, trace};
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-hooked", about = "Explanation of git-hooked usage.")]
struct Cli {
    #[structopt(
        long,
        help = "specify custom config path, if not set git_hooked will look for `git_hooked.config.toml` in the current working directory, and home directory subsequently.",
        parse(from_os_str)
    )]
    config: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Pull,
    Push,
    Remove {
        #[structopt(possible_values = &HookType::variants(), case_insensitive = true, help = "What hooks should gitHooked remove ?")]
        hooks: Vec<HookType>,
    },
}

fn main() -> Result<(), Error> {
    let start = Instant::now();

    pretty_env_logger::init_custom_env("LOG");

    let args = Cli::from_args();
    let root_path: String = git_root_path()?;

    let config: Config = get_config(args.config, &root_path)?;

    match args.cmd {
        Command::Push => hook::push(&config, &root_path)?,
        Command::Pull => hook::pull(&config, &root_path)?,
        Command::Remove { hooks } => trace!("remove: {:?}", hooks),
    }

    // NOTE: we are done log total execution time
    let duration = start.elapsed();
    info!("'FINISHED after {:?}", duration);

    return Ok(());
}
