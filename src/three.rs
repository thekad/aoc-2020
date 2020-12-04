use crate::io;
use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    if let Ok(lines) = io::lines(path) {
        println!(
            "all slopes multiplied: {}",
            ski(1, 1, &lines)
                * ski(3, 1, &lines)
                * ski(5, 1, &lines)
                * ski(7, 1, &lines)
                * ski(1, 2, &lines)
        )
    }

    Ok(())
}

fn ski(shift: usize, down: usize, lines: &Vec<String>) -> usize {
    println!("skiing down with slope {}/{}", shift, down);
    let size = lines[0].len();
    let mut idx = 0;
    let mut trees = 0;
    for line in lines.iter().step_by(down) {
        if line.chars().nth(idx).unwrap() == '#' {
            trees += 1;
        }
        idx = (idx + shift) % size;
    }

    println!("Hit {} trees on the way here", trees);

    trees
}
