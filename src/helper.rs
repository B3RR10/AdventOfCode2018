use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
