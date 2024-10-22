use procfs::{page_size, process::{all_processes, Process, Stat}, ticks_per_second};

use std::cmp::max;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

thread_local! {
    static TICKS_PER_SECOND: f64 = ticks_per_second() as f64;
    static PAGE_SIZE: f64 = page_size() as f64;
}

pub type StdErr = Box<dyn std::error::Error>;

pub fn get_process_and_children(pid: i32) -> Result<Vec<Process>, StdErr> {
    let parent = Process::new(pid)?;
    let mut everyone = vec![parent];
    let child_iter = all_processes()?
        .filter_map(|process| match process {
            Ok(process) => match process.stat() {
                Ok(stat) => if stat.ppid == pid { Some(process) } else { None },
                Err(_) => None,
            },
            Err(_) => None,
        });
    everyone.extend(child_iter);
    Ok(everyone)
}

pub fn get_cpu_and_memory(processes: &[Process]) -> (u64, u64) {
    let mut cpu_ticks = 0u64;
    let mut memory_pages = 0u64;
    for process in processes {
        let stat = match process.stat() {
            Ok(stat) => stat,
            Err(_) => continue,
        };
        cpu_ticks += get_cpu_ticks_from_stat(&stat);
        memory_pages += get_memory_pages_from_stat(&stat);
    }
    (cpu_ticks, memory_pages)
}

pub fn get_cpu_ticks_from_stat(stat: &Stat) -> u64 {
    // user-mode + kernel-mode + waiting for children
    stat.utime + stat.stime + stat.cutime as u64 + stat.cstime as u64
}

pub fn get_memory_pages_from_stat(stat: &Stat) -> u64 {
    stat.rss
}

pub fn ticks_to_seconds(ticks: u64) -> f64 {
    ticks as f64 / TICKS_PER_SECOND.with(|&val| val)
}

pub fn pages_to_megabytes(pages: u64) -> f64 {
    pages as f64 * PAGE_SIZE.with(|&val| val) / 1_000_000f64
}

pub fn monitor(pid: i32, is_quiet: bool) -> Result<(), StdErr> {
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
