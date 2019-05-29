use crate::runner::TmuxIO;
use crate::tmux::*;
use dirs;

mod tmux;
mod parser;
mod data;
mod runner;

fn main() {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config");
            path.push("trust");
            path.push("sessions");
            path.set_extension("xml");
            let parsed = parser::Parser::from_file(path.as_path());
            for session in parsed {
                if !has_session(&session.name) {
                    session.unsafe_run(&mut Target::new()).unwrap();
                }
            }
            println!("using config {}", path.as_path().display());
        },
        None => panic!("Could not get home directory"),
    }
}
