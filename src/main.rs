mod io;
mod one;
mod three;
mod two;
use std::num::ParseIntError;
use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
#[structopt(about = "Advent of Code 2020")]
enum App {
    #[structopt(name = "1", about = "Runs the first day's exercise(s)")]
    One {
        #[structopt(parse(from_os_str))]
        #[structopt(default_value = "data/expense-report.txt")]
        #[structopt(help = "Path to the expense report")]
        path: PathBuf,
    },
    #[structopt(name = "2", about = "Runs the second day's exercise(s)")]
    Two {
        #[structopt(parse(from_os_str))]
        #[structopt(default_value = "data/password-policies.txt")]
        #[structopt(help = "Path to the file containing the passwords and policies")]
        path: PathBuf,
    },
    #[structopt(name = "3", about = "Runs the third day's exercise(s)")]
    Three {
        #[structopt(parse(from_os_str))]
        #[structopt(default_value = "data/tree-map.txt")]
        #[structopt(help = "Path to the file containing the tree map")]
        path: PathBuf,
    },
}

fn main() -> Result<(), ParseIntError> {
    let args = App::from_args();
    return match args {
        App::One { path } => one::cmd(path),
        App::Two { path } => two::cmd(path),
        App::Three { path } => three::cmd(path),
    };
}
