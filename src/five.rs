use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let mut seats: Vec<i32> = vec![];

    if let Ok(lines) = crate::io::read_lines(path) {
        for line in lines {
            seats.push(
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
    println!("Max seat: {:?}", seats.iter().max());

    Ok(())
}
