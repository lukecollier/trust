use crate::runner::Runner;
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
            Runner::from(parsed).run();
            println!("using config {}", path.as_path().display());
        },
        None => panic!("Could not get home directory"),
    }
}
