use std::path::Path;
use crate::runner::Runner;

mod parser;
mod data;
mod tmux;
mod runner;

fn main() {
    let path = Path::new("./src/resources/test.xml");
    let parsed = parser::Parser::from_file(path);
    Runner::from(parsed).run();
    println!("{:?}", parser::Parser::from_file(path));
}
