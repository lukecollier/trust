use std::path::Path;

mod parser;
mod data;
mod tmux;

fn main() {
    let path = Path::new("./src/resources/test.xml");
    println!("{:?}", parser::Parser::from_file(path));
}
