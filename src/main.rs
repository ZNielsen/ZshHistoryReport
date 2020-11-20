use structopt::StructOpt;
use std::fs::File;
use chrono::Duration;
use std::time::SystemTime;

const HISTORY_FILE: &'static str = "/home/elemental/.zsh_history"

fn main() {
    let opt = Opt::from_args();

    let days_to_parse = match opt.days_to_parse {
        Some(val) => val,
        None => 1,
    };

    // Get file
    let file = File::open(HISTORY_FILE);
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read file");

    let lines = text.lines().rev()

    let mut last_time = SystemTime::now();
    for line in lines {
        // Parse the info from this line

    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "HistoryReport",
    about = "Information about how long your commands take"
)]
struct Opt {
    /// Days to parse
    days_to_parse: Option<u32>
}
