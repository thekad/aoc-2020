use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Numbers {
    one: i32,
    two: i32,
    total: i32,
    product: i32,
}

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let numbers = read_numbers(path);
    let mut nums = Numbers {
        one: 0,
        two: 0,
        total: 0,
        product: 0,
    };

    for idx in 0..numbers.len() {
        let this = numbers[idx];
        let rest = &numbers[idx + 1..numbers.len()];
        for (_, item) in rest.iter().enumerate() {
            if this + item == 2020 {
                nums.one = this;
                nums.two = *item;
                nums.total = this + *item;
                nums.product = this * *item;
            }
        }
    }
    println!("{:?}", nums);

    Ok(())
}

fn read_numbers(path: std::path::PathBuf) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                if let Ok(num) = line.parse() {
                    numbers.push(num);
                }
            }
        }
    }

    return numbers;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
