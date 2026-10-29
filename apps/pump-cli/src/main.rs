//! `pump-cli`: command-line front end for the pump performance-test tools.

use std::process::ExitCode;

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("--version" | "-V") => {
            println!("pump-cli {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: pump-cli --version  (commands arrive in 004_pump-cli)");
            ExitCode::from(2)
        }
    }
}
