extern crate heck;

use failure::{bail, Error};
use heck::KebabCase;

use serde::{Deserialize, Serialize};
use structopt::clap::arg_enum;
use structopt::StructOpt;

use log::{debug, error, info, warn};
use std::os::unix::fs;

use crate::config::Config;
use crate::path_utils::PathResult::{
    ExistingDir, ExistingFile, ExistingLink, InvalidLink, NonExisting, Unknown,
};
use crate::path_utils::{visit_path, PathResult};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Hook {
    pub name: HookType,
    // NOTE: Specifies if the git hook should be created if necessary.
    #[serde(default = "default_true")]
    pub create: bool,
    // NOTE: Specifies if the git hook should be forcibly linked. This can cause irreversible data loss! Use with caution!
    #[serde(default = "default_false")]
    pub force: bool,
    // NOTE: Specifies if incorrect symbolic links should be automatically overwritten
    #[serde(default = "default_true")]
    pub relink: bool,
    // NOTE: Specifies if the hook should be tracked when running the pull command.
    #[serde(default = "default_true")]
    pub pull: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

arg_enum! {
    #[derive(Serialize, Deserialize, StructOpt, Debug, Clone, PartialEq)]
    pub enum HookType {
        ApplypatchMsg,
        PreApplyPatch,
        PostApplyPatch,
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

// NOTE: create symlinks for existing files in git_hooks folder
pub fn push(config: &Config, root_path: &String) -> Result<(), Error> {
    match &config.hooks {
        Some(hooks) => {
            hooks
                .into_iter()
                .try_for_each(|hook: &Hook| push_hook(&root_path, &hook))?;
        }
        None => {
            error!("No hooks defined in provide configuration file");
        }
    }

    Ok(())
}

// NOTE: copy over existing symlinks to the git_hooks folder
pub fn pull(config: &Config, root_path: &String) -> Result<(), Error> {
    match &config.hooks {
        Some(hooks) => {
            let destination_path: PathBuf = git_hooked_path(&root_path, None)?;
            let destination_path_state: PathResult = visit_path(&destination_path)?;

            match destination_path_state {
                ExistingFile | ExistingLink | InvalidLink => {
                    bail!("git_hooked destination path already exists")
                }

                Unknown => bail!("git_hooked destination path unknown"),
                NonExisting => {
                    std::fs::create_dir(&destination_path);

                    hooks
                        .into_iter()
                        .try_for_each(|hook: &Hook| pull_hook(&root_path, &hook))?;
                }
                ExistingDir => {
                    hooks
                        .into_iter()
                        .try_for_each(|hook: &Hook| pull_hook(&root_path, &hook))?;
                }
            }
        }
        None => {
            error!("No hooks defined in provide configuration file");
        }
    }

    Ok(())
}

fn push_hook(root_path: &String, hook: &Hook) -> Result<(), Error> {
    if hook.create {
        let source_path: PathBuf = git_hooked_path(&root_path, Some(&hook))?;
        let destination_path: PathBuf = git_hooks_path(&root_path, &hook)?;

        let hook_destination_result: PathResult = visit_path(&destination_path)?;

        info!("Creating symlink for {:?}..", &hook.name);
        debug!("{0:?} -> {1:?}", &source_path, &destination_path);

        match hook_destination_result {
            // NOTE: we can safely create a symlink, destination is up for grabs
            NonExisting => {
                fs::symlink(&source_path, &destination_path)?;
            }
            // NOTE: There is an existing file or link at the destination
            // We can only create a symlink if the `force` option is set
            ExistingFile | ExistingLink => {
                if hook.force {
                    warn!("Destination already exists, `force` option set, destination will be overwritten..");
                    std::fs::remove_file(&destination_path)?;
                    fs::symlink(&source_path, &destination_path)?;
                } else {
                    error!("Destination already exists. `force` option *not* set. Did not create {:?} symlink.", hook.name);
                }
            }
            // NOTE: There is an existing directory at the destination
            // We can only create a symlink if the `force` option is set
            ExistingDir => {
                if hook.force {
                    warn!("Destination already exists, `force` option set, destination will be overwritten..");
                    std::fs::remove_dir(&destination_path)?;
                    fs::symlink(&source_path, &destination_path)?;
                } else {
                    error!("Destination already exists. `force` option *not* set. Did not create {:?} symlink.", hook.name);
                }
            }
            // NOTE: There is an existing invalid symlink at the destination
            // We can only create a symlink if the `relink` option is set
            InvalidLink => {
                if hook.relink {
                    warn!("Destination already exists, `relink` option set, destination will be overwritten..");
                    std::fs::remove_file(&destination_path)?;
                    fs::symlink(&source_path, &destination_path)?;
                } else {
                    error!("Destination already exists. `force` option *not* set. Did not create {:?} symlink.", hook.name);
                }
            }
            Unknown => {
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

fn pull_hook(root_path: &String, hook: &Hook) -> Result<(), Error> {
    let git_hooks_path: PathBuf = git_hooks_path(&root_path, &hook)?;
    let git_hooked_path: PathBuf = git_hooked_path(&root_path, Some(&hook))?;

    let git_hooks_path_state: PathResult = visit_path(&git_hooks_path)?;
    info!("Adding git hook {:?} to version control..", &hook.name);
    debug!("mv {0:?} -> {1:?}", &git_hooks_path, &git_hooked_path);
    match git_hooks_path_state {
        ExistingFile => {
            let git_hooked_path_state: PathResult = visit_path(&git_hooked_path)?;
            match git_hooked_path_state {
                ExistingFile | ExistingLink | InvalidLink | ExistingDir => {
                    error!("destination {:?} already exists", &git_hooked_path);
                }
                Unknown => {
                    error!("Unknown destination");
                }
                NonExisting => {
                    std::fs::rename(&git_hooks_path, &git_hooked_path)?;
                    info!("Creating symlink for {:?}..", &hook.name);
                    debug!("ln {0:?} -> {1:?}", &git_hooks_path, &git_hooked_path);
                    fs::symlink(&git_hooked_path, &git_hooks_path)?;
                }
            }
        }
        ExistingLink | InvalidLink => {
            error!("source path {0:?} for hook {1:?} is a symbolic link and can therefore not be moved", &git_hooks_path, &hook.name);
        }
        ExistingDir => {
            error!(
                "source path {0:?} for hook {1:?} is a directory and can therefore not be moved",
                &git_hooks_path, &hook.name
            );
        }
        NonExisting => {
            error!(
                "source path {0:?} for hook {1:?} does not exist and can therefore not be moved",
                &git_hooks_path, &hook.name
            );
        }
        Unknown => {
            error!("source path {0:?} for hook {1:?} is of unknown type and can therefore not be moved", &git_hooks_path, &hook.name);
        }
    }

    Ok(())
}

fn git_hooked_path(root_path: &String, hook: Option<&Hook>) -> Result<PathBuf, Error> {
    let mut path: PathBuf = PathBuf::new();
    path.push(&root_path);
    path.push(".git_hooks");

    match hook {
        Some(v) => {
            path.push(v.name.to_string().as_str().to_kebab_case());
        }
        None => {
            debug!("no hook provided to fn git_hooked_path(), returning full path");
        }
    }

    Ok(path)
}

fn git_hooks_path(root_path: &String, hook: &Hook) -> Result<PathBuf, Error> {
    let mut path: PathBuf = PathBuf::new();
    path.push(&root_path);
    path.push(".git");
    path.push("hooks");
    path.push(hook.name.to_string().as_str().to_kebab_case());

    Ok(path)
}
