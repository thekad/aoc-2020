use crate::io;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct Numbers {
    one: i32,
    two: i32,
    total: i32,
    three: i32,
    product: i32,
}

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let numbers = read_numbers(path);
    let mut nums1 = Numbers {
        one: 0,
        two: 0,
        three: 0,
        total: 0,
        product: 0,
    };
    let mut nums2 = nums1.clone();

    for idx1 in 0..numbers.len() {
        let num1 = numbers[idx1];
        let rest = &numbers[idx1 + 1..numbers.len()];
        for (idx2, num2) in rest.iter().enumerate() {
            let rest2 = &numbers[idx2 + 1..numbers.len()];
            for (_, num3) in rest2.iter().enumerate() {
                if num1 + num2 + num3 == 2020 {
                    nums2.one = num1;
                    nums2.two = *num2;
                    nums2.three = *num3;
                    nums2.total = num1 + *num2 + *num3;
                    nums2.product = num1 * *num2 * *num3;
                    break;
                }
            }
            if num1 + num2 == 2020 {
                nums1.one = num1;
                nums1.two = *num2;
                nums1.total = num1 + *num2;
                nums1.product = num1 * *num2;
            }
            if nums1.total != 0 && nums2.total != 0 {
                break;
            }
        }

        if nums1.total != 0 && nums2.total != 0 {
            break;
        }
    }
    println!("{:?}", nums1);
    println!("{:?}", nums2);

    Ok(())
}

fn read_numbers(path: std::path::PathBuf) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    if let Ok(lines) = io::read_lines(path) {
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
