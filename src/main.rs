use std::fs::File;
use std::io::{self, BufRead, Read};

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let file = File::open("inputs/day8normalize").unwrap();

    let mut contents = String::new();

    let _bytes = io::BufReader::new(file)
        .read_to_string(&mut contents)
        .unwrap();

    let lines = contents.lines().collect::<Vec<&str>>();
    let val = *lines.get(2).unwrap();

    let unicode: Vec<&str> = val.unicode_words().collect();

    println!("\x27");
}
