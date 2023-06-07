use dirs::home_dir;
use std::path::PathBuf;

use clap::Parser;

/// Your trusty session manager
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Activate debug mode
    #[arg(short, long = "debug")]
    debug: bool,

    /// Path to config file
    #[arg(short, long = "config")]
    config: Option<PathBuf>,
}

fn main() {
    let opt = Cli::parse();
    if opt.config.is_none() {
        match home_dir() {
            Some(mut path) => {
                path.push(".config");
                path.push("trust");
                path.push("sessions");
                path.set_extension("xml");
                if opt.debug {
                    println!("[DEBUG] using config {}", path.as_path().display());
                }
            }
            None => panic!("Could not get home directory"),
        }
    } else {
        let path_buf = opt.config.unwrap();
        let path = path_buf.as_path();
        if path.exists() {
            let parsed = core::parser::Parser::from_file(path);
            // todo: run set up
            if opt.debug {
                println!("[DEBUG] using config {}", &path.display());
            }
        } else {
            panic!("Could not find config file");
        }
    }
}
