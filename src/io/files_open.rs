// use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
// use std::io::{BufRead, BufReader, Error, Write};

fn read_file(path: String) -> String {
    let mut file = File::open(path.as_str()).expect("file does not exist");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Cannot read file");
    contents
}
