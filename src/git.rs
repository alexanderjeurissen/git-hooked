use failure::Error;
use log::{info, trace};
use std::os::unix::fs;
use std::path::PathBuf;
use std::process::Command;

// NOTE: Create a stashed backup of the working directory
pub fn git_create_backup() -> Result<(), Error> {
    let output = Command::new("git").arg("stash").arg("create").output()?;

    let stash_hash: String = String::from_utf8(output.stdout)?;

    let output = Command::new("git").arg("stash").arg("-u").output()?;

    Ok(())
}

// NOTE: create symlinks for existing files in git_hooks folder
pub fn git_symlink_hooks() -> Result<(), Error> {
    fs::symlink("a.txt", "b.txt")?;
    Ok(())
}

// NOTE: get the root path of current git repository
pub fn git_root_path() -> Result<String, Error> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    let output_str: String = String::from_utf8(output.stdout)?;

    Ok(output_str.trim().to_string())
}
