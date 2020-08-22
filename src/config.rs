use failure::{bail, Error};
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Hook {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GitHookedConfig {
    name: Hook,
    // NOTE: Specifies if the git hook should be created if necessary.
    #[serde(default = "default_create")]
    create: bool,
    // NOTE: Specifies if the git hook should be forcibly linked. This can cause irreversible data loss! Use with caution!
    #[serde(default = "default_force")]
    force: bool,
    // NOTE: Specifies if incorrect symbolic links should be automatically overwritten
    #[serde(default = "default_relink")]
    relink: bool,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hooks: Option<Vec<GitHookedConfig>>,
}

pub fn get_config(config: Option<PathBuf>, root_path: &String) -> Result<Config, Error> {
    trace!("root_path: {}", &root_path);

    let config_path: PathBuf = match config {
        Some(v) => {
            trace!("using user provided config path");
            v
        }
        None => {
            let mut config_path: PathBuf = PathBuf::new();
            config_path.push(&root_path);
            config_path.push("git_hooked.config.toml");

            trace!("fallback to default config_path");

            config_path
        }
    };

    debug!("config_path: {:?}", config_path);

    let config_string = std::fs::read_to_string(&config_path);

    match config_string {
        Ok(v) => {
            let serialized_toml_config = toml::from_str(&v)?;

            trace!("config: {:?}", serialized_toml_config);

            Ok(serialized_toml_config)
        }
        Err(e) => {
            error!("cant read config file from {:?}", &config_path);
            bail!(e)
        }
    }
}
