use trust_core::runner::TmuxIO;
use trust_core::tmux::{Target, has_session};
use trust_core::parser::{Parser};
use dirs::{home_dir};

use structopt::StructOpt;
use std::path::PathBuf;

/// Your trusty session manager
#[derive(StructOpt, Debug)]
#[structopt(name = "trust")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Path to config file
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: Option<PathBuf>,

    /// Session name to attach too
    #[structopt(name = "SESSION")]
    session_name: String,
}

fn main() {
    let opt = Opt::from_args();
    if opt.config.is_none() {
        match home_dir() {
            Some(mut path) => {
                path.push(".config");
                path.push("trust");
                path.push("sessions");
                path.set_extension("xml");
                let parsed = Parser::from_file(path.as_path());
                for session in parsed {
                    if !has_session(&session.name) 
                        && &session.name == &opt.session_name {
                        session.unsafe_run(&mut Target::new()).unwrap();
                    }
                }
                if opt.debug {
                    println!("[DEBUG] using config {}", path.as_path().display());
                }
            },
            None => panic!("Could not get home directory"),
        }
    } else {
        let path_buf = opt.config.unwrap();
        let path = path_buf.as_path();
        if path.exists() {
            let parsed = Parser::from_file(path);
            for session in parsed {
                if !has_session(&session.name) && &session.name == &opt.session_name {
                    session.unsafe_run(&mut Target::new()).unwrap();
                }
            }
            if opt.debug {
                println!("[DEBUG] using config {}", &path.display());
            }
        } else {
            panic!("Could not find config file");
        }
    }
}
