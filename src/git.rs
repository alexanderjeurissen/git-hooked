use failure::Error;
use log::{info, trace};
use std::path::PathBuf;
use std::process::Command;

// NOTE: run git diff --staged --name-only
// and parse the stdout to a String
fn git_staged_file_names() -> Result<String, Error> {
    let output = Command::new("git")
        .arg("diff") // git diff
        .arg("--staged") // only staged files
        .arg("--diff-filter=ACMR") // only Added, Copied, Modified, Renamed changes
        .arg("--no-color") // dont color output
        .arg("--no-ext-diff") // dont allow external diff tools as they might mess up diffs
        .arg("--name-only") // only fetch the filename
        .output()?;

    let output_str: String = String::from_utf8(output.stdout)?;

    info!("staged files: \n{}", output_str);

    Ok(output_str)
}

// NOTE: Create a stashed backup of the working directory
pub fn git_create_backup() -> Result<(), Error> {
    let output = Command::new("git").arg("stash").arg("create").output()?;

    let stash_hash: String = String::from_utf8(output.stdout)?;

    let output = Command::new("git").arg("stash").arg("-u").output()?;

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
