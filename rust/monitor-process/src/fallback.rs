pub type StdErr = Box<dyn std::error::Error>;

pub fn monitor(_pid: i32, _is_quiet: bool) -> Result<(), StdErr> {
    eprintln!("monitor-process CLI tool only works on linux");
    Ok(())
}
