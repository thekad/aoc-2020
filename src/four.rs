use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    if let Ok(input) = fs::read_to_string(path) {
        let required: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .collect();
        let mut valid = 0;

        for data in input.split("\n\n") {
            let mut fields: HashSet<&str> = HashSet::new();
            let data = &data.replace("\n", " ").to_string();
            for item in data.split_whitespace() {
                fields.insert(item.split(":").next().unwrap());
            }
            if required.is_subset(&fields) {
                valid += 1;
            } else {
                println!("Missing fields: {:?}", required.difference(&fields));
            }
        }
        println!("Valid passports: {:?}", valid);
    }

    Ok(())
}
