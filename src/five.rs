use std::collections::HashSet;
use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let mut seats: HashSet<i32> = HashSet::new();

    if let Ok(lines) = crate::io::read_lines(path) {
        for line in lines {
            seats.insert(
                i32::from_str_radix(
                    line.unwrap()
                        .replace("F", "0")
                        .replace("B", "1")
                        .replace("L", "0")
                        .replace("R", "1")
                        .as_str(),
                    2,
                )
                .unwrap(),
            );
        }
    }
    let min = seats.iter().min().unwrap();
    let max = seats.iter().max().unwrap();
    let r = std::ops::Range {
        start: *min,
        end: *max,
    };

    println!("Max seat: {}", max);
    for i in r {
        if !seats.contains(&i) {
            println!("My seat: {}", i);
            break;
        }
    }

    Ok(())
}
