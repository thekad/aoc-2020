use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// this is a bufreader
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// this returns a vector of strings
#[allow(dead_code)]
pub fn lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
}
