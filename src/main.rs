use std::{
    fs,
    path::Path,
};

const COSTUME_SAVES_PATH: &str = "/mnt/c/Program Files (x86)/Steam/steamapps/common/Champions Online/Champions Online/Live/screenshots";

// TODO
// * Only return files that are valid costume saves.
// * Ensure this is performant when the dir contains many files.
// * Actual error handling, maybe return a result containing the vec.
fn get_saved_costumes(saves_dir: &Path) -> Vec<fs::DirEntry> {
    fs::read_dir(saves_dir).unwrap().map(|p| p.unwrap()).collect()
}

fn main() {
    let costumes = get_saved_costumes(Path::new(COSTUME_SAVES_PATH));
    println!("{:?}", costumes);
}
