use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let mut groups_any: Vec<usize> = Vec::new();
    let mut groups_all: Vec<Vec<String>> = Vec::new();
    if let Ok(input) = fs::read_to_string(path) {
        for group in input.split("\n\n") {
            let group_hs: HashSet<char> =
                HashSet::from_iter(group.replace("\n", "").replace(" ", "").chars());
            let mut group_vec: Vec<String> = Vec::new();
            for person in group.replace(" ", "").split("\n") {
                if !person.is_empty() {
                    group_vec.push(person.to_string());
                }
            }
            groups_any.push(group_hs.len());
            groups_all.push(group_vec);
        }
    }

    // this whole thing would be done more elegantly if I knew how
    // to intersect an arbitrary number of hash sets
    let mut agreed = 0;
    for group in groups_all {
        let gl = group.len();
        for c in (0..26).map(|x| (x + b'a') as char) {
            if (group.join("").matches(c).count()) == gl {
                println!("Every person in group agreed on {}", c);
                agreed += 1;
            }
        }
    }

    println!("Total groups: {:?}", groups_any.len());
    println!(
        "Total any 'yes' responses: {:?}",
        groups_any.iter().sum::<usize>()
    );
    println!("Total all 'yes' responses: {:?}", agreed);

    Ok(())
}
