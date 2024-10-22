fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target != "linux" {
        println!("cargo:warning=monitor-process can only be built for Linux targets. However it's an optional dependency that is only used to monitor CPU and RAM usage during benchmarking, you don't need it to run any of the servers!");
        std::process::exit(0);
    }
}
