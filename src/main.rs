extern crate pretty_env_logger;

mod config;
mod git;
mod hook;

use failure::Error;
use hook::HookType;
use log::{info, trace};
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-hooked", about = "Explanation of git-hooked usage.")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Hook {
        #[structopt(help = "specify custom config path", parse(from_os_str))]
        config: Option<PathBuf>,
    },
    UnHook {
        #[structopt(possible_values = &HookType::variants(), case_insensitive = true, help = "What hooks should gitHooked configure ?")]
        hooks: Vec<HookType>,
    },
}

fn main() -> Result<(), Error> {
    let start = Instant::now();

    pretty_env_logger::init_custom_env("LOG");

    let args = Cli::from_args();

    match args.cmd {
        Command::Hook { config } => hook::hook(config)?,
        Command::UnHook { hooks } => trace!("unhook: {:?}", hooks),
    }
    // DO STUFF
    //
    //
    //

    // NOTE: we are done log total execution time
    let duration = start.elapsed();
    info!("'FINISHED after {:?}", duration);

    return Ok(());
}
