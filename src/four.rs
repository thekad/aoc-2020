use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;

struct YearValidation {
    lower: i32,
    upper: i32,
}

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    if let Ok(input) = fs::read_to_string(path) {
        let required: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .collect();
        let mut yrs = HashMap::new();
        yrs.insert(
            "byr",
            YearValidation {
                lower: 1920,
                upper: 2002,
            },
        );
        yrs.insert(
            "iyr",
            YearValidation {
                lower: 2010,
                upper: 2020,
            },
        );
        yrs.insert(
            "eyr",
            YearValidation {
                lower: 2020,
                upper: 2030,
            },
        );
        let mut complete = 0;
        let mut valid = 0;

        for data in input.split("\n\n") {
            let mut passport = HashMap::new();
            let data = &data.replace("\n", " ").to_string();
            let mut errors = HashMap::new();
            for item in data.split_whitespace() {
                let sp = item.split(":").collect::<Vec<&str>>();
                let k = sp[0];
                let v = sp[1];
                passport.insert(k, v);
            }
            let fields: HashSet<&str> = passport.keys().cloned().collect();
            // required fields are complete
            if required.is_subset(&fields) {
                complete += 1;

                // start validations
                // validate year fields
                for (yrk, val) in yrs.iter() {
                    if let Some(yr) = passport.get(yrk) {
                        if let Ok(yr) = yr.parse::<i32>() {
                            let (ok, err) = valid_year(yr, val);
                            if !ok {
                                errors.insert(yrk, err);
                            }
                        } else {
                            errors.insert(yr, format!("Invalid {} (not an int)", yrk));
                        }
                    }
                }

                // validate height
                let hgt = passport.get("hgt").unwrap();
                let (ok, err) = valid_height(hgt);
                if !ok {
                    errors.insert(&"hgt", err);
                }

                // validate hair color
                let hgl = passport.get("hcl").unwrap();
                let (ok, err) = valid_hair(hgl);
                if !ok {
                    errors.insert(&"hgl", err);
                }

                // validate eye color
                let ecl = passport.get("ecl").unwrap();
                let (ok, err) = valid_eye(ecl);
                if !ok {
                    errors.insert(&"ecl", err);
                }

                // validate passport ID
                let pid = passport.get("pid").unwrap();
                let (ok, err) = valid_pid(pid);
                if !ok {
                    errors.insert(&"pid", err);
                }

                if errors.is_empty() {
                    valid += 1;
                } else {
                    println!("Validation errors found: {:?}", errors);
                }
            } else {
                println!("Missing fields: {:?}", required.difference(&fields));
            }
        }
        println!("Complete passports: {:?}", complete);
        println!("Valid passports: {:?}", valid);
    }

    Ok(())
}

fn valid_year(y: i32, v: &YearValidation) -> (bool, String) {
    if y >= v.lower && y <= v.upper {
        return (true, "".to_string());
    }
    (
        false,
        format!("{} is not within {} and {}", y, v.lower, v.upper),
    )
}

fn valid_height(h: &str) -> (bool, String) {
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in){1}$").unwrap();
    }
    for cap in HEIGHT_RE.captures_iter(&h) {
        if let Some(metric) = cap.get(1) {
            let metric: i32 = metric.as_str().parse().unwrap();
            if let Some(units) = cap.get(2) {
                match units.as_str() {
                    "cm" => {
                        return (
                            metric >= 150 && metric <= 193,
                            format!("Centimeters not in 150-193 range: {}", metric),
                        );
                    }
                    "in" => {
                        return (
                            metric >= 59 && metric <= 76,
                            format!("Inches not in 59-76 range: {}", metric),
                        );
                    }
                    _ => {
                        return (false, format!("Wrong unit provided: {}", units.as_str()));
                    }
                }
            }
        }
    }

    (false, format!("No valid height: {}", h))
}

fn valid_hair(h: &str) -> (bool, String) {
    lazy_static! {
        static ref HAIR_RE: Regex = Regex::new(r"^#[[:xdigit:]]{6}$").unwrap();
    }

    if HAIR_RE.is_match(&h) {
        return (true, "".to_string());
    }

    (false, format!("No valid hair color: {}", h))
}

fn valid_eye(e: &str) -> (bool, String) {
    lazy_static! {
        static ref EYE_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth){1}$").unwrap();
    }

    if EYE_RE.is_match(&e) {
        return (true, "".to_string());
    }

    (false, format!("No valid eye color: {}", e))
}

fn valid_pid(p: &str) -> (bool, String) {
    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    if PID_RE.is_match(&p) {
        return (true, "".to_string());
    }

    (false, format!("No valid passport ID: {}", p))
}
