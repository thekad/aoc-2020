use crate::io;
use regex::Regex;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug)]
struct PasswordPol {
    min: i32,
    max: i32,
    chr: String,
    txt: String,
    cnt: i32,
    ok: bool,
}

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let re = Regex::new(r#"(\d+)\-(\d+)\s(\w):\s(\w+)"#).unwrap();
    let mut valid_ppols: Vec<PasswordPol> = Vec::new();
    let mut invalid_ppols: Vec<PasswordPol> = Vec::new();
    if let Ok(lines) = io::read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                for cap in re.captures_iter(&line) {
                    let min = String::from(&cap[1]).parse().unwrap();
                    let max = String::from(&cap[2]).parse().unwrap();
                    let chr = String::from(&cap[3]);
                    let txt = String::from(&cap[4]);
                    let cnt = txt.matches(&cap[3]).count() as i32;
                    let ok = cnt >= min && cnt <= max;

                    let ppol = PasswordPol {
                        min,
                        max,
                        chr,
                        txt,
                        cnt,
                        ok,
                    };
                    if ok {
                        valid_ppols.push(ppol);
                    } else {
                        println!("{:?}", ppol);
                        invalid_ppols.push(ppol);
                    }
                }
            }
        }
    }
    println!(
        "Valid: {} Invalid: {}",
        valid_ppols.len(),
        invalid_ppols.len()
    );

    Ok(())
}

/*
fn read_password_policies(path: std::path::PathBuf) -> Vec<i32> {
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
*/
