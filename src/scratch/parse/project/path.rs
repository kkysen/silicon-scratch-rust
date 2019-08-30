use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsString;

#[derive(Debug)]
pub struct ProjectPath {
    pub path: PathBuf,
    pub version: u32,
}

#[derive(Debug)]
pub enum Error {
    NoPathGiven,
    NoExtension,
    NotUtf8Extension(OsString),
    NotScratchFileExtension(String),
    InvalidScratchVersion(String),
}

type Result = std::result::Result<ProjectPath, Error>;

impl ProjectPath {

    pub fn from_path(path: &Path) -> Result {
        let ext = path.extension().ok_or(Error::NoExtension)?;
        let ext = ext.to_str().ok_or(Error::NotUtf8Extension(ext.to_owned()))?;
        const EXT_PREFIX: &str = "sb";
        if !ext.starts_with(EXT_PREFIX) {
            return Err(Error::NotScratchFileExtension(ext.to_owned()));
        }
        let version: &str = &ext[EXT_PREFIX.len()..];
        Ok(ProjectPath {
            version: version.parse().map_err(|_| Error::InvalidScratchVersion(version.to_owned()))?,
            path: path.to_owned(),
        })
    }

    pub fn from_args() -> Result {
        env::args()
            .nth(1)
            .as_ref()
            .ok_or(Error::NoPathGiven)
            .map(Path::new)
            .and_then(ProjectPath::from_path)
    }

}
