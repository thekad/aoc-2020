use std::num::ParseIntError;
use std::path::PathBuf;

pub fn cmd(path: PathBuf) -> Result<(), ParseIntError> {
    let mut executed: Vec<i32> = Vec::new();

    if let Ok(lines) = crate::io::lines(path) {
        let mut idx: i32 = 0;
        let mut acc: i32 = 0;
        let mut line;
        while idx < lines.len() as i32 {
            line = lines[idx as usize].as_str();
            if executed.contains(&idx) {
                println!("error: repeated instruction {} at ln {}", line, idx);
                break;
            }
            executed.push(idx);
            let inst: &str = line.split(" ").collect::<Vec<&str>>()[0];
            let para: &str = line.split(" ").collect::<Vec<&str>>()[1];
            print!("ln {}: ", idx);
            match inst {
                "acc" => {
                    println!("acc, altering acc by {}", para);
                    acc += para.parse::<i32>().unwrap();
                    idx += 1;
                }
                "jmp" => {
                    println!("jmp, jumping {} lines", para);
                    idx += para.parse::<i32>().unwrap();
                }
                _ => {
                    println!("nop, moving to next line");
                    idx += 1;
                }
            }
        }
        dbg!(acc);
    }

    Ok(())
}
