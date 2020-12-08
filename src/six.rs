use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let mut groups: Vec<usize> = Vec::new();
    if let Ok(input) = fs::read_to_string(path) {
        for data in input.split("\n\n") {
            let chars: HashSet<char> =
                HashSet::from_iter(data.replace("\n", "").replace(" ", "").chars());
            groups.push(chars.len())
        }
    }
    println!("Total groups: {:?}", groups.len());
    println!("Total 'yes' responses: {:?}", groups.iter().sum::<usize>());

    Ok(())
}
