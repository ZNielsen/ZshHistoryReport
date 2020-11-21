use std::time::{Duration, SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use std::fs::File;
use std::io::Read;

const HISTORY_FILE: &'static str = "/home/elemental/.zsh_history";

#[derive(Debug)]
struct TimeCmd {
    time: u64,
    cmd: String,
}

impl Eq for TimeCmd {}
impl std::cmp::Ord for TimeCmd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.time.cmp(&other.time) }
}
impl PartialOrd for TimeCmd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for TimeCmd {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

fn main() {
    let opt = Opt::from_args();

    // Parse args
    let days_to_parse = match opt.days_to_parse {
        Some(val) => val,
        None => 1,
    };

    // Set up the stop time
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("We reversed the flow of time");

    // Get file
    let mut file = File::open(HISTORY_FILE).expect("Can't open file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read in file");

    // Read from the bottom up (pull it all into memory)
    let lines = text.lines().rev();

    // Setup some initial stuff
    let mut timecmds: Vec<TimeCmd> = Vec::new();
    let mut max_cmd_len: u32 = 0;

    // Parse
    for line in lines {
        // Parse the info from this line
        let info: Vec<&str> = line.split(":").collect();
        let ran_at_str = info[1].trim();
        let ran_at: u64 = ran_at_str.parse().expect("Can't parse command's run at time");

        // Check if we should bail
        match now.checked_sub(Duration::from_secs(ran_at)) {
            Some(time) => {
                let days = time.as_secs() / (24 * 60 * 60);
                if days >= days_to_parse {
                    break;
                }
            }
            None => println!("Duration areithmatic problems"),
        };

        let info = info[2];
        let time_cmd: Vec<&str> = info.split(";").collect();
        let runtime: u64 = time_cmd[0].parse().expect("Could not parse runtime");
        let runcmd = time_cmd[1];
        max_cmd_len = std::cmp::max(max_cmd_len, runcmd.len() as u32);
        if runtime != 0 {
            timecmds.push( TimeCmd {
                time: runtime,
                cmd: runcmd.to_owned(),
            });
        }
    }

    // Sort and display
    timecmds.sort_by(|b, a| a.partial_cmp(b).unwrap());
    println!("Command Time Report:");
    for timecmd in timecmds {
        let time = chrono::Duration::seconds(timecmd.time as i64);

        // Right pad command
        print!("\t{}",timecmd.cmd);
        let this_pad_amount = max_cmd_len as usize- timecmd.cmd.len();
        for _ in 0..this_pad_amount {
            print!(".");
        }
        print!(": ");
        if time.num_minutes() > 0 {
            print!("{:02} min, {:02} sec", time.num_minutes(), time.num_seconds()/60);
        }
        else {
            print!("        {:02} sec", time.num_seconds()/60);
        }
        println!();
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "HistoryReport",
    about = "Information about how long your commands take"
)]
struct Opt {
    /// Days to parse
    days_to_parse: Option<u64>
}
