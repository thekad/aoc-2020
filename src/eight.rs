use std::collections::HashMap;
use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    if let Ok(mut lines) = crate::io::lines(path) {
        //let mut lines = content.split("\n").collect::<Vec<String>>();
        // we expect this to fail
        let _ = compile(&lines);
        // get all the possible jmps and nops
        let suspects = get_instructions(&lines, vec!["jmp", "nop"]);
        for (num, instr) in suspects {
            let orig = instr.to_string();
            let flipped = flip(orig.to_string());
            println!("trying flipping ln {} to {}", num, flipped);
            lines[num] = flipped;
            let success = compile(&lines);
            // if succeeded, end the loop, otherwise revert the line to original value
            if success {
                break;
            }
            lines[num] = orig;
        }
    }

    Ok(())
}

fn get_instructions(lines: &Vec<String>, matches: Vec<&str>) -> HashMap<usize, String> {
    let mut results: HashMap<usize, String> = HashMap::new();

    for (num, line) in lines.iter().enumerate() {
        let (instr, _) = split(line.to_string());
        if matches.contains(&instr.as_str()) {
            results.insert(num, line.to_string());
        }
    }

    results
}

fn flip(line: String) -> String {
    let (instr, param) = split(line);
    match instr.as_str() {
        "jmp" => {
            return format!("nop {}", param);
        }
        "nop" => {
            return format!("jmp {}", param);
        }
        _ => {
            return format!("{} {}", instr, param);
        }
    }
}

fn split(line: String) -> (String, String) {
    let instr: String = line.chars().take(3).collect();
    let param: String = line.chars().skip(4).collect();

    (instr, param)
}

fn compile(lines: &Vec<String>) -> bool {
    let mut idx: usize = 0;
    let mut acc: i32 = 0;
    let mut line;
    let mut executed: HashMap<usize, String> = HashMap::new();
    while idx < lines.len() {
        line = lines[idx].to_string();
        if executed.contains_key(&idx) {
            println!("error: repeated instruction {} at ln {}", line, idx);
            break;
        }
        executed.insert(idx, line.to_string());
        let (instr, param) = split(line);
        match instr.as_str() {
            "acc" => {
                acc += param.parse::<i32>().unwrap();
                idx += 1;
            }
            "jmp" => {
                // param can be negative, so we need it signed. One extra
                // variable to make it readable
                let idx32 = idx as i32 + param.parse::<i32>().unwrap();
                // it yields an unsigned in the end
                idx = idx32 as usize;
            }
            _ => {
                idx += 1;
            }
        }
    }
    dbg!(acc);

    idx == lines.len()
}
