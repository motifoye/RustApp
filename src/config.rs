use std::path::Path;
use std::fs;
use std::fs::File;

pub const PATH_DIR_DATA: &str = "data";
pub const PATH_FILE_1: &str = "data/doc_1.txt";
pub const PATH_FILE_2: &str = "data/doc_2.txt";
pub const PATH_FILE_3: &str = "data/doc_3.txt";

pub fn init(){
    let a = Path::new(PATH_DIR_DATA);
    if !a.exists() {
        fs::create_dir(a).unwrap();
    }
    let a = Path::new(PATH_FILE_1);
    if !a.exists() {
        File::create(a).unwrap();
    }
    let a = Path::new(PATH_FILE_2);
    if !a.exists() {
        File::create(a).unwrap();
    }
    let a = Path::new(PATH_FILE_3);
    if !a.exists() {
        File::create(a).unwrap();
    }
}
