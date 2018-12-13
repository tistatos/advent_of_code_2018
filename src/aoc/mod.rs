use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn get_string_rows(file: &str) -> Vec<String> {
    let mut output = Vec::new();
    let input = File::open(file).unwrap();
    let buf_reader = BufReader::new(input);

    for line in buf_reader.lines() {
        match line {
            Ok(string) => output.push(string),
            Err(err) => panic!("Errror reading input: {}", err)
        };
    }

    return output;
}
