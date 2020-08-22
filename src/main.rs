extern crate pretty_env_logger;

mod config;
mod git;

use config::*;
use failure::Error;
use git::git_root_path;
use log::{debug, info, trace, warn};
use std::path::PathBuf;
use std::str::{FromStr, ParseBoolError};
use std::time::Instant;
use structopt::clap::arg_enum;
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

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Hook {
        #[structopt(possible_values = &HookArg::variants(), case_insensitive = true, default_value = "All", help = "What hooks should gitHooked configure ?")]
        hooks: Vec<HookArg>,
    },
}

arg_enum! {
    #[derive(StructOpt, Debug)]
    enum HookArg {
        All,
        ApplypatchMsg,
        PreApplypatch,
        PostApplypatch,
        PreCommit,
        PreMergeCommit,
        PrepareCommitMsg,
        CommitMsg,
        PostCommit,
        PreRebase,
        PostCheckout,
        PostMerge,
        PrePush,
        PreReceive,
        Update,
        PostReceive,
        PostUpdate,
        ReferenceTransaction,
        PushToCheckout,
        PreAutoGc,
        PostRewrite,
        SendemailValidate,
        FsmonitorWatchman,
        P4Changelist,
        P4PrepareChangelist,
        P4PostChangelist,
        P4PreSubmit,
        PostIndexChange,
    }
}

fn main() -> Result<(), Error> {
    let start = Instant::now();

    pretty_env_logger::init_custom_env("LOG");

    let args = Cli::from_args();

    let root_path: String = git_root_path()?;

    let config = get_config(args.config, &root_path)?;

    let hooks = config.hooks.unwrap();

    trace!("hooks: {:?}", hooks);

    // DO STUFF
    //
    //
    //

    // NOTE: we are done log total execution time
    let duration = start.elapsed();
    info!("'FINISHED after {:?}", duration);

    return Ok(());
}
