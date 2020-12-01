mod one;
use std::num::ParseIntError;
use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
#[structopt(about = "Advent of Code 2020")]
enum App {
    #[structopt(about = "Runs the first exercise")]
    One {
        #[structopt(parse(from_os_str))]
        #[structopt(default_value = "report.txt")]
        #[structopt(help = "Path to the expense report")]
        path: PathBuf,
    },
}

fn main() -> Result<(), ParseIntError> {
    let args = App::from_args();
    return match args {
        App::One { path } => one::cmd(path),
    };
}
