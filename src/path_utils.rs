use failure::Error;
use std::fs::{metadata, read_link};
use std::path::PathBuf;
use structopt::StructOpt;

// NOTE: enum used te denote the state of a hook destination
// ExistingFile: there is a file at the hook destination
// ExistingLink: there is a symlink at the hook destination
// InvalidLink: there is a broken symlink at the hook destitaniot
#[derive(StructOpt, Debug, Clone, PartialEq)]
pub enum PathResult {
    ExistingFile,
    ExistingDir,
    ExistingLink,
    InvalidLink,
    Unknown,
    NonExisting,
}

pub fn visit_path(destination: &PathBuf) -> Result<PathResult, Error> {
    match read_link(destination) {
        Ok(path) => {
            if path.exists() {
                Ok(PathResult::ExistingLink)
            } else {
                Ok(PathResult::InvalidLink)
            }
        }
        Err(_e) => match metadata(destination) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Ok(PathResult::ExistingFile)
                } else if metadata.is_dir() {
                    Ok(PathResult::ExistingDir)
                } else {
                    Ok(PathResult::Unknown)
                }
            }
            Err(_e) => Ok(PathResult::NonExisting),
        },
    }
}
