use super::path::ProjectPath;
use std::fs::File;
use std::io::BufReader;
use zip::{
    result::{ZipError, ZipResult},
    ZipArchive,
};

pub struct ProjectArchive {
    path: ProjectPath,
    archive: ZipArchive<BufReader<File>>,
    is_sprite: bool,
}

fn is_sprite(archive: &mut ZipArchive<BufReader<File>>) -> ZipResult<bool> {
    if archive.by_name("project.json").is_ok() {
        return Ok(false);
    }
    if archive.by_name("sprite.json").is_ok() {
        return Ok(true);
    }
    Err(ZipError::FileNotFound)
}

impl ProjectArchive {
    pub fn new(path: ProjectPath) -> ZipResult<ProjectArchive> {
        let file = File::open(&path.path)?;
        let reader = BufReader::new(file);
        let mut archive = ZipArchive::new(reader)?;
        let archive = ProjectArchive {
            path,
            archive,
            is_sprite: is_sprite(&mut archive)?,
        };
        Ok(archive)
    }

    pub fn print_entries(&mut self) {
        for i in 0..self.archive.len() {
            let file = self.archive.by_index(i).unwrap();
            println!("{}", file.name());
        }
    }
}
