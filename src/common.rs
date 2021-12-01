use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(&filename);
    assert!(file.is_ok(), "Cannot open file {}.\n{}", filename.as_ref().to_str().unwrap(), file.err().unwrap());
    io::BufReader::new(file.unwrap()).lines()
}

pub fn in_data_folder(file: &str) -> PathBuf {
    let error_str = "Cannot locate data folder";
    let exe = std::env::current_exe().expect(error_str);
    let mut dir: PathBuf = PathBuf::from(exe.parent().expect(error_str));
    dir.push("../../data/");
    dir.push(file);
    dir
}