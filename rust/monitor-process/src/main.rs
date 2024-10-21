use monitor_process::get_cpu_and_memory;
use monitor_process::get_process_and_children;
use monitor_process::pages_to_megabytes;
use monitor_process::ticks_to_seconds;
use monitor_process::StdErr;
use std::cmp::max;
use std::process;
use std::env;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

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

    let processes = get_process_and_children(pid)?;

    println!(
        "monitoring {} processes with pids: {}",
        processes.len(),
        processes.iter().map(|p| p.pid.to_string()).collect::<Vec<_>>().join(", "),
    );

    let stop = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(
        signal_hook::consts::SIGINT,
        Arc::clone(&stop)
    )?;
    signal_hook::flag::register(
        signal_hook::consts::SIGTERM,
        Arc::clone(&stop)
    )?;
    if !is_quiet {
        println!("Duration (sec), CPU (sec), Memory (MB)");
    }
    let mut total_cpu_ticks = 0u64;
    let mut max_memory_pages = 0u64;
    let mut duration = 0;
    let (cpu_ticks, _) = get_cpu_and_memory(&processes);
    let start_cpu_ticks = cpu_ticks;
    let mut previous_cpu_ticks = cpu_ticks;
    sleep(Duration::from_secs(1));
    duration += 1;
    while !stop.load(Ordering::Relaxed) {
        let (cpu_ticks, memory_pages) = get_cpu_and_memory(&processes);
        total_cpu_ticks = cpu_ticks;
        max_memory_pages = max(max_memory_pages, memory_pages);
        if !is_quiet {
            println!(
                "{}, {:.1}, {:.1}",
                duration,
                ticks_to_seconds(cpu_ticks - previous_cpu_ticks),
                pages_to_megabytes(memory_pages),
            );
        }
        previous_cpu_ticks = cpu_ticks;
        sleep(Duration::from_secs(1));
        duration += 1;
    }
    println!("Total CPU (sec): {:.1}", ticks_to_seconds(total_cpu_ticks - start_cpu_ticks));
    println!("Max Memory (MB): {:.1}", pages_to_megabytes(max_memory_pages));
    Ok(())
} 