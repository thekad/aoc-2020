use regex::Regex;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug)]
struct PasswordPol1 {
    min: i32,
    max: i32,
    chr: String,
    txt: String,
    cnt: i32,
    ok: bool,
}

#[derive(Debug)]
struct PasswordPol2 {
    pos1: i32,
    pos2: i32,
    chr: String,
    txt: String,
    ok: bool,
}

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let re = Regex::new(r#"(\d+)\-(\d+)\s(\w):\s(\w+)"#).unwrap();
    let mut first_ppols: Vec<PasswordPol1> = Vec::new();
    let mut second_ppols: Vec<PasswordPol2> = Vec::new();
    if let Ok(lines) = crate::io::read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                for cap in re.captures_iter(&line) {
                    let min = String::from(&cap[1]).parse().unwrap();
                    let max = String::from(&cap[2]).parse().unwrap();
                    let chr = String::from(&cap[3]);
                    let chr2 = String::from(&cap[3]);
                    let txt = String::from(&cap[4]);
                    let txt2 = String::from(&cap[4]);
                    let txtv = txt2.split("").collect::<Vec<&str>>();
                    let cnt = txt.matches(&cap[3]).count() as i32;
                    let ok_count = cnt >= min && cnt <= max;
                    let ok_xor = (txtv[min as usize] == chr) ^ (txtv[max as usize] == chr);

                    let ppol1 = PasswordPol1 {
                        min,
                        max,
                        chr,
                        txt,
                        cnt,
                        ok: ok_count,
                    };
                    let ppol2 = PasswordPol2 {
                        pos1: min,
                        pos2: max,
                        chr: chr2,
                        txt: txt2,
                        ok: ok_xor,
                    };
                    if ok_count {
                        first_ppols.push(ppol1);
                    }
                    if ok_xor {
                        second_ppols.push(ppol2);
                    }
                }
            }
        }
    }
    println!(
        "First policy: {} Second policy: {}",
        first_ppols.len(),
        second_ppols.len()
    );

    Ok(())
}
