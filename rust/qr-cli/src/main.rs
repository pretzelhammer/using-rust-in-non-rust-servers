use std::{env, process};
use std::io::{self, BufWriter, Write};
use qr_lib::StdErr;

fn main() -> Result<(), StdErr> {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Usage: qr_cli <text>");
        process::exit(1);
    }

    let text = args.nth(1).unwrap();
    let qr_png = qr_lib::generate_qr_code(&text)?;

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());
    handle.write_all(&qr_png)?;

    Ok(())
}
