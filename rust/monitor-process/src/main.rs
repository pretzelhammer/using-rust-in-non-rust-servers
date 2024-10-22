
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as monitor;

#[cfg(not(target_os = "linux"))]
mod fallback;
#[cfg(not(target_os = "linux"))]
use fallback as monitor;

use std::process;
use std::env;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    let mut args = env::args();
    let args_len = args.len();

    if args_len < 2 || args_len > 3 {
        eprintln!("Usage: monitor-process {{pid}} {{--quiet}}");
        process::exit(1);
    }

    let pid = match args.nth(1).unwrap().parse::<i32>() {
        Ok(pid) => pid,
        Err(_) => {
            eprintln!("pid must be an integer");
            process::exit(1);
        }
    };

    let mut is_quiet = false;
    if args_len == 3 {
        if args.nth(0).unwrap() != "--quiet" {
            eprintln!("only supported optional flag is --quiet");
            process::exit(1);
        } else {
            is_quiet = true;
        }
    }

    monitor::monitor(pid, is_quiet)
}
