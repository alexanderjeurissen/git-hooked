extern crate heck;

use failure::Error;
use heck::SnakeCase;

use serde::{Deserialize, Serialize};
use structopt::clap::arg_enum;
use structopt::StructOpt;

use log::{debug, error, info, warn};
use std::os::unix::fs;

use std::path::PathBuf;

use crate::config::get_config;
use crate::git::git_root_path;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Hook {
    pub name: HookType,
    // NOTE: Specifies if the git hook should be created if necessary.
    #[serde(default = "default_create")]
    pub create: bool,
    // NOTE: Specifies if the git hook should be forcibly linked. This can cause irreversible data loss! Use with caution!
    #[serde(default = "default_force")]
    pub force: bool,
    // NOTE: Specifies if incorrect symbolic links should be automatically overwritten
    #[serde(default = "default_relink")]
    pub relink: bool,
}

fn default_create() -> bool {
    true
}

fn default_force() -> bool {
    false
}

fn default_relink() -> bool {
    true
}

arg_enum! {
    #[derive(Serialize, Deserialize, StructOpt, Debug, Clone, PartialEq)]
    pub enum HookType {
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

// NOTE: enum used te denote the state of a hook destination
// ExistingFile: there is a file at the hook destination
// ExistingLink: there is a symlink at the hook destination
// InvalidLink: there is a broken symlink at the hook destitaniot
#[derive(StructOpt, Debug, Clone, PartialEq)]
enum HookState {
    ExistingFile,
    ExistingDir,
    ExistingLink,
    InvalidLink,
    Unknown,
    None,
}

// NOTE: create symlinks for existing files in git_hooks folder
pub fn hook(config: Option<PathBuf>) -> Result<(), Error> {
    let root_path: String = git_root_path()?;

    let config = get_config(config, &root_path)?;

    let hooks = config.hooks.unwrap();

    hooks
        .into_iter()
        .try_for_each(|hook: Hook| install_hook(&root_path, &hook))?;

    Ok(())
}

fn install_hook(root_path: &String, hook: &Hook) -> Result<(), Error> {
    if hook.create {
        let source_path: PathBuf = hook_source_path(&root_path, &hook)?;
        let destination_path: PathBuf = hook_destination_path(&root_path, &hook)?;

        let hook_destination_result: HookState = inspect_hook_destination(&destination_path)?;
        match hook_destination_result {
            // NOTE: we can safely create a symlink, destination is up for grabs
            HookState::None => {
                info!("Creating symlink for {:?}..", hook.name);
                create_symlink(&source_path, &destination_path)?;
            }
            // NOTE: There is an existing file or link at the destination
            // We can only create a symlink if the `force` option is set
            HookState::ExistingFile | HookState::ExistingLink => {
                if hook.force {
                    info!("Creating symlink for {:?}..", hook.name);
                    warn!("Destination already exists, `force` option set, destitation will be overwritten..");
                    std::fs::remove_file(&destination_path)?;
                    create_symlink(&source_path, &destination_path)?;
                } else {
                    error!("Destination {0:?} already taken. Did not create {1:?} hook symlink. Set `force` to true in your configuration or manually delete the destination and try again", destination_path, hook.name);
                }
            }
            // NOTE: There is an existing directory at the destination
            // We can only create a symlink if the `force` option is set
            HookState::ExistingDir => {
                if hook.force {
                    info!("Creating symlink for {:?}..", hook.name);
                    warn!("Destination already exists, `force` option set, destitation will be overwritten..");
                    std::fs::remove_dir(&destination_path)?;
                    create_symlink(&source_path, &destination_path)?;
                } else {
                    error!("Destination {0:?} already taken. Did not create {1:?} hook symlink. Set `force` to true in your configuration or manually delete the destination and try again", destination_path, hook.name);
                }
            }
            // NOTE: There is an existing invalid symlink at the destination
            // We can only create a symlink if the `relink` option is set
            HookState::InvalidLink => {
                if hook.relink {
                    info!("Creating symlink for {:?}..", hook.name);
                    warn!("Destination already exists, `relink` option set, destitation will be overwritten..");
                    std::fs::remove_file(&destination_path)?;
                    create_symlink(&source_path, &destination_path)?;
                } else {
                    error!("Destination {0:?} already taken. Did not create {1:?} hook symlink. Set `force` to true in your configuration or manually delete the destination and try again", destination_path, hook.name);
                }
            }
            HookState::Unknown => {
                error!("destination is unknown");
            }
        }
    } else {
        debug!(
            "{:?}.create = false, skipping creation of symlink",
            hook.name
        )
    }

    Ok(())
}

fn create_symlink(source_path: &PathBuf, destination_path: &PathBuf) -> Result<(), Error> {
    debug!("{0:?} -> {1:?}", source_path, destination_path);

    fs::symlink(source_path, destination_path)?;

    Ok(())
}

// NOTE: check if we can safely create a new symlink
fn inspect_hook_destination(destination: &PathBuf) -> Result<HookState, Error> {
    match std::fs::read_link(destination) {
        Ok(path) => {
            if path.exists() {
                Ok(HookState::ExistingLink)
            } else {
                Ok(HookState::InvalidLink)
            }
        }
        Err(_e) => match std::fs::metadata(destination) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Ok(HookState::ExistingFile)
                } else if metadata.is_dir() {
                    Ok(HookState::ExistingDir)
                } else {
                    Ok(HookState::Unknown)
                }
            }
            Err(_e) => Ok(HookState::None),
        },
    }
}

pub fn hook_source_path(root_path: &String, hook: &Hook) -> Result<PathBuf, Error> {
    let mut path: PathBuf = PathBuf::new();
    path.push(&root_path);
    path.push("git_hooks");
    path.push(hook.name.to_string().as_str().to_snake_case());

    Ok(path)
}

pub fn hook_destination_path(root_path: &String, hook: &Hook) -> Result<PathBuf, Error> {
    let mut path: PathBuf = PathBuf::new();
    path.push(&root_path);
    path.push(".git");
    path.push("hooks");
    path.push(hook.name.to_string().as_str().to_snake_case());

    Ok(path)
}
