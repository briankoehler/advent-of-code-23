use std::{path::Path, fs::File, io::{BufReader, BufRead}};

pub fn read_input_to_string_vec(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.expect("Could not parse line")).collect()
}

