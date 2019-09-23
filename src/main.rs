#![feature(slice_from_raw_parts)]

#[allow(dead_code)]
mod scratch;

use crate::scratch::parse::project;
use project::path::ProjectPath;
use project::archive::ProjectArchive;

fn main() {
    let path = ProjectPath::from_args().unwrap();
    println!("{:?}", path);
    let mut archive = ProjectArchive::new(path).unwrap();
    archive.print_entries();
}
